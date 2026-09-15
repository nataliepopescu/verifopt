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
# runs against each other.
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
#   CARGO_TARGET_DIR="$(dirname "$(dirname "$(dirname "$not_rw_bin")")")" \
#       "$not_rw_bin" --bench
#   CARGO_TARGET_DIR="$(dirname "$(dirname "$(dirname "$mir_rw_bin")")")" \
#       "$mir_rw_bin" --bench

set -euo pipefail

EXAMPLE_DIR=""
BIN_NAME=""
NOT_RW_NAME=""
MIR_RW_NAME=""
SKIP_DISCOVERY=0

while getopts "d:b:n:m:sh" opt; do
    case "$opt" in
        d) EXAMPLE_DIR="$OPTARG" ;;
        b) BIN_NAME="$OPTARG" ;;
        n) NOT_RW_NAME="$OPTARG" ;;
        m) MIR_RW_NAME="$OPTARG" ;;
        s) SKIP_DISCOVERY=1 ;;
        h)
            sed -n '2,82p' "$0" | sed 's/^# \{0,1\}//'
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
if [ -z "$NOT_RW_NAME" ]; then
    echo "error: -n NOT_REWRITTEN_BENCH is required" >&2
    exit 1
fi
if [ -z "$MIR_RW_NAME" ]; then
    echo "error: -m MIR_REWRITTEN_BENCH is required" >&2
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
    echo "=== discovery pass: cargo verifopt --release --bin $BIN_NAME (main binary) ===" >&2
    (cd "$EXAMPLE_DIR" && cargo clean --target-dir target-discovery "${extra_args[@]}") >&2
    rm -f "$EXAMPLE_DIR/verifopt_store.json"
    discovery_output="$(cd "$EXAMPLE_DIR" && cargo verifopt --release --bin "$BIN_NAME" --target-dir target-discovery --message-format=json "${extra_args[@]}")" || true
    if [ -z "$discovery_output" ]; then
        echo "error: discovery pass (cargo verifopt --release --bin $BIN_NAME) produced no output - build likely failed" >&2
        exit 1
    fi
fi

echo "=== baseline build: cargo verifopt --bench $NOT_RW_NAME --skip-analysis --skip-rewrite ===" >&2
not_rw_output="$(cd "$EXAMPLE_DIR" && cargo verifopt --bench "$NOT_RW_NAME" --skip-analysis --skip-rewrite --target-dir target-not-rw --message-format=json "${extra_args[@]}")" || true
if [ -z "$not_rw_output" ]; then
    echo "error: baseline build (cargo verifopt --bench $NOT_RW_NAME --skip-analysis --skip-rewrite) produced no output - build likely failed" >&2
    exit 1
fi
not_rw_bin="$(discover_binary "$not_rw_output" "bench")" || true
if [ -z "$not_rw_bin" ] || [ ! -x "$not_rw_bin" ]; then
    echo "error: could not discover a built, executable baseline bench binary - compiler diagnostics follow:" >&2
    print_compiler_errors "$not_rw_output"
    exit 1
fi

echo "=== rewritten build: cargo verifopt --bench $MIR_RW_NAME --skip-analysis ===" >&2
mir_rw_output="$(cd "$EXAMPLE_DIR" && cargo verifopt --bench "$MIR_RW_NAME" --skip-analysis --target-dir target-mir-rw --message-format=json "${extra_args[@]}")" || true
if [ -z "$mir_rw_output" ]; then
    echo "error: rewritten build (cargo verifopt --bench $MIR_RW_NAME --skip-analysis) produced no output - build likely failed" >&2
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
