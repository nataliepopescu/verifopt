# VerifOpt Project

## Goal(s)

1. eliminate virtual function calls in certain scenarios

2. explore traits vs generics tradeoff space?

## Why now / Rust / etc?

Rust
- codegen units (parallel compilation) only applies to LLVM?

now
- something about single-unit compilation being the rising technique, thus
  whole-program optimization will be easier
    - is this just for Rust or is it for languages in general?

## Technique(s)

- flow-sensitive program analysis/abstract interpretation

## Considerations

by design
- correctness

empirical
- compilation speed
- compile-time memory usage
- compiler compilation speed (if tool is in-compiler)
- runtime speedup
- compiled binary size
