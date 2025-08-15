# VerifOpt Project

## Goal(s)

1. eliminate virtual function calls in certain scenarios

2. explore traits vs generics tradeoff space?
- maybe a followup proj

## Why now / Rust / etc?

Rust
- codegen units (parallel compilation) only applies to LLVM?

now
- something about single-unit compilation being the rising technique, thus
  whole-program optimization will be easier
    - is this just for Rust or is it for languages in general?

## Technique(s)

- flow-sensitive program analysis/abstract interpretation
- (To Be Confirmed) @ MIR level in the rust compiler b/c
    - need types to be fully resolved
    - (maybe) need vtable access (if not, then THIR would be sufficient)
- want to stay as high-level as possible b/c simpler to work with

location
- in-compiler?
- source-to-source?
    - con: (dumped) MIR syntax is not supposed to be parseable? 
        - https://users.rust-lang.org/t/rust-ast-with-types/24289

## Considerations

by design
- correctness?
    - this will be pretty important since we're likely operating after many of
      Rust's safety checks are already executed, so there is a risk of
      subverting those safety properties
    - but the "unsafety" is also necessary b/c we are doing something that the
      compiler hasn't been able to reason about yet

empirical
- compilation speed
- compile-time memory usage
- compiler compilation speed (if tool is in-compiler)
- runtime speedup of `dyn` uses
- (maybe) runtime match of crates/projs that heavily use generics to avoid
  `dyn`, when rewritten to just use `dyn` (more ergonomic for devs)
- compiled binary size

