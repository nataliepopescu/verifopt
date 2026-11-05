# verifopt — handoff notes

Context for continuing this work in Claude Code. Started at the end of a long
debugging/design conversation (Sept–Oct 2026), last updated Nov 4 2026. Verify against
`git log` in each repo before assuming anything below is applied.

Lives in the verifopt repo (`~/hack/verifopt/CLAUDE.md`, versioned there);
`~/hack/verifopt-examples/ripgrep/CLAUDE.md` is a symlink to it, so sessions
started in the ripgrep checkout load it too.

## 1. What verifopt is

A devirtualization tool for Rust: a whole-program abstract interpretation
("FSA") finds which concrete functions each dynamic dispatch (`dyn Trait`
method call) can reach; a modified rustc then rewrites those MIR call sites
into direct calls (optionally guarded by a runtime check).

Two halves:

- **Analysis plugin** — `~/hack/verifopt/monomorph` (repo `nataliepopescu/verifopt`,
  branch `ripgrep`). A rustc driver (`verifopt` binary, run through the
  `cargo-verifopt` cargo subcommand) using rustc_public / stable MIR.
  Key files: `src/interp.rs` (interpreter), `src/constraints.rs`,
  `src/convert.rs`, `src/merge.rs`, `src/stdlib_stubs.rs` (hand-written
  summaries for BTreeMap/BTreeSet, Box/Arc/Rc::new, …), `src/rewrite.rs`
  (after_analysis: writes the store, CHAmax, stats), `src/chamax.rs`,
  `src/assumptions.rs`, `src/logger.rs` (stats), `src/bin/cargo-verifopt.rs`.
- **Modified compiler** — `~/hack/rust` (repo `nataliepopescu/rust`, branch
  `verifopt-monomorph-rewrite-pinned`). Rewrite in
  `compiler/rustc_codegen_ssa/src/mir/verifopt_rewrite.rs`; shared format/hashing
  crate `compiler/rustc_verifopt/` (store, shapes, hashing, instance manifests);
  partitioning hook in `compiler/rustc_monomorphize/src/partitioning.rs`.

Examples: `~/hack/verifopt-examples/ripgrep`, `~/hack/verifopt-examples/tock`
(tock fork `nataliepopescu/tock`, branch `verifopt`; board `boards/imix`).
Fixtures + harness: `~/hack/verifopt/testing_examples/*`,
`monomorph/tests/dispatch_examples.rs`, `monomorph/tests/support/mod.rs`,
goldens in `monomorph/tests/golden/`.

## 2. Build & run

**Compiler** (toolchain name `rust-mono-rewrite`, linked to `build_cp/.../stage2`):

```
cd ~/hack/rust
./x.py build --stage 2 && ./x.py dist rustc-dev --stage 2 && ./cp.sh && rm -rf build_cp && cp -r build/ build_cp
# for VERIFOPT_BUILD_STD: the toolchain needs the library sources
mkdir -p build_cp/x86_64-unknown-linux-gnu/stage2/lib/rustlib/src && \
  ln -sfn ~/hack/rust build_cp/x86_64-unknown-linux-gnu/stage2/lib/rustlib/src/rust
```

`cp.sh` copies rustc-private .rmeta/.so into the stage2 sysroot and then prunes
stale builds of *compiler workspace crates only* (those with a dir under
`compiler/`). Never prune other crates (std's libc, rustc_hash…) — that broke
the sysroot once. If "multiple candidates for rustc_driver" appears, rerun
cp.sh; if std's `liblibc-*.rlib` is missing: `rm -rf build/.../stage2` and
rebuild.

**Plugin**: `cd ~/hack/verifopt/monomorph && cargo --locked install --path .`
(rebuild after every compiler rebuild). **Don't reinstall while a long
analysis run is in progress** — it broke two runs (`cargo-verifopt (deleted)`).
Install to a separate `--root` if you need to keep working.

**Running** (from the example's dir):

```
cargo clean --target-dir verifopt_target
VERIFOPT_LOG=monomorph=warn RUSTC_LOG=rustc_codegen_ssa::mir::verifopt_rewrite=debug,rustc_monomorphize::partitioning=debug \
  cargo verifopt --release --target-dir verifopt_target --bin rg &> out
```

- `--skip-analysis`: rewrite-only against an existing store (refuses to run
  without one, unless `--skip-rewrite`). `--skip-rewrite`: no rewriting.
  `--rewrite-only`: plain rebuild against the store, no clean.
  Baseline build: `--target-dir plain_target --skip-analysis --skip-rewrite`.
- `--max-candidates N` (env `VERIFOPT_MAX_CANDIDATES`, default 4): max
  candidates for a Pointers rewrite.
- `--unguarded-single` (env `VERIFOPT_UNGUARDED_SINGLE`): also emit unguarded
  `single` for 1-target sites that aren't CHAmax-proven. Unsound if FSA
  missed a target; off by default (those sites get `pointers n=1`).
- `VERIFOPT_BUILD_STD=1`: build std/core from source with the fork (adds
  `-Zbuild-std` + `--target`; pass `--target x86_64-unknown-linux-gnu`
  explicitly unless the toolchain-target patch is in). Output then under
  `<target-dir>/x86_64-unknown-linux-gnu/release/`.
- Use `--bin rg` for ripgrep: every analyzed target overwrites the shared
  results (known limitation).
- Results dir: `verifopt_results/` (no `--target-dir`) or
  `verifopt_results_<target dir name>/` — contains `verifopt_store.json`,
  `stats`, `verifopt_edit_kind_stats.txt`, `verifopt_mir_dumps/<crate>-<id>.txt`,
  `instance_manifests/`, the two-pass marker. `VERIFOPT_STORE_DIR` overrides.
- A store is only valid for the exact build config it was analyzed with
  (profile, features, flags, toolchain → crate ids → DefPathHashes). Changing
  any of these (even codegen-units) silently matches nothing; newer stores
  record crate ids and the compiler warns on mismatch.
- **CHAmax needs a clean build**: instance manifests are written when a crate
  is *compiled*; deps cargo considers fresh write none, and every site's
  CHAmax becomes "incomplete" (no `single exact`). Always `cargo clean
  --target-dir …` before an analysis run.
- Current ripgrep cost (Nov 4 2026, build-std, `c85db8c`, uncapped): the
  `rg` analysis takes ~16 min and ~4.6 GB (whole pipeline ~24 min); several
  fit on the 62 GB machine at once - but each needs its *own* install root
  (run scripts stop every `verifopt` of their root: two runs from one root
  killed each other twice). Run experiments from a separate install root
  (`cargo install --path . --root /tmp/vo_x`, then put `/tmp/vo_x/bin`
  first on PATH: cargo-verifopt runs the `verifopt` next to itself).
- **Diagnostics** (all at warn level, so visible with
  `VERIFOPT_LOG=monomorph=warn`):
  - `[verifopt progress]` lines (always on): every 4096th merge, logged when
    the merged value's size at least doubles - steadily climbing sizes mean
    values keep growing. `VERIFOPT_SIZE_HISTOGRAM=1` adds a composition
    histogram (node kinds, max depth) for values over 1M nodes.
  - `VERIFOPT_CACHE_SIZES=N`: every N bb visits, `[verifopt memory]` with
    RSS, memo/summary entries and value/key sizes, worklist, scope stores.
    Its size walks are expensive — not for timing runs.
  - `VERIFOPT_TIMING=1`: per-scope timing spans (off by default; they cost
    memory per call context).
  - `VERIFOPT_TRACE_DISPATCH=<part of a trait method name>` (e.g.
    `Display::fmt`): every virtual_call_memo miss for that method - site,
    argument fingerprint and size, reason (no entry / stale scopes with their
    names / statics changed with their names), caller-on-stack count - and
    each simulation's length in bb visits (`[verifopt dispatch end]`).
  - `VERIFOPT_MERGE_STATS=<seconds>`: per window, context merges after
    candidate simulation - count, time, scope keys visited / actually merged
    - and the scopes and variables (scope + place) whose merges took
    longest, with the merged values' sizes.
  - `VERIFOPT_CHECK_DROPPED=1` (or `=<file>`, appended to - fixture runs
    keep no log): a read that finds a dropped temporary missing
    (`[verifopt dropped read]`), with the reading block, where the local is
    assigned, the block's predecessors, whether the block is reachable in
    its walk (a path from bb0 through visited blocks; "unreachable here"),
    and for reachable reads the blocks themselves (the analysis's MIR numbers
    blocks differently from `-Zunpretty=mir`). Reads made by an assignment's
    old-value lookup show up too (false positives).
    `VERIFOPT_TRACE_DROP=<name part>`: each walk of matching scopes (id,
    ordering, blocks visited, how it ends) and each drop.
    `VERIFOPT_KEEP_TEMPORARIES=1` turns dropping off. `[verifopt memory
    extra]` counts dropped entries (`dropped_temporaries=`).
  - `VERIFOPT_TRACE_SCOPE=<name part>` also logs, per switch in a matching
    scope, the discriminant's value and branch counts.
  - Visit counts don't compare across plugin versions that re-simulate
    differently (Nov 3: the epoch fix halved visits for the same results);
    compare runs by wall time to the same state (memo entries) or to the end.
  - `STACK DESYNC` panics print the last 64 call/key-stack operations.
  - Profiling a running analysis: `perf record -F 49 --call-graph
    dwarf,16384 -p <pid> -- sleep 8` (read-only); `heaptrack` from
    `nix-build '<nixpkgs>' -A heaptrack` (wrap the `verifopt` binary for the
    `rg` crate only; ~15 min of a run is plenty).

**Tests**: `cd monomorph && cargo test --test dispatch_examples` (add
`BLESS_GOLDEN=1` to re-bless; checks stats → MIR dump → call trace in order).
Fixture call traces only catch wrong targets if behaviour depends on the
call's result. The harness runs the `cargo-verifopt` built in monomorph's own
target dir (`CARGO_BIN_EXE_cargo-verifopt`), so tests need no reinstall and
are safe while a long analysis runs on the installed binary.

`cargo` isn't on the base PATH: build/test the plugin inside
`nix-shell ~/hack/rust/shell.nix --run 'export PATH=$HOME/.cargo/bin:$PATH; …'`
(it provides the libxml2/zlib/LLVM libraries the plugin links against; the
examples' `~/hack/verifopt-examples/shell.nix` is enough for `cargo verifopt`
runs), and tock
inside `nix-shell ../../shell-local.nix` (from `boards/imix`). `python3` isn't
on the base PATH either (the dump script below needs a nix-shell or awk).

**Useful greps / scripts**

```
grep -F '[verifopt skip]' out | sed -E 's/.*crate=([^ ]*) reason=([^ ]*).*/\1 \2/' | sort | uniq -c | sort -rn
grep -F '[verifopt drop]' out | head
grep -F '[fn_op] FAILED' out | sed -E 's/.*\[fn_op\] (FAILED[^,:=]*).*/\1/' | sort | uniq -c
sort verifopt_results_*/verifopt_edit_kind_stats.txt | uniq -c
python3 - <<'EOF'   # rewritten functions per crate
import re, glob
for p in sorted(glob.glob('verifopt_results_*/verifopt_mir_dumps/*.txt')):
    t=open(p).read(); b=re.findall(r'######### MIR (before|after) #########\n(.*?)######### END \1 #########', t, re.S)
    print(p, len(b)//2, "dumped,", sum(b[i][1]!=b[i+1][1] for i in range(0,len(b)-1,2)), "rewritten")
EOF
```

Environment (NixOS, tock nix-shell): a `rust-lld` wrapper on PATH
(`writeShellScriptBin "rust-lld" "exec ${pkgs.lld}/bin/ld.lld \"$@\""`),
the Jan-13 nightly cargo first on PATH (for `-Zbuild-std`), LD_LIBRARY_PATH
pointing at the `rust-mono-rewrite` toolchain.

## 3. How the pipeline works (current design)

- **Two-pass flow** (`cargo-verifopt` `build_targets`): pass 1 builds all
  targets; the final crate's analysis writes the store (+ marker if dispatch
  sites live in dependency crates); if marked: one `cargo clean`, pass 2
  rebuilds everything with `--skip-analysis` so deps are rewritten with the
  store. Outputs (dumps, edit kinds) are cleared at the start and before pass 2.
- **Store keys**: (caller DefPathHash, bb index, caller instance generic-arg
  hashes). Targets: (function DefPathHash, Option<arg hashes>) — normally the
  *Self ADT's* args (or none, for CHA candidates). `Store::exact` marks sites
  whose single target is an *exact instance* (function + its own args).
  Shapes (`ArgShape`) decode structural sentinel hashes.
- **Edit kinds** (compute_edits → plan in apply_edits):
  `single exact` (CHAmax-proven single instance; the only unguarded kind by
  default), `single` (unguarded direct call, only with `--unguarded-single`),
  `pointers n=K [of=M]` (compare vtable slot to each candidate's fn ptr,
  direct call on match, original dyn call as fallback; any subset of real
  targets is sound), `tagged n=K` (switch on a tag set at the cast sites;
  fallback = dyn call; only when all casts are in the dispatching function —
  currently never applies in ripgrep/tock), `skipped too_many_targets n=K`.
- **Safety checks before any edit**: the call at the stored bb must resolve
  to `InstanceKind::Virtual`; every target must implement the dispatched trait
  method (default body, impl item, or closure/fn via Fn-traits); targets must
  be linkable from the current codegen unit (`Linkable`).
- **Target resolution**: `resolve_target_via_trait` (build Self ADT from
  recorded args, resolve the trait method → impl instance); CHA candidates
  without args are expanded to the instances compiled in this crate
  (Pointers only). Unresolvable/unlinkable candidates are dropped
  individually; Single/Tagged fall back to Pointers. Skip reason
  `unresolved` = every target of the site was dropped (e.g. a CHA candidate
  of a generic impl with no compiled instance: `[fn_op] FAILED at args.len()
  check: args=[]` then `[verifopt drop] … no linkable instance`); the dyn
  call is kept, which is sound.
- **CHA impl filter** (`get_cha_tyconstraint_defids` →
  `trait_params_compatible`): call-site trait args vs each impl's `TraitRef`
  args are compared with `generic_arg_may_unify` (lifetimes ignored, params
  match anything, otherwise structural) — never `==`, which dropped every
  impl of traits with their own lifetime/type params (commit `4875a88`,
  fixture `cha_lifetime_trait`). The analysis log's `CHA missing impl:` error
  (FSA found an impl CHA lacks) is the cross-check for this class of bug;
  such sites get CHAmax `incomplete: CHA lacks FSA's targets (analysis bug)`.
- **Partitioning hook** (`add_verifopt_rewrite_uses`): adds "caller uses
  target" edges for stored sites so targets aren't internalized into another
  CGU (fixed ripgrep's `rg::search`/`rg::files` being "unlinkable").
- **Instance manifests / CHAmax**: every crate compiled under cargo-verifopt
  writes its compiled trait-method/default/closure instances (+ shapes) to
  `instance_manifests/`. The analysis (after writing the store, it triggers
  `collect_and_partition_mono_items`) unions them: CHAmax = compiled
  instances of each site's CHA candidates (whole program). "incomplete" when
  a crate lacks a manifest (e.g. precompiled std); "n/a" for closure dispatch
  (CHA has no closures). Complete CHAmax == 1 → recorded as exact single.
- **Stats** (`stats`): maybe/non-examples by FSA vs CHA (parsed by harness:
  only CHA/FSA lines after `Span:`), plus CHAmax lines and summary, plus a
  `--ASSUMPTIONS--` section (no-MIR calls with risk flags, stubs, widenings,
  memo reuse under widened keys, fn-pointer calls not followed, recursion
  depth limit, captured values widened).

### Analysis internals (plugin, as of Oct 20 2026)

- **Values**: a pointer's value is its pointee's; `Index` projections are
  no-ops, so an array's value is the union of its elements (`Idk`) and a
  pointer to an array is a pointer to any element. Array literals carry
  their elements; `BinOp::Offset` keeps the base pointer's value; a
  constant fn item / closure cast to a fn pointer stays `FnDef` / `Closure`,
  so calls through it are interpreted (this is what lets `fmt::write` reach
  `Display::fmt`).
- **Closures** carry their captured values (`Closure(def, args, captures)`,
  in both cfc and toc), read/written like ADT fields. Calls in the
  rust-call form `Fn*::call*(self, (a, ..))` - static or virtual - bind
  `self` to `_1` and untuple the arguments into `_2..` (`rust_call_arity`,
  `resolve_rust_call_args`); closures called through a fn pointer get their
  parameters at `_2..` directly.
- **Value sets**: `Constraints` caches its hash; `push` merges an incoming
  ADT/closure disjunct with any existing one of the same shape that differs
  in at most one field (lossless - no field-value combination added).
  Captured values are widened above 5000 nodes, like written fields.
- **Per-context state** (scope = instance + generic args + call context):
  MIR bodies and block orderings are cached per *instance*; the alias map
  `refs` is keyed by scope and a scope's aliases are dropped when it
  finishes and is off the call stack (`finish_frame`); per-scope timing is
  opt-in. Merging a candidate's context back only visits the scope stores
  either side wrote (`TrackedMap` records writes while a candidate
  simulation runs) and skips entries identical by pointer.
- **Memo validity** (as of Nov 3, `536c27e`, `f48c511`): a memoized result
  (exact memo, virtual-call memo, summaries) records, for each static it
  read, a fingerprint of that static's store as the call first saw it
  (`store_fingerprint`); a hit needs the current context to hold the same
  value (`reads_fresh(log, cstore)`). No global per-static versions any more:
  values are per context, and a version bumped by writes in other contexts
  (candidate copies, branches) invalidated results whose inputs hadn't
  changed. Frozen statics (immutable allocation, no interior mutability) are
  never recorded; materializing a static's starting value in a context is not
  a write (`materialize_static`). A scope's epoch (the other staleness check)
  moves only when merging new arguments into its store changes its entries
  (`TrackedMap::same_entries`), not when the merge merely builds a new map.
  Remaining stale misses are real: a scope's store accumulates the union of
  all argument values it is called with.
- **Finished calls' temporaries** (Nov 4, `02e9c1d`, `a38e632`): when a
  walk of a scope's body ends (`visit_body` - not at the Return terminator:
  the walk order can put blocks after it, and finish_frame's final traverse
  after recursive calls is a walk nested in the outer one), no other walk of
  the scope is in progress (`active_walks`) and the scope is off the call
  stack, its droppable locals leave its store (`drop_scope_locals`):
  locals after the arguments that no borrow mentions (`&`, `&mut`, `&raw`
  of the local, a field of it, or through it - `&(*L)` is an alias of
  `(*L)`, resolved through L's value), in bodies that create no closure or
  coroutine, have no inline assembly and aren't coroutines. `_0` and the
  arguments stay. Before, every helper's temporaries were merged back after
  each candidate simulation - most of ripgrep's analysis time.
- **Switch pruning by reachability** (Nov 4, `4ace1b6`): `BBDeps::prune`
  records the dead switch edge (`dead_edges`, per walk; empty in the
  template) and keeps only blocks reachable from bb0 over the other edges.
  Predecessor counting kept dead loops (their back edge stays a
  predecessor) and cut join blocks another live block still jumps to.
- **Calls without a result** (Nov 4, `c85db8c`): a call that returns
  `Ok(None)` writes nothing to its destination only for a reason that makes
  that right (`NoResult`: no reachable Return in the callee's walk, or a
  recursive call still being solved), or a `!` / `()` destination;
  otherwise the destination gets `any_value` of its type, recorded as
  `CallResultUnknown`. Before, it kept an earlier call's value.
- **Errors**: call error paths truncate both stacks to their pre-call length
  (`unwind_stacks_to`); nested re-interpretation deeper than `MAX_DEPTH`
  (50) answers the call from its signature ("recursion depth limit").
  `InterpPass::run`'s result is ignored (`lib.rs`), so any error that reaches
  it silently ends the analysis with partial results — watch for low site
  counts.

## 4. Results so far

- **ripgrep, Oct 19–20** (build-std, `--bin rg`, `--max-candidates 1`, after
  the fn-pointer / captures / performance / desync work, commits
  `367c7dc`..`aa1eea0`): analysis completes - 13,766 s (3 h 49 min), peak
  ~28.8 GB, 163 sites (more code reachable than the 147 before), no panics.
  11 `pointers n=1` edits (std 8 functions, ignore 1, log 1);
  `Formatter::write_fmt` is no longer a single-target site. Output identical
  to plain on 18 commands (`--json` with timings stripped). That run reused
  its target dir, so CHAmax was "incomplete" everywhere. Assumptions grew
  with reachability: 9.8M widened merges, 91k memo reuses under widened keys
  (40 functions), 1,236 functions with args mismatch, 4 sites with >3,000
  stored targets.
- **ripgrep, Oct 20 clean run** (`clean_target`, same flags, after `cargo
  clean`, commit `797d637` - dirty-key merges + cheap identity checks):
  analysis **196 min** (was 229), peak ~28 GB, whole pipeline 3 h 18 min; no
  panics; every site's CHA/FSA sets identical to the previous (full-merge)
  run. CHAmax complete at 140 sites, n/a at 23: only 2 are exactly 1 (both
  `ignore::WalkParallel::visit` → `single exact`), 2 are 0 (dead); most
  are large (60× 104 Flag impls, 19× 24 `fmt::Write` writers, 880 / 506 /
  225 for formatting dispatch). FSA < CHAmax at 28 sites, == at 2, CHA
  fallback at 106, FSA > CHAmax at 4. Edits: 2 `single exact`, 9 `pointers
  n=1` (std 8 functions, ignore 1, log 1). Output identical to plain on 19
  commands. Results in `verifopt_results_clean_target/`.
- History of that work (Oct 18–19): closure captures made the first run
  take 6 h+ (values of 2–5M nodes: `Vec`/`String` internals differing only in
  scalars) → cached hashes + lossless merge; then memory/CPU in per-context
  state (heaptrack: alias map ~17% heap and ~28% CPU via a conflict scan
  whose result was never read; timing ~9%; per-scope MIR copies) →
  per-instance caches, alias pruning; merges ~80% CPU → identity fast path
  (240k visits: 534 s → 56 s). STACK DESYNC came from error paths popping
  the caller's frame; fixing it exposed a RecurseLimit error that silently
  truncated the analysis (51 sites).
- **ripgrep, older**: rewritten binary output identical to stock (search, `--sort
  path`, `--files`, error cases). Text +8.2 KiB (+0.17%); growth in `rg::run`
  (rewritten loops) and `FilterMap<Walk,…>::next` (kept alive as compared
  target). Older edit kinds: 34 single, 10 pointers. 152 sites; many
  CHA-fallback sites in flag parsing (104 Flag impls) — not rewritable.
  Remaining skips: `Display::fmt` sites whose only target is
  `<String as fmt::Write>::write_str` (inline std fn, not compiled locally;
  cold code; also a soundness question — formatter writer may be stderr's
  adapter through `core::fmt::write`, invisible without build-std).
- **tock (imix)**: imix 15/15 functions rewritten, components 1/1, sam4l 1/12,
  kernel/capsules 0 — 72 library-crate sites "unresolved" (separate
  compilation: targets like `ProcessStandard<'_, Sam4l, …>::processid` only
  exist in imix). Edit kinds: 52 `pointers n=1`, 1 `pointers n=4`, 14 single.
  Text 168396 → 169564 (+1.1 KiB ≈ the 52 guarded single-candidate sites).
  CHAmax: all 181 sites complete; 122 have exactly one instance → exact-single
  patch should turn most `pointers n=1` into `single exact` (needs a fresh
  analysis). Boot test on hardware still pending.
- **tock (imix), merged crate** (run Oct 17, `--max-candidates 1`, build-std;
  *before* the CHA fix and the guarded-single mitigation): 181 sites; edit
  kinds 99 `single exact`, 7 `single`, 19 `skipped too_many_targets`;
  4 `unresolved` (sam4l I2C slave closures → `I2CMasterSlaveDriver`, never
  instantiated in imix). 41 of 45 dumped functions rewritten (was 17).
  `.text` 167200 (plain) → 166940 (−260 B). CHAmax: 174 complete, FSA found
  nothing at 138 (CHA fallback) and only matched CHAmax at the other 36 — the
  wins come from CHAmax, not FSA precision. 32 sites have CHAmax 0 (no
  compiled impl: dead). The 7 `single` were the CHA-bug sites below (stats
  mislabelled them "n/a (closure dispatch)"); 27 more had CHA = FSA = ∅, so
  no callee was interpreted there. Biggest skips: `Kernel::handle_syscall`
  closure and `try_allocate_grant` (81 SyscallDriver targets),
  `Ast::handle_interrupt` (39), `core::fmt::write` (9).
  Pending: re-run with the CHA fix (expect 0 `CHA missing impl`, ~1
  `CHA (0)` site — `eic::Client` has no impls); consider `--max-candidates 4`
  (would add 5× `RF233::reset` GPIO n=2, `SpiHw` n=2, `DMAChannel` n=4);
  look at the many `[args mismatch] ProcessStandard … (generic arg (may be a
  closure))` assumption entries.

- **ripgrep, Oct 21** (`replay_target`, memo write replay + both Oct 21
  performance fixes, `--max-candidates 1`, with `VERIFOPT_CACHE_SIZES=5000`):
  pipeline 13,340 s, analysis ~13,130 s (inflated by the size walks), 2.37M
  bb visits (was ~1.8M), peak 17.4 GB. Every site's FSA set identical to the
  Oct 20 `memo_target` run (163 sites); 74,738 of 480,980 memo hits replayed
  writes (681 functions, mostly fmt / `Vec<u8>`). Same 11 edits; the binary
  is byte-identical to `clean_target`'s; output identical to a plain build
  (`baseline_target`) on 22 commands (`bash rg_cmp.sh PLAIN NEW` in the ripgrep checkout; it
  searches `crates/` - the untracked `*_target` dirs make whole-repo
  searches crawl).
- **tock (imix), merged crate, Oct 21** (`diag_target`, same plugin build,
  `--max-candidates 1`): first completed run since the merge - pipeline
  2,538 s (plain 60 s), 471k bb visits, peak 2.7 GB; 211 sites, CHAmax
  complete at all; FSA found nothing at 188 (CHA fallback). Edits: 134
  `single exact`, 2 `pointers n=1`, 41 `too_many_targets` (largest 1,968 /
  1,969); 66 of 70 dumped functions rewritten; `.text` 164,068 → 163,544
  (−524 B, plain build in `plain2_target`). No aborts, no `CHA missing
  impl`. Boot test still pending.

- **Oct 21 timing runs** (installed plugin `bd24ae9`, no diagnostics):
  ripgrep `timing_target` pipeline 12,487 s (Oct 20 without replay:
  12,067 s), results and binary identical to `replay_target`; tock
  `boards/imix/timing_target` pipeline 2,493 s (analysis ~39 min; plain
  60 s), identical to `diag_target`. (The ripgrep run shared the machine with
  the tock run for its first 40 min.)
- **Size comparison: plain / partitioning baseline / rewritten** (Oct 21).
  Partitioning baseline = `--skip-analysis --skip-rewrite` with
  `VERIFOPT_STORE_DIR` at a *copy* of the analyzed results (the hook reads
  any store it finds; the rewrite is off; a skip-analysis build clears the
  dir's dumps/edit kinds). Dirs `plain_target`, `hook_target`,
  `timing_target` in both projects.
  ripgrep `.text`: 2,787,554 / 2,778,626 / 2,780,034 - the hook alone −8.9 KB
  (CGU regrouping changes inlining), the 11 edits +1.4 KB on top. tock
  `.text`: 164,068 / 164,068 / 163,544 - hook build byte-identical to plain
  (codegen-units=1 + fat LTO: partitioning can't matter), so −524 B is the
  rewrite's own. Outputs (rg_cmp.sh): plain = hook = rewritten.
- **tock "167 exact vs 134 single exact"** (investigated Oct 21): the 33
  exact sites never rewritten are all in code imix never compiles - 24 in
  `Grant::access_grant_with_allocator` instances for the test/tutorial
  capsules `double_grant_entry`, `syscall_return_test`,
  `encryption_oracle_chkpt1`, 8 in `imix::test::{i2c_dummy,spi_loopback}`,
  1 in `ProcessGrant::new_if_allocated` (same test T). They're reached
  because an empty-FSA dispatch interprets every non-generic CHA candidate
  impl, used or not; conversely the generic capsules imix does use (adc,
  alarm, console, ...) are never interpreted from CHA (no args), so sites
  inside them go unrecorded. Idea: fall back to the candidates' *compiled
  instances* (mono collection before the analysis) instead of CHA impls.

- **tock (imix), Oct 22** (`boards/imix/statics_target`, plugin `371af2f` +
  uncommitted union-read fix and per-static memo versions, installed in
  `/tmp/vo_ptrs`, `--max-candidates 1`, no diagnostics): pipeline 8,091 s
  (Oct 21: 2,493 s), 369 sites (was 211; writes through raw pointers /
  statics reach more code), CHAmax complete at all; FSA found nothing at
  332. Edits: 270 `single exact`, 2 `pointers n=1`, 52 `too_many_targets`;
  `.text` 164,068 → 162,972 (−1,096 B). 22 `core::fmt` caller sites; no
  aborts, no `CHA missing impl`. New assumption: 78 fn-pointer calls not
  followed in `core::fmt::rt::Argument::fmt` (callee only a typed `FnPtr` -
  the `formatter` field isn't known there), not yet investigated.

- **tock size breakdown + VFE** (Oct 23, `statics_target` vs plain; scripts
  in the job tmp `py/sites.py`, `vtables.py`, `fndiff.py`): 272 rewritten
  sites, 212 of them `Process` → `ProcessStandard` (`processid` 85,
  `enter_grant` 62; 120 in `access_grant_with_allocator` instances). No
  vtable disappears (all `dyn` casts remain); the −1,096 B is inlining of
  the now-direct calls (`blx rN` 731 → 526; `kernel_loop` −356 B, 17×
  `allocate_grant` −12 B). Tock ships `boards/cargo/virtual_function_elimination.toml`
  (`-Zvirtual-function-elimination`, not included by imix): with it rustc
  emits `llvm.type.checked.load`, so LTO's whole-program devirtualization
  turns single-impl calls direct by itself and dead vtable slots are nulled.
  `.text`: plain 164,068, rewritten 162,972, plain+VFE 158,668 (`blx` 250),
  rewritten+VFE 158,428 (`plainvfe_target`, `rwvfe_target`; RUSTFLAGS must
  repeat tock_flags.toml's flags, since the env var replaces them). So for
  size, VFE alone gets ~5× the rewrite's gain (−5,400 vs −1,096 B); the rewrite adds −240 B on
  top (`ProcessStandard`'s vtable: 9/60 slots null with VFE, 15/60 with
  both). Baselines that matter: compare against plain+VFE.
  **Unread vtables** (pre-LTO IR of the rewritten VFE build, `-C save-temps`,
  `rwvfeir_target/.../imix-*.lto.after-restriction.bc`; script
  `py/vtread.py`): 158 vtables (3,660 B, 71 traits); 595 tagged dyn calls
  remain, so only 11 vtables (248 B) are never read; dropping them and what
  only they reach saves ~580 B (+334 B of methods). 231 of the remaining
  calls are `Process` calls inside capsules (`process_console`,
  `console_ordered`, adc, crc, ...) at sites the analysis never recorded (the
  generic capsules aren't interpreted - see the "167 exact vs 134" note);
  148 are `core::fmt::Write`. Rewriting every call of the 43 single-vtable
  traits (what LLVM's devirtualization does) would free 1,452 B more of
  vtables (`Process` 252 B); the 17 multi-vtable traits hold 105 vtables /
  1,960 B (fmt::Debug 33, SyscallDriver 19, fmt::Write 9).

- **ripgrep finishes uncapped again, Nov 2-3** (build-std, `--bin rg`,
  `--max-candidates 1`, no cap; job tmp `rg_full4.sh`, `rg_exp.done`).
  - Value growth (Nov 2): `fb516e9` builds `Vec`'s buffer (`RawVec`,
    `RawVecInner` and their plumbing) unflattened, `23abac1` keeps a sequence
    copied by `copy_nonoverlapping` one `List` (it nested `List(List(..))`
    per copy). Values ~6x smaller; a cap-64 analysis completes in ~5 min. The
    uncapped run (`full1`, Nov 2-3) still didn't finish: stopped at 06:00
    after 50,538 s, 4.1M bb visits, in nested `Display::fmt` simulation.
  - Cost of the nested formatting dispatches (256 Display candidates, 3-4
    levels deep: `&dyn Error` / `io::Error` / anyhow's `BoxedError` dispatch
    `Display::fmt` again). A `VERIFOPT_TRACE_DISPATCH` trace showed the
    memo missing over and over for the same arguments: first because
    immutable lookup tables (`ryu`'s `DIGIT_TABLE`, `serde_json`'s
    `ESCAPE`, ...) got new versions each time a candidate's context copy
    materialized them, then because backtrace's `MAPPINGS_CACHE` (1-3 nodes,
    `None`/`Some`) passed 150 versions in 1M visits from writes in other
    contexts, then because a scope's epoch moved on every argument merge that
    built a new map. Fixed by `536c27e` (value fingerprints, frozen statics,
    materialization) and `f48c511` (epoch only on a real store change); see
    "Memo validity" in §3. Widening a static after 16 versions was tried
    and removed: the values never grew, the versions did.
  - Runs (job tmp `rg_exp.done`; outputs `out_exp_<name>`):

    | run | plugin | pipeline | analysis | bb visits | peak RSS |
    |---|---|---|---|---|---|
    | `full1` | growth fixes | stopped after 50,538 s | - | 4.1M | 9.9 GB |
    | `full3` | + cheaper merges (`37ec8c0`) | stopped after 10,969 s | - | 3.25M | 4.8 GB |
    | `full7` | + value-checked memo (`536c27e`) | 7,643 s | ~7,110 s | 4.75M | 8.7 GB |
    | `full8` | + epoch fix (`f48c511`), = `a9ad236` | **6,866 s** | ~6,340 s | 2M | 8.2 GB |

    `full7` and `full8`: identical edits, byte-identical `rg`; one of 181
    sites differs (`DebugTuple::field`'s closure, 3,149 CHA candidates:
    `full8` adds `<String as Debug>::fmt`).
  - `full8` results (`verifopt_results_exp_full8_target/`): 181 sites;
    CHAmax complete 157, n/a 24; FSA = CHA fallback at 116, FSA == CHAmax at
    3, FSA < CHAmax at 36, FSA > CHAmax at 2. Edits: 2 `single exact` (both
    `ignore::WalkParallel::visit`), 11 `pointers n=1`, 259 skipped
    `too_many_targets`. Output = plain on 21 of 22 `rg_cmp.sh` commands; the
    22nd, `rg fn core`, has no `--sort` and differs between two plain runs
    too (same 1,054 lines sorted).
  - Where the time still goes (`perf` on `full8`, `VERIFOPT_MERGE_STATS` run
    `mstats`): ~96% in merging each candidate's context back
    (`merge_contexts_timed`), 89-96% of wall time per window in the slow
    region; each context merge visits 300-1,500 written scopes. Most of it
    is temporaries of finished formatting helpers: `Formatter::pad` 35-57 s
    of a 120 s window (its `_7`, 43k nodes, 20.6 s; eleven ~240-node
    temporaries ~3 s each), `Formatter::write_fmt`'s `_5`/`_6`,
    `write_str`'s `_3`. Arguments (`write_fmt`/`write_str` `_1`, the
    `Formatter`, 33-36k nodes) are the next largest. See §6 "Analysis cost".

- **ripgrep in ~24 min, Nov 4** (same flags; runs `full9`-`full14`, job
  tmp `rg_exp.done`, `out_exp_<name>`, script `rg_full9.sh` - no pace
  rule). Dropping finished calls' temporaries took the uncapped analysis
  from ~1 h 46 min to ~25 min; the dropped-read check
  (`VERIFOPT_CHECK_DROPPED`) then found what was relying on leftover
  values, and each find became a fix:
  - drop at the end of the walk, not at the Return terminator (blocks after
    it in walk order: `TwoWaySearcher::next`, serde_json's
    `PrettyFormatter::end_object`), and not while another walk of the scope
    is active (the nested final traverse after recursion);
  - blocks walked although unreachable (35 of 46 reports in `full11`): dead
    loops behind pruned switch edges (`ByteStr`'s `Display::fmt`: bb0 ->
    bb7, bb7's preds bb6 pruned and bb28 a back edge) - fixed by pruning by
    reachability, which also restored two `PadAdapter` dispatch sites the old
    pruning had cut (the join block after `if self.state.on_newline { .. }`
    is also the `?`'s Ok target; once on_newline was known exactly, the
    false arm's pruning removed it);
  - `&(*L)` keeps L (regex_syntax's `lift_common_prefix`: `_26 =
    &(*_119)`, `_119` a raw pointer built from parts, read around a loop);
  - call destinations left unwritten (regex_automata's `c_look`):
    destination gets any value unless the callee can't return / is pending
    recursion. 580 calls to 23 functions remain unknown - mostly trait
    methods resolving to shims / builtin impls (`Clone::clone` 266,
    `slice::repeat` 138, `RegexInfo::config` 60, `FromResidual::
    from_residual` 28, `Any::type_id` 12): candidates for models or stubs.

    | run | change | pipeline | sites | dropped reads (unreachable) |
    |---|---|---|---|---|
    | `full8` | Nov 3 baseline (`a9ad236`) | 6,866 s | 181 | - |
    | `full11` | drop temporaries (`02e9c1d`) | 1,605 s | 181 (2 lost, 2 new) | 46 (35) |
    | `full12` | + pruning by reachability, `&(*L)` rule | 1,630 s | 182 (none lost) | 2 (0) |
    | `full13` | + call results | 1,464 s | 182 | 0 |
    | `full14` | + `()` excluded (`c85db8c`) | **1,460 s** | 182 | 0 |

    Last memory reading ~4.6 GB (at 1.75M visits; these runs end before
    2M). Every run: 2 `single exact`, 11 `pointers n=1`; `full12`'s rg
    matches plain on all 22 `rg_cmp.sh` commands, `full13`/`full14`'s rg is
    byte-identical to it. Against `full8`, all 181 sites are in `full14`
    (170 identical in `full12`, the rest supersets) plus a
    `DebugStruct::field` closure site. Exception: three `fmt::write` sites
    went from 23 writers (= CHAmax-ish) to 11 with the call-result fix - the
    lost ones (serde_json's `collect_str` adapter, serde's
    `LookForDecimalPoint`, anyhow's writer) are never constructed in code
    either run interprets (`collect_str` never entered, `VERIFOPT_TRACE_SCOPE`
    runs `trcs12`/`trcs13`), so they came from an unknown `&mut dyn Write`
    an unwritten destination had produced.
  - `full14`'s rg: file-backed 4,376,775 B, +3,804 B vs `full8`'s with
    identical rewrites - partitioning: the hook adds edges for every stored
    site (§6, pinned), and the bigger target sets add more.

## 5. Done: tock merged into one crate (boards/imix)

Kernel/capsules/chip code is compiled inside `imix`, so every dispatch site
can name its targets (avoids the separate-compilation limit). Completed
Oct 2026. The tock results in §4 predate the merge — re-run the analysis
(all DefPathHashes changed).

Merged crates (copied to `boards/imix/src/<name>/`, `lib.rs` → `mod.rs`):
`kernel capsules_core capsules_extra capsules_system sam4l components
cortexm4 cortexm cortexv7m segger` (segger too: components depends on it and
it depends on kernel). Rule: any crate depending (transitively) on a
merged crate must be merged too (else cargo builds the original `kernel` as
well → two kernels). Leaf libs (tock-registers, tock-cells, tock-tbf,
enum_primitive, tickv) stay as dependencies. Check:
`cargo tree -e normal --target thumbv7em-none-eabi | grep -E 'kernel|capsules|sam4l|cortex|components|segger'` → empty.
Tock commits `1f0fbb60f`..`19d340a16` on branch `verifopt`; build with
`cargo build --release --target thumbv7em-none-eabi` in the tock nix-shell.

Recipe (for redoing it, e.g. after rebasing tock): rewrite own-root paths
(`crate::` → `crate::<m>::`, `$crate::` likewise) in each `src/<m>/` first,
then `<m>::` → `crate::<m>::` everywhere; move `#![feature]`s to `main.rs`
and drop other crate attributes; `pub mod <m>;` in `main.rs`; re-export
`#[macro_export]` macros from each `<m>/mod.rs`; carry over cargo features;
`edition = "2024"` for imix (kernel/capsules/cortexm* are 2024,
sam4l/components/imix were 2021 and needed migrating).

Gotchas hit doing it:
- Start from pristine copies (dir names with `_`, not `-`): the first copies
  had a partial `kernel::` → `crate::kernel::` rewrite that clashed with the
  own-root step.
- Edition migration: run `cargo fix --edition -p imix -p sam4l -p components
  --lib --bins --target thumbv7em-none-eabi` in a scratch worktree of the
  unmerged commit (`--lib --bins`: lib test targets fail on thumb), apply the
  same path rewrites to the diff, remap paths, `git apply`.
- Macro re-export lists must include macros with doc comments between
  `#[macro_export]` and `macro_rules!` (e.g. `gpio_component_helper`); drop
  main.rs's `use kernel::{static_init, debug, …}` (clashes with the root
  definitions). Only clash: the stream macros — `capsules_extra/net/stream.rs`
  is now `pub use crate::capsules_core::stream::*;` (identical file), and
  `#[macro_use] pub mod capsules_core;` in main.rs for textual scope.
- One crate = conflicting extern declarations get renamed by LLVM: sam4l
  declares `_estack` as `fn`, cortexv7m/components as `static u8` →
  undefined `_estack.1`; all are `fn _estack()` now.
- Lints that were hidden as external-macro lints now fire per call site
  (~1000 warnings): `#[allow(missing_docs)]` on the merged `pub mod`s,
  `#![warn(static_mut_refs)]` (deny in 2024), statement-level
  `#[allow(static_mut_refs, unused_mut, unsafe_op_in_unsafe_fn)]` inside
  `static_init!`/`static_buf!`/`static_named_buf!`. ~30 real 2024
  `unsafe_op_in_unsafe_fn` warnings remain in imix's own `test/*.rs`.

## 6. Open work / known issues (not yet done)

- **Fn-pointer-reached receivers / `Formatter::write_fmt`** — mitigated
  (Oct 17) and the root cause fixed (Oct 18: fn items through casts,
  `Offset`, array literals, closure captures, rust-call binding; see §3).
  With build-std, `Formatter::write_fmt` / `pad` / `fmt::write` now see the
  real writers. History below.
  - Bug: FSA gave `Formatter::write_fmt` bb13 (`self.buf.write_str`) one
    writer, `String::write_str`, and the rewrite made it an unguarded
    `single` in ~20 crates. Cause: `fmt::rt::Argument` holds
    `<T as Display>::fmt` as a fn pointer; `convert_cast`
    (`convert.rs`, constant-operand branch) turns the fn item into
    `FnPtr(sig)`, and `interp_fn_ptr` answers from the signature without
    interpreting. So Display impls are only seen via `ToString` (String).
  - Mitigation (fork `verifopt_rewrite.rs` `compute_edits`): a 1-target site
    is unguarded only if `Store::exact` (CHAmax-proven); otherwise
    `pointers n=1` (compare + dyn fallback). `cargo verifopt
    --unguarded-single` (env `VERIFOPT_UNGUARDED_SINGLE`) restores the old
    behaviour, for experiments only. ripgrep: 43 `single` → 43 `pointers
    n=1`; outputs identical to plain (cap 1 and 4).
  - Fn-pointer calls are now listed in `--ASSUMPTIONS--` as
    "fn-pointer call (not followed)" (`Reason::FnPtrCall`), including
    indirect calls whose callee value is empty or partly unknown. These
    show `std::thread::LocalKey::try_with` losing its accessor (an fn pointer
    in a `static`) wherever `thread_local!` is used; `fmt::rt::Argument::fmt`
    should show up here too (callee lost through `BinOp::Offset`).
  - CHAmax is "n/a" for any site dispatching a trait with builtin impls
    (`Fn*`, `AsyncFn*`, `Future`, `Coroutine`, `AsyncIterator`), not only
    when FSA found a closure: CHA lists only explicit impls such as
    `<Box<F> as Fn>`, so their instance count bounds nothing (`chamax.rs`
    `has_builtin_impls`).
  - Fixture `fmt_fnptr_writer` reproduces it (unguarded: garbage counts;
    guarded: passes). Note: `BLESS_GOLDEN=1` returns before the call-trace
    check, so a bless run never catches wrong behaviour — run once more
    without it.
  - Done Oct 18: fixtures `fnptr_args_table` (fmt's argument-table shape
    without std) and `closure_captures` (one site reached directly, through a
    generic closure's capture and through a `dyn FnMut` closure).
- **`fmt::rt::Argument`'s `formatter` field is lost** (open, seen Oct 22
  in tock `statics_target`, after `c479271`): `Argument::fmt`'s indirect
  call `(self.formatter)(self.value, f)` finds only a typed `FnPtr` callee,
  so the `<T as Display>::fmt` behind it isn't interpreted ("fn pointer
  called in core::fmt::rt::Argument::<'_>::fmt x78 (&mut arg)" in
  `--ASSUMPTIONS--`). Before `c479271` a union field read returned every
  field's value, which happened to supply the fn items (plus junk); the
  exact read now shows the field itself isn't known at that point - a
  missing field reads as "any value of its type". Consequences: Display
  impls reached only this way aren't interpreted - their dispatch sites go
  unrecorded (not rewritten: sound) and their writes through the
  `&mut Formatter` are skipped (reported). Where to look: `Argument::new`
  builds `ArgumentType::Placeholder { value, formatter: transmute(f), .. }`
  - check the fn item survives the transmute and the enum aggregate, the
  `&[Argument; N]` → `NonNull` → `add(i).as_ref()` path (fixture
  `fnptr_args_table` covers the shape without std and still passes), and
  whether the read goes through a field path / offset-0 cast that now hits
  `unknown_field_value`. `VERIFOPT_TRACE_SCOPE=Argument` names the read.
- **Parked (Oct 23, behind the §8 ladder): statics / raw-pointer phases.**
  Done in `371af2f`, `c479271`. Left: Phase 1 (statics as locations) -
  `static_init_buf` sound but imprecise; summaries don't check static
  versions. Phase 2 (raw-pointer alias edges, return aliases, ptr
  intrinsics) - `raw_ptr_static_write` still red; pointers stored in fields
  (`write_ref_in_field`) wait on Phase 4.
- **CHA candidates of generic impls are never interpreted** (Oct 23): a CHA
  candidate comes without generic args (`(DefId, None)`), and
  `simulate_static_calls` skips any candidate whose fn has generic params
  ("args mismatch" / "non-monomorphic", answered from the signature) -
  *lifetimes count*, so even `Fxos8700cq<'_>` or `<Formatter<'_> as
  fmt::Write>::write_str` (x4,860) are skipped. tock: 667 functions over
  60,128 calls; it is why the capsules' own dyn calls (process_console,
  fxos8700cq, rf233, isl29035, sha256, ...) are never recorded. Cheap part:
  fill lifetime-only params with erased regions. Type params: take them
  from the compiled instances (instance manifests / mono collection) - the
  compiled-instances fallback.
- **`Drop` terminators are ignored** (found Oct 23; `interp.rs` `interp_terminator`,
  `TerminatorKind::Drop { .. } => Ok(None)`): drop glue and `Drop::drop`
  impls are never interpreted. Coverage: their dispatch sites are never
  recorded - tock's `Drop for EnteredGrantKernelManagedLayout`
  (`self.process.leave_grant(..)`, `kernel/grant.rs:511`) is 108 of the 593
  dyn calls left in the rewritten IR. Soundness: writes done by drops (and
  drops through `Box<dyn T>` vtables) are skipped and not reported in
  `--ASSUMPTIONS--`. Fix: interpret `Drop` as a call to
  `drop_in_place::<T>` (resolve the drop-glue instance; for `dyn T` a virtual
  call through the vtable's drop slot).
- **Analysis cost** (Nov 4): ripgrep's uncapped analysis now takes ~16 min
  / ~4.6 GB (pipeline ~24 min; §4, "ripgrep in ~24 min"), after dropping
  finished calls' temporaries (§3). Merging candidate contexts back is still
  over half the time (56% in the second 10-min window of `full11`). Next
  levers: merge each key's candidate values in one pass instead of 255
  pairwise merges (pairwise is quadratic in disjuncts); per-candidate
  summaries reused across contexts (option 3, Oct 26) - what would remove
  the argument merges (`Formatter` values, 33-36k nodes). Smaller: the 4
  sites with >3,000 CHA-sized targets; memo pressure (50 arg sets per
  function before widening).
- **Calls with unknown results** (Nov 4): `CallResultUnknown` in
  `--ASSUMPTIONS--` lists the callees whose result the analysis loses (any
  value instead) - ripgrep: 580 calls to 23 functions, mostly trait methods
  that resolve to compiler shims or builtin impls (`Clone::clone`,
  `FromResidual::from_residual`, `Any::type_id`, `Iterator::next`,
  `Borrow::borrow`), plus `slice::repeat` and regex_automata's
  `RegexInfo::config`: models or stubs for the frequent ones.
- **Soundness gaps the bigger reach exposes** (see the assumptions report):
  memo reuse under widened keys (plan (B) below) - 91k calls in ripgrep;
  `std::thread::LocalKey::try_with` loses its accessor (fn pointer in a
  `static`); the `Memmem` prefilters' fn-pointer calls in aho_corasick /
  regex_automata (search hot path, `&mut` args) aren't followed.
- **Memo hits skipped the callee's writes** - fixed Oct 21 (`write_log.rs`,
  `054f40d`): memo keys see a pointer argument only
  as its pointee's value, so `f(&mut x)` and `f(&mut y)` with equal `x`/`y`
  collide, and a hit used to return the cached value without writing into
  `y`. ripgrep: 188k of 295k memo hits were for callees with a `&mut`/`*mut`
  parameter (Oct 20 `memo_target` run). Now every interpreted call (and
  virtual dispatch, and recursive re-interpretation) logs the values written
  through each operand's caller-side place ("ports" on the alias path);
  exact_memo / virtual_call_memo / summaries store the log and a hit
  replays it through its own operands (always a merge). Fixtures
  `memo_side_effects` (now green), `memo_nested_replay`,
  `memo_virtual_replay`, `memo_recursive_replay` (doesn't isolate summary
  replay - see its comment). Report: "memo hit, writes replayed".
- **Oct 21 performance fixes** (`48f0fc5`, `8d9edcc`; diagnostics `bd24ae9`):
  (1) `memoize_by_rc` caches (flatten_all, widen, cast, lift,
  contains_param) never dropped entries whose key had died, and they hold
  their results strongly - a leak growing with value churn. They now purge
  dead entries at each power-of-two size. ripgrep: 1.3M bb visits at 4.5 GB
  instead of 35.8 GB; peak 17.4 GB (was 28.8). (2) `Constraints::push`
  called `merge_if_one_field_differs` once to test and again to merge,
  doubling the work at every nesting level (2^depth): tock's deeply nested,
  heavily shared values (~68 billion tree nodes in ~2 GB) stalled at ~25k bb
  visits for hours; now merged once. `[verifopt memory]` also reports the
  replay logs and the rc caches (entries/dead).
- **Writes that never reach the caller** (not memo-related; fixtures
  `write_raw_ptr_param`, `write_ref_in_field`, `write_interior_mut`, red):
  (1) `resolve_args` adds alias edges only for `&`/`&mut` parameters, so a
  write through a `*mut` parameter stays in the callee; (2) a pointer stored
  in a field holds a *copy* of its pointee (pointer = pointee value), so a
  write through it updates the copy, not the original place; (3)
  `resolve_mut_ref` stops at a shared borrow, so `Cell`/`RefCell` writes
  through `&` land in the callee's own local. (3) is likely the most common
  (`thread_local!`, `RefCell` state). The Oct 20 report counted 1.5k memo
  hits each for (2)- and (3)-shaped parameters, but these writes are lost
  with or without the memo.
- **`InterpPass::run`'s error** is reported since `e78984c`: a warning, an
  `ANALYSIS ABORTED` assumption, and every site's CHAmax marked incomplete.
- **Partitioning hook adds edges for sites that are never rewritten** (pinned):
  `add_verifopt_rewrite_uses` adds "uses" edges for every stored site,
  including `too_many_targets` skips (5,595 edges in `rg` at cap 1, mostly
  the 104-impl `Flag` sites). The edges alone reshuffle inlining of
  `rg::run`/`rg::search` (hook-only build: `.text` −7.9 KB vs plain). Make it
  apply the rewrite's own selection (max-candidates, CHA expansion).
- **"Widening soundness"** (deferred, potentially breaking): a missing ADT
  field reads as *empty* ("nothing"). `widen_constraints` (merge > 50
  disjuncts, field writes > 5000 nodes or into > 50 disjuncts) drops ADT
  fields → can lose trait objects → possible wrong Single rewrites; memo
  reuse under widened keys (after 50 distinct arg sets) can reuse a result
  for different args. Plan: (A) missing field = "any value of the field
  type" via a pure `unknown_value(ty)` (refs→pointee, dyn→Dynamic, ADT→
  fieldless = unknown, tuple/array element-wise) in `step_field_one`,
  `wrapper_contents_or_flatten`, flattening; (B) on a widened-key memo miss,
  interpret with the widened args; (C) fixtures that fail before the fix
  (>50-arm match building a struct with `Box<dyn T>`; memo reuse). The
  assumptions report shows how often these occur.
- **Unify Single/SingleExact**: record every target's resolved instance args
  in the analysis; drop `resolve_target_via_trait`.
- **Fn items / fn pointers as `dyn Fn` objects**: no candidates today (FSA
  and CHA both empty → site not rewritten, and the function body isn't
  interpreted at that dispatch). Model them as Fn-trait implementors.
- **Analysis precision for statically wired objects** (tock's processes
  array, capsules; ripgrep's flag table): FSA falls back to CHA.
- **Stubs**: unsummarized/handle-returning collection methods widen
  (sound); `any_value` loses trait objects nested in other ADTs until (A).
- Per-target result/store files (multiple analyzed targets overwrite each
  other); debug-profile harness runs; value-dependent fixture behaviour.
- tock boot test on hardware; perf measurements (e.g. hyperfine for ripgrep).
- **Branch-aware refinement** (todo, added Oct 29): when a `switchInt`
  branch is taken, remember the value that selected it for the duration of
  that branch - the discriminant is the branch's value (the `otherwise` edge:
  none of the listed values), and so is its source: the enum behind
  `_d = discriminant(x)` is narrowed to that variant (its other variants'
  fields and trait objects drop out), the operands of an `Eq`/`Ne` compare
  feeding the switch are narrowed to the constant. Until the branches join,
  where values merge again. Today `interp_switchint` only prunes successors
  no value can take (`set_bytemap` / `prune_switchint_targets`); the
  successors are then interpreted with the unrefined value, so e.g. a
  `match opt { Some(x) => x.method(), None => .. }` arm still sees the
  `None`-side value, and an enum holding different `dyn` objects per variant
  dispatches each arm to all of them. Decided (Oct 29): a refinement
  *overlay*, not per-block entry states - merging is already the dominant
  cost on ripgrep, and per-block states would add a merge at every join.
  Sketch: entering a successor of the switch pushes (place -> allowed values /
  variant) for the discriminant and its source; reads of a refined place
  intersect the stored value with the refinement (applied on read, nothing
  copied); a write to a refined place (or through an alias of it) drops its
  refinement; the overlay is popped at the switch's immediate post-dominator,
  where the branches rejoin. To check while designing: blocks reachable from
  several arms before the post-dominator (shared tails), loops whose back
  edge re-enters the branch, and refined values passed to callees (memo keys
  see the refined argument - intended).
  Related: the low-priority constant-byte pruning below (same switch code).
- **(Low priority) Prune branches on constant byte data**: precision/noise
  only, not soundness. The analysis is flow-sensitive but merges at joins
  and prunes `switchInt` targets only for a single known scalar / variant
  (`interp_switchint` → `set_bytemap`). So `core::fmt::write` walks every
  template branch in every context, including the `0xC0` placeholder
  branch for argument-less `Arguments` (`write!(f, "lit")`), whose fn-pointer
  call then has an empty callee — the likely source of the benign
  `indirect call in std::fmt::write (callee value empty)` entries (not
  verified per context). Fix: (1) `convert_allocated_const`: decode byte
  arrays behind constant references (e.g. the promoted `&[u8; N]` template)
  into the set of byte values (`Idk` of `Scalar(Some(b))`); (2)
  `convert_binop_helper`: compute small sets of known scalars (capped,
  unknown above the cap) instead of requiring exactly one value each.
  Step 3's pointer changes already carry the set through `add`/`read`.
  Alternative (rejected for now): a `fmt::write` stub — more precise and
  faster, but tied to `fmt::Arguments`' internal layout. Revisit if a fresh
  ripgrep report has many benign "callee value empty" entries.

## 7. Recent commits (Oct 17 – Nov 4 2026; `git log` is authoritative)

Plugin (`~/hack/verifopt`, branch `ripgrep`, not pushed as of Nov 4):

- `4875a88` CHA matches impls of traits with their own generic params
- `ce9a201` guard non-proven single rewrites (`--unguarded-single`);
  `Reason::FnPtrCall`; fixture `fmt_fnptr_writer`
- `72ec90e` CHAmax n/a for traits with builtin impls (`Fn*`, `Future`, …)
- `d44dfae` assumptions record indirect calls with an unknown callee
- `367c7dc` follow fn pointers through casts, `Offset`, array literals;
  fixture `fnptr_args_table`
- `65e8fd1` closure captures; rust-call argument binding; fixture
  `closure_captures`
- `4e627c1` cached hashes, lossless same-shape merge, capture size bound
- `a161e1d` alias pruning (two-level `refs`), per-instance body/WTO caches,
  timing opt-in, refs conflict scan only with debug logging
- `621b032` diagnostics (`[verifopt progress]`, `VERIFOPT_CACHE_SIZES`,
  `VERIFOPT_SIZE_HISTOGRAM`)
- `4ff9bd2` STACK DESYNC fix (`unwind_stacks_to`), indirect-call errors
  propagated, recursion depth limit as a value, stack-op ring buffer
- `aa1eea0` merge skips pointer-identical entries
- `797d637` dirty-key context merges (`TrackedMap`), cheap identity check
  (`EnclosingScopes` is `Option<Rc<Vec<VOID>>>`)
- `e78984c` report an aborted analysis (warning, assumption, CHAmax
  incomplete) instead of silently using partial results
- `f30b5c7` red fixtures `memo_side_effects`, `memo_widened_reuse`,
  `merge_widened_field`
- `054f40d` replay memoized calls' writes through their operands (`write_log.rs`);
  fixtures `memo_nested_replay`, `memo_virtual_replay`, `memo_recursive_replay`,
  red `write_raw_ptr_param` / `write_ref_in_field` / `write_interior_mut`
- `48f0fc5` purge dead entries from the memoize_by_rc caches (memory leak)
- `8d9edcc` merge a same-shape disjunct once in `Constraints::push` (2^depth)
- `bd24ae9` `[verifopt memory]` reports replay logs and rc-cache sizes
- `371af2f` writes through raw pointers, shared borrows and statics
  (statics as places, alias edges, return aliases, plan A, fixtures)
- `c479271` union reads see one field; memo hits check per-static versions
- `05ab55c` `[verifopt memory]` shows the innermost call-stack scopes
- `b0eb702` purge dead memoize_by_rc entries every 16k insertions as well
- `ec94242` memory diagnostics: `malloc_in_use`, largest memo keys,
  unmeasured structures, `[verifopt memory sim]` nesting
- `b0a4586` CHAmax-only mode (`VERIFOPT_CHA_ONLY`, rung 3)
- `ee0d35c` opt-in megamorphic cutoff (`VERIFOPT_MEGAMORPHIC_CAP`,
  experiment only) + step-0 switches `VERIFOPT_CUT_NO_HAVOC`,
  `VERIFOPT_CUT_KEEP_ENUMS`
- `dc530df` switchInt branch counts saturate (a u8 wrapped to 0 at 256
  disjuncts: "no possible branches" panic)
- `fb516e9` `Vec`'s buffer (`RawVec`, `RawVecInner`, plumbing) built
  unflattened; `23abac1` `copy_nonoverlapping` keeps one `List`
  (value growth, Nov 2)
- `37ec8c0` cheaper merges: exact duplicates by hash lookup, no copy for
  no-op pushes, `append` returns at once for x ∪ x
- `536c27e` memo hits check statics by value (fingerprints), frozen statics
  untracked, materialization not a write; `VERIFOPT_TRACE_DISPATCH`
- `f48c511` a scope's epoch moves only when the argument merge changes its
  store (`TrackedMap::same_entries`)
- `a9ad236` fewer copies in value merges (borrowed field map,
  allocation-free `differing_field`, `TagProv::covers`) - ~2%, results
  identical
- `3a48a2a` `VERIFOPT_MERGE_STATS`
- `02e9c1d` drop a finished call's temporaries from its scope store
  (`VERIFOPT_CHECK_DROPPED`, `VERIFOPT_TRACE_DROP`,
  `VERIFOPT_KEEP_TEMPORARIES`)
- `4ace1b6` prune switch targets by reachability, not predecessor counts
- `a38e632` keep locals borrowed through a deref; dropped-read reports show
  the blocks
- `c85db8c` a call that returns no value gives its destination any value of
  its type (`CallResultUnknown`)

Fork (`~/hack/rust`): `73da158f0d5` disable unguarded/unproven single
rewrites (on top of `5eec5c673e1` flexible max-candidates,
`0bea3c87cc6` max-candidates logging, `d7a86258cfd`).

## 8. Plan: evaluation ladder and the oracle upper bound (agreed Oct 23 2026)

Goal: place verifopt between what LLVM does on its own and a "speed of
light" bound for any devirtualization mechanism. Not implemented yet.

**Ladder** (per project; every rewritten rung takes K = `--max-candidates`
per run; stores hold each site's full target set, so one store serves every
K; results dirs are named with K):

| # | rung | shows |
|---|------|-------|
| 1 | plain | baseline (ripgrep: also the partitioning-hook baseline) |
| 2 | plain + VFE | LLVM alone: trait-level single-impl devirtualization + dead-slot removal |
| 3 | CHAmax everywhere (K) | best sound result without points-to: every compiled site rewritten from compiled instances, no FSA |
| 4 | verifopt (K) | CHAmax + FSA (today's pipeline) |
| 5 | verifopt + VFE (K) | what LLVM still adds on top of the rewrite (5 vs 4), and what verifopt adds on top of VFE (5 vs 2) |
| 3a | RTA (K) | classic baseline: CHA restricted to types whose vtable for the trait is created in reachable code (planned Oct 27, see "Analysis baselines") |
| 4a | rupta (K) | published Rust pointer analysis (rupta, CC'24 / CGO'25) in place of FSA, same rewrite (planned Oct 27, see "Analysis baselines") |
| 6s | size oracle | speed of light for code size on the workload (see "Two oracles" below) |
| 6p | performance oracle | speed of light for run time on the workload (see "Two oracles" below) |

Extra column on rungs 4-6, "unread-vtable saving": what a vtable-elimination
mechanism (not built) would save on top of that rung's binary. After a
rewrite, a trait whose dynamic calls were all rewritten has vtables nothing
reads any more - only their addresses are used, as identifiers - so their
contents, and methods only they reference, could go. Computed rather than
built: from a VFE build's pre-LTO IR (every remaining dynamic call is a
`llvm.type.checked.load` naming its trait), traits with no call left = unread
vtables; their bytes + the functions only reachable through them
(`py/vtread.py`). tock rung 4: 11 of 158 vtables unread, 248 B + 334 B of
methods = ~580 B. Build it only if worth it (second pass emitting placeholders
in `get_vtable`, PointersExact comparing addresses).
Metrics (per rung): the full section table, plus
- file-backed size: ripgrep `.text + .rodata + .data.rel.ro + .eh_frame_hdr
  + .eh_frame + .gcc_except_table` (x86-64 PIE: vtables are in
  `.data.rel.ro` - their fn pointers need load-time relocation - and the
  unwind tables change with the code); tock flash `.text + .ARM.exidx +
  .storage` (its linker script puts `.rodata`, vtables included, inside
  `.text`);
- RAM: `.data + .bss` (+ `.tdata/.tbss`), tock `.sram` - a check: the
  rewrite shouldn't change it (tock: identical in every build; ripgrep
  `.bss` moves by 8-16 B);
- vtable bytes (tock: `py/vtables.py` heuristic scan, approximate - no
  symbols, `-icf=all`; the pre-LTO IR count is 158 vtables / 3,660 B);
  indirect-call counts (`blx rN` / `call *`);
- ripgrep also runtime (hyperfine) on the oracle workload.
`.text` alone undercounts ripgrep (vtables, unwind tables) - don't report it
on its own. Rungs 1-5 are sound and must match plain on `rg_cmp.sh`; rung 6
only on its own workload.

**Oracle workloads: several per project, unioned** (decided Nov 5, replacing
"one workload, never a union"). A single run gives the tightest sets but an
overfit bound - "how small if it only ever ran this"; a union over several
representative workloads is the bound for a class of uses. Per site, S = the
union of observed targets over the workloads: |S| = 1 in every workload ->
single; S small and stable -> narrowed (PointersExact over S, no fallback);
S large, close to CHAmax, or varying a lot across workloads -> left dyn;
never executed in any workload -> left alone, unless it is the last read of
a trait's vtables (then rewritten from VTmax, which is complete) - on top of
the per-trait size decision below. Measure the oracle build (size, output
check) on the same workloads; report executed and never-executed sites
separately. Deferred (Nov 5, rough numbers first): cross-validation (build
from all workloads but one, run the held-out one - unseen targets at
narrowed sites show how optimistic the bound is and when enough workloads
are in) and a sound companion row (same decisions, guarded `pointers` with
dyn fallback at narrowed sites).
- ripgrep: e.g. `rg -n 'fn \w+\(' ~/hack/rust/library` (walker, glob/type
  filters, regex engine, searcher, printer) plus other searches and modes
  (`--json`, `--files`, error paths).
- tock: boot + several apps' workloads. imix has no emulator (QEMU has no
  SAM4L); for Cortex-M none of tock's boards has working emulation (QEMU's
  STM32 boards: tracking issue #1827). Tried first (Nov 5): Renode's
  nRF52840 model with the merged `nrf52840dk-min` / `-min-app` boards
  (unofficial - tock lists no Renode support); fallback `qemu_rv32_virt`
  (RISC-V, supported and used in tock's CI with app tests; needs a merged
  crate and new baselines). Verilator boards (`litex/sim`, `veer_el2_sim`)
  only if both fail - too slow for repeated workloads.
  **Renode works** (Nov 5, nixpkgs `renode` 1.15.1, ~20 min to get there):
  the full upstream `nrf52840dk` boots to `tock$` and runs libtock-c's
  `c_hello` (prints) and `blink` (31 LED changes in 3 s); the merged
  `nrf52840dk-min-app` (plain build) runs `blink` (23 LED changes in 3 s).
  Runner: job tmp `renode/run_tock.sh KERNEL_ELF SECONDS OUT_PREFIX
  [APP_TBF ...]` (apps placed one after another from 0x40000; writes
  `OUT_PREFIX.uart` and `.log` with LED state changes). Emulator fixes:
  UICR and FICR as plain memory (the board writes UICR's reset pin /
  APPROTECT through NVMC and resets until they read back - Renode doesn't
  model UICR, so it reset forever) and a reset macro reloading the images;
  peripherals on gpio0 are named `gpio0.led0` etc. (`logLevel -1
  gpio0.led0` logs LED changes). The minimal boards print nothing
  (`debug_noop`), so their workloads are observed through LEDs (and later
  the recorder's table, read from RAM in Renode's monitor).
  Apps: libtock-c cloned to `~/hack/libtock-c`; build with `nix-shell -p
  gcc-arm-embedded gnumake unzip --run 'make TOCK_TARGETS=cortex-m4'` in an
  example dir (it fetches prebuilt newlib). Install `elf2tab` (>= 0.13)
  beforehand with plain `cargo install elf2tab` - from inside the libtock-c
  build it fails (the makefiles export `AR=-ar`, which breaks the `ring`
  C build).

**Analysis baselines: RTA and rupta** (added Oct 27, not implemented). Each
produces a store (per site: its target set) that goes through the *same*
rewrite as verifopt, so rungs differ only in the analysis. The rewrite
guards every non-CHAmax-proven target set (`pointers` with dyn fallback),
so even an unsound analysis gives a sound binary; `single exact` still comes
only from CHAmax. Besides the binary metrics, report per-site precision
(target-set sizes against CHA / CHAmax / the oracle's observed sets) - and
use the oracle's observed sets as a soundness check: an observed target an
analysis left out is a missed target (it only costs speed/size in the
binary, but it makes the analysis's "precision" look better than it is).
- **RTA** (Bacon & Sweeney): dyn targets = impls of the method for the types
  whose vtable for that trait is *created* (unsize coercion to `dyn Trait`,
  `GlobalAlloc::VTable` in constants/statics) in reachable code, iterated
  with reachability from the entry points. Close to CHAmax, which must be
  kept apart: CHAmax counts *compiled instances* of the method, which include
  instances compiled for static calls on types never cast to `dyn` - RTA's
  set is a subset. Cheapest implementation: the fork's instance manifests
  also list the vtables each crate emits ((type, trait) pairs; rustc's mono
  collector creates them only for reachable unsize casts), and RTA = impls
  for types with a vtable of the dispatched trait. Collector reachability
  roots every non-generic item of library crates, so it over-approximates a
  from-`main` RTA. Decided Oct 27: start with this manifest version; later
  (or in addition, as a second RTA row) a precise from-entry-points walk in
  the plugin - iterate reachable bodies from the same roots verifopt uses,
  collecting vtable-creating casts and resolving dyn calls against them
  until nothing new is reachable. The manifest pass also gives the
  *address-taken functions* that §9 gap 4(b) needs as its fn-pointer
  fallback.
  *Definition (from the paper, Bacon & Sweeney, "Fast Static Analysis of
  C++ Virtual Function Calls", OOPSLA '96, doi 10.1145/236337.236371; from
  memory - the ACM PDF needs a login, check against it):* RTA builds the
  call graph optimistically from `main`; a class is *live* once an object of
  it is created in a function already known to be reachable; a virtual call
  resolves to the CHA targets in live classes only, and targets deferred
  because their class wasn't live yet are added when it becomes live.
  Instantiation counts only in reachable code. *Mapping to Rust:* in C++,
  creating an object of a polymorphic class is creating its vptr; in Rust
  construction and vtable creation are separate, and a value can only reach
  a dyn call through an unsize coercion (or a vtable constant). So
  "instantiated" = "its vtable for the dispatched trait is created in
  reachable code" - the faithful analog, not mere construction (which would
  be a weaker RTA). Three nested sets: CHA (all impls) >= CHAmax (compiled
  instances, including ones compiled only for static calls) >= RTA (types
  whose vtable is created). The manifest version uses rustc's collector
  reachability (every non-generic item of every crate is a root), so it lies
  between CHAmax and the paper's from-`main` RTA; the plugin walk is the
  faithful one. RTA works on types; CHAmax on instances (a generic impl's
  instances per type args) - compare at the instance level.
- **rupta** (https://github.com/rustanlys/rupta; Li et al., "A
  Context-Sensitive Pointer Analysis Framework for Rust and Its Application
  to Call Graph Construction", CC'24; "Stack Filtering", CGO'25): Andersen
  (`--pta-type ander`) and callsite-sensitive (`cs`, `--context-depth k`,
  default 1); `cargo pta -- --entry-func main`; dumps a DOT call graph and
  points-to sets. Pinned to `nightly-2024-02-03` (rustc_private). Run both
  `ander` and `cs` k=1 (k=2 if it scales). Problems to solve first:
  (1) toolchain - ripgrep and merged tock are edition 2024 (stable 1.85, Feb
  2025), so they don't build on rupta's nightly; and our store keys are the
  fork's DefPathHashes/bb indices, so rupta must run on the fork's compiler.
  Decided Oct 27: port rupta to the fork's nightly (rustc 1.94.0, ~Jan 2026
  base) rather than run old sources. Existing ports (checked Oct 27, all 14
  GitHub forks): every fork pins `nightly-2024-02-03` except
  `kevindadi/rupta` branch `kevin` (one commit, "update nightly-2024-12-11",
  27 files, 3 commits behind upstream `main`), still a year older than the
  fork - so ported ourselves, from upstream (decided Oct 27: don't build on
  kevin's commit).
  **Port status (Oct 27): builds and runs on `rust-mono-rewrite`.** Done
  directly from upstream, *not* on kevin's commit (that one is mostly rustfmt
  reflow and introduced bugs - `&raw const` treated as a copy, `todo!()`
  panics, a guessed raw-pointer aggregate; it was only a reference for API
  mappings). Worktree `~/hack/rupta-port`, branch `port-fork-nightly` of the
  `~/hack/rupta` clone, from upstream `main` (`b19f187`), port uncommitted
  (22 files, +305/-255, in-place edits, no reformatting). Unmodified upstream
  for comparison: worktree `~/hack/rupta-orig` on `nightly-2024-02-03`
  (already installed). (A first attempt on kevin's commit was deleted.)
  Changes: API renames (`erase_and_anonymize_regions`, `rustc_abi`,
  `GenericArg::kind`, `TypingEnv::fully_monomorphized` for
  `ParamEnv::reveal_all` and `post_analysis` for `param_env(def)`,
  `try_resolve`, `InstanceKind`, `hir_*`, `run_compiler`, `FnPtr(sig_tys,
  hdr)`, `Dynamic` without kind, `ExistentialTraitRef::new_from_args`,
  `in_memory_order`, `Operand::RuntimeChecks`, removed `Rvalue::Len` /
  `NullaryOp` / `CheckedBinaryOp`, `AddressOf` -> `RawPtr` (still an
  address), `Subtype` projection -> `UnwrapUnsafeBinder`, `CastKind::Subtype`
  as a direct assignment, ...); `Ord` dropped from `FunctionReference` /
  `GenericArgE` (unused; `DefId`/`Ty` no longer implement it); `cargo-pta`
  links `rustc_driver` (else std is linked twice). Semantic additions needed
  by the newer core: intrinsics are now plain `core::intrinsics` fns (no
  `extern "rust-intrinsic"` block), so the intrinsic name table applies to
  them too (`transmute`/`offset`/... resolve again); models for
  `read_via_copy` / `write_via_move` (newer `mem::replace`, `ptr::read`);
  `AggregateKind::RawPtr` (`ptr::from_raw_parts`) points where its data
  pointer points; `AggregateKind::CoroutineClosure` like a closure.
  Check: every fixture that builds standalone on both toolchains (strip
  `votrace`, edition 2021; job tmp `rupta_eq/run.sh`) gives the same
  `--dump-dyn-calls` targets as upstream, except printing (`Binder(..)` vs
  `Binder {..}`, moved std impls) and one extra site in the newer std's
  `fmt`; e.g. `fnptr_static_table`: `Holder::handle` -> A, B (not D),
  `run_all`'s fn pointer -> the closure, in `cs` and `ander`. Usage:
  `pta [rupta opts] -- file.rs --edition 2021 --sysroot $SR`, or
  `cargo-pta pta -- --entry-func main --pta-type ander|cs ...` in a crate
  (`LD_LIBRARY_PATH=$SR/lib`). Next: run on tock / ripgrep (std MIR needs
  build-std or `-Zalways-encode-mir`; ripgrep on the bigger machine), then
  the per-site store export. `--dump-dyn-calls` already lists per call site
  (caller instance, `bbN[i]`) its targets - the export needs DefPathHashes /
  instance arg hashes instead of names. (2) output - the DOT
  graph is function-level; for a store we need per call site (caller
  instance, bb) the dyn targets, so add a dump inside rupta (it resolves dyn
  calls per call site internally; union contexts per site). (3) scale - its
  README suggests ~128 GB for large projects; this machine has 62 GB - a
  bigger machine is available, so not a blocker (run ripgrep there).
  (4) entry points: rupta gets exactly the roots verifopt gets. Whatever
  additional entry points verifopt uses (§9 gap 3: panic handler,
  interrupt/fault handlers, `#[used]`/`#[no_mangle]` roots, thread spawns)
  must be given to rupta too (rupta takes one `--entry-func`; add a
  multi-root option in the port), and to the RTA plugin walk - otherwise the
  comparison measures entry points, not the analyses.

**Experiment: tock deferred calls back to `dyn`** (planned Oct 27). Tock's
`kernel/deferred_call.rs` hand-devirtualizes: `DynDefCallRef { data: *const
(), callback: fn(*const ()) }`, the callback a closure in generic
`new<T>` calling `T::handle_deferred_call`, kept in `static DEFCALLS:
SingleThreadValue<[OptionalCell<DynDefCallRef>; 32]>`, called by
`service_next_pending` ("saves the 20 B vtable per client"). Question: does
verifopt let tock go back to plain `&dyn DeferredCallClient` (willtrojniak's
port, https://github.com/willtrojniak/tock/blob/deferred-call-dyn-dispatch-port/kernel/src/deferred_call.rs)
at no cost? Branch `verifopt-dyn-defcall` (tock worktree
`~/hack/verifopt-examples/tock-dyndefcall`, commit `63d5217de` on
`verifopt`): the port applied to the merged imix crate, keeping this tree's
`SingleThreadValue` API (values passed at bind time, not in `new`). Plain
build (fork compiler, `--skip-analysis --skip-rewrite`, its `plain_target`):
`.text` 164,068 -> 164,356 (+288 B, ~16 client vtables minus the closure
shims), `.sram` unchanged; imix has 16 `DeferredCallClient` impls.
Expectation: the one new site, `dc.handle_deferred_call()` in
`service_next_pending`, is megamorphic by design (`bit` comes from
`BITMASK`; an array's value is the union of its elements) - FSA ~= CHAmax ~=
the ~16 registered clients, so K=1 skips it and K>=16 is a compare chain
bigger than the indirect call: plain verifopt can't close the +288 B. What
could: VFE nulls the never-virtually-called `register` slots; the size
oracle (6s) rewrites the site with PointersExact over the ~16 clients so the
`DeferredCallClient` vtables become unread and shrink to placeholders (~16 x
8 B of compares vs ~16 x 20 B of vtables) - i.e. rewriting + vtable
elimination would recover the hand optimization automatically. Coverage
side effect: through `&dyn` the analysis follows the clients without any
fn-pointer casts (gap 4a doesn't arise). Runs: both versions x {plain,
plain+VFE, verifopt K=1, K=16, verifopt+VFE}, later 6s; metrics flash,
vtable bytes, `blx` count, FSA/CHAmax at the site. Needs a ~5 h tock
analysis on the branch (after `fsa3_target`).

**Two oracles: size and performance** (agreed Oct 27). Both use the same
instrumentation and recorder (steps 1-3 below); they differ in what they
rewrite, the edit kinds they need, the build profile and the metric. Both
are valid only on their workloads.

| | size oracle (6s) | performance oracle (6p) |
|---|---|---|
| goal | smallest binary that still behaves on the workload | fastest workload run |
| data | target set per site, unioned over several workloads | target set **and hit counts** per site |
| decided per | **trait**, whole program | **site**, from its profile |
| never-executed sites | left alone, unless the last read of a trait's vtables (then rewritten from VTmax) | left as dyn calls (no effect on time; extra code only costs i-cache) |
| executed multi-target sites | rewritten only if that lets the trait's vtables go | hot targets first, by frequency; cold tail stays behind the dyn fallback |
| edit kinds | PointersExact + vtable elimination (steps 4-5) | today's `pointers`, candidates ordered by frequency; unguarded direct call where only one target was ever seen |
| build profile | the project's size profile (`opt-level = "z"`/`"s"`) | `opt-level = 3` |
| metric | file-backed size / flash | hyperfine on the workload |
| main project | tock (ripgrep too) | ripgrep |

- Size: a rewrite saves bytes only through vtable removal (a trait's vtables
  unread → they and vtable-only methods go) and inlining of direct calls; each
  guarded candidate costs bytes (compare + branch + direct call, ~8 B Thumb,
  ~10-15 B x86). Hence a per-trait choice: rewrite *every* site of the trait
  with address compares and drop its vtables, or leave the trait alone. Cost
  model: guard bytes vs vtable bytes + vtable-only methods (inputs mostly from
  `py/vtread.py`: tock's 11 unread vtables, 43 single-vtable traits ~1.45 KB,
  17 multi-vtable traits), verified by a real build. Observed sets still help
  (fewer compares). Without vtable elimination the size gain is only
  inlining, which rung 5 already approaches - the size oracle is mostly steps
  4-5.
- Performance: profile-guided indirect-call promotion as a pure
  devirtualization bound. Reference point: plain + rustc PGO
  (`-Cprofile-generate`/`-Cprofile-use` profiles indirect-call targets and
  LLVM promotes them) - reported next to 6p, not as the oracle, since PGO also
  changes inlining and layout everywhere. Needs only frequency ordering in the
  existing `pointers` plan plus "observed-monomorphic → unguarded", so it is
  the cheaper oracle and can come first on ripgrep, before PointersExact.
- The recorder keeps hit counts from the start (8 B per entry); the dynamic
  metric (hit-weighted share of executed dyn calls made direct, per rung) is
  not reported for now but stays possible from the same table.

**Implementation steps**
1. Fork, `verifopt_rewrite.rs`: a site-enumeration / instrumentation mode
   (`VERIFOPT_INSTRUMENT=1`) covering *every* `InstanceKind::Virtual` call in
   every codegen'd instance (not only stored sites), in
   `rewrite_monomorphized` (sees every instance's monomorphized MIR, std's
   too under build-std). Site id = a `u64` hash of the store key (caller
   DefPathHash, bb, instance args) - crates/CGUs compile separately, so no
   global counter. Each crate writes a site table next to its instance
   manifests (id -> key, trait method, span). Enumeration alone gives rung
   3 (a store generator fills every site's targets from CHAmax; needs a
   clean build) and is a better coverage yardstick than the plugin's
   `enumerate_sites` (which misses generic instances a dependency compiles
   for itself).
2. Instrumentation proper: before each virtual call insert
   `record(site_id, slot_fnptr)`, reusing the `pointers` plan's slot load
   (`PtrMetadata` → slots → offset → load); the call itself stays. Also
   record dyn drops (`Drop` terminators on `dyn` places), `size_of_val` /
   `align_of_val` and upcasts (vtable reads that aren't method calls - only
   vtable elimination needs them). Recorder: a hidden function in the fork's
   `core` (both projects build core from source; adding a function changes no
   existing DefPathHash, unlike linking a recorder via RUSTFLAGS, which may
   change crate ids): fixed open-addressing table of (site, fnptr) → hit
   count, atomics (rg is threaded). ripgrep: the fork's `std` runtime cleanup
   writes it to `VERIFOPT_ORACLE_OUT`, plus one known function's runtime
   address (PIE base). tock: the table sits in `.bss` (512-1,024 entries,
   ~8-16 KB next to 23 KB `.sram` used of 64 KB), read with gdb/openocd by
   symbol. Instrumentation is an env var the compiler reads (no crate-id
   change); cargo doesn't track it - separate target dir + clean as usual.
3. Run the workload; resolve pointers to exact instances: instance
   manifests record each instance's symbol name, address → symbol → instance
   (tock's `-icf=all` folds identical functions: one address may name several
   instances - keep all, they're the same code, and say so in counts). Write
   an oracle store (store format, all targets exact, all sites `exact`;
   never-executed sites = CHAmax, empty CHAmax = dead) plus the hit counts.
   Through today's edit kinds this already gives a first rung 6 (1 target ->
   `single exact`, 2..K -> `pointers`, > K -> dyn); 6p adds frequency
   ordering.
4. Edit kind PointersExact: compare the vtable *address* against each
   candidate's vtable, direct call on match, no dyn fallback (so the site
   reads no vtable). One kind, two justifications: *oracle* (6s: the targets
   observed on the workload - valid only there) and *complete set* (sound: the
   site's candidate set is provably complete, e.g. complete CHAmax; then the
   last candidate is called unconditionally - n vtables, n-1 compares).
   "Complete" is over *vtables*, not methods: every type whose vtable for the
   trait is created - the vtable list the manifest RTA records (the same
   manifest data serves both). Oracle rewrite: 1 target -> `single exact`,
   2..K -> PointersExact, > K -> stays dyn. Needs one vtable per (type,
   trait): Rust doesn't guarantee it (several CGUs can each emit one), and
   with no fallback a duplicate takes the wrong branch. tock is safe
   (codegen-units=1, fat LTO); generally the fork emits vtables
   `linkonce_odr` under a mangled (type, trait) name so LTO merges them.
5. Vtable elimination - see "Vtable elimination at LTO" below.
6. VFE rungs: RUSTFLAGS must repeat the project's config flags (the env var
   replaces them) and add `-Zvirtual-function-elimination` (needs fat LTO:
   tock has it; ripgrep needs `-C lto`). Check the store still matches
   (tock's did: crate ids unchanged; edit counts identical).

**Vtable elimination at LTO** (plan, Oct 27; prioritized over Phase 4).
What exists: with `-Zvirtual-function-elimination` + fat LTO, the fork's
`rustc_codegen_ssa/src/meth.rs` `load_vtable` emits every vtable read whose
type has a principal trait as `llvm.type.checked.load(vtable, offset,
typeid(trait))` - method slots and drop/size/align (`get_usize`) alike -
and vtables carry `!type` + `!vcall_visibility`
(`rustc_codegen_llvm/src/debuginfo/metadata.rs`
`apply_vcall_visibility_metadata`). LLVM's GlobalDCE then nulls every slot
no checked load reads and drops the functions that become dead - but never
shrinks the vtable objects (nulled slots stay; a vtable nobody reads stays
full size).
Design - an LTO pass in the fork's `rustc_llvm` (C++,
`llvm-wrapper/PassWrapper.cpp`; LLVM is the system LLVM 21 from nix -
`download-ci-llvm = false`, `llvm-config` in `bootstrap.toml` - so the pass
lives in rustc's wrapper, not in LLVM), registered at the start of the fat-LTO
pipeline: run GlobalDCE (VFE) first so only live code counts; collect per
trait typeid the offsets any remaining `type.checked.load` reads; then
shrink each tagged vtable of that trait: (a) *placeholder* - no offset read:
a minimal unique global (address kept distinct for identity compares);
(b) *truncate* - drop the tail past the last read offset; (c) *compact* -
renumber the live slots per trait and rewrite every checked load's offset
for it (possible because fat LTO sees all of them; biggest win, most work).
Preconditions: (1) every vtable read must be typed - our `pointers` guard
reads the slot with a plain MIR load (make it a typed load - keeps that slot
- or don't read: PointersExact); audit reads that bypass `load_vtable`
(`vtable_size`/`vtable_align` intrinsics, ...); vtables without a principal
trait get no `!type` and are never touched. (2) Rewritten sites must stop
reading: `single exact` does; guarded `pointers` keeps a checked load in its
dyn fallback, so it never frees anything - complete-set PointersExact does.
Steps: (1) analytic estimate - extend `py/vtread.py` (job tmp) for (a)/(b)/(c)
on tock rungs 2-5 and the `verifopt-dyn-defcall` branch, from VFE +
`-C save-temps` builds (LTO input IR, `*.lto.after-restriction.bc`); decides
whether (c) is worth building; (2) fork: a flag (e.g.
`-Zverifopt-vtable-elim`, implies VFE, requires fat LTO), typed guard loads,
the untyped-read audit; (3) the LTO pass, (a)+(b) first, (c) if the numbers
justify it; (4) PointersExact (complete set) + canonical vtables; (5)
validation - ripgrep output equivalence (`rg_cmp.sh`; needs `-C lto` + VFE),
fixtures under a fat-LTO profile, tock size (correctness waits on hardware).
Note: VFE already frees the *methods* of unread slots (including drop glue at
offset 0), so what this adds over a VFE build is mainly vtable bytes - plus
whatever more sites become rewritten without a vtable read.
Step 1 results (Oct 27, `py/vtlevels.py` in the job tmp - reproduces VFE's
liveness from the LTO input IR `*.lto.after-restriction.bc` (llvm-dis from
nix llvm-21), then the bytes each level saves on the live vtables; builds:
VFE + `-C save-temps`, dirs `*vfeir_target` / `rwvfeir_target`, script
`vtir_builds.sh`). tock, cumulative savings beyond VFE:

| build (+VFE) | `.text` | live vtables | (a) | (b) | (c) | left after (c) |
|---|---|---|---|---|---|---|
| plain | 158,668 | 139 / 3,340 B | 12 | 160 | 2,144 | 1,196 B |
| rung 3 CHAmax everywhere | 157,616 | 137 / 3,288 B | 964 | 1,124 | 2,472 | 816 B |
| rung 5 (Oct 22 store) | 158,428 | 138 / 3,308 B | 220 | 356 | 2,340 | 968 B |
| verifopt, `fsa3` store | 157,940 | 138 / 3,308 B | 516 | 716 | 2,428 | 880 B |
| dyn deferred calls, plain | 158,932 | 149 / 3,540 B | 12 | 200 | 2,304 | 1,236 B |
| dyn deferred calls, rung 3 | 157,872 | 147 / 3,488 B | 964 | 1,164 | 2,632 | 856 B |

(c) is the level that matters (64-75% of live vtable bytes; much of it the
3-word drop/size/align header tock never reads); removing words in the middle
needs offset renumbering anyway, so build (a)+(b)+(c). Rewriting and
elimination compound: traits still read 70 (plain) / 40 (fsa3) / 28 (rung 3).
Dyn deferred calls: +264 B over the fn-pointer version (plain+VFE), +256 B
(rung 3); with (c) ~+100 B (the 10 `DeferredCallClient` vtables, 200 B,
compact to one word each; the rest is code). Caveats: assumes every vtable
read is typed (the verifopt builds' 4 `pointers n=1` guards read untyped -
those slots must stay); (c) must handle a deduplicated vtable serving
several traits (several `!type`s - union of their reads) and trait
upcasting (a `dyn Sub` vtable read through `dyn Super` with `Super`'s typeid
but no `!type` for it - the fork should emit supertrait `!type`s; possibly a
VFE soundness question too).

Steps 2-3 done (Oct 28), fork branch `verifopt-vtable-elim` (from
`verifopt-monomorph-rewrite-pinned` `73da158f0d5`): `70c77a914f3` flags +
VFE fixes, `a19ce4c390a` the LTO pass (`rustc_llvm/llvm-wrapper/VerifoptVtableShrink.cpp`)
+ typed guard loads, `bb8ccddc2c1` prebuilt-std warning. Installed as
`rust-mono-rewrite` (old build kept in `~/hack/rust/build_cp_pinned`; the
`rust-vtelim` rustup toolchain links `build/`). Flags:
- `-Zverifopt-vfe-fixes`: fixes for upstream VFE bugs, off = upstream VFE
  (so plain+VFE is measurable both ways): (1) `size_of_val` on a `dyn` ICEs
  ("only allows integer types in const_uint_big" - the typed load returns a
  pointer for the size/align words; fix: ptrtoint); (2) `vtable_size`/
  `vtable_align` (`DynMetadata`) hit `bug!` (no `dyn` in the intrinsic's
  type; fix: wildcard typeid `verifopt.any`); (3) trait upcasting to a prefix
  supertrait reuses the vtable pointer but the vtable has only the subtrait's
  `!type`, so VFE nulls the slot and the call jumps to 0 (fix: `!type` for
  each prefix supertrait).
- `-Zverifopt-vtable-elim` (requires VFE + fat LTO, implies the fixes): the
  pass; `VERIFOPT_VTABLE_ELIM_LEVEL=a|b|c` (default c),
  `VERIFOPT_VTABLE_ELIM_STATS=1`; Pointers guards' slot loads become typed.
- Warning under these flags if a crate in the LTO unit comes from the
  sysroot: VFE with a prebuilt std is unsound (std reads vtables untyped -
  e.g. `lang_start`'s `dyn Fn` call of `main` traps); use `-Zbuild-std`.
Correctness test (job tmp `vtelim_test`, `run_bstd.sh`: drops, `size_of_val`,
`Any`, upcast, `DynMetadata`): levels a/b/c with build-std print exactly
the plain output; plain VFE (no fixes) segfaults at the upcast.
tock `.text` (job tmp `tock_elim2.sh`; dirs `*vfefix_target`, `*elim_target`):

| build | VFE | VFE + fixes | + pass (c) | vtable bytes with pass |
|---|---|---|---|---|
| plain | 158,668 | 158,592 | 157,100 | 3,496 -> 1,700 |
| rung 3 | 157,616 | 157,616 | 155,656 | 3,416 -> 1,456 |
| verifopt (`fsa3` store) | 157,940 | 157,864 | 156,176 | 3,436 -> 1,524 |
| dyn deferred calls, plain | 158,932 | 158,856 | 157,244 | 3,696 -> 1,780 |
| dyn deferred calls, rung 3 | 157,872 | 157,872 | 155,796 | 3,616 -> 1,536 |

The dyn deferred-call overhead over the fn-pointer version: +264 B with VFE,
+140-144 B with the pass. Real savings are a bit below the estimate (plain:
1,796 B of vtables vs 2,144 estimated). tock correctness unchecked (no
hardware). Gotchas from installing it: `cargo install --path` builds in the
package's own `target/` and reused binaries linked to the old
`librustc_driver` (cp.sh prunes it) - `cargo clean` in both monomorph dirs
after a compiler rebuild; with RUSTFLAGS carrying `-Zverifopt-*`, cargo's
target probe must find the fork's rustc (put
`~/.rustup/toolchains/rust-mono-rewrite/bin` before the nightly on PATH);
cargo's probe also needs `-C lto` in RUSTFLAGS for VFE (the profile's `lto`
isn't applied there). Next: step 4 (PointersExact), ripgrep validation.

**Oct 28 follow-ups** (fork `e23f8c809cb`, `5c0f29b3ca6`; installed as
`rust-mono-rewrite`):
- VFE bugs vs upstream: known upstream - rust-lang/rust#98132 (open since
  2022: VFE with a prebuilt std; their diagnosis matches ours) and #164774
  (opened Oct 23 2026: a vtable created by const evaluation - a static
  `&dyn`, a promoted constant, a supertrait vtable another vtable points
  to - gets no `!type`, so VFE devirtualizes calls on it to another impl).
  Not found upstream (apparently new): the `size_of_val` ICE, the
  `vtable_size`/`vtable_align` `bug!`, the prefix-upcast miscompile.
  Fixed locally under `-Zverifopt-vfe-fixes`: all three new ones and
  #164774 (`e23f8c809cb`: const-created vtables typed like `get_vtable`'s -
  it also matters for the shrinking, which would renumber reads such a
  vtable's layout doesn't follow); #98132 only warned about.
- `5c0f29b3ca6`: the prefix-supertrait `!type` had also tagged 127 of
  tock's 160 vtables with `core::marker::MetaSized` (every trait's implicit,
  method-less supertrait), which put almost every trait in one shrinking
  group; now the sizedness markers and non-dyn-compatible supertraits are
  skipped. Big precision gain (table below).
- Correctness test (`vtelim_test`) now also covers #164774 (a static `&dyn`)
  and a non-prefix upcast: VFE + fixes and every shrinking level print the
  non-VFE output; upstream VFE ICEs (`size_of_val`).
- tock has no const-created vtables (160 typed vtables with and without the
  fixes; `py/typecount.py`), so #164774 didn't affect its numbers.
- **tock imix `.text`** (K=1, verifopt = gap-fixed `fsa3` store, VFE rows
  with our VFE fixes, after `5c0f29b3ca6`):

| build | `.text` (B) | vs plain |
|---|---|---|
| plain | 164,068 | - |
| plain + VFE | 158,592 | -5,476 |
| plain + VFE + vtable elim | 156,016 | -8,052 |
| verifopt | 161,916 | -2,152 |
| verifopt + VFE | 157,864 | -6,204 |
| verifopt + VFE + vtable elim | 154,132 | -9,936 (-6.1%) |

  Upstream VFE (no fixes): plain 158,668, verifopt 157,940. Also rung 3
  (CHAmax everywhere) + VFE + elim: 153,724. Rows "+ vtable elim" without
  VFE don't exist: the pass needs VFE's `!type` tags and typed loads (could
  be decoupled - emit them without LLVM's slot nulling).
- Vtables left after LTO (post-LTO IR, `py/vtfinal.py`; tock): plain + VFE
  + fixes 122 / 3,040 B; plain + VFE + elim 122 / 1,164 B (2 placeholders);
  verifopt + VFE + elim 120 / 868 B (30 placeholders, 79 one-word vtables,
  largest 52 B). The pass never deletes a vtable: fat pointers still hold
  their addresses. Option (undecided): placeholders of traits no PointersExact
  site compares could share one global (Rust doesn't guarantee vtable
  uniqueness anyway) - ~116 B on tock.
- The 4 `pointers n=1` guards in that build (`dyn Iterator<Item = u32>::next`
  x2, `dyn Uart::set_transmit_client` / `set_receive_client`) are the only
  readers of their slots; their fallback dyn calls still use the slot value,
  so those slots stay - with complete-set PointersExact they'd go (the
  `Iterator<Item = u32>` vtables entirely).
- ripgrep (build-std, fat LTO via `CARGO_PROFILE_RELEASE_LTO=fat` - `-C lto`
  in RUSTFLAGS alone clashes with cargo's `-C embed-bitcode=no` for deps;
  keep it in RUSTFLAGS too for cargo's probe): upstream VFE can't build rg
  (`size_of_val` ICE). VFE + fixes and VFE + elim produce the plain output
  on all 22 `rg_cmp.sh` commands (`rg_cmp.sh` now also strips `--json`
  `nanos` and `--stats` seconds; `rg fn core` has no `--sort` and can
  differ run to run). Vtable bytes 75,104 -> 73,328 (1,501 vtables; was
  -> 75,024 before the MetaSized fix). The Oct 21 store matches nothing under
  the LTO profile (0 edits), so verifopt + VFE + elim on ripgrep needs a new
  analysis.

**Minimal tock kernels (Oct 29).** The minimal nRF52840DK targets of
https://www.tockos.org/blog/2024/tock-binary-size-bloat/ (tweedegolf/tock
branches `min-size-kernel-uart` = kernel only, `min-size-kernel-with-app-support`
= one process + LED/alarm drivers; fetched as `tg/*` refs in the tock
checkout), ported to current tock: worktree
`~/hack/verifopt-examples/tock-minsize`, branch `verifopt-minsize` (from
`verifopt`). Boards in `boards/nordic/`: `nrf52840dk-min`, `nrf52840dk-min-app`
(faithful unmerged ports) and `*-merged` (everything merged into the board
crate, mostly reusing imix's merged copies; script `minsize/merge.sh` in the
job tmp, nrf52/nrf52840 migrated to 2024 with `cargo fix --edition`).
Differences from the blog: peripherals cut in a board-local struct (not by
editing `chips/nrf52`); `debug!` a no-op via a new kernel feature
`debug_noop` (off by default); process printing off via the existing
`no_debug_panics`; a board-local layout without the `.storage` page padding;
the blog's `memmove` commit has no counterpart (that code is gone). Sanity:
full upstream `nrf52840dk` 172,972 B `.text` (blog 2024: ~196.6 KB); unmerged
ports 5,636 / 21,512 B (blog: 6.8 / 19.8 KB). No hardware: sizes only.
Analysis (`fsa-gaps` plugin, `/tmp/vo_fsa4`) < 2 min each; FSA sites vs
rung 3: 15 vs 42 (kernel only), 105 vs 141 (app). Merged boards, K=1, VFE =
with our fixes (upstream VFE gives identical numbers on both); script
`minsize/matrix.sh`, results `minsize/results.txt`, dirs `<board>/<row>_target`.
"Indirect calls" counts `blx rN` only - see the note below the tables.

Kernel only (`nrf52840dk-min-merged`):

| build | `.text` | vs plain | indirect calls | vtables left after LTO |
|---|---|---|---|---|
| plain | 5,476 | - | 45 | - |
| plain + VFE | 5,516 | +40 | 51 | 2 / 36 B |
| plain + VFE + vtable elim | 5,488 | +12 | 51 | 2 / 8 B |
| verifopt | 5,460 | -16 | 43 | - |
| verifopt + VFE | 5,512 | +36 | 51 | 2 / 36 B |
| verifopt + VFE + vtable elim | 5,480 | +4 | 51 | 2 / 8 B (2 placeholders) |
| rung 3 + VFE + vtable elim | 5,480 | +4 | 51 | 2 / 8 B (2 placeholders) |

App (`nrf52840dk-min-app-merged`):

| build | `.text` | vs plain | indirect calls | vtables left after LTO |
|---|---|---|---|---|
| plain | 21,012 | - | 100 | - |
| plain + VFE | 19,212 | -1,800 | 25 | 6 / 348 B |
| plain + VFE + vtable elim | 19,016 | -1,996 | 25 | 6 / 180 B |
| verifopt | 20,600 | -412 | 38 | - |
| verifopt + VFE | 19,024 | -1,988 | 25 | 6 / 348 B |
| verifopt + VFE + vtable elim | 18,424 | -2,588 (-12.3%) | 25 | 6 / 48 B (1 placeholder) |
| rung 3 + VFE + vtable elim | 18,408 | -2,604 | 25 | 6 / 40 B (2 placeholders) |

Why VFE makes the kernel-only board bigger (investigated Oct 29; IR and
`calllines.py` in `minsize/`): (1) `SysTick::start` is called through a
`dyn SchedulerTimer` (one vtable). In both builds the call is direct before
the LTO inliner (plain: folded before LTO; VFE: whole-program
devirtualization early in LTO). Plain inlines it (remark: cost -14,960 =
40 minus the 15,000 last-call-to-a-local-function bonus); VFE doesn't ("too
costly, cost=40, threshold=5"), because the `SysTick` vtable still
references `start` - VFE keeps every slot until its GlobalDCE at the end of
LTO, so `start` has two uses (+76 B, `kernel_loop` -32 B,
`Uarte::handle_interrupt` -16 B, outliner stubs ~+6 B). A general VFE cost:
functions referenced only from (eventually dead) vtable slots lose the
single-use inlining bonus; on the app board slot removal outweighs it. (An
earlier reading - the checked load stays opaque until after the inliner -
was wrong: replaying the LTO pipeline with `opt` on the saved LTO input shows
the direct call before both inliner runs.) (2) The extra "indirect calls" are a counting artifact: the
post-LTO IR has the same indirect call sites in `kernel_loop` (52 plain, 51
VFE); in plain, the machine outliner moved 9 of them into two stubs ending in
`bx r2`, which a `blx` count misses. Counting `bx rN` too: kernel only 47
plain / 52 VFE, app 103 / 25 - still per instruction, so it understates call
sites when stubs are shared; count call sites (IR) for the indirect-call
metric. (3) Of kernel-only's remaining indirect calls, 46 are `dyn Process`
calls in `kernel/kernel.rs` and `kernel/memop.rs`, dead with `NUM_PROCS = 0`
(no `Process` type is ever created); LLVM can't prove it (process array in a
static), and the rewrite doesn't delete sites with an empty target set.

**Vtable elimination without VFE, VTmax, every site decided** (Oct 29 - Nov 2;
fork `7e3e57baefe`, `e9f83e3e836`, `83c3793b9dd`; `fsa-gaps` `8748acf`,
`2222fdc`, `caedf03`; `ripgrep` `dc530df`; none cherry-picked onto `ripgrep`
yet - pinned).
- **Standalone vtable elimination** (`-Zverifopt-vtable-elim` without VFE):
  rustc still types vtable reads and tags vtables, but sets no VFE module
  flag; the LTO pass alternates plain GlobalDCE and shrinking until nothing
  changes, then lowers every checked load to a plain load and drops the type
  metadata - LTO continues as without VFE (no slot nulling, no whole-program
  devirtualization). `VERIFOPT_VTABLE_ELIM_LEVEL=n`: lowering only (control:
  the typed reads cost ~nothing, within 36 B of plain everywhere).
- **Why VFE costs bytes** (corrected): VFE keeps every vtable slot until its
  GlobalDCE at the end of LTO, so a function referenced only by a soon-dead
  slot has two uses when the inliner runs and loses LLVM's 15,000
  last-call-to-local bonus (kernel-only board: `SysTick::start`, cost 40 vs
  threshold 5). Standalone elimination drops unread slots before inlining.
- **VTmax** (manifests list each crate's vtables: type, trait, slot
  instances; plugin `VTmax (n)` stats line, `VERIFOPT_EXACT_FROM=chamax|
  vtables`): CHAmax restricted to instances in some vtable - RTA's criterion
  with the collector's reachability (between CHAmax and from-`main` RTA).
  imix rung 3: single exact 513 -> 533, `.text` 160,312 -> 160,232, with
  elim 153,784 -> 153,808 (the newly direct calls' callees get inlined: code
  moves, 18 fewer `blx`). The 35 calls LLVM's devirtualization still makes
  direct over rung 3 + elim are capsule calls to hardware trait objects (UART,
  GPIO pins, MAC device, AES client) whose other compiled impl is never a
  vtable of that trait; 5 of them keep 2 vtables (probably same slot
  function - unverified).
- **Every compiled site gets a decision**: sites the analysis never reached
  (rung 3's enumeration) get CHA candidates, so CHAmax/VTmax decides them as
  in rung 3 (`VERIFOPT_NO_UNREACHED_FALLBACK=1` off; stats `Source:` per site
  and a `Sources:` summary; the harness skips not-reached sites). The bound
  replaces the analysis's set wherever it is smaller: 1 instance -> single
  exact, 2..=K -> exact instances as targets (fork `pointers n=K exact`;
  `VERIFOPT_NO_EXACT_MULTI=1` off; not over planned tags). The old "single
  instance isn't among FSA's targets" refusal was a bug in the check: at all
  129 such tock sites the compiled function is in FSA's set (the extras are
  never-compiled impls); a real miss now logs "analysis bug" and uses the
  bound. `--no-guarded` (`VERIFOPT_NO_GUARDED`): only single exact rewrites,
  every guarded edit stays dyn (size setting).
- imix K=1: fallback run (`fsafb_target`, 6.4 h) single exact 453 -> 512
  (rung 3 513), `.text` 161,916 -> 160,368 (rung 3 160,312); `--no-guarded`
  160,336 (the 4 unproven RNG / UART-mux guards); the rest (+24 B) is the
  I2C interrupt site the bound rule now fixes (run `fsabd_target`, not
  needed for now). K=4 (rung 3): 7 more sites rewritten, `.text` 162,768 ->
  163,180 - guards add code, so K=1 is the size setting on tock.

imix `.text` (K=1; VFE = with our fixes; verifopt = `fsa3` store unless noted):

| build | `.text` | vs plain |
|---|---|---|
| plain | 164,068 | - |
| typed reads only (control) | 164,032 | -36 |
| elim (no VFE) | 159,712 | -4,356 |
| VFE | 158,592 | -5,476 |
| VFE + elim | 156,016 | -8,052 |
| verifopt + elim (no VFE) | 155,760 | -8,308 |
| verifopt + VFE + elim | 154,132 | -9,936 |
| rung 3 + elim (no VFE) | 153,784 | -10,284 |
| rung 3 + VFE + elim | 153,724 | -10,344 |

Minimal nRF52840DK boards (K=1; verifopt = final plugin: fallback + bound
rule, `fsabd` stores; `--no-guarded` identical in every verifopt row):

| build | kernel-only | app |
|---|---|---|
| plain | 5,476 | 21,012 |
| plain + VFE | 5,516 | 19,212 |
| plain + elim (no VFE) | 5,412 | 19,564 |
| plain + VFE + elim | 5,488 | 19,016 |
| verifopt | 5,460 | 20,496 |
| verifopt + VFE | 5,512 | 19,012 |
| verifopt + elim (no VFE) | **5,368** | 18,440 |
| verifopt + VFE + elim | 5,480 | **18,408** |
| rung 3 | 5,460 | 20,496 |
| rung 3 + VFE | 5,512 | 19,012 |
| rung 3 + elim (no VFE) | 5,368 | 18,440 |
| rung 3 + VFE + elim | 5,480 | 18,408 |

(Rung-3 rows: Oct 29 runs, before the vtable-manifest change, which doesn't
affect rung 3 at K=1. Verifopt equals rung 3 in every build on both boards.)

Best: kernel-only verifopt + elim without VFE (-108 B), app verifopt + VFE +
elim (-2,604 B, -12.4%; equal to rung 3 + VFE + elim).

**Rung 3 implementation** (Oct 23, plugin only, uncommitted): env
`VERIFOPT_CHA_ONLY=1` - `chamax::compiled_instances` lists this crate's mono
items plus every dependency's non-generic functions (generic instances a
dependency compiles for itself are missed); `InterpPass::enumerate_sites`
records every virtual call in their bodies with its CHA candidates as the
"FSA" set; the usual store/CHAmax code then makes CHAmax == 1 sites
`single exact` and caps the rest at K. No interpretation. It runs the
partitioning before the store exists (no hook edges in pass 1; pass 2
rebuilds with them). Installed in `/tmp/vo_rung3`.
tock (K=1, `boards/imix/chamax_target`, 123 s for the whole pipeline):
668 sites (FSA reached 369), 513 `single exact`, 141 `too_many_targets`,
18 dead (CHAmax 0); CHAmax complete at 667, n/a at 1. All 28 core sites
have >= 9 candidates (fmt::Write), so nothing in core is rewritten at K=1.
`.text` 160,312, `blx` 287 - better than verifopt (162,972 / 526) because
it covers every compiled site; still above plain+VFE (158,668 / 250).

**Ladder table (Oct 26, K=1; bytes).** tock (`boards/imix/<dir>/thumbv7em-none-eabi/release/imix`):

| rung | dir | `.text` | flash | `.sram` | vtables (scan) | `blx rN` |
|---|---|---|---|---|---|---|
| 1 plain | `plain_target` | 164,068 | 167,156 | 23,128 | 132 / 3,104 | 731 |
| 2 plain+VFE | `plainvfe_target` | 158,668 | 161,756 | 23,128 | 131 / 3,052 (39 null slots) | 250 |
| 3 CHAmax everywhere | `chamax_target` | 160,312 | 163,400 | 23,128 | 131 / 3,084 | 287 |
| 4 verifopt (Oct 22) | `statics_target` | 162,972 | 166,060 | 23,128 | 132 / 3,104 | 526 |
| 5 verifopt+VFE | `rwvfe_target` | 158,428 | 161,516 | 23,128 | 131 / 3,032 (47 null slots) | 249 |

ripgrep (`<dir>/x86_64-unknown-linux-gnu/release/rg`; unwind = `.eh_frame_hdr
+ .eh_frame + .gcc_except_table`):

| rung | dir | `.text` | `.rodata` | `.data.rel.ro` | unwind | file-backed | `.data`/`.bss` |
|---|---|---|---|---|---|---|---|
| 1 plain | `baseline_target` | 2,787,554 | 761,828 | 335,696 | 495,956 | 4,381,034 | 2,912 / 304 |
| 1 hook baseline (Oct 21 store) | `hook_target` | 2,778,626 | 762,112 | 335,928 | 496,900 | 4,373,566 | 2,912 / 288 |
| 3 CHAmax everywhere (undercounts) | `chamax_target` | 2,783,010 | 762,708 | 335,304 | 496,668 | 4,377,690 | 2,912 / 296 |
| 4 verifopt (Oct 21 plugin, pre-statics) | `timing_target` | 2,780,034 | 762,112 | 335,928 | 497,348 | 4,375,422 | 2,912 / 288 |
| 4 verifopt (Nov 3, `a9ad236`) | `exp_full8_target` | 2,777,667 | 762,220 | 336,056 | 497,028 | 4,372,971 | 2,912 / 296 |
| 4 verifopt (Nov 4, `c85db8c`) | `exp_full14_target` | 2,779,955 | 762,716 | 336,072 | 498,032 | 4,376,775 | 2,920 / 296 |

Indirect calls (`call *`, excluding calls through the GOT): plain 3,503,
Oct 21 rung 4 3,526, Nov 3 rung 4 3,514, Nov 4 rung 4 3,526. Both are
-4 to -8 KB file-backed vs plain with the same 13 rewrites, but no hook
baseline (partitioning edges from the same store, no rewrite) exists for
either: the Oct 21 one alone was -7,468 B, so the differences are probably
mostly partitioning (Nov 4 vs Nov 3: +3,804 B with identical rewrites, from
the hook's edges for more sites and bigger target sets).

Missing: hook baselines for the Nov 3/4 stores, rung 4/5 under fat LTO (the
LTO rows on the dashboard are rungs 1-2 + elimination; needs a new analysis
under the LTO profile - stores are per build config), rung 3's own hook
baseline, vtable bytes for ripgrep (a scan of `.data.rel.ro`); rung 6 both.

Tock rung 4 re-checked Oct 26 on `ee0d35c` (cap unset, `boards/imix/cur_target`,
pipeline 8,362 s): same 369 sites, same edits, byte-identical `.text`.

**Status**: tock rungs 1, 2, 3, 4, 5 exist at K=1 (`plain_target`,
`plainvfe_target`, `statics_target`, `rwvfe_target`: `.text` 164,068 /
158,668 / 162,972 / 158,428). ripgrep: rungs 1 and 4 (Nov 3,
`exp_full8_target`) exist in the default profile, rungs 1-2 + elimination
under fat LTO; rung 5 needs an analysis under the LTO profile. Order (revised Oct 27): steps 1-3 (shared
instrumentation, recorder with counts, oracle store) after the current tock
run (compiler rebuild); then ripgrep 6p (frequency-ordered `pointers`, no new
edit kind); then PointersExact + vtable elimination for 6s (tock first, then
ripgrep with merged vtables); tock's workload waits for hardware.
Known coverage gaps that limit rungs 3-5 (fix separately, §6): `Drop`
terminators ignored; CHA candidates of generic impls (lifetimes included)
never interpreted.

## 9. Plan: FSA coverage gaps (written Oct 26 2026, not implemented)

Measured on tock (Oct 23, `py/missed.py` on rung 3 vs FSA stats): of 668
compiled dispatch sites, FSA records 369; of the 343 it misses, 169 are in
capsule code, 91 are other instances of recorded sites, 82 elsewhere
(panic path ~39, deferred calls ~27, fmt number formatting ~7, generic
chip-driver candidates ~6, introspection 3), 1 in drop glue. Rung 3's site
set is the yardstick: after each step, rerun tock and check how much of it
FSA covers (FSA sites outside rung 3 are uncompiled code, harmless).

Order by expected payoff; each adds interpreted code, so measure tock's
time/memory after each, ripgrep only once tock is fine.

**1. CHA fallback from compiled instances ("CHAmax fallback").** Today a
CHA candidate is a bare impl fn (`(DefId, None)`); `simulate_static_calls`
skips any whose fn has generic params - lifetimes included - as "args
mismatch" (tock: 667 fns / 60k calls), and interprets non-generic impls
nothing ever instantiates (imix's test clients, AT24C). Instead, when FSA
is empty, take the *compiled instances* of the CHA candidates (the final
crate's mono items, `chamax::compiled_instances`; erased lifetimes, concrete
type args) and simulate each with its args. Sound (CHAmax is complete with
build-std) and more precise than CHA. Caveats: running mono collection
before the analysis means the partitioning hook sees no store in pass 1
(pass 2 must always run - write the marker); generic instances a dependency
compiles for itself aren't in the final crate's mono items (fine for tock;
ripgrep needs the fork's manifests to list every compiled instance -
compiler rebuild). Cheap partial step if needed first: fill lifetime-only
generics with erased regions.

**2. `Drop` terminators** (`interp_terminator`: `Drop { .. } => Ok(None)`).
Interpret `drop(place)` as a call to `Instance::resolve_drop_in_place(ty)`
(rustc_public) with `_1` = a pointer to `place` (alias edge to the caller's
place, as for `&raw mut`), through the usual memo/write-log path. For a
`dyn T` place: the receiver's concrete types from its value (else CHA's
types of `T`), each type's drop glue simulated like a dispatch's candidates
(not a store site - the rewrite doesn't touch drops - but needed for
coverage and the vtable-read analysis). Cost control: skip glue that can
only deallocate (no user `Drop` impl and no trait object reachable in the
type) - the analysis doesn't model deallocation, so that's sound.

**3. Other entry points.** (a) Panic handler: a call to core's
`panic_impl` (foreign, answered "no MIR" today) goes to the crate's
`#[panic_handler]` (`tcx.lang_items().panic_impl()`), or std's
`begin_panic_handler` in std programs (that pulls in std's panic hook and
backtrace printing - watch ripgrep's cost). (b) Interrupt/fault handlers
and other roots: functions referenced from `#[used]`/`#[link_section]`
statics (tock's `.vectors`/IRQ tables in `sam4l/mod.rs`: their allocations'
`GlobalAlloc::Function` entries) and exported `#[no_mangle]` fns. After
`main`, interpret each root on the same context with "any value" args;
handlers can run at any point, so repeat main + roots until no static's
version changes (bounded rounds; per-static memo versions make reruns
cheap). (c) Threads (ripgrep): `thread::spawn`'s closure runs through an
`extern "C"` fn pointer handed to pthread - check whether it's lost; if so,
a spawn stub that calls the closure directly.

**4. Calls through fn pointers** (`interp_fn_ptr`: answered from the
signature, "fn pointer called" in `--ASSUMPTIONS--`). (a) Fix the losses:
tock's deferred calls store `callback: fn(*const ())` (a closure cast to a
fn pointer) in `DynDefCallRef`, kept in a static behind `SingleThreadValue`
- trace where the `Closure` value is lost (same class as the open
`fmt::rt::Argument` formatter-field issue, §6), with
`VERIFOPT_TRACE_SCOPE`. (b) Sound fallback for an unknown callee, like CHA
for dyn: the *address-taken* functions - targets of `ReifyFnPointer` /
`ClosureFnPointer` casts in compiled bodies and `GlobalAlloc::Function`
entries in statics - with a compatible signature (arity and argument kinds;
`fmt::Argument` transmutes `fn(&T, &mut Formatter)` to `fn(NonNull<()>,
..)`, so match loosely). Can be megamorphic (every `Display::fmt`) - same
cost concerns as ripgrep's `&dyn Debug` sites.

**Status, Oct 27.** Step 1 implemented on branch `fsa-gaps` (worktree
`~/hack/verifopt-fsa`, from `ee0d35c`): compiled instances
(chamax::compiled_instances) collected before the analysis; at every
dispatch, candidates without generic args (CHA fallback *and* FSA's own
CHA-sized sets for unknown `dyn` receivers - expanding only on an empty
FSA set changed almost nothing) are simulated as their compiled instances;
the second build pass always runs. tock (`boards/imix/fsa2_target`, K=1):
606 sites (was 369; rung 3 has 668), FSA misses 111 rung-3 sites (was 343:
capsules 169 -> 32, other instances 91 -> 6), args-mismatch skips 667 ->
513 functions; edits 454 `single exact`, 4 `pointers n=1`, 99
`too_many_targets`; `.text` 161,892 (was 162,972; rung 3 160,312);
pipeline 19,073 s (was 8,362 s). What's left of the 111: the panic path
(~39), deferred-call / fn-pointer code (~27), fmt number formatting (~7) -
steps 3 and 4 - plus 32 capsule sites.
Committed on `fsa-gaps` (Oct 27): `a2a6ad2` step 1; `cfebc51` closure kind
read at index len-3 (panicked for closures made in generic fns and called
through fn pointers); `882dd06` static values written over the initial
"any" value survive field-path reads (`project_value`) and array-element
writes (`type_skeleton`), fixture `fnptr_static_table` (tock's deferred
calls; FSA {A, B}); `3395c28` value-shape diagnostics
(`VERIFOPT_TRACE_STATICS`, assignments / indirect callees under
`VERIFOPT_TRACE_SCOPE`). tock run `boards/imix/fsa3_target` (plugin
`/tmp/vo_fsa3`, started Oct 27 13:56) measures these against `fsa2_target`:
deferred-call coverage, remaining fn-pointer calls (unknown vs partly
unknown callee - decides step 4b's scope), cost.

**Branches `ripgrep` and `fsa-gaps` stay separate** (decided Oct 27) until
ripgrep's value growth is fixed - no merge before then, so the ripgrep work
doesn't have to be sorted through alongside the gap fixes. `ripgrep` has the
flatten fix (`2cff554`), growth diagnostics (`5a6d58a`), warning fix and
fixture updates (`1951384`, `f2692c9`: 53 pass / 6 red); `fsa-gaps` has the
gap work above but not those (7 red there: `merge_widened_field` and
`raw_ptr_static_write` still fail on their old goldens). Consequences: tock
runs (incl. the deferred-call dyn experiment) use `fsa-gaps`; ripgrep runs
use `ripgrep` - never analyze ripgrep from `fsa-gaps` (no flatten fix: it
explodes). If a fixed ripgrep run needs gap fixes before the merge,
cherry-pick just those commits. (Nov 3: ripgrep's uncapped analysis finishes
on `ripgrep` - `a9ad236`, §4 - so the condition for merging is met; the
merge itself hasn't been decided or done.)

Related: the megamorphic cutoff (`VERIFOPT_MEGAMORPHIC_CAP`, opt-in, ripgrep
experiments only) and its havoc of `&mut` args are *not* part of the tool;
the principled replacement for ripgrep's cost is per-candidate summaries
reused across contexts (option 3 in the Oct 26 discussion), to be planned
if the cap-128 run doesn't finish.
