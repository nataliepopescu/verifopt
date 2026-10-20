//! Integration tests that run the `verifopt` tool against the fixture crates in
//! `../testing_examples/<name>` and check the results.
//!
//! This turns the manual checklist that used to live only in
//! `testing_examples/README.md` into something `cargo test` enforces:
//!
//! - `Passing` examples must (a) compile under `cargo verifopt --release` and
//!   (b) produce dispatch-analysis results matching a checked-in golden file
//!   in `tests/golden/<name>.json`. This is asserted for every fixture below,
//!   including the ones that don't currently work — if a fixture doesn't
//!   compile or analyze correctly yet, its test is *supposed* to fail (red),
//!   rather than being special-cased to pass.
//! - `Unclassified` examples just run and print their normalized output; nothing
//!   is asserted. Use this only while you're first characterizing a brand new
//!   fixture and don't yet know what "correct" looks like.
//!
//! ## Running
//!
//! Requires the pinned nightly toolchain from `rust-toolchain.toml` (with the
//! `rustc-dev`/`llvm-tools` components) to be active, since this builds and
//! runs the real `cargo-verifopt`/`verifopt` binaries.
//!
//! ```sh
//! cargo test --test dispatch_examples
//! ```
//!
//! Each fixture is a separate #[test] fn, so `cargo test --test
//! dispatch_examples -- casting_traitobj --exact` runs just one, and
//! `--nocapture` will show the tool's stdout/stderr on failure.
//!
//! ## Adding / promoting a fixture
//!
//! 1. Add a new `testing_examples/<name>` crate (or point at an existing one).
//! 2. Add an `example_test!(fn_name, "<name>", Unclassified);` line below and
//!    run it once to see what the tool currently produces.
//! 3. Once the behavior looks right, change it to `Passing` and generate its
//!    golden file:
//!
//!    ```sh
//!    BLESS_GOLDEN=1 cargo test --test dispatch_examples -- <fn_name> --exact
//!    ```
//!
//!    Then diff the resulting `tests/golden/<name>.json` and commit it
//!    deliberately — don't run BLESS_GOLDEN across the whole suite blindly,
//!    since that would silently paper over regressions.

mod support;

use support::Expectation::{Passing, Unclassified};

macro_rules! example_test {
    ($fn_name:ident, $dir:literal, Passing) => {
        #[test]
        fn $fn_name() {
            support::run_example($dir, Passing);
        }
    };
    ($fn_name:ident, $dir:literal, Unclassified) => {
        #[test]
        fn $fn_name() {
            support::run_example($dir, Unclassified);
        }
    };
}

// Every fixture below is expected to compile and match its golden file.
// Several currently don't (see testing_examples/README.md's TODOs: unconverted
// MIR constructs, unhandled inline asm) — that's fine, their tests are
// expected to fail (red) until the underlying tool issue is fixed. That's a
// truer signal than pretending they pass.
example_test!(btreemap, "btreemap", Passing);
example_test!(casting_traitobj, "casting_traitobj", Passing);
example_test!(closures, "closures", Passing);
example_test!(constraint_hierarchy, "constraint_hierarchy", Passing);
example_test!(default, "default", Passing);
example_test!(dep_rewrite, "dep_rewrite", Passing);
example_test!(diff_sites, "diff_sites", Passing);
example_test!(fields, "fields", Passing);
example_test!(fnptrs, "fnptrs", Passing);
example_test!(generic, "generic", Passing);
example_test!(generic_traitobj, "generic_traitobj", Passing);
example_test!(list_fields, "list_fields", Passing);
example_test!(nested_struct, "nested_struct", Passing);
example_test!(recursive, "recursive", Passing);
example_test!(shims, "shims", Passing);
example_test!(switchint, "switchint", Passing);
example_test!(one_variant, "one_variant", Passing);
example_test!(two_variants, "two_variants", Passing);
example_test!(two_variants_static, "two_variants_static", Passing);
example_test!(
    two_variants_static_nonzst,
    "two_variants_static_nonzst",
    Passing
);

example_test!(recursive_dyn, "recursive_dyn", Passing);
example_test!(recursive_dyn2, "recursive_dyn2", Passing);
example_test!(recursive_dyn3, "recursive_dyn3", Passing);
example_test!(refs_conflict, "refs_conflict", Passing);
example_test!(mut_refs, "mut_refs", Passing);
example_test!(mut_refs2, "mut_refs2", Passing);
example_test!(simple, "simple", Passing);
example_test!(simple_single, "simple_single", Passing);
example_test!(r#static, "static", Passing);
// Same shape as `static`, but the selector is an `extern` static (no
// initializer, value unknown to Rust): evaluating it used to ICE the
// analysis. All three impls must stay possible targets.
example_test!(extern_static, "extern_static", Passing);
example_test!(sip_repro, "sip_repro", Passing);
example_test!(vec_growth, "vec_growth", Passing);
example_test!(wtos_conflict, "wtos_conflict", Passing);

// Not yet characterized at all — run and report, don't assert.
example_test!(rand_, "rand_", Unclassified);
example_test!(two_variants_rand, "two_variants_rand", Unclassified);

// ripgrep's `HiArgs::sort` -> `for haystack in haystacks` shape: a generic
// fn boxes one of several concrete iterators as `Box<dyn Iterator>`, and a
// `for` loop consumes it (after inlining, `<Box<dyn Iterator> as
// Iterator>::next` becomes a dyn call on the pointee). On ripgrep this
// site got a spurious `<Box<I, A> as Iterator>::next` candidate with
// unsubstituted args `[T/#0, Global]`, making it unrewritable. Expected
// targets: exactly `<Counter as Iterator>::next` and `<Countdown as
// Iterator>::next` - no Box.
example_test!(box_dyn_iter, "box_dyn_iter", Passing);

// The other side of box_dyn_iter's fix: `Box<Box<Countdown>>` unsized to
// `Box<dyn Iterator>` makes the *inner* Box the concrete type. Expected
// targets: `<Counter as Iterator>::next` and `<Box<Countdown, Global> as
// Iterator>::next` (the forwarding impl) - the outer Box must not appear, the
// inner one must. The runtime trace should show `<Countdown as
// Iterator>::next`, reached through the inner Box's forwarding `next`.
// `<Countdown as Iterator>::next` itself must *not* be a dispatch target:
// it's only ever called statically by the inner Box's `next`
// (resolve_adt_helper no longer searches inside an ADT that is the object).
example_test!(box_box_dyn_iter, "box_box_dyn_iter", Passing);

// The other half of ripgrep's `sort`: an iterator adapter boxed as `dyn
// Iterator`. Expected targets: `<Map<I, F> as Iterator>::next` (with args
// `[vec::IntoIter<u32, Global>, {closure}]`) and `<Counter as
// Iterator>::next` - not `<vec::IntoIter<T, A> as Iterator>::next` from
// Map's `iter` field, nor the closure from its `f` field.
example_test!(map_dyn_iter, "map_dyn_iter", Passing);

// A CHA-fallback site with exactly one compiled instance in the whole program
// (CHA: the generic `Driver<C>` impl plus an unused one; CHAmax: just
// `<Driver<Sam> as Device>::poll`). Expected: rewritten as "single exact" -
// a direct call to that instance - with `Sam::id` traced once.
example_test!(cha_single_instance, "cha_single_instance", Passing);

// CHA-fallback sites dispatching traits with their own lifetime / type params
// (`Client<'a>`, `Sink<F>` with a generic impl): CHA must still list their
// impls, and with one compiled instance each, both are rewritten as "single
// exact". Before the fix CHA was empty there and nothing was rewritten.
example_test!(cha_lifetime_trait, "cha_lifetime_trait", Passing);

// `Formatter::write_fmt`'s writer dispatch, reached directly from
// `to_string()` (a String) and, through `core::fmt::write`'s fn-pointer call
// to `Display::fmt`, with a custom `fmt::Write` type. FSA can miss the
// second writer, so the single-target site must get a guarded rewrite
// (pointers, with the dyn call as fallback): the traced byte/call counts
// are only right if the Counter's own write_str runs.
example_test!(fmt_fnptr_writer, "fmt_fnptr_writer", Passing);

// `core::fmt`'s argument-passing shape without std: a table of transmuted
// fn pointers walked with `NonNull::add`, each called with a wrapper around
// a `&mut dyn Sink`. FSA at `Fmt::emit` must see `Counter` (reached only
// through the fn pointers) as well as `Text` (reached directly).
example_test!(fnptr_args_table, "fnptr_args_table", Passing);

// Values captured by closures: `emit`'s dispatch is reached with a `B` only
// through a generic closure's capture, and with a `C` only through a closure
// called as `dyn FnMut`. FSA at `emit` must list A, B and C.
example_test!(closure_captures, "closure_captures", Passing);

// Memo reuse under a widened key: after 50 distinct argument sets, `take`'s
// memo keys drop ADT fields, so a `Holder` with a `C` must not reuse the
// result cached for a `Holder` with a `B`. FSA at `use_last` must list C.
// Red until the widened-key memo is fixed (plan B in CLAUDE.md §6): today
// the C call hits B's entry and FSA there is {A, B}. The JSON golden is
// hand-written (the correct FSA); re-bless the MIR dump once it passes.
example_test!(memo_widened_reuse, "memo_widened_reuse", Passing);

// Writes through `&mut` arguments of a memoized call: `set` is called on two
// slots holding equal values, so the second call is a memo hit - it must
// still store its A in `y`. FSA at `use_slot` (on `y`) must list A.
// Red until memo hits replay their side effects: today the hit only returns
// the cached value and FSA there is {B}. The JSON golden is hand-written
// ({A, B}: the same weak update `x` gets); re-bless the MIR dump once it
// passes.
example_test!(memo_side_effects, "memo_side_effects", Passing);

// Trait objects in a widened value's fields: `take`'s parameter collects more
// than 50 `Holder`s differing in two fields, so it's widened (fields dropped)
// and the last one's `B` must still reach `use_last`. Red until a dropped
// field reads as "any value of its type" (plan A in CLAUDE.md §6): today
// FSA there lacks B. The JSON golden is hand-written ({A, B}; a fix that
// reads the field as any `Box<dyn Sink>` also adds D, which is fine);
// re-bless the MIR dump once it passes.
example_test!(merge_widened_field, "merge_widened_field", Passing);
