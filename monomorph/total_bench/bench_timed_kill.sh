#!/usr/bin/env bash
# Method (a): run for a fixed wall-clock budget, kill, record how far the
# analysis got (last TOTAL WALL CLOCK checkpoint before the kill).
# Repeats N times for a median.
#
# Runs `rm -f stats && cargo clean` before each iteration, matching your
# own workflow - necessary here, not just for consistency: without it,
# cached dependency builds on later iterations would leave more of the
# fixed budget for analysis than uncached iterations, corrupting the
# comparison. Budget needs to cover a full dependency rebuild AND
# meaningful analysis time.
#
# Usage:
#   ./bench_timed_kill.sh <project_dir> <budget_seconds> <n_runs>

set -uo pipefail

PROJECT_DIR="${1:?usage: bench_timed_kill.sh <project_dir> <budget_seconds> <n_runs>}"
BUDGET="${2:?usage: bench_timed_kill.sh <project_dir> <budget_seconds> <n_runs>}"
N_RUNS="${3:?usage: bench_timed_kill.sh <project_dir> <budget_seconds> <n_runs>}"

cd "$PROJECT_DIR" || { echo "error: could not cd to $PROJECT_DIR" >&2; exit 1; }

RESULTS=()
BUILD_MS_RESULTS=()

for i in $(seq 1 "$N_RUNS"); do
    rm -f stats
    cargo clean
    LOG_FILE=$(mktemp)
    timeout --signal=TERM "$BUDGET" stdbuf -oL -eL env VERIFOPT_LOG=warn cargo verifopt --release \
        > /dev/null 2> "$LOG_FILE"
    LAST_LINE=$(grep "TOTAL WALL CLOCK" "$LOG_FILE" | tail -1)
    if [[ -z "$LAST_LINE" ]]; then
        echo "run $i: no checkpoint reached - log preserved at $LOG_FILE, last 15 lines:"
        tail -15 "$LOG_FILE" | sed 's/^/    /'
        RESULTS+=("0")
        continue
    else
        BB_VISIT=$(echo "$LAST_LINE" | grep -oP 'bb_visit=\K[0-9]+')
        VERIFOPT_MS=$(echo "$LAST_LINE" | grep -oP 'elapsed_ms=\K[0-9.]+')
        # timeout kills at ~budget, so wall_ms ~= budget*1000. Upper bound,
        # not exact: the kill can land mid-window.
        BUILD_MS=$(python3 -c "print($BUDGET * 1000 - $VERIFOPT_MS)")
        echo "run $i: reached bb_visit=$BB_VISIT  verifopt_ms=$VERIFOPT_MS  build_ms_upper_bound=$BUILD_MS"
        RESULTS+=("$BB_VISIT")
        BUILD_MS_RESULTS+=("$BUILD_MS")
    fi
    rm -f "$LOG_FILE"
done

echo ""
echo "=== summary across $N_RUNS runs (budget=${BUDGET}s) ==="
printf '%s\n' "${RESULTS[@]}" | python3 -c "
import sys
vals = sorted(int(x) for x in sys.stdin)
n = len(vals)
median = vals[n // 2] if n % 2 else (vals[n // 2 - 1] + vals[n // 2]) / 2
print(f'  bb_visit reached: median={median}  min={min(vals)}  max={max(vals)}  all={vals}')
"
if [[ ${#BUILD_MS_RESULTS[@]} -gt 0 ]]; then
    printf '%s\n' "${BUILD_MS_RESULTS[@]}" | python3 -c "
import sys
vals = sorted(float(x) for x in sys.stdin)
n = len(vals)
median = vals[n // 2] if n % 2 else (vals[n // 2 - 1] + vals[n // 2]) / 2
print(f'  build_ms upper bound: median={median:.1f}  min={min(vals):.1f}  max={max(vals):.1f}')
"
fi
