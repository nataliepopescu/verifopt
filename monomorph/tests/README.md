# Testing

`dispatch_examples.rs` runs the real `cargo-verifopt`/`verifopt` binaries
against the fixture crates in `../testing_examples/<name>` and checks the
results. It replaces the manual checklist that used to live only in
`testing_examples/README.md` with something `cargo test` enforces.

## Requirements

Same as building the tool itself: the pinned nightly toolchain in
`../rust-toolchain.toml` (with `rustc-dev` / `llvm-tools`) active. The harness
tries to set `LD_LIBRARY_PATH` for you (see monomorph/README.md's
Troubleshooting section on `librustc_driver.so`), but if that fails, export it
yourself first:

```sh
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH
```

## Running

```sh
cargo test --test dispatch_examples
```

Run a single fixture (each has its own `#[test]` fn, named after the
directory, e.g. `rand_` for `testing_examples/rand_`):

```sh
cargo test --test dispatch_examples -- casting_traitobj --exact --nocapture
```

`--nocapture` is useful because on failure the harness prints the tool's
stdout/stderr and, for `Unclassified` fixtures, its normalized JSON output.

## The two states (see `Expectation` in `support/mod.rs`)

- **`Passing`** — must compile under `cargo verifopt --release` *and* produce
  dispatch results matching `tests/golden/<name>.json`. This is used for
  every fixture that has a defined "correct" answer, including the ones that
  don't work yet (per the TODOs in `testing_examples/README.md`: unconverted
  MIR constructs, unhandled inline asm, etc). Those tests are *expected to
  fail* right now — that's an honest signal, not something to special-case
  away, and the failure will go away on its own once the underlying tool
  issue is fixed.
- **`Unclassified`** — for fixtures nobody has characterized yet (currently
  `no_vtable_check`, `recursive_dyn`). Runs, prints the result, asserts
  nothing. Use this only while you're first figuring out what "correct"
  output even looks like for a new fixture.

## Golden files

Golden files store a *normalized* view of the `stats` file: the source span
text plus sorted candidate-function names for CHA and FSA at each call site.
Volatile internals (`DefId` numeric ids, `Span` ids, `GenericArgs` debug
output) are stripped out during parsing, since those aren't stable across
compiler sessions and would otherwise cause false-positive diffs.

`tests/golden/casting_traitobj.json` is seeded directly from the example in
`testing_examples/README.md`'s "Building VerifOpt and Running Examples"
section — treat it as a starting point and re-bless it once you've run the
suite locally, rather than assuming it's already verified against a live
build.

The other 13 `Passing` fixtures don't have golden files yet. Running them
will fail — for `closures`, `default`, `fnptrs`, `generic`, `recursive`,
`shims`, `switchint` that's just a missing-golden-file message pointing you
at the bless command below; for `one_variant`, `rand_`, `two_variants`,
`two_variants_rand`, `two_variants_static`, `two_variants_static_nonzst` it's
a genuine compile/analysis failure in the tool (see
`testing_examples/README.md`'s TODOs) and blessing won't help until that's
fixed.

### Generating / updating a golden file

```sh
BLESS_GOLDEN=1 cargo test --test dispatch_examples -- <name> --exact
```

Then **review the diff before committing** — `BLESS_GOLDEN` always writes,
whether the change is an intended fix or a regression, so don't run it across
the whole suite blindly.

## Adding a new fixture

1. Add `testing_examples/<name>/` (a normal Cargo crate; see the existing
   fixtures for the pattern of using `#[inline(never)]` / vtable-metadata
   tricks to defeat trivial inlining).
2. In `dispatch_examples.rs`, add:
   ```rust
   example_test!(my_new_fixture, "my_new_fixture", Unclassified);
   ```
3. Run it, read the printed normalized output, and once it looks right,
   change `Unclassified` to `Passing` and bless a golden file as above.

## Known limitation

`stats`/`found_ex`/`notfound_ex` are opened in **append** mode by
`VOLogger`, so a stale file from a previous manual run in a fixture directory
would get mixed into the next run's parsed output. The harness deletes them
before each run; if you're ever running the tool manually in these
directories outside the harness, do the same (`rm -f stats found_ex
notfound_ex`) before comparing output.
