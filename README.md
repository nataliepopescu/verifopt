# Verification / Optimization Project

## Goals

use verification at source level 
- flux
- verus

propagate info to compiler / optimizations
- either just leverage existing machinery to trigger existing optimization passes
- or, if needed, (for certain source code constructs/styles of info needed) may need to extend somehow (new optimization type? pull new info into the compiler?)

will need to think about which optimizations happen where, and what info is available there (MIR vs LLVM IR)

one specific goal/application: panic-free tock kernel
- optimize out the panics for a smaller kernel binary

## Work in Progress

for panic-freeness, how to determine which panics are left in the binary vs optimized out? how to track/check this?

## Completed Work

no panics in tock compiled for qemu

compiled tock for imix board
- found panics

## Potential Related Work

### Using Static Analysis for Optimization

[From Verification to Optimizations](https://link.springer.com/chapter/10.1007/978-3-662-46081-8_17)
- link external static analysis tools into compilers
- propagate source-level info into optimization pipeline

### Optimize + Verify

[VSync: Push-Button Verification and Optimization for Synchronization Primitives on Weak Memory Models](https://dl.acm.org/doi/abs/10.1145/3445814.3446748)
- automatically optimize + verify? how?

### Verified Optimizers/Compilers

[Verified Tensor-Program Optimization Via High-Level Scheduling Rewrites](https://dl.acm.org/doi/pdf/10.1145/3498717)
- optimizer w verified rewrite rules (Coq proofs)
- "easy" extension (adding new rewrite rules)

[CompCert - A Formally Verified Optimizing Compiler](https://inria.hal.science/hal-01238879v1/document)

[Verified Software Toolchain](https://link.springer.com/chapter/10.1007/978-3-642-19718-5_1)

[Formal Verification of SSA-Based Optimizations for LLVM](https://dl.acm.org/doi/pdf/10.1145/2491956.2462164)

[Verified Peephole Optimizations for CompCert](https://dl.acm.org/doi/pdf/10.1145/2908080.2908109)

### Using Optimization Problems for Verification

unsure what is meant by "verification" in these contexts

[Thesis: Optimization-Based Methods for Nonlinear and Hybrid Systems Verification](https://thesis.library.caltech.edu/2155/1/thesis.pdf)

[Modeling, Optimization and Computation for Software Verification](https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=1edbc45f98bfa52b43f06e6bcb07b490f7bc127e)

[Optimixation of Lyapunov Invariants in Verification of Software Systems](https://ieeexplore.ieee.org/abstract/document/6416001)
- search for Lyapunov invariants == convex optimization problem

[Optimization-Based Verification and Stability Characterizaation of Piecewise Affine and Hybrid Systems](https://link.springer.com/chapter/10.1007/3-540-46430-1_8)
