# VerifOpt Project

## Goal(s)

1. eliminate virtual function calls in certain scenarios
- consequences:
    - faster perf + potentially smaller code size (exposes code to more optimizations)
    - also could improve other static analysis tool accuracy if they skip 
    dynamically dispatched function calls for simplicity (MIRChecker)

2. explore traits vs generics tradeoff space?
- maybe a followup proj

## Why now / Rust / etc?

Rust
- codegen units (parallel compilation) only applies to LLVM?

now
- something about single-unit compilation being the rising technique, thus
  whole-program optimization will be easier
    - is this just for Rust or is it for languages in general?

during analysis
- Rust handles a lot of the checks one might have to do during analysis already
    - in particular: type checking/safety
    - also mutability/borrow checking
- as a result the analysis can be cheaper by trusting that Rust has done these
  checks correctly (as opposed to doing them itself)
    - e.g. the analysis would benefit from type checking if we're merging the
      states for one particular value after a conditional branch. if no type
      checking happened earlier, analysis would have to check that the new
      resulting values are of the same/compatible types. but because Rust gives
      us this guarantee earlier, analysis can merge states without checking 
      this property
- lots of information that our analysis can just know/rely on

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

## End-to-End Tool

user interface: should somehow collect all deps together (cargo)
- as in [MirChecker](https://dl.acm.org/doi/pdf/10.1145/3460120.3484541)

analysis phase

rewrite phase

final binary/source code (depending where rewrite happens)

## Considerations

by design
- correctness?
    - this will be pretty important since we're likely operating after many of
      Rust's safety checks are already executed, so there is a risk of
      subverting those safety properties
    - but the "unsafety" is also necessary b/c we are doing something that the
      compiler hasn't been able to reason about yet
- "**MIR makes all types explicit.** Since we are constructing MIR after the main
  type-checking is done, MIR can include full type information. This is useful
  for analyses like the borrow checker, which require the types of local
  variables and so forth to operate, but also means we can run the type-checker
  periodically as a kind of sanity check to ensure that the MIR is well-formed."
  - from https://blog.rust-lang.org/2016/04/19/MIR/
  - so, could potentially re-run type-checker after our pass 
- however, because MIR is a superset of Rust, modifying the MIR can introduce 
  unwanted behavior
- could also run [miri](https://github.com/rust-lang/miri) after our pass

empirical
- compilation speed
- compile-time memory usage
- compiler compilation speed (if tool is in-compiler)
- runtime speedup of `dyn` uses
- (maybe) runtime match of crates/projs that heavily use generics to avoid
  `dyn`, when rewritten to just use `dyn` (more ergonomic for devs)
- compiled binary size

