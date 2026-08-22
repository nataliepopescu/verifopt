#!/usr/bin/env bash
# Method (b): run repeatedly until each run crashes naturally, measuring
# shell-level wall-clock time to the crash. Repeats N times for a median.
#
# Runs `rm -f stats && cargo clean` before each iteration, matching your
# own workflow - forces a full dependency rebuild every time, so the
# shell-level wall_ms includes that (roughly constant) cost on top of
# the analysis itself. The bb_visit checkpoint reported alongside it is
# unaffected by caching either way, since that timer only starts once
# the verifopt pass itself begins.
#
# CAVEAT: only comparable across runs hitting the SAME first crash - once
# that crash is fixed, later runs go further and this baseline is stale.
#
# Usage:
#   ./bench_run_to_crash.sh <project_dir> <n_runs>

set -uo pipefail

PROJECT_DIR="${1:?usage: bench_run_to_crash.sh <project_dir> <n_runs>}"
N_RUNS="${2:?usage: bench_run_to_crash.sh <project_dir> <n_runs>}"

cd "$PROJECT_DIR" || { echo "error: could not cd to $PROJECT_DIR" >&2; exit 1; }

WALL_MS_RESULTS=()
VERIFOPT_MS_RESULTS=()
BUILD_MS_RESULTS=()

for i in $(seq 1 "$N_RUNS"); do
    rm -f stats
    cargo clean
    LOG_FILE=$(mktemp)
    START=$(date +%s.%N)
    stdbuf -oL -eL env VERIFOPT_LOG=warn cargo verifopt --release > /dev/null 2> "$LOG_FILE"
    EXIT_CODE=$?
    END=$(date +%s.%N)
    WALL_MS=$(python3 -c "print(($END - $START) * 1000)")
    LAST_LINE=$(grep "TOTAL WALL CLOCK" "$LOG_FILE" | tail -1)
    BB_VISIT=$(echo "$LAST_LINE" | grep -oP 'bb_visit=\K[0-9]+' || echo "n/a")
    VERIFOPT_MS=$(echo "$LAST_LINE" | grep -oP 'elapsed_ms=\K[0-9.]+' || echo "")
    if [[ -n "$VERIFOPT_MS" ]]; then
        BUILD_MS=$(python3 -c "print($WALL_MS - $VERIFOPT_MS)")
        VERIFOPT_MS_RESULTS+=("$VERIFOPT_MS")
        BUILD_MS_RESULTS+=("$BUILD_MS")
    else
        BUILD_MS="n/a"
    fi
    echo "run $i: wall_ms=$WALL_MS  build_ms=$BUILD_MS  verifopt_ms=$VERIFOPT_MS  bb_visit=$BB_VISIT  exit=$EXIT_CODE"
    WALL_MS_RESULTS+=("$WALL_MS")
    rm -f "$LOG_FILE"
done

echo ""
echo "=== summary across $N_RUNS runs ==="
printf '%s\n' "${WALL_MS_RESULTS[@]}" | python3 -c "
import sys
vals = sorted(float(x) for x in sys.stdin)
n = len(vals)
median = vals[n // 2] if n % 2 else (vals[n // 2 - 1] + vals[n // 2]) / 2
print(f'  wall_ms (total, incl. build):    median={median:.1f}  min={min(vals):.1f}  max={max(vals):.1f}')
"
if [[ ${#BUILD_MS_RESULTS[@]} -gt 0 ]]; then
    printf '%s\n' "${BUILD_MS_RESULTS[@]}" | python3 -c "
import sys
vals = sorted(float(x) for x in sys.stdin)
n = len(vals)
median = vals[n // 2] if n % 2 else (vals[n // 2 - 1] + vals[n // 2]) / 2
print(f'  build_ms (before verifopt pass): median={median:.1f}  min={min(vals):.1f}  max={max(vals):.1f}')
"
    printf '%s\n' "${VERIFOPT_MS_RESULTS[@]}" | python3 -c "
import sys
vals = sorted(float(x) for x in sys.stdin)
n = len(vals)
median = vals[n // 2] if n % 2 else (vals[n // 2 - 1] + vals[n // 2]) / 2
print(f'  verifopt_ms (analysis pass only): median={median:.1f}  min={min(vals):.1f}  max={max(vals):.1f}')
"
fi
echo "REMINDER: only comparable across runs hitting the same first crash."
