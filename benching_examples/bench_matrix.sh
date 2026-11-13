#!/usr/bin/env bash
# Build / check / time matrix for a benchmark harness binary (see
# twovariant-ex/src/bin/bench_twovariant.rs).
#
#   bench_matrix.sh build  [EXAMPLE_DIR BIN]   build every (config, row)
#   bench_matrix.sh check  [EXAMPLE_DIR BIN]   checksums vs plain + call structure
#   bench_matrix.sh time   [EXAMPLE_DIR BIN]   shuffled timing runs
#
# Defaults: EXAMPLE_DIR=twovariant-ex BIN=bench_twovariant (relative to this
# script). Run inside ~/hack/verifopt-examples/shell.nix.
#
# Rows: plain; per K in $KS: hook-only (verifopt's store, --skip-rewrite),
# rung 3 (VERIFOPT_CHA_ONLY=1, CHAmax everywhere), verifopt. Rewrites are
# unguarded (the default). Every build compiles std from source (-Zbuild-std
# via VERIFOPT_BUILD_STD): VFE needs it - with a prebuilt std it is unsound,
# std reads vtables untyped - and all columns then use the same std. Configs:
#   default   release profile
#   lto       fat LTO
#   lto_vfe   fat LTO + VFE (with verifopt's fixes)
#
# Environment: KS (default "1 4"), CONFIGS (default "default lto lto_vfe"),
# ROWS (only these rows, e.g. "hook_k1 hook_k4"; hook rows need verifopt_k* built),
# PLUGIN_ROOT (cargo-verifopt install root, default /tmp/vo_lp),
# INVOCATIONS (timing runs per build and workload, default 40),
# ITERS (dispatches per timed pass, default 100000000), SEED (default 7),
# CORE (taskset core, default 3), CHECK_ITERS (default 1000003).
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
MODE=${1:?usage: bench_matrix.sh build|check|time [EXAMPLE_DIR BIN]}
EX=$HERE/${2:-twovariant-ex}
BIN=${3:-bench_twovariant}
KS=${KS:-1 4}
CONFIGS=${CONFIGS:-default lto lto_vfe}
PLUGIN_ROOT=${PLUGIN_ROOT:-/tmp/vo_lp}
INVOCATIONS=${INVOCATIONS:-40}
ITERS=${ITERS:-100000000}
SEED=${SEED:-7}
CORE=${CORE:-3}
CHECK_ITERS=${CHECK_ITERS:-1000003}
TARGET=x86_64-unknown-linux-gnu
RESULTS=$HERE/results/$(basename "$EX")
mkdir -p "$RESULTS"

TC=$HOME/.rustup/toolchains/rust-mono-rewrite
export PATH="$TC/bin:$PLUGIN_ROOT/bin:$HOME/.rustup/toolchains/nightly-2026-01-13-x86_64-unknown-linux-gnu/bin:$HOME/.cargo/bin:$PATH"
export LD_LIBRARY_PATH="$TC/lib:${LD_LIBRARY_PATH:-}"
LLVM_BIN=$TC/lib/rustlib/x86_64-unknown-linux-gnu/bin

# rows of a config: plain, then verifopt / rung3 / hook per K (verifopt before
# hook: the hook row builds against a copy of verifopt's store)
rows() { [ -n "${ROWS:-}" ] && { echo $ROWS; return; }; echo plain; for k in $KS; do echo "verifopt_k$k rung3_k$k hook_k$k"; done; }
tdir() { echo "$EX/bm_${1}_${2}_target"; }          # config row
sdir() { echo "$EX/verifopt_results_bm_${1}_${2}_target"; }
binpath() { echo "$(tdir "$1" "$2")/$TARGET/release/$BIN"; }

# cargo verifopt for one (config, row); extra args follow
cv() {
  local cfg=$1 row=$2; shift 2
  local flags="" lto=false
  case $cfg in
    lto) flags="-C lto=fat"; lto=fat ;;
    lto_vfe) flags="-C lto=fat -Zvirtual-function-elimination -Zverifopt-vfe-fixes"; lto=fat ;;
  esac
  ( cd "$EX" && CARGO_PROFILE_RELEASE_LTO=$lto RUSTFLAGS="$flags" VERIFOPT_BUILD_STD=1 \
      VERIFOPT_LOG=monomorph=warn cargo verifopt --release --target-dir "$(tdir "$cfg" "$row")" \
      --bin "$BIN" --target $TARGET "$@" ) > "$RESULTS/build_${cfg}_${row}.log" 2>&1
}

build() {
  for cfg in $CONFIGS; do
    for row in $(rows); do
      rm -rf "$(tdir "$cfg" "$row")" "$(sdir "$cfg" "$row")"
      # a row name ending in _canon builds that row with --canonical
      local base=${row%_canon} canon=()
      [ "$base" != "$row" ] && canon=(--canonical)
      case $base in
        plain) cv "$cfg" "$row" --skip-analysis --skip-rewrite "${canon[@]}" ;;
        verifopt_k*) cv "$cfg" "$row" --max-candidates "${base#verifopt_k}" "${canon[@]}" ;;
        rung3_k*) VERIFOPT_CHA_ONLY=1 cv "$cfg" "$row" --max-candidates "${base#rung3_k}" "${canon[@]}" ;;
        hook_k*)
          # verifopt's store for this K, rewrite off: partitioning only
          local k=${base#hook_k} src
          src=$(sdir "$cfg" "verifopt_k$k")
          [ -f "$src/verifopt_store.json" ] || src=$(sdir "$cfg" "verifopt_k${k}_canon")
          [ -f "$src/verifopt_store.json" ] || { echo "$cfg $row: no verifopt store to copy"; continue; }
          cp -r "$src" "$(sdir "$cfg" "$row")"
          VERIFOPT_STORE_DIR=$(sdir "$cfg" "$row") cv "$cfg" "$row" --skip-analysis --skip-rewrite --max-candidates "$k" "${canon[@]}" ;;
      esac
      echo "$cfg $row rc=$? $(date +%T)"
    done
  done
}

# per build: checksums against plain, edit kinds, and per timed function its
# size and calls (indirect = `call *` / `jmp *`; vector = SIMD registers)
check() {
  local out=$RESULTS/check.txt; : > "$out"
  for cfg in $CONFIGS; do
    local pb; pb=$(binpath "$cfg" plain)
    for row in $(rows); do
      local b; b=$(binpath "$cfg" "$row")
      [ -x "$b" ] || { echo "$cfg $row: no binary" | tee -a "$out"; continue; }
      local bad=0
      for wl in $("$pb" --list); do
        local got rc
        got=$("$b" "$wl" "$CHECK_ITERS" "$SEED" 2>/dev/null); rc=$?
        [ $rc = 0 ] || { bad=1; echo "  EXIT CODE $rc $cfg $row $wl" | tee -a "$out"; }
        [ "$(echo "$got" | cut -d, -f4)" = "$("$pb" "$wl" "$CHECK_ITERS" "$SEED" | cut -d, -f4)" ] || { bad=1; echo "  CHECKSUM MISMATCH $cfg $row $wl" | tee -a "$out"; }
      done
      local edits=""
      [ -f "$(sdir "$cfg" "$row")/verifopt_edit_kind_stats.txt" ] && \
        edits=$(sort "$(sdir "$cfg" "$row")/verifopt_edit_kind_stats.txt" | uniq -c | awk '{c=$1; $1=""; printf "%s%s;", c, $0}')
      echo "$cfg $row checksums+exit $([ $bad = 0 ] && echo ok || echo BAD) | edits $edits" | tee -a "$out"
      "$LLVM_BIN/llvm-nm" -C "$b" | awk '/bench::wl_/ { addr=$1; sub(/^[^ ]+ [^ ]+ /, ""); sub(/.*bench::/, ""); m[addr]=m[addr] (m[addr] ? "," : "") $0 } END { for (a in m) print "    " a ": " m[a] }' | sort | tee -a "$out"
      "$LLVM_BIN/llvm-objdump" -d --no-show-raw-insn -C "$b" | awk '
        /^[0-9a-f]+ <[^>]*bench::wl_/ { fn=$1; n=0; ind=0; dir=0; v=0 }
        fn && /\s(callq?|jmpq?)\s+\*/ { ind++ }
        fn && /\scallq?\s+[0-9a-f]/ { dir++ }
        fn && /%(x|y|z)mm/ { v++ }
        fn && /^$/ { printf "    %-48s instrs %3d indirect %d direct %d vector %d\n", fn, n, ind, dir, v; fn="" }
        fn { n++ }' | tee -a "$out"
    done
  done
}

# INVOCATIONS shuffled runs of every (config, row, workload), pinned to one
# core; one CSV line per run: config,row,workload,iters,ns_per_dispatch,checksum
time_runs() {
  local csv=$RESULTS/timing_$(date +%Y%m%d_%H%M%S).csv
  echo "config,row,workload,iters,ns_per_dispatch,checksum" > "$csv"
  local plan; plan=$(mktemp)
  for cfg in $CONFIGS; do
    for row in $(rows); do
      local b; b=$(binpath "$cfg" "$row")
      for wl in $("$b" --list); do
        for _ in $(seq "$INVOCATIONS"); do echo "$cfg $row $wl"; done
      done
    done
  done | shuf > "$plan"
  local n=0 total; total=$(wc -l < "$plan")
  while read -r cfg row wl; do
    echo "$cfg,$row,$(taskset -c "$CORE" "$(binpath "$cfg" "$row")" "$wl" "$ITERS" "$SEED")" >> "$csv"
    n=$((n + 1)); [ $((n % 100)) = 0 ] && echo "$n / $total $(date +%T)"
  done < "$plan"
  rm -f "$plan"
  echo "timing results: $csv"
}

case $MODE in
  build) build ;;
  check) check ;;
  time) time_runs ;;
  *) echo "unknown mode $MODE"; exit 2 ;;
esac
