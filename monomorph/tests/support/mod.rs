//! Shared helpers for `tests/dispatch_examples.rs`.
//!
//! Lives at `tests/support/mod.rs` (not `tests/support.rs`) so Cargo treats it
//! as a plain module rather than its own integration-test binary.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Copy, Clone, Debug)]
pub enum Expectation {
    /// Must compile and match tests/golden/<name>.json exactly.
    Passing,
    /// Must currently fail to compile/analyze. Fails loudly if it starts
    /// succeeding, so a fix gets noticed and the fixture gets promoted.
    KnownBroken,
    /// Not yet characterized: run it, print the normalized result, assert
    /// nothing.
    Unclassified,
}

/// One call-site's dispatch results, normalized so that volatile internal ids
/// (DefId numbers, Span ids, GenericArgs debug reprs) don't cause false-positive
/// diffs across compiler sessions. Only the human-meaningful parts are kept:
/// the source span text and the sorted set of candidate function names.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DispatchSite {
    /// e.g. "src/main.rs:25:5: 25:19"
    pub span: String,
    /// true if this call site appeared under "--MAYBE EXAMPLES--" (i.e. FSA
    /// found fewer targets than CHA), false if under "--NOT EXAMPLES--".
    pub is_maybe_example: bool,
    /// Sorted, fully-qualified candidate function names per CHA.
    pub cha: Vec<String>,
    /// Sorted, fully-qualified candidate function names per FSA.
    pub fsa: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExampleResult {
    pub maybe_count: usize,
    pub not_count: usize,
    pub sites: Vec<DispatchSite>,
}

/// Parse a `stats` file (see `monomorph::logger::VOLogger::log_stats`) into a
/// normalized, comparable form.
pub fn parse_stats(text: &str) -> ExampleResult {
    let mut maybe_count = 0;
    let mut not_count = 0;
    let mut in_maybe_section = false;
    let mut sites = Vec::new();

    let mut lines = text.lines();
    while let Some(raw_line) = lines.next() {
        let line = raw_line.trim();
        if let Some(rest) = line.strip_prefix("Num maybe examples = ") {
            maybe_count = rest.trim().parse().unwrap_or(0);
        } else if let Some(rest) = line.strip_prefix("Num non-examples = ") {
            not_count = rest.trim().parse().unwrap_or(0);
        } else if line == "--MAYBE EXAMPLES--" {
            in_maybe_section = true;
        } else if line == "--NOT EXAMPLES--" {
            in_maybe_section = false;
        } else if let Some(rest) = line.strip_prefix("Span:") {
            let span = extract_quoted_field(rest, "repr: \"").unwrap_or_else(|| rest.trim().to_string());
            let cha_line = lines.next().unwrap_or("");
            let fsa_line = lines.next().unwrap_or("");
            let mut cha = extract_all_quoted_fields(cha_line, "name: \"");
            let mut fsa = extract_all_quoted_fields(fsa_line, "name: \"");
            cha.sort();
            fsa.sort();
            sites.push(DispatchSite {
                span,
                is_maybe_example: in_maybe_section,
                cha,
                fsa,
            });
        }
    }

    sites.sort_by(|a, b| a.span.cmp(&b.span));
    ExampleResult {
        maybe_count,
        not_count,
        sites,
    }
}

fn extract_quoted_field(s: &str, marker: &str) -> Option<String> {
    let idx = s.find(marker)?;
    let rest = &s[idx + marker.len()..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn extract_all_quoted_fields(s: &str, marker: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut rest = s;
    while let Some(idx) = rest.find(marker) {
        rest = &rest[idx + marker.len()..];
        match rest.find('"') {
            Some(end) => {
                names.push(rest[..end].to_string());
                rest = &rest[end + 1..];
            }
            None => break,
        }
    }
    names
}

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn example_dir(name: &str) -> PathBuf {
    manifest_dir().join("../testing_examples").join(name)
}

fn golden_path(name: &str) -> PathBuf {
    manifest_dir().join("tests/golden").join(format!("{name}.json"))
}

/// Best-effort sysroot lookup so the child process can find `librustc_driver.so`
/// (see monomorph/README.md's Troubleshooting section) without every
/// developer having to export LD_LIBRARY_PATH by hand before running tests.
fn ld_library_path_with_sysroot() -> String {
    let sysroot = Command::new("rustc")
        .args(["--print", "sysroot"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    let lib_dir = format!("{sysroot}/lib");
    match std::env::var("LD_LIBRARY_PATH") {
        Ok(existing) if !existing.is_empty() => format!("{lib_dir}:{existing}"),
        _ => lib_dir,
    }
}

struct RunOutcome {
    success: bool,
    stdout: String,
    stderr: String,
    stats: Option<String>,
}

fn run_verifopt(dir: &Path) -> RunOutcome {
    // `stats`/`found_ex`/`notfound_ex` are opened in append mode by VOLogger,
    // so stale files from a previous run would silently corrupt this run's
    // parsed output. Clear them first.
    for f in ["stats", "found_ex", "notfound_ex"] {
        let _ = fs::remove_file(dir.join(f));
    }
    let _ = Command::new("cargo").arg("clean").current_dir(dir).output();

    let bin = PathBuf::from(env!("CARGO_BIN_EXE_cargo-verifopt"));
    let output = Command::new(&bin)
        .arg("verifopt")
        .arg("--release")
        .current_dir(dir)
        .env("LD_LIBRARY_PATH", ld_library_path_with_sysroot())
        .output()
        .unwrap_or_else(|e| {
            panic!(
                "failed to spawn {:?} in {:?}: {e}\n\
                 (is the pinned nightly toolchain from rust-toolchain.toml active?)",
                bin, dir
            )
        });

    let stats = fs::read_to_string(dir.join("stats")).ok();

    RunOutcome {
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        stats,
    }
}

pub fn run_example(name: &str, expectation: Expectation) {
    let dir = example_dir(name);
    assert!(
        dir.exists(),
        "fixture directory {:?} does not exist; is testing_examples/{name} still there?",
        dir
    );

    let outcome = run_verifopt(&dir);

    match expectation {
        Expectation::KnownBroken => {
            assert!(
                !outcome.success,
                "'{name}' is marked KnownBroken in dispatch_examples.rs but it just \
                 compiled/ran successfully!\n\
                 If this is a genuine fix: change its expectation to `Passing` in \
                 dispatch_examples.rs and generate a golden file with:\n\
                 \n    BLESS_GOLDEN=1 cargo test --test dispatch_examples -- {name}\n\n\
                 stdout:\n{}\nstderr:\n{}",
                outcome.stdout, outcome.stderr
            );
        }
        Expectation::Unclassified => {
            if outcome.success {
                let stats = outcome.stats.unwrap_or_default();
                let result = parse_stats(&stats);
                eprintln!(
                    "[{name}] compiled successfully; not yet checked against a golden file.\n\
                     Parsed result:\n{}\n\
                     Once you've confirmed this looks right, move this fixture to `Passing` \
                     in dispatch_examples.rs and run:\n\n    \
                     BLESS_GOLDEN=1 cargo test --test dispatch_examples -- {name}\n",
                    serde_json::to_string_pretty(&result).unwrap()
                );
            } else {
                eprintln!(
                    "[{name}] did not compile/run successfully. stderr:\n{}",
                    outcome.stderr
                );
            }
            // Intentionally no assertion: this fixture hasn't been classified yet.
        }
        Expectation::Passing => {
            assert!(
                outcome.success,
                "'{name}' is marked Passing but failed to compile/run.\nstdout:\n{}\nstderr:\n{}",
                outcome.stdout, outcome.stderr
            );

            let stats = outcome.stats.unwrap_or_else(|| {
                panic!("'{name}' ran successfully but produced no `stats` file in {dir:?}")
            });
            let actual = parse_stats(&stats);

            let golden_file = golden_path(name);
            if std::env::var("BLESS_GOLDEN").is_ok() {
                if let Some(parent) = golden_file.parent() {
                    fs::create_dir_all(parent).expect("failed to create tests/golden");
                }
                fs::write(
                    &golden_file,
                    serde_json::to_string_pretty(&actual).unwrap() + "\n",
                )
                .unwrap_or_else(|e| panic!("failed to write {:?}: {e}", golden_file));
                eprintln!("[{name}] wrote golden file {:?}", golden_file);
                return;
            }

            let expected_text = fs::read_to_string(&golden_file).unwrap_or_else(|e| {
                panic!(
                    "'{name}' has no golden file at {:?} ({e}).\n\
                     Generate one with:\n\n    BLESS_GOLDEN=1 cargo test --test dispatch_examples -- {name}\n\n\
                     then review it before committing.",
                    golden_file
                )
            });
            let expected: ExampleResult = serde_json::from_str(&expected_text).unwrap_or_else(|e| {
                panic!("failed to parse golden file {:?}: {e}", golden_file)
            });

            assert_eq!(
                actual, expected,
                "'{name}' dispatch results changed. If this is an intended \
                 improvement/fix, review the diff above, then re-bless with:\n\n    \
                 BLESS_GOLDEN=1 cargo test --test dispatch_examples -- {name}"
            );
        }
    }
}
