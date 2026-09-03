#!/usr/bin/env bash
#
# compare_verifopt.sh - for each example project under a testing_examples
# directory, build it two ways - a control build (`cargo verifopt
# --release -- --no-rewrite`, going through the exact same pipeline and
# RUSTFLAGS as a real verifopt build, but with every rewrite skipped)
# and a real rewritten build (`cargo verifopt --release`) - run each
# resulting binary N times, and report summary timing stats comparing
# the two.
#
# Using --no-rewrite for the "plain" leg (rather than a bare `cargo
# build --release`) isolates the effect of the rewrites themselves from
# any effect of the pipeline/flags alone (e.g. -Z always_encode_mir
# potentially changing inlining or other codegen decisions even with
# zero rewrites applied) - see --no-rewrite's own documentation in
# monomorph/src/util/options.rs for the full rationale.
#
# Not wired up as `cargo bench` on purpose: this orchestrates two entirely
# separate build pipelines (different rustc wrappers) across multiple
# independent example crates and then times the resulting *binaries* as
# black boxes - criterion/cargo-bench is built for in-process
# micro-benchmarking of Rust functions within one crate, not this. A plain
# script is the more natural fit, and matches this repo's own existing
# convention (see monomorph/total_bench/*.sh).
#
# Usage:
#   ./compare_verifopt.sh [options]
#
# Options:
#   -d DIR      Directory containing example crates (default: testing_examples)
#   -n N        Number of timed runs per binary (default: 10)
#   -w N        Number of untimed warmup runs before the timed ones (default: 2)
#   -t SECS     Per-run timeout in seconds, applies to warmup and timed runs (default: 60)
#   -o FILE     Timing CSV output path (default: bench_results.csv)
#   -z FILE     Binary size CSV output path (default: bench_sizes.csv)
#   -e NAMES    Comma-separated list of example dir names to run (default: all)
#   -h          Show this help and exit
#
# Per-example run arguments (one or more named scenarios):
#   If an example directory contains a file named `bench_args.txt`, each
#   non-blank, non-comment (#) line is a separate benchmark scenario,
#   run independently (its own warmup + timed runs, its own row in the
#   output) against both the plain and the verifopt build. A line of
#   the form
#       label: arg1 arg2 ...
#   uses `label` to identify the scenario in output/CSV; a line with no
#   such prefix is auto-labeled args1, args2, ... by position. Blank
#   lines and lines starting with # are ignored. If the file is absent,
#   the binary is run once with no arguments, labeled "default".
#   Relative paths in any scenario's args resolve against the example's
#   own directory, not wherever this script was invoked from.
#
#   Example bench_args.txt with two scenarios:
#       large_corpus: -n TODO /home/user/big_repo
#       flag_heavy: -i -w --hidden --no-ignore -C 3 -g *.rs nomatch small_dir/
#
# Skipping the verifopt build for a specific example:
#   If an example directory contains a file named
#   `prebuilt_verifopt_binary.txt`, its first line is used as the path
#   to an already-built verifopt binary, and `cargo clean` + `cargo
#   verifopt --release` are skipped entirely for that example - useful
#   when that build is expensive enough (e.g. a full ripgrep build) that
#   re-running it on every benchmark invocation isn't practical. A
#   relative path resolves against the example's own directory; an
#   absolute path is used as-is. Remove or empty the file to go back to
#   rebuilding normally.
#
# Specifying the --bin target explicitly:
#   Both cargo-verifopt invocations pass --bin <name> explicitly, so
#   cargo doesn't also compile every test target in the package (its
#   own default behavior without --bin - see cargo-verifopt.rs's own
#   docs), which can otherwise be mistaken for the real binary. The
#   name is normally auto-discovered via `cargo metadata`, but if an
#   example directory contains a file named `bin_target_name.txt`, its
#   first line is used directly instead, skipping the metadata call
#   entirely - useful for a crate with more than one [[bin]], or any
#   other case where auto-discovery can't uniquely determine one.
#
# Output:
#   A comparison table per (example, scenario) (plain vs verifopt:
#   mean/median/stddev/min/max, plus % change), a final summary across
#   all of them, and a CSV with one row per (example, scenario, build
#   kind) for further analysis. Also a binary size comparison (via the
#   `size` command, plus raw file size) per example - one row per
#   (example, build kind), since size doesn't depend on scenario args.
#   Skipped with a warning if `size` isn't available.

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

EXAMPLES_DIR="testing_examples"
NUM_RUNS=10
NUM_WARMUP=2
RUN_TIMEOUT=60
OUTPUT_CSV="bench_results.csv"
OUTPUT_SIZES_CSV="bench_sizes.csv"
ONLY_EXAMPLES=""

usage() {
    sed -n '2,86p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
}

while getopts "d:n:w:t:o:z:e:h" opt; do
    case "$opt" in
        d) EXAMPLES_DIR="$OPTARG" ;;
        n) NUM_RUNS="$OPTARG" ;;
        w) NUM_WARMUP="$OPTARG" ;;
        t) RUN_TIMEOUT="$OPTARG" ;;
        o) OUTPUT_CSV="$OPTARG" ;;
        z) OUTPUT_SIZES_CSV="$OPTARG" ;;
        e) ONLY_EXAMPLES="$OPTARG" ;;
        h) usage; exit 0 ;;
        *) usage; exit 1 ;;
    esac
done

if ! [[ "$NUM_RUNS" =~ ^[0-9]+$ ]] || [ "$NUM_RUNS" -lt 1 ]; then
    echo "error: -n must be a positive integer" >&2
    exit 1
fi

if ! [[ "$NUM_WARMUP" =~ ^[0-9]+$ ]]; then
    echo "error: -w must be a non-negative integer" >&2
    exit 1
fi

if [ ! -d "$EXAMPLES_DIR" ]; then
    echo "error: examples directory not found: $EXAMPLES_DIR" >&2
    exit 1
fi

STATS_PY="$SCRIPT_DIR/bench_stats.py"
if [ ! -f "$STATS_PY" ]; then
    echo "error: expected helper script at $STATS_PY" >&2
    exit 1
fi

SIZES_PY="$SCRIPT_DIR/bench_sizes.py"
if [ ! -f "$SIZES_PY" ]; then
    echo "error: expected helper script at $SIZES_PY" >&2
    exit 1
fi

HAVE_SIZE_CMD=1
if ! command -v size >/dev/null 2>&1; then
    HAVE_SIZE_CMD=0
    echo "warning: 'size' command not found - binary size comparison will be skipped" >&2
fi

RESULTS_JSONL="$(mktemp)"
SIZES_JSONL="$(mktemp)"
# Both legs now go through `cargo verifopt` (control build via
# --no-rewrite, or the real thing), so both get their own isolated
# --target-dir - without this, running this script against an example
# whose real target/ is also being used by something else (e.g. a
# separate, long-running `cargo verifopt` invocation you're not routing
# through prebuilt_verifopt_binary.txt) would have this script's own
# `cargo clean` wipe out that other build's in-progress dependency
# artifacts out from under it. Each is shared across all examples in
# this invocation (safe: examples are processed one at a time, and each
# is cleaned immediately before its own build, so there's no
# cross-example contamination within a single run).
PLAIN_TARGET_DIR="$(mktemp -d)"
VERIFOPT_TARGET_DIR="$(mktemp -d)"
trap 'rm -f "$RESULTS_JSONL" "$SIZES_JSONL"; rm -rf "$PLAIN_TARGET_DIR" "$VERIFOPT_TARGET_DIR"' EXIT

echo "kind,example,run_index,seconds" > /dev/null # (CSV header written by bench_stats.py at the end)

# --- helpers ---------------------------------------------------------------

# Discover the executable cargo just built, via --message-format=json.
# More robust than assuming target/release/<dir-name> - respects any
# [[bin]] name override, and works the same regardless of build flavor.
discover_binary() {
    local build_output="$1"
    python3 -c '
import json, sys
candidates = []
for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    try:
        msg = json.loads(line)
    except json.JSONDecodeError:
        continue
    if msg.get("reason") == "compiler-artifact" and msg.get("executable"):
        candidates.append(msg)

if not candidates:
    sys.exit(0)

# Prefer an artifact whose target is actually a "bin" - matters when
# more than one compiler-artifact message shows up (e.g. `cargo verifopt`
# without an explicit --bin, run against a pure-workspace example with
# no root package, builds every member). Falls back to "whichever came
# last" if that field is missing for some reason, matching the simpler
# heuristic this replaced.
bin_candidates = [msg["executable"] for msg in candidates if "bin" in ((msg.get("target") or {}).get("kind") or [])]
if bin_candidates:
    print(bin_candidates[-1])
else:
    print(candidates[-1]["executable"])
' <<< "$build_output"
}

# Record a binary's size (via the `size` command, plus raw on-disk file
# size) as one JSON object appended to $SIZES_JSONL. Size doesn't depend
# on scenario args - called once per (example, kind), not per scenario.
# Skips silently (size command unavailable, checked once up front) or
# with a warning (size/wc failed on this specific binary) rather than
# treating this as fatal - it's a nice-to-have alongside the timing
# comparison, not the main point of a run.
record_binary_size() {
    local kind="$1" example="$2" binary="$3"
    if [ "$HAVE_SIZE_CMD" -ne 1 ]; then
        return
    fi
    local size_line text data bss dec file_size
    size_line="$(size "$binary" 2>/dev/null | tail -n 1)"
    if [ -z "$size_line" ]; then
        echo "  warning: 'size' failed on $binary - skipping size record for ($kind, $example)" >&2
        return
    fi
    read -r text data bss dec _ <<< "$size_line"
    if ! [[ "$text" =~ ^[0-9]+$ && "$data" =~ ^[0-9]+$ && "$bss" =~ ^[0-9]+$ && "$dec" =~ ^[0-9]+$ ]]; then
        echo "  warning: could not parse 'size' output for $binary - skipping size record for ($kind, $example)" >&2
        return
    fi
    file_size="$(wc -c < "$binary" | tr -d '[:space:]')"
    printf '{"kind": "%s", "example": "%s", "text": %s, "data": %s, "bss": %s, "dec": %s, "file_size_bytes": %s}\n' \
        "$kind" "$example" "$text" "$data" "$bss" "$dec" "$file_size" >> "$SIZES_JSONL"
}

# Parse bench_args.txt (if present) into one or more scenarios. Prints
# one "label<TAB>args..." line per scenario. A file line of the form
# "label: args..." uses that label; a line with no such prefix is
# auto-labeled args1, args2, ... by position. Blank lines and lines
# starting with # are skipped. If the file is absent or has no usable
# lines, prints a single "default<TAB>" scenario (no args).
parse_bench_scenarios() {
    local example_dir="$1"
    local args_file="$example_dir/bench_args.txt"
    local -a raw_lines=()

    if [ -f "$args_file" ]; then
        local line
        while IFS= read -r line || [ -n "$line" ]; do
            # trim leading/trailing whitespace
            line="${line#"${line%%[![:space:]]*}"}"
            line="${line%"${line##*[![:space:]]}"}"
            [ -z "$line" ] && continue
            [[ "$line" == \#* ]] && continue
            raw_lines+=("$line")
        done < "$args_file"
    fi

    if [ "${#raw_lines[@]}" -eq 0 ]; then
        printf 'default\t\n'
        return
    fi

    local n=0 label args
    for line in "${raw_lines[@]}"; do
        n=$((n + 1))
        if [[ "$line" =~ ^([A-Za-z0-9_-]+):[[:space:]]+(.*)$ ]]; then
            label="${BASH_REMATCH[1]}"
            args="${BASH_REMATCH[2]}"
        else
            label="args$n"
            args="$line"
        fi
        printf '%s\t%s\n' "$label" "$args"
    done
}

# Read prebuilt_verifopt_binary.txt (if present): a path to an
# already-built verifopt binary to use instead of rebuilding one. Meant
# for examples (like a full ripgrep build) where `cargo verifopt
# --release` takes long enough that re-running it on every benchmark
# invocation isn't practical. A relative path is resolved against the
# example's own absolute directory; an absolute path is used as-is.
# Prints nothing if the file is absent or empty.
read_prebuilt_verifopt_binary() {
    local example_abs_dir="$1"
    local marker_file="$example_abs_dir/prebuilt_verifopt_binary.txt"
    if [ ! -f "$marker_file" ]; then
        return
    fi
    local raw
    raw="$(head -n 1 "$marker_file" | tr -d '[:space:]')"
    if [ -z "$raw" ]; then
        return
    fi
    if [[ "$raw" == /* ]]; then
        printf '%s' "$raw"
    else
        printf '%s' "$example_abs_dir/$raw"
    fi
}

# Read bin_target_name.txt (if present): an explicit override for the
# package's binary target name, checked before ever calling `cargo
# metadata`. Meant for any example where auto-discovery (below) can't
# uniquely determine one - e.g. a crate with more than one [[bin]], or
# any other cargo-metadata quirk specific to that example - since this
# skips the metadata call entirely, it's also a way to bypass whatever
# is causing auto-discovery to fail rather than needing to diagnose it.
# Prints nothing if the file is absent or empty.
read_bin_target_name_override() {
    local example_dir="$1"
    local marker_file="$example_dir/bin_target_name.txt"
    if [ ! -f "$marker_file" ]; then
        return
    fi
    head -n 1 "$marker_file" | tr -d '[:space:]'
}

# Find the package's own binary target name via `cargo metadata` (a
# fast, side-effect-free introspection call - no compilation happens),
# so both cargo-verifopt invocations below can pass --bin <name>
# explicitly. Without it, `cargo verifopt --release` (no --bin) falls
# back to cargo-verifopt.rs's own default behavior of also running
# `cargo test --no-run` for every test target in the package - its
# --message-format=json output then includes compiled integration-test
# harnesses alongside the real binary, with no reliable way to tell
# them apart from that output alone (a compiled test harness looks like
# an ordinary compiler-artifact too). Prints nothing (callers fall back
# to the old, ambiguous discovery with a warning) if zero or more than
# one bin target is found - this is meant for the common single-binary
# case, not to arbitrate a genuinely multi-binary example. See
# read_bin_target_name_override above for a direct way to skip this
# entirely for a specific example.
discover_bin_target_name() {
    local example_dir="$1"
    local metadata
    metadata="$(cd "$example_dir" && cargo metadata --format-version 1 --no-deps 2>/dev/null)"
    if [ -z "$metadata" ]; then
        return
    fi
    python3 -c '
import json, sys
try:
    data = json.loads(sys.stdin.read())
except json.JSONDecodeError:
    sys.exit(0)
bin_names = []
for pkg in data.get("packages", []):
    for target in pkg.get("targets", []):
        if "bin" in (target.get("kind") or []):
            bin_names.append(target["name"])
if len(bin_names) == 1:
    print(bin_names[0])
' <<< "$metadata"
}

# Run a binary NUM_WARMUP times, discarding the results entirely - not
# timed, not recorded, just executed to warm up OS page/disk cache for
# the binary (and anything it reads) before the runs that actually get
# measured. A warmup run failing or timing out doesn't affect anything
# else; it's just warned about.
warmup_runs() {
    local kind="$1" example="$2" scenario="$3" binary="$4" run_dir="$5"
    shift 5
    local -a args=("$@")

    local i
    for i in $(seq 1 "$NUM_WARMUP"); do
        if ! (cd "$run_dir" && timeout "${RUN_TIMEOUT}s" "$binary" "${args[@]}") >/dev/null 2>&1; then
            echo "  warning: warmup run $i/$NUM_WARMUP ($kind, $scenario) exited non-zero or timed out" >&2
        fi
    done
}

# Time N runs of a binary, appending one JSON object per run to
# $RESULTS_JSONL. Skips (with a warning) rather than aborting on a
# per-run failure or timeout, so one bad run doesn't lose the rest.
time_runs() {
    local kind="$1" example="$2" scenario="$3" binary="$4" run_dir="$5"
    shift 5
    local -a args=("$@")

    local i seconds
    for i in $(seq 1 "$NUM_RUNS"); do
        local start end
        start=$EPOCHREALTIME
        if ! (cd "$run_dir" && timeout "${RUN_TIMEOUT}s" "$binary" "${args[@]}") >/dev/null 2>&1; then
            echo "  warning: run $i/$NUM_RUNS ($kind, $scenario) exited non-zero or timed out - excluded from stats" >&2
            continue
        fi
        end=$EPOCHREALTIME
        seconds=$(python3 -c "print(f'{$end - $start:.6f}')")
        printf '{"kind": "%s", "example": "%s", "scenario": "%s", "run_index": %d, "seconds": %s}\n' \
            "$kind" "$example" "$scenario" "$i" "$seconds" >> "$RESULTS_JSONL"
    done
}

# Clean an isolated --target-dir, but only if it's actually been used
# before (has cargo's own CACHEDIR.TAG marker). cargo refuses to clean a
# --target-dir that doesn't already contain that marker (a safety check
# against accidentally wiping a directory it didn't create), and on the
# first use of a freshly mktemp'd directory there's nothing to clean
# anyway - only later examples sharing the same directory actually need
# a real clean. Writes cargo's own output to $log_file; returns cargo's
# exit status (or 0, untouched log file, when skipped as a no-op).
clean_isolated_target_dir() {
    local example_dir="$1" target_dir="$2" log_file="$3"
    if [ -f "$target_dir/CACHEDIR.TAG" ]; then
        (cd "$example_dir" && cargo clean --target-dir "$target_dir") >"$log_file" 2>&1
    else
        : > "$log_file"
        return 0
    fi
}

# --- main --------------------------------------------------------------

mapfile -t example_dirs < <(find -L "$EXAMPLES_DIR" -maxdepth 1 -mindepth 1 -type d | sort)

if [ -n "$ONLY_EXAMPLES" ]; then
    IFS=',' read -ra wanted <<< "$ONLY_EXAMPLES"
    filtered=()
    for d in "${example_dirs[@]}"; do
        base="$(basename "$d")"
        for w in "${wanted[@]}"; do
            if [ "$base" == "$w" ]; then
                filtered+=("$d")
                break
            fi
        done
    done
    example_dirs=("${filtered[@]}")
fi

if [ "${#example_dirs[@]}" -eq 0 ]; then
    echo "error: no example directories found under $EXAMPLES_DIR" >&2
    exit 1
fi

echo "Found ${#example_dirs[@]} example(s) under $EXAMPLES_DIR: ${example_dirs[*]/#$EXAMPLES_DIR\//}"
echo "Warmup runs: $NUM_WARMUP   Timed runs per binary: $NUM_RUNS   Per-run timeout: ${RUN_TIMEOUT}s"
echo

for example_dir in "${example_dirs[@]}"; do
    example="$(basename "$example_dir")"
    echo "=== $example ==="

    if [ ! -f "$example_dir/Cargo.toml" ]; then
        echo "  skipping: no Cargo.toml found"
        echo
        continue
    fi

    mapfile -t scenario_lines < <(parse_bench_scenarios "$example_dir")
    echo "  scenarios:"
    for scenario_line in "${scenario_lines[@]}"; do
        IFS=$'\t' read -r s_label s_args_str <<< "$scenario_line"
        if [ -n "$s_args_str" ]; then
            echo "    $s_label: $s_args_str"
        else
            echo "    $s_label: (no args)"
        fi
    done

    example_abs_dir="$(cd "$example_dir" && pwd)"
    plain_clean_log="$example_abs_dir/.bench_plain_clean.log"
    plain_build_stderr="$example_abs_dir/.bench_plain_build.stderr"
    verifopt_clean_log="$example_abs_dir/.bench_verifopt_clean.log"
    verifopt_build_log="$example_abs_dir/.bench_verifopt_build.log"

    bin_target_name="$(read_bin_target_name_override "$example_dir")"
    if [ -n "$bin_target_name" ]; then
        echo "  bin target: $bin_target_name (from bin_target_name.txt)"
    else
        bin_target_name="$(discover_bin_target_name "$example_dir")"
        if [ -n "$bin_target_name" ]; then
            echo "  bin target: $bin_target_name"
        fi
    fi
    bin_flag=()
    if [ -n "$bin_target_name" ]; then
        bin_flag=(--bin "$bin_target_name")
    else
        echo "  warning: could not uniquely determine a --bin target via cargo metadata - cargo verifopt may also compile test harnesses, which could be mistaken for the real binary. Add a bin_target_name.txt to this example to specify it directly." >&2
    fi

    # --- plain (control) build: cargo verifopt --no-rewrite ---
    echo "  [plain]    cargo clean + cargo verifopt --release -- --no-rewrite (isolated target-dir, control build) ..."
    if ! clean_isolated_target_dir "$example_dir" "$PLAIN_TARGET_DIR" "$plain_clean_log"; then
        echo "  skipping: cargo clean failed (plain)"
        echo "  --- last 30 lines of output ---"
        tail -n 30 "$plain_clean_log" | sed 's/^/    /'
        rm -f "$plain_clean_log"
        echo
        continue
    fi
    rm -f "$plain_clean_log"
    plain_build_output="$(cd "$example_dir" && cargo verifopt --release --target-dir "$PLAIN_TARGET_DIR" "${bin_flag[@]}" --message-format=json -- --no-rewrite 2>"$plain_build_stderr")"
    if [ -z "$plain_build_output" ]; then
        echo "  skipping: cargo verifopt --release -- --no-rewrite produced no output (plain/control build likely failed)"
        echo "  --- last 30 lines of stderr ---"
        tail -n 30 "$plain_build_stderr" 2>/dev/null | sed 's/^/    /'
        echo
        continue
    fi
    plain_binary="$(discover_binary "$plain_build_output")"
    if [ -z "$plain_binary" ] || [ ! -x "$plain_binary" ]; then
        echo "  skipping: could not discover a built, executable binary (plain build)"
        echo
        continue
    fi
    echo "  [plain]    binary: $plain_binary"
    record_binary_size "plain" "$example" "$plain_binary"

    # --- verifopt build ---
    prebuilt_verifopt_binary="$(read_prebuilt_verifopt_binary "$example_abs_dir")"
    if [ -n "$prebuilt_verifopt_binary" ]; then
        if [ ! -x "$prebuilt_verifopt_binary" ]; then
            echo "  skipping verifopt leg: prebuilt_verifopt_binary.txt points to a missing or non-executable file: $prebuilt_verifopt_binary"
            echo
            continue
        fi
        echo "  [verifopt] using prebuilt binary from prebuilt_verifopt_binary.txt (skipping cargo clean + cargo verifopt --release)"
        verifopt_binary="$prebuilt_verifopt_binary"
    else
        # Now that --target-dir is confirmed to work with cargo
        # verifopt (it's forwarded as an ordinary arg to the real
        # `cargo build` underneath - see cargo-verifopt.rs's
        # call_cargo_on_target), this leg gets the same isolation and
        # --message-format=json discovery as the plain leg above,
        # instead of sharing the example's own default target/ and
        # reusing the plain build's binary name.
        echo "  [verifopt] cargo clean + cargo verifopt --release (isolated target-dir) ..."
        if ! clean_isolated_target_dir "$example_dir" "$VERIFOPT_TARGET_DIR" "$verifopt_clean_log"; then
            echo "  skipping verifopt leg: cargo clean failed"
            echo "  --- last 30 lines of output ---"
            tail -n 30 "$verifopt_clean_log" | sed 's/^/    /'
            rm -f "$verifopt_clean_log"
            echo
            continue
        fi
        rm -f "$verifopt_clean_log"
        verifopt_build_output="$(cd "$example_dir" && cargo verifopt --release --target-dir "$VERIFOPT_TARGET_DIR" "${bin_flag[@]}" --message-format=json 2>"$verifopt_build_log")"
        if [ -z "$verifopt_build_output" ]; then
            echo "  skipping verifopt leg: cargo verifopt --release produced no output (likely failed)"
            echo "  --- last 30 lines of output ---"
            tail -n 30 "$verifopt_build_log" | sed 's/^/    /'
            echo
            continue
        fi
        verifopt_binary="$(discover_binary "$verifopt_build_output")"
        if [ -z "$verifopt_binary" ] || [ ! -x "$verifopt_binary" ]; then
            echo "  skipping verifopt leg: could not discover a built, executable binary"
            echo
            continue
        fi
    fi
    echo "  [verifopt] binary: $verifopt_binary"
    record_binary_size "verifopt" "$example" "$verifopt_binary"

    # Both binaries are already built above - only the *timed* runs are
    # interleaved here (plain then verifopt, scenario by scenario),
    # rather than every scenario's plain runs finishing before any
    # verifopt run starts. This keeps any systematic drift over the
    # course of the run (thermal throttling, background load changing
    # over time, etc.) from landing disproportionately on one leg - a
    # long, uninterrupted block of one leg's runs would otherwise let
    # that drift bias its own mean more than the other leg's.
    for scenario_line in "${scenario_lines[@]}"; do
        IFS=$'\t' read -r s_label s_args_str <<< "$scenario_line"
        read -ra s_args <<< "$s_args_str"

        if [ "$NUM_WARMUP" -gt 0 ]; then
            echo "  [plain]    [$s_label] warming up ($NUM_WARMUP run(s), untimed) ..."
            warmup_runs "plain" "$example" "$s_label" "$plain_binary" "$example_abs_dir" "${s_args[@]}"
        fi
        echo "  [plain]    [$s_label] running $NUM_RUNS times ..."
        time_runs "plain" "$example" "$s_label" "$plain_binary" "$example_abs_dir" "${s_args[@]}"

        if [ "$NUM_WARMUP" -gt 0 ]; then
            echo "  [verifopt] [$s_label] warming up ($NUM_WARMUP run(s), untimed) ..."
            warmup_runs "verifopt" "$example" "$s_label" "$verifopt_binary" "$example_abs_dir" "${s_args[@]}"
        fi
        echo "  [verifopt] [$s_label] running $NUM_RUNS times ..."
        time_runs "verifopt" "$example" "$s_label" "$verifopt_binary" "$example_abs_dir" "${s_args[@]}"
    done

    rm -f "$plain_build_stderr" "$verifopt_build_log"
    echo
done

echo "=== Summary ==="
python3 "$STATS_PY" "$RESULTS_JSONL" --csv "$OUTPUT_CSV"
echo
echo "Full per-run data written to: $OUTPUT_CSV"

if [ "$HAVE_SIZE_CMD" -eq 1 ]; then
    echo
    echo "=== Binary Size Summary ==="
    python3 "$SIZES_PY" "$SIZES_JSONL" --csv "$OUTPUT_SIZES_CSV"
    echo
    echo "Binary size data written to: $OUTPUT_SIZES_CSV"
fi
