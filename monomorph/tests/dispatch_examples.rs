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
example_test!(casting_traitobj, "casting_traitobj", Passing);
example_test!(closures, "closures", Passing);
example_test!(default, "default", Passing);
example_test!(fnptrs, "fnptrs", Passing);
example_test!(generic, "generic", Passing);
example_test!(recursive, "recursive", Passing);
example_test!(shims, "shims", Passing);
example_test!(switchint, "switchint", Passing);
example_test!(one_variant, "one_variant", Passing);
example_test!(rand_, "rand_", Passing);
example_test!(two_variants, "two_variants", Passing);
example_test!(two_variants_rand, "two_variants_rand", Passing);
example_test!(two_variants_static, "two_variants_static", Passing);
example_test!(
    two_variants_static_nonzst,
    "two_variants_static_nonzst",
    Passing
);

// Not yet characterized at all — run and report, don't assert.
example_test!(no_vtable_check, "no_vtable_check", Unclassified);
example_test!(recursive_dyn, "recursive_dyn", Unclassified);
