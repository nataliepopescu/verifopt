# `rustc_driver` Hook

## Function Collection

trying to populate a table of all functions that may be reachable from some
entry point.

this includes those from non-local crate dependencies.

for the `examples/simple.rs::main` entry point, crate "deps" should only be the
local crate and std?

```sh
RUSTFLAGS="-Zalways-encode-mir=yes" cargo run examples/simple.rs -Zbuild-std
```

1 found function call
- get_animal

2 not found
- std::boxed::Box::<Cat>::new
- <dyn Animal as Animal>::speak

focusing on the Box::new() call first
- from std crate so should also add `-Zbuild-std`
  (https://doc.rust-lang.org/cargo/reference/unstable.html#build-std)
    - "Using -Z build-std will implicitly compile the stable crates core, std,
      alloc, and proc_macro"
    - "Note that for this to work you need to have the source code for the
      standard library available, and at this time the only supported method of
      doing so is to add the rust-src rust rustup component:"
    - seems generally difficult to combine rustc_driver w a custom rustc
      build...
- actually cannot get `-Zbuild-std` to work?
    - but at that call site tcx.is_mir_available() returns true! even without
      `-Zbuild-std`; maybe we can ignore this for now
    - so perhaps we can lazily populate/memoize this table?

note, our speak() call's DefId returns false when `is_mir_available()` is
queried
- TODO
