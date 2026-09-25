#!/usr/bin/env bash
#
# run_mir_bench.sh - re-runs discovery against an example's own main
# --bin target, then builds two --bench targets off the results: a
# baseline that's never rewritten, and one that genuinely gets the real
# MIR-level rewrite applied. All three steps always run, in this same
# invocation, so a stale verifopt_store.json (from an earlier discovery
# run, possibly against different source) never silently gets reused
# without the caller explicitly asking for that via -s, and the two
# bench targets can never end up built from two different discovery
# runs against each other. Discovery also deletes any leftover
# verifopt_needs_rewrite_pass marker for the same reason - left over
# from an earlier run, its mere presence (regardless of this
# invocation's own flags) makes cargo-verifopt's own
# call_cargo_on_target always trigger a second, clean-and-rebuild pass,
# since that decision is based purely on the marker file's own
# existence on disk, not on what flags actually got passed this time.
# Deleted again, right after discovery's own flow genuinely completes
# (only inside its own branch below - not when -s skips discovery
# entirely, since a reused store might still genuinely need the
# baseline/rewritten builds to see a marker left from an earlier run) -
# without this second deletion, discovery's own, already-completed
# second pass (if it ran one) leaves the marker sitting on disk, and
# the baseline/rewritten builds' own, separate call_cargo_on_target
# calls each see it too, redundantly re-triggering their own copy of a
# pass the store no longer needs by that point - which is exactly how
# a single edit could end up applied three times instead of once: one
# genuine application during discovery's own second pass, plus two
# redundant, duplicate ones from the rewritten build's own two
# internal passes.
# Each of the three builds also cleans its own --target-dir immediately
# before running - discovery's own target-discovery was already always
# cleaned this way; target-not-rw/target-mir-rw now are too, since
# rustc's own incremental compilation only ever fingerprints
# source-level inputs, with no visibility into the store's own,
# external state changing between runs - so a function whose own
# rewrite already, genuinely succeeded once can otherwise have that
# same, cached codegen result silently reused on a later run, without
# the rewrite hook ever genuinely re-running for it at all, even after
# the store itself changed.
#
# The discovery pass explicitly targets --bin BIN_NAME rather than
# omitting --bin entirely - without an explicit target, cargo-verifopt
# falls through to building *every* target in the package (see its own
# call_cargo/call_cargo_on_each_package_target), which - once any
# [[bench]] targets are registered in Cargo.toml at all - now also
# means every bench target too. Since the discovery pass carries
# --release (valid for the primary binary, invalid for `cargo bench`),
# that would fail with "unexpected argument '--release' found" before
# this script ever reaches its own, later, explicit --bench builds.
#
# Discovery also passes --skip-rewrite - its own job is only ever to
# find edits and write them to the store, never to actually apply any
# of them to its own, throwaway target-discovery build (nothing ever
# benchmarks against that build's own output at all). Without this,
# the modified compiler's own codegen_mir hook still applies any
# matching rewrite it finds unconditionally, purely as an unintended
# side effect - which is also what let a single edit end up looking
# like it applied twice in mir_dump.txt, once genuinely in the
# rewritten build and once, redundantly, here.
#
# Discovery also explicitly passes --bench BENCH_NAME alongside --bin
# BIN_NAME, in the same invocation, rather than discovering against
# --bin alone - confirmed directly, empirically necessary: cargo's own
# -C metadata (its crate-disambiguation hash, which feeds directly into
# rustc's stable crate id, and hence every DefPathHash computed for
# anything inside that crate) depends on the *whole unit graph* a given
# cargo invocation builds - not just the specific target passed to
# --bin/--bench - since building alongside a dev-dependency-requiring
# bench target also pulls in --test-enabled variants of the same
# package that a plain --bin build alone never needs. A --bin-only
# discovery build therefore gives the shared library crate a genuinely
# different -C metadata (and so a different DefPathHash) than the
# --bench build later, separately, recompiles it with - so any edit
# discovery finds for a function defined in that shared crate can never
# match the store's own lookup during the later --bench build at all,
# silently, permanently, regardless of how correct the edit itself is.
# Passing both together puts --bin's own target into the *same* unit
# graph the --bench build already builds on its own, giving the shared
# crate the same metadata hash either way - while --bin itself still
# gives discovery a directly-traceable, static entry point to analyze,
# entirely independent of the --bench target's own, separate entry
# point (criterion's own harness obscures the call path enough that
# discovering through it directly, alone, doesn't work at all).
#
# Both bench targets pass --skip-analysis, so neither one's own codegen
# ever triggers a *second*, redundant discovery pass over its own call
# graph - each only ever picks up whatever the discovery step already
# wrote to verifopt_store.json. The baseline additionally passes
# --skip-rewrite, unconditionally suppressing any rewrite regardless of
# what's in the store - necessary because the modified compiler's own
# codegen_mir hook applies rewrites unconditionally, for every
# monomorphized function in every target it's asked to build; without
# --skip-rewrite, a "baseline" built the same way as the rewritten
# target would silently get rewritten too, since both are genuine dyn
# dispatch call sites with the same discoverable targets. See
# cargo-verifopt's own TargetKind::Bench handling and
# monomorph/src/bin/cargo-verifopt.rs's own --skip-analysis/
# --skip-rewrite docs for more.
#
# Usage:
#   ./run_mir_bench.sh -d EXAMPLE_DIR -b BIN_NAME -n NOT_REWRITTEN_BENCH \
#       -m MIR_REWRITTEN_BENCH [-s] [-- EXTRA_ARGS...]
#
#   -d EXAMPLE_DIR   Directory containing the example's own Cargo.toml
#                    (required).
#   -b BIN_NAME      Name of the [[bin]] target discovery should run
#                    against - usually the package's own name, unless
#                    overridden in Cargo.toml (required).
#   -n BENCH_NAME    Name of the [[bench]] target that should be built
#                    as a never-rewritten baseline (required).
#   -m BENCH_NAME    Name of the [[bench]] target that should be built
#                    with the real MIR-level rewrite applied (required).
#   -s               Skip the discovery pass entirely, reusing whatever
#                    verifopt_store.json already exists on disk. The
#                    explicit opt-out mentioned above - the default,
#                    with this flag absent, always re-runs discovery.
#   -v               Verbose: sets RUSTC_LOG=rustc_codegen_ssa::mir::
#                    verifopt_rewrite=debug for all three builds, so the
#                    rewrite mechanism's own debug!(...) tracing (store
#                    load/hit messages, fn_op's own per-call failure
#                    points, rewrite_monomorphized's own entry log)
#                    prints to stderr. Off (silent) by default.
#   -- EXTRA_ARGS    Forwarded as-is to every cargo verifopt invocation.
#                    Each of the three builds (discovery, baseline,
#                    rewritten) already defaults to its own, separate
#                    --target-dir (target-discovery/target-not-rw/
#                    target-mir-rw) - so cargo can never clean up one
#                    build's own artifact as part of a later,
#                    unrelated build sharing the same directory. Pass
#                    your own --target-dir here to override this
#                    default (it comes after this script's own, so it
#                    wins) - but note doing so reintroduces the
#                    original risk this default exists to avoid.
#
# On success, prints exactly two lines on stdout - nothing else - so
# it's safe to source directly, e.g.:
#   eval "$(./run_mir_bench.sh -d ../../benching_examples/visitor-ex/visitor-use \
#       -b visitor-use -n visitor_not_rw -m visitor_mir_rw)"
#
# Running either binary directly (as below) - rather than through
# cargo bench itself - means nothing tells criterion which --target-dir
# it was originally built with, so it falls back to `cargo metadata`'s
# own default (target/) rather than the target-not-rw//target-mir-rw/
# actually used here; set CARGO_TARGET_DIR explicitly (each binary's
# own target dir is always three directories up from the binary
# itself) so criterion's own reports land where they're expected:
#   CARGO_TARGET_DIR="$(dirname "$(dirname "$(dirname "$not_rw_bin")")")" "$not_rw_bin" --bench
#   CARGO_TARGET_DIR="$(dirname "$(dirname "$(dirname "$mir_rw_bin")")")" "$mir_rw_bin" --bench

set -euo pipefail

SPINNER_PID=""

# start_spinner/stop_spinner: a simple progress indicator for each of
# the three, long-running builds below, written directly to /dev/tty -
# the controlling terminal device itself, a genuinely separate target
# from both stdout (fd 1, captured by the caller's own $(...) command
# substitution, per this script's own "prints exactly two lines on
# stdout" contract above) and stderr (fd 2, typically redirected by the
# caller to its own log file, e.g. `... 2> err.log`). Writing here
# instead means the spinner shows up on-screen without polluting
# either. The actual write attempt below is wrapped in a subshell so
# its own stderr redirection catches a shell-level "no such device"
# setup error too, not just a command-level one - a simpler
# `[ ! -w /dev/tty ]` file test isn't reliable enough on its own, since
# /dev/tty can exist as a device node and pass that test even without
# a genuine, usable controlling terminal attached (e.g. running
# non-interactively, in CI) - in which case the actual write still
# fails, and its own error message would otherwise leak straight to
# stderr regardless of any redirection on the write itself.

start_spinner() {
    if ! ( : > /dev/tty ) 2>/dev/null; then
        return
    fi
    (
        chars='|/-\'
        i=0
        while true; do
            printf '\r%s %s' "${chars:i%${#chars}:1}" "$1" > /dev/tty 2>/dev/null || exit
            i=$((i + 1))
            sleep 0.1
        done
    ) &
    SPINNER_PID=$!
    disown
}

stop_spinner() {
    if [ -n "$SPINNER_PID" ]; then
        kill "$SPINNER_PID" 2>/dev/null || true
        wait "$SPINNER_PID" 2>/dev/null || true
        SPINNER_PID=""
        ( printf '\r%*s\r' 60 "" > /dev/tty ) 2>/dev/null || true
    fi
}

trap stop_spinner EXIT

EXAMPLE_DIR=""
BIN_NAME=""
#NOT_RW_NAME=""
#MIR_RW_NAME=""
BENCH_NAME=""
SKIP_DISCOVERY=0
VERBOSE=0

while getopts "d:b:n:m:svh" opt; do
    case "$opt" in
        d) EXAMPLE_DIR="$OPTARG" ;;
        b) BIN_NAME="$OPTARG" ;;
        #n) NOT_RW_NAME="$OPTARG" ;;
        #m) MIR_RW_NAME="$OPTARG" ;;
        n) BENCH_NAME="$OPTARG" ;;
        s) SKIP_DISCOVERY=1 ;;
        v) VERBOSE=1 ;;
        h)
            sed -n '2,149p' "$0" | sed 's/^# \{0,1\}//'
            exit 0
            ;;
        *)
            echo "error: unknown flag" >&2
            exit 1
            ;;
    esac
done
shift $((OPTIND - 1))
extra_args=("$@")

if [ "$VERBOSE" -eq 1 ]; then
    export RUSTC_LOG="rustc_codegen_ssa::mir::verifopt_rewrite=debug"
fi

if [ -z "$EXAMPLE_DIR" ]; then
    echo "error: -d EXAMPLE_DIR is required" >&2
    exit 1
fi
if [ ! -f "$EXAMPLE_DIR/Cargo.toml" ]; then
    echo "error: no Cargo.toml found under $EXAMPLE_DIR" >&2
    exit 1
fi
if [ -z "$BIN_NAME" ]; then
    echo "error: -b BIN_NAME is required" >&2
    exit 1
fi
#if [ -z "$NOT_RW_NAME" ]; then
#    echo "error: -n NOT_REWRITTEN_BENCH is required" >&2
#    exit 1
#fi
#if [ -z "$MIR_RW_NAME" ]; then
#    echo "error: -m MIR_REWRITTEN_BENCH is required" >&2
#    exit 1
#fi
if [ -z "$BENCH_NAME" ]; then
    echo "error: -n BENCH_NAME is required" >&2
    exit 1
fi

discover_binary() {
    local build_output="$1"
    local want_kind="$2"
    python3 -c '
import json, sys
want_kind = sys.argv[1]
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

# Require an exact kind match - no fallback at all. cargo always
# includes a "kind" field on every compiler-artifact message, so there
# is no genuine "field missing" case to fall back for. A fallback here
# is never actually safe: even when there is only one candidate, it can
# still be the wrong one if its own kind does not match what was asked
# for - e.g. the primary --bin target, still sitting in the same build
# graph from an earlier pass, re-reported here even though the actual,
# requested target (a --bench) never produced an artifact at all. If
# nothing matches the requested kind, that is a real failure and should
# be reported as such, not silently papered over by guessing.
matching = [msg["executable"] for msg in candidates if want_kind in ((msg.get("target") or {}).get("kind") or [])]
if matching:
    print(matching[-1])
' "$want_kind" <<< "$build_output"
}

# Extracts the actual compiler diagnostics (errors/panics) from
# --message-format=json output and prints them to stderr - called
# whenever a build fails to produce a binary, since --message-format=
# json redirects the detailed diagnostic text into JSON on stdout,
# where this script's own $(...) captures it into a variable rather
# than letting it reach the terminal at all. Without this, a failed
# build looks like it produced "no output" even though it actually
# produced a real, specific error - just one this script was
# swallowing rather than showing.
print_compiler_errors() {
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
    if msg.get("reason") == "compiler-message":
        inner = msg.get("message") or {}
        if inner.get("level") == "error":
            rendered = inner.get("rendered")
            if rendered:
                print(rendered, end="")
' <<< "$build_output" >&2
}

if [ "$SKIP_DISCOVERY" -eq 1 ]; then
    echo "=== -s passed: skipping discovery pass, reusing existing verifopt_store.json ===" >&2
else
    echo "=== discovery pass: cargo verifopt --bench $BENCH_NAME --bin $BIN_NAME --skip-rewrite (combined unit graph) ===" >&2
    (cd "$EXAMPLE_DIR" && cargo clean --target-dir target-discovery "${extra_args[@]}") >&2
    rm -rf "$EXAMPLE_DIR/verifopt_results"
    start_spinner "discovery pass running..."
    discovery_output="$(cd "$EXAMPLE_DIR" && cargo verifopt --bench "$BENCH_NAME" --bin "$BIN_NAME" --skip-rewrite --target-dir target-discovery --message-format=json "${extra_args[@]}")" || true
    stop_spinner
    if [ -z "$discovery_output" ]; then
        echo "error: discovery pass (cargo verifopt --bench $BENCH_NAME --bin $BIN_NAME --skip-rewrite) produced no output - build likely failed" >&2
        exit 1
    fi
    rm -f "$EXAMPLE_DIR/verifopt_results/verifopt_needs_rewrite_pass"
fi

echo "=== baseline build: cargo verifopt --bench $BENCH_NAME --skip-analysis --skip-rewrite ===" >&2
(cd "$EXAMPLE_DIR" && cargo clean --target-dir target-not-rw "${extra_args[@]}") >&2
start_spinner "baseline build running..."
not_rw_output="$(cd "$EXAMPLE_DIR" && cargo verifopt --bench "$BENCH_NAME" --skip-analysis --skip-rewrite --target-dir target-not-rw --message-format=json "${extra_args[@]}")" || true
stop_spinner
if [ -z "$not_rw_output" ]; then
    echo "error: baseline build (cargo verifopt --bench $BENCH_NAME --skip-analysis --skip-rewrite) produced no output - build likely failed" >&2
    exit 1
fi
not_rw_bin="$(discover_binary "$not_rw_output" "bench")" || true
if [ -z "$not_rw_bin" ] || [ ! -x "$not_rw_bin" ]; then
    echo "error: could not discover a built, executable baseline bench binary - compiler diagnostics follow:" >&2
    print_compiler_errors "$not_rw_output"
    exit 1
fi

echo "=== rewritten build: cargo verifopt --bench $BENCH_NAME --skip-analysis ===" >&2
(cd "$EXAMPLE_DIR" && cargo clean --target-dir target-mir-rw "${extra_args[@]}") >&2
start_spinner "rewritten build running..."
mir_rw_output="$(cd "$EXAMPLE_DIR" && cargo verifopt --bench "$BENCH_NAME" --skip-analysis --target-dir target-mir-rw --message-format=json "${extra_args[@]}")" || true
stop_spinner
if [ -z "$mir_rw_output" ]; then
    echo "error: rewritten build (cargo verifopt --bench $BENCH_NAME --skip-analysis) produced no output - build likely failed" >&2
    exit 1
fi
mir_rw_bin="$(discover_binary "$mir_rw_output" "bench")" || true
if [ -z "$mir_rw_bin" ] || [ ! -x "$mir_rw_bin" ]; then
    echo "error: could not discover a built, executable rewritten bench binary - compiler diagnostics follow:" >&2
    print_compiler_errors "$mir_rw_output"
    exit 1
fi

echo "not_rw_bin=$not_rw_bin"
echo "mir_rw_bin=$mir_rw_bin"
