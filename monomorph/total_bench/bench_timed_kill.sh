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

for i in $(seq 1 "$N_RUNS"); do
    rm -f stats
    cargo clean
    LOG_FILE=$(mktemp)
    timeout --signal=TERM "$BUDGET" stdbuf -oL -eL env VERIFOPT_LOG=warn cargo verifopt --release \
        > /dev/null 2> "$LOG_FILE"
    LAST_LINE=$(grep "TOTAL WALL CLOCK" "$LOG_FILE" | tail -1)
    if [[ -z "$LAST_LINE" ]]; then
        echo "run $i: no checkpoint reached (budget too short, crashed early, or still building deps)"
        RESULTS+=("0")
    else
        BB_VISIT=$(echo "$LAST_LINE" | grep -oP 'bb_visit=\K[0-9]+')
        echo "run $i: reached bb_visit=$BB_VISIT"
        RESULTS+=("$BB_VISIT")
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
