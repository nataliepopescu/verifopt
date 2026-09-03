// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.
//
// Modified by <Natalie Popescu>.

//! This provides an implementation for the "cargo verifopt" subcommand.
//!
//! The subcommand is the same as "cargo build" but with three differences:
//! 1) It implicitly adds the options "-Z always_encode_mir" to the rustc invocation.
//! 2) It calls `verifopt` rather than `rustc` for all the targets of the current package.
//! 3) It runs `cargo test --no-run` for test targets.
//!
//! Every build's own whole-program reachability analysis can find
//! devirtualization opportunities *inside* dependency crates' own code
//! (which have no entry point of their own to analyze from directly) -
//! when it does, a second, `cargo clean`-then-rebuild pass runs
//! automatically to apply what the first pass found, since a
//! dependency crate's own compilation never sees the primary crate's
//! whole-program findings otherwise. This costs nothing beyond the
//! ordinary, single build when nothing was found outside the primary
//! crate, which is the common case. See rewrite.rs's own
//! dep_rewrite_store_path/needs_rewrite_pass_marker_path docs for the
//! underlying mechanism.

#![feature(rustc_private)]

extern crate rustc_driver;

use cargo_metadata::{Package, TargetKind};
use log::info;
use serde_json;
use std::env;
use std::ffi::OsString;
use std::ops::Index;
use std::path::Path;
use std::process::{Command, Stdio};

use monomorph::util;

/// The help message for `cargo-verifopt`
const CARGO_VERIFOPT_HELP: &str = r#"Flow-sensitive analysis tool for Rust programs
Usage:
    cargo verifopt [--rewrite-only]

    --rewrite-only  Skip analysis and the automatic discovery-then-decide
                     flow entirely - just re-run the rewrite pass against
                     whatever verifopt_store.json already exists on disk
                     from an earlier, successful run. No cargo clean, so
                     cargo's own caching applies as normal. For iterating
                     on the rewrite logic itself without re-paying for
                     analysis or a full dependency rebuild each time.
"#;

/// Set the environment variable `VERIFOPT_BUILD_STD` to enable the building of std library when running verifopt.
const VERIFOPT_BUILD_STD: &str = "VERIFOPT_BUILD_STD";

pub fn main() {
    if std::env::args()
        .take_while(|a| a != "--")
        .any(|a| a == "--help" || a == "-h")
    {
        println!("{}", CARGO_VERIFOPT_HELP);
        return;
    }

    match std::env::args().nth(1).as_ref().map(AsRef::<str>::as_ref) {
        Some(s) if s.ends_with("verifopt") => {
            // Get here for the top level cargo execution, i.e. "cargo verifopt".
            call_cargo();
        }
        Some(s) if s.ends_with("rustc") => {
            // 'cargo rustc ..' redirects here because RUSTC_WRAPPER points to this binary.
            // execute rustc with VerifOpt applicable parameters for dependencies and call VerifOpt
            // to analyze targets in the current package.
            call_rustc_or_verifopt();
        }
        Some(arg) => {
            eprintln!(
                "`cargo-verifopt` called with invalid first argument: {arg}; please only invoke this binary through `cargo verifopt`"
            );
        }
        _ => {
            eprintln!("current args: {:?}", std::env::args());
            eprintln!(
                "`cargo-verifopt` called without first argument; please only invoke this binary through `cargo verifopt`"
            );
        }
    }
}

/// Read the toml associated with the current directory and
/// recursively execute cargo for each applicable package target/workspace member in the toml
fn call_cargo() {
    let manifest_path =
        get_arg_flag_value("--manifest-path").map(|m| Path::new(&m).canonicalize().unwrap());

    let mut cmd = cargo_metadata::MetadataCommand::new();
    if let Some(ref manifest_path) = manifest_path {
        cmd.manifest_path(manifest_path);
    }

    let metadata = if let Ok(metadata) = cmd.exec() {
        metadata
    } else {
        eprintln!("Could not obtain Cargo metadata; likely an ill-formed manifest");
        std::process::exit(1);
    };

    // If a binary is specified, analyze this binary only.
    if let Some(target) = get_arg_flag_value("--bin") {
        call_cargo_on_target(&target, &TargetKind::Bin);
        return;
    }

    if let Some(root) = metadata.root_package() {
        call_cargo_on_each_package_target(root);
        return;
    }

    // There is no root, this must be a workspace, so call_cargo_on_each_package_target on each workspace member
    for package_id in &metadata.workspace_members {
        let package = metadata.index(package_id);
        call_cargo_on_each_package_target(package);
    }
}

fn call_cargo_on_each_package_target(package: &Package) {
    let lib_only = has_arg_flag("--lib");
    for target in &package.targets {
        let kind = target
            .kind
            .first()
            .expect("bad cargo metadata: target::kind");
        if lib_only && *kind != TargetKind::Lib {
            continue;
        }
        call_cargo_on_target(&target.name, kind);
    }
}

/// Resolves a binary (`cargo`, `rustc`, ...) belonging to the exact
/// toolchain that this build of `verifopt` was compiled against, via
/// `rustup which`.
///
/// `verifopt` links `rustc_driver` directly (`#![feature(rustc_private)]`),
/// which has no stable ABI across nightlies. Trusting an ambient `$CARGO`/
/// `$RUSTC`/`$PATH` (e.g. a project-local fenix/rustup toolchain override)
/// risks driving the build with a *different* cargo/rustc than the one
/// `verifopt` was built against, producing flag mismatches (e.g. a flag
/// that is stable in the ambient toolchain but still unstable in
/// verifopt's pinned one).
///
/// Note: cargo does NOT, by default, resolve and pass an absolute rustc
/// path to RUSTC_WRAPPER — it hands the wrapper the bare string "rustc"
/// and relies on `$PATH` (this is what rustup's own shim mechanism
/// depends on). Since we invoke the pinned toolchain's `cargo` directly
/// rather than through rustup's shim, nothing corrects that PATH lookup
/// for us — so both `cargo` and `rustc` must be resolved explicitly here,
/// and `rustc` must additionally be threaded through as the `RUSTC` env
/// var on the child `cargo build` invocation (see `call_cargo_on_target`)
/// so cargo doesn't fall back to a bare, PATH-resolved "rustc" itself.
fn pinned_toolchain_bin(name: &str) -> OsString {
    let toolchain = option_env!("RUSTUP_TOOLCHAIN").expect(
        "verifopt must be built under rustup with a pinned toolchain \
         (RUSTUP_TOOLCHAIN was not set at compile time)",
    );

    let output = Command::new("rustup")
        .args(["which", "--toolchain", toolchain, name])
        .output()
        .unwrap_or_else(|e| panic!("could not invoke `rustup which` to resolve {name}: {e}"));

    if !output.status.success() {
        panic!(
            "`rustup which --toolchain {toolchain} {name}` failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let path = String::from_utf8(output.stdout)
        .unwrap_or_else(|_| panic!("`rustup which {name}` produced non-UTF-8 output"))
        .trim()
        .to_owned();

    OsString::from(path)
}

fn call_cargo_on_target(target: &String, kind: &TargetKind) {
    // Debugging escape hatch: re-run just the rewrite pass against
    // whatever verifopt_store.json already exists on disk (from an
    // earlier, successful discovery run), skipping analysis
    // (--rewrite-pass) and skipping this function's own automatic
    // discovery-then-decide flow entirely - no cargo clean, so cargo's
    // own caching does whatever it would normally do (e.g. leaving
    // already-successfully-built dependencies alone, retrying only
    // whatever previously failed). Useful for iterating on the rewrite
    // logic itself without re-paying for analysis or a full dependency
    // rebuild each time. Stripped here, before anything else looks at
    // the arg list, matching how --lib is already skipped in
    // run_cargo_build for the same reason.
    if has_arg_flag("--rewrite-only") {
        run_cargo_build(target, kind, &["--rewrite-pass".to_owned()]);
        return;
    }

    // This first build *is* the ordinary, single-pass build - nothing
    // extra is paid here regardless of what it finds, since
    // RewriteCallbacks already rewrites this crate's own code within
    // this same pass either way. It's also the discovery pass for the
    // two-pass dependency-rewrite flow: FsaCallbacks's own analysis
    // (see rewrite.rs's after_analysis) writes a small marker file,
    // but only when it actually found a dispatch site whose containing
    // function lives outside this crate - i.e. only when there's
    // something a second pass would need to act on.
    run_cargo_build(target, kind, &[]);

    if !std::path::Path::new(monomorph::rewrite::needs_rewrite_pass_marker_path()).exists() {
        return;
    }

    info!(
        "found a dispatch site inside dependency code during the discovery pass - \
         cleaning and rebuilding once more to apply it (see rewrite.rs's own \
         dep_rewrite_store_path/needs_rewrite_pass_marker_path docs)"
    );
    let mut clean_cmd = Command::new(pinned_toolchain_bin("cargo"));
    clean_cmd.arg("clean");
    let clean_status = clean_cmd
        .spawn()
        .expect("could not run cargo clean")
        .wait()
        .expect("failed to wait for cargo clean");
    if !clean_status.success() {
        std::process::exit(clean_status.code().unwrap_or(-1));
    }

    run_cargo_build(target, kind, &["--rewrite-pass".to_owned()]);
}

/// Builds and runs the actual `cargo build`/`cargo test` invocation.
/// Factored out of `call_cargo_on_target` so it can be called either
/// once (the ordinary case - this is the whole of what
/// `call_cargo_on_target` used to do directly) or twice, with a
/// `cargo clean` in between, when the first (discovery) pass's own
/// analysis found a dispatch site living outside the primary crate: a
/// dependency crate has no entry point of its own to analyze from, so
/// devirtualizing dispatch sites *inside* dependency code needs a
/// separate discovery pass (the primary crate's own whole-program
/// reachability analysis, run first) whose findings get persisted to
/// disk and read back by a second, --rewrite-pass build of the whole
/// graph - see rewrite.rs's own dep_rewrite_store_path doc for the
/// full mechanism this drives.
///
/// `extra_verifopt_flags` is appended into VERIFOPT_FLAGS alongside
/// whatever the user already passed after `--` - this is how the
/// second pass's own `--rewrite-pass` gets threaded through without
/// disturbing the ordinary, single-pass call site.
fn run_cargo_build(target: &String, kind: &TargetKind, extra_verifopt_flags: &[String]) {
    // Build a cargo command for target. Always use the cargo binary paired
    // with the toolchain verifopt itself was built against (see
    // `pinned_toolchain_bin`), rather than an ambient `$CARGO`/`$PATH`
    // cargo that may belong to a different, incompatible nightly.
    let mut cmd = Command::new(pinned_toolchain_bin("cargo"));
    // Cargo's own default rustc resolution is just the bare string
    // "rustc" via $PATH — it does not hand RUSTC_WRAPPER an absolute
    // path unless told to. Set RUSTC explicitly so cargo (and, in turn,
    // whatever it passes to our own RUSTC_WRAPPER dispatch) stays pinned
    // to the same toolchain as the cargo binary above, regardless of
    // what else is first on the caller's $PATH.
    cmd.env("RUSTC", pinned_toolchain_bin("rustc"));
    match kind {
        TargetKind::Bin => {
            cmd.arg("build");
            if get_arg_flag_value("--bin").is_none() {
                cmd.arg("--bin").arg(target);
            }
        }
        TargetKind::Lib => {
            cmd.arg("build");
            cmd.arg("--lib");
        }
        TargetKind::Test => {
            cmd.arg("test");
            cmd.arg("--no-run");
        }
        _ => {
            return;
        }
    }
    cmd.arg("--verbose");

    let mut args = std::env::args().skip(2);
    // Add cargo args to cmd until first `--`.
    for arg in args.by_ref() {
        if arg == "--" {
            break;
        }
        if arg == "--lib" || arg == "--rewrite-only" {
            continue;
        }
        cmd.arg(arg);
    }

    // Enable Cargo to compile the standard library from source code as part of a crate graph compilation.
    if env::var(VERIFOPT_BUILD_STD).is_ok() {
        cmd.arg("-Zbuild-std");

        if !has_arg_flag("--target") {
            let toolchain_target = toolchain_target().expect("could not get toolchain target");
            cmd.arg("--target").arg(toolchain_target);
        }
    }

    // Serialize the remaining args into an environment variable.
    let mut args_vec: Vec<String> = args.collect();
    args_vec.extend(extra_verifopt_flags.iter().cloned());
    if !args_vec.is_empty() {
        cmd.env(
            "VERIFOPT_FLAGS",
            serde_json::to_string(&args_vec).expect("failed to serialize args"),
        );
    }

    // Force cargo to recompile all dependencies with VerifOpt friendly flags
    //cmd.env("RUSTFLAGS", "-Z always_encode_mir");
    let mut rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();
    if !rustflags.is_empty() {
        rustflags.push(' ');
    }
    rustflags.push_str("-Z always_encode_mir");
    //rustflags.push_str(" -C save-temps");
    cmd.env("RUSTFLAGS", rustflags);

    // Replace the rustc executable through RUSTC_WRAPPER environment variable so that rustc
    // calls generated by cargo come back to cargo-verifopt.
    let path = std::env::current_exe().expect("current executable path invalid");
    cmd.env("RUSTC_WRAPPER", path);

    // Communicate the name of the root crate to the calls to cargo-verifopt that are invoked via
    // the RUSTC_WRAPPER setting.
    cmd.env("VERIFOPT_CRATE", target.replace('-', "_"));

    // Communicate the target kind of the root crate to the calls to cargo-verifopt that are invoked via
    // the RUSTC_WRAPPER setting.
    cmd.env("VERIFOPT_TARGET_KIND", kind.to_string());

    // Belt-and-suspenders: `pinned_cargo_path()` above already invokes the
    // exact toolchain binary directly (not a rustup shim), so this env var
    // shouldn't be load-bearing anymore. Kept in case any child process in
    // the build graph re-resolves `cargo`/`rustc` via a rustup proxy rather
    // than inheriting the binary we already selected.
    if let Some(toolchain) = option_env!("RUSTUP_TOOLCHAIN") {
        cmd.env("RUSTUP_TOOLCHAIN", toolchain);
    }

    // Execute cmd
    info!("cmd: {:?}", cmd);
    let exit_status = cmd
        .spawn()
        .expect("could not run cargo")
        .wait()
        .expect("failed to wait for cargo");

    if !exit_status.success() {
        std::process::exit(exit_status.code().unwrap_or(-1))
    }
}

fn call_rustc_or_verifopt() {
    if let Some(crate_name) = get_arg_flag_value("--crate-name") {
        if let Ok(verifopt_crate) = std::env::var("VERIFOPT_CRATE") {
            if crate_name.eq(&verifopt_crate) {
                if let Ok(kind) = std::env::var("VERIFOPT_TARGET_KIND") {
                    if let Some(t) = get_arg_flag_value("--crate-type") {
                        if kind.eq(&t) {
                            call_verifopt();
                            return;
                        }
                    } else if kind == "test" {
                        call_verifopt();
                        return;
                    }
                }
            }
        }
    }
    call_rustc()
}

fn call_verifopt() {
    let mut path = std::env::current_exe().expect("current executable path invalid");
    let extension = path.extension().map(|e| e.to_owned());
    path.pop(); // remove the cargo_verifopt bit
    path.push("verifopt");
    if let Some(ext) = extension {
        path.set_extension(ext);
    }
    let mut cmd = Command::new(path);
    cmd.args(std::env::args().skip(2));
    let exit_status = cmd
        .spawn()
        .expect("could not run verifopt")
        .wait()
        .expect("failed to wait for verifopt");

    if !exit_status.success() {
        std::process::exit(exit_status.code().unwrap_or(-1))
    }
}

fn call_rustc() {
    // NOTE: cargo's RUSTC_WRAPPER protocol hands us argv[1] as "the rustc
    // to run" — but cargo's *default* resolution for that is just the
    // bare string "rustc" (verified empirically: cargo -v output showed
    // literally `cargo-verifopt rustc --crate-name ...`, not an absolute
    // path). Cargo relies on `$PATH` to resolve that, which is exactly
    // what rustup's own shim mechanism is designed to intercept — but we
    // invoke the pinned toolchain's cargo directly (bypassing rustup's
    // shim), so nothing corrects that PATH lookup anymore. Trusting
    // argv[1] here previously reproduced the *same* wrong-compiler bug
    // as the original `$RUSTC`/`$PATH` fallback it replaced. Resolve the
    // pinned toolchain's rustc explicitly instead, exactly as
    // `call_cargo_on_target` resolves cargo.
    let mut cmd = Command::new(pinned_toolchain_bin("rustc"));
    cmd.args(std::env::args().skip(2));
    let exit_status = cmd
        .spawn()
        .expect("could not run rustc")
        .wait()
        .expect("failed to wait for rustc");

    if !exit_status.success() {
        std::process::exit(exit_status.code().unwrap_or(-1))
    }
}

/// Determines whether a flag `name` is present before `--`.
/// For example, has_arg_flag("-v")
fn has_arg_flag(name: &str) -> bool {
    let mut args = std::env::args().take_while(|val| val != "--");
    args.any(|val| val == name)
}

/// Gets the value of `name`.
/// `--name value` or `--name=value`
fn get_arg_flag_value(name: &str) -> Option<String> {
    let mut args = std::env::args().take_while(|val| val != "--");
    loop {
        let arg = match args.next() {
            Some(arg) => arg,
            None => return None,
        };
        if !arg.starts_with(name) {
            continue;
        }
        // Strip `name`.
        let suffix = &arg[name.len()..];
        if suffix.is_empty() {
            // This argument is `name` and the next one is the value.
            return args.next();
        } else if let Some(arg_value) = suffix.strip_prefix('=') {
            return Some(arg_value.to_owned());
        }
    }
}

/// Returns the target of the toolchain, e.g. "x86_64-unknown-linux-gnu".
fn toolchain_target() -> Option<String> {
    let sysroot = util::find_sysroot();

    // get the supported rustup targets
    let output = String::from_utf8(
        Command::new("rustup")
            .arg("target")
            .arg("list")
            .stdout(Stdio::piped())
            .output()
            .expect("could not run 'rustup target list'")
            .stdout,
    )
    .unwrap();

    let target = output.lines().find_map(|line| {
        let target = line.split_whitespace().next().unwrap().to_owned();
        if sysroot.ends_with(&target) {
            Some(target)
        } else {
            None
        }
    });

    target
}
