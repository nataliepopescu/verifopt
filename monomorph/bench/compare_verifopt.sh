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
# Output:
#   A comparison table per (example, scenario) (plain vs verifopt:
#   mean/median/stddev/min/max, plus % change), a final summary across
#   all of them, and a CSV with one row per (example, scenario, build
#   kind) for further analysis.

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

EXAMPLES_DIR="testing_examples"
NUM_RUNS=10
NUM_WARMUP=2
RUN_TIMEOUT=60
OUTPUT_CSV="bench_results.csv"
ONLY_EXAMPLES=""

usage() {
    sed -n '2,52p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
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
    for scenario_line in "${scenario_lines[@]}"; do
        IFS=$'\t' read -r s_label s_args_str <<< "$scenario_line"
        read -ra s_args <<< "$s_args_str"
        if [ "$NUM_WARMUP" -gt 0 ]; then
            echo "  [plain]    [$s_label] warming up ($NUM_WARMUP run(s), untimed) ..."
            warmup_runs "plain" "$example" "$s_label" "$plain_binary" "$example_abs_dir" "${s_args[@]}"
        fi
        echo "  [plain]    [$s_label] running $NUM_RUNS times ..."
        time_runs "plain" "$example" "$s_label" "$plain_binary" "$example_abs_dir" "${s_args[@]}"
    done

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
    fi
    echo "  [verifopt] binary: $verifopt_binary"
    for scenario_line in "${scenario_lines[@]}"; do
        IFS=$'\t' read -r s_label s_args_str <<< "$scenario_line"
        read -ra s_args <<< "$s_args_str"
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
