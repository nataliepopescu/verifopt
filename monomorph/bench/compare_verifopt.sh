#!/usr/bin/env bash
#
# compare_verifopt.sh - for each example project under a testing_examples
# directory, build it two ways (plain `cargo build --release` and
# `cargo verifopt --release`), run each resulting binary N times, and
# report summary timing stats comparing the two.
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
#   -o FILE     CSV output path (default: bench_results.csv)
#   -e NAMES    Comma-separated list of example dir names to run (default: all)
#   -h          Show this help and exit
#
# Per-example run arguments:
#   If an example directory contains a file named `bench_args.txt`, its
#   entire contents are word-split and passed as CLI arguments to the
#   binary on every timed run (both the plain and the verifopt build).
#   If absent, the binary is run with no arguments. This is the only
#   per-example convention this script assumes - add a bench_args.txt to
#   any example that needs real arguments to do meaningful work (e.g. a
#   ripgrep invocation with a real pattern and directory).
#
# Output:
#   A comparison table per example (plain vs verifopt: mean/median/
#   stddev/min/max, plus % change), a final summary across all examples,
#   and a CSV with one row per (example, build kind) for further analysis.

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

EXAMPLES_DIR="testing_examples"
NUM_RUNS=10
NUM_WARMUP=2
RUN_TIMEOUT=60
OUTPUT_CSV="bench_results.csv"
ONLY_EXAMPLES=""

usage() {
    sed -n '2,41p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
}

while getopts "d:n:w:t:o:e:h" opt; do
    case "$opt" in
        d) EXAMPLES_DIR="$OPTARG" ;;
        n) NUM_RUNS="$OPTARG" ;;
        w) NUM_WARMUP="$OPTARG" ;;
        t) RUN_TIMEOUT="$OPTARG" ;;
        o) OUTPUT_CSV="$OPTARG" ;;
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

RESULTS_JSONL="$(mktemp)"
trap 'rm -f "$RESULTS_JSONL"' EXIT

echo "kind,example,run_index,seconds" > /dev/null # (CSV header written by bench_stats.py at the end)

# --- helpers ---------------------------------------------------------------

# Discover the executable cargo just built, via --message-format=json.
# More robust than assuming target/release/<dir-name> - respects any
# [[bin]] name override, and works the same regardless of build flavor.
discover_binary() {
    local build_output="$1"
    python3 -c '
import json, sys
for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    try:
        msg = json.loads(line)
    except json.JSONDecodeError:
        continue
    if msg.get("reason") == "compiler-artifact" and msg.get("executable"):
        print(msg["executable"])
' <<< "$build_output" | tail -n 1
}

# Read bench_args.txt (if present) into an array, word-split.
read_bench_args() {
    local example_dir="$1"
    local args_file="$example_dir/bench_args.txt"
    if [ -f "$args_file" ]; then
        # shellcheck disable=SC2046,SC2086
        printf '%s' "$(cat "$args_file")"
    fi
}

# Run a binary NUM_WARMUP times, discarding the results entirely - not
# timed, not recorded, just executed to warm up OS page/disk cache for
# the binary (and anything it reads) before the runs that actually get
# measured. A warmup run failing or timing out doesn't affect anything
# else; it's just warned about.
warmup_runs() {
    local kind="$1" example="$2" binary="$3"
    shift 3
    local -a args=("$@")

    local i
    for i in $(seq 1 "$NUM_WARMUP"); do
        if ! timeout "${RUN_TIMEOUT}s" "$binary" "${args[@]}" >/dev/null 2>&1; then
            echo "  warning: warmup run $i/$NUM_WARMUP ($kind) exited non-zero or timed out" >&2
        fi
    done
}

# Time N runs of a binary, appending one JSON object per run to
# $RESULTS_JSONL. Skips (with a warning) rather than aborting on a
# per-run failure or timeout, so one bad run doesn't lose the rest.
time_runs() {
    local kind="$1" example="$2" binary="$3"
    shift 3
    local -a args=("$@")

    local i seconds
    for i in $(seq 1 "$NUM_RUNS"); do
        local start end
        start=$EPOCHREALTIME
        if ! timeout "${RUN_TIMEOUT}s" "$binary" "${args[@]}" >/dev/null 2>&1; then
            echo "  warning: run $i/$NUM_RUNS ($kind) exited non-zero or timed out - excluded from stats" >&2
            continue
        fi
        end=$EPOCHREALTIME
        seconds=$(python3 -c "print(f'{$end - $start:.6f}')")
        printf '{"kind": "%s", "example": "%s", "run_index": %d, "seconds": %s}\n' \
            "$kind" "$example" "$i" "$seconds" >> "$RESULTS_JSONL"
    done
}

# --- main --------------------------------------------------------------

mapfile -t example_dirs < <(find "$EXAMPLES_DIR" -maxdepth 1 -mindepth 1 -type d | sort)

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

    read -ra bench_args < <(read_bench_args "$example_dir")
    if [ "${#bench_args[@]}" -gt 0 ]; then
        echo "  args: ${bench_args[*]}"
    else
        echo "  args: (none - add bench_args.txt to this example dir to set some)"
    fi

    example_abs_dir="$(cd "$example_dir" && pwd)"
    plain_build_stderr="$example_abs_dir/.bench_plain_build.stderr"
    verifopt_build_log="$example_abs_dir/.bench_verifopt_build.log"

    # --- plain build ---
    echo "  [plain]    cargo clean + cargo build --release ..."
    if ! (cd "$example_dir" && cargo clean) >/dev/null 2>&1; then
        echo "  skipping: cargo clean failed (plain)"
        echo
        continue
    fi
    plain_build_output="$(cd "$example_dir" && cargo build --release --message-format=json 2>"$plain_build_stderr")"
    if [ -z "$plain_build_output" ]; then
        echo "  skipping: cargo build --release produced no output (plain build likely failed)"
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
    if [ "$NUM_WARMUP" -gt 0 ]; then
        echo "  [plain]    warming up ($NUM_WARMUP run(s), untimed) ..."
        warmup_runs "plain" "$example" "$plain_binary" "${bench_args[@]}"
    fi
    echo "  [plain]    running $NUM_RUNS times ..."
    time_runs "plain" "$example" "$plain_binary" "${bench_args[@]}"

    # --- verifopt build ---
    echo "  [verifopt] cargo clean + cargo verifopt --release ..."
    if ! (cd "$example_dir" && cargo clean) >/dev/null 2>&1; then
        echo "  skipping verifopt leg: cargo clean failed"
        echo
        continue
    fi
    if ! (cd "$example_dir" && cargo verifopt --release) >"$verifopt_build_log" 2>&1; then
        echo "  skipping verifopt leg: cargo verifopt --release failed"
        echo "  --- last 30 lines of output ---"
        tail -n 30 "$verifopt_build_log" | sed 's/^/    /'
        echo
        continue
    fi
    # cargo-verifopt is a rustc-wrapper substitution, not a different
    # build/output layout - it produces its binary at the exact same
    # path a plain `cargo build --release` would, so reuse the path we
    # already discovered rather than re-parsing (verifopt's own build
    # doesn't cleanly support --message-format=json passthrough).
    verifopt_binary="$plain_binary"
    if [ ! -x "$verifopt_binary" ]; then
        echo "  skipping verifopt leg: expected binary not found or not executable at $verifopt_binary"
        echo
        continue
    fi
    echo "  [verifopt] binary: $verifopt_binary"
    if [ "$NUM_WARMUP" -gt 0 ]; then
        echo "  [verifopt] warming up ($NUM_WARMUP run(s), untimed) ..."
        warmup_runs "verifopt" "$example" "$verifopt_binary" "${bench_args[@]}"
    fi
    echo "  [verifopt] running $NUM_RUNS times ..."
    time_runs "verifopt" "$example" "$verifopt_binary" "${bench_args[@]}"

    rm -f "$plain_build_stderr" "$verifopt_build_log"
    echo
done

echo "=== Summary ==="
python3 "$STATS_PY" "$RESULTS_JSONL" --csv "$OUTPUT_CSV"
echo
echo "Full per-run data written to: $OUTPUT_CSV"
