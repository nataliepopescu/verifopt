# verifopt — handoff notes

Context for continuing this work in Claude Code. Started at the end of a long
debugging/design conversation (Sept–Oct 2026), last updated Oct 27 2026. Verify against
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
- Current ripgrep cost (Oct 2026, build-std, after the Oct 18–20 fixes):
  the `rg` analysis takes hours and ~30 GB; don't run two at once on the
  62 GB machine. Run experiments from a separate install root
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
- **Analysis cost after the precision fixes**: hours and ~30 GB for ripgrep.
  Context merges after candidate simulation were the hotspot; the Oct 20
  dirty-key merges took the analysis from 229 to 196 min - a smaller gain
  than hoped, so profile again before the next optimization. Next levers:
  memo pressure (50 arg sets per function before widening; keys hold full
  argument values), and the 4 sites with >3,000 CHA-sized targets.
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

## 7. Recent commits (Oct 17–26 2026; `git log` is authoritative)

Plugin (`~/hack/verifopt`, branch `ripgrep`, not pushed as of Oct 26):

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

**Oracle workload: one per project, never a union** (a single run keeps the
observed sets tight; more runs only widen them toward what a sound analysis
must assume). Measure the oracle build (speed, output check) on that same
run; report executed and never-executed sites separately.
- ripgrep: `rg -n 'fn \w+\(' ~/hack/rust/library` (walker, glob/type filters,
  regex engine, searcher, printer).
- tock: boot + one app's workload on the imix board (QEMU has no sam4l;
  waits for hardware, like the boot test).

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

**Two oracles: size and performance** (agreed Oct 27). Both use the same
instrumentation and recorder (steps 1-3 below); they differ in what they
rewrite, the edit kinds they need, the build profile and the metric. Both
are valid only on their workload.

| | size oracle (6s) | performance oracle (6p) |
|---|---|---|
| goal | smallest binary that still behaves on the workload | fastest workload run |
| data | target set per site | target set **and hit counts** per site |
| decided per | **trait**, whole program | **site**, from its profile |
| never-executed sites | still rewritten from CHAmax where that helps make a trait's vtables unread (agreed) | left as dyn calls (no effect on time; extra code only costs i-cache) |
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
4. New edit kind PointersExact (6s): compare the vtable *address* against each
   candidate's, direct call on match, no dyn fallback (so the comparison
   doesn't keep vtables read). Oracle rewrite: 1 target -> `single exact`,
   2..K -> PointersExact, > K -> stays dyn. Catch: Rust doesn't guarantee one
   vtable per (type, trait) - with several CGUs it can be emitted more than
   once, and with no fallback a duplicate takes the wrong branch even on the
   workload. tock is safe (codegen-units=1, fat LTO); ripgrep needs the fork
   to give vtables a fixed symbol name with linkonce semantics so the linker
   merges duplicates.
5. Vtable elimination: unread = no remaining dyn call of its trait and no
   drop/size/align/upcast read. Measure analytically first; to build it, a
   second pass (like the two-pass flow) where `get_vtable`
   (`rustc_codegen_ssa/src/meth.rs`) emits a small unique placeholder for each
   unread (type, trait) - PointersExact compares by identity - and the linker
   drops the methods.
6. VFE rungs: RUSTFLAGS must repeat the project's config flags (the env var
   replaces them) and add `-Zvirtual-function-elimination` (needs fat LTO:
   tock has it; ripgrep needs `-C lto`). Check the store still matches
   (tock's did: crate ids unchanged; edit counts identical).

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

Missing: ripgrep rungs 2, 5 (fat-LTO builds), a current rung 4 (no
complete analysis since the statics work), rung 3's own hook baseline,
vtable bytes for ripgrep (a scan of `.data.rel.ro`); rung 6 both.

Tock rung 4 re-checked Oct 26 on `ee0d35c` (cap unset, `boards/imix/cur_target`,
pipeline 8,362 s): same 369 sites, same edits, byte-identical `.text`.

**Status**: tock rungs 1, 2, 3, 4, 5 exist at K=1 (`plain_target`,
`plainvfe_target`, `statics_target`, `rwvfe_target`: `.text` 164,068 /
158,668 / 162,972 / 158,428). ripgrep: rung 1 exists; 4-5 wait for the
`statics_target` analysis run. Order (revised Oct 27): steps 1-3 (shared
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
`~/hack/verifopt-fsa`, from `ee0d35c`, uncommitted): compiled instances
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

Related: the megamorphic cutoff (`VERIFOPT_MEGAMORPHIC_CAP`, opt-in, ripgrep
experiments only) and its havoc of `&mut` args are *not* part of the tool;
the principled replacement for ripgrep's cost is per-candidate summaries
reused across contexts (option 3 in the Oct 26 discussion), to be planned
if the cap-128 run doesn't finish.
