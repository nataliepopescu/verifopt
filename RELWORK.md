# Potentially Related Work

## Rust Verification Tools

[Flux](https://dl.acm.org/doi/pdf/10.1145/3591283)
- refinement types / liquid inference ?
    - TODO: difference between refinement / dependent / liquid types
    - "synthesizing refinements via liquid type inference"
- type checker
- extends rust types
- quantifier-free (easier to reason about)
    - "using a program logic comes at the cost of complex user-specified universally quantified invariants"
- also uses stacked borrows rules apparently

- declarative type system == plug in to Rust compiler
    - spatial phase: use fxn sigs, program ids -> heap loc (map)
        - location refinements?
        - intermediate refinements still unknown
    - checking phase: refinement type checking (Horn vars for unknown refinements) -> system of Horn constraints
    - inference phase: solve the constraints

- 2 A TOUR OF FLUX

- 4 ALGORITHMIC VERIFICATION
    - flux == compiler plugin; operates on programs that have _already_ been analyzed by the compiler
        - benefits: IR contains inferred type info + assume Rust borrowing rules are satisfied
    - at MIR level; CFG
    - therefore flux doesn't technically pass info to the compiler, but rather the other way around (leverages compiler info)

[Verus](https://www.andrew.cmu.edu/user/bparno/papers/verus-sys.pdf)

- check out ghost types/values

RustBelt
- proving unsafe impls encapsulated in well-typed interface

Oxide

heavyweight verification
- Prusti
    - encodes programs into Viper?
- RustHorn
- Creusot

What is "bounded" verification? 

## Using Static Analysis for Optimization

[From Verification to Optimizations](https://link.springer.com/chapter/10.1007/978-3-662-46081-8_17)
- link external static analysis tools into compilers
- propagate source-level info into optimization pipeline

## Optimize + Verify

[VSync: Push-Button Verification and Optimization for Synchronization Primitives on Weak Memory Models](https://dl.acm.org/doi/abs/10.1145/3445814.3446748)
- automatically optimize + verify? how?

## Verified Optimizers/Compilers

[Verified Tensor-Program Optimization Via High-Level Scheduling Rewrites](https://dl.acm.org/doi/pdf/10.1145/3498717)
- optimizer w verified rewrite rules (Coq proofs)
- "easy" extension (adding new rewrite rules)

[CompCert - A Formally Verified Optimizing Compiler](https://inria.hal.science/hal-01238879v1/document)

[Verified Software Toolchain](https://www.cs.princeton.edu/~appel/papers/vst.pdf)

[Formal Verification of SSA-Based Optimizations for LLVM](https://dl.acm.org/doi/pdf/10.1145/2491956.2462164)

[Verified Peephole Optimizations for CompCert](https://dl.acm.org/doi/pdf/10.1145/2908080.2908109)

## Using Optimization Problems for Verification

unsure what is meant by "verification" in these contexts

[Thesis: Optimization-Based Methods for Nonlinear and Hybrid Systems Verification](https://thesis.library.caltech.edu/2155/1/thesis.pdf)

[Modeling, Optimization and Computation for Software Verification](https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=1edbc45f98bfa52b43f06e6bcb07b490f7bc127e)

[Optimixation of Lyapunov Invariants in Verification of Software Systems](https://ieeexplore.ieee.org/abstract/document/6416001)
- search for Lyapunov invariants == convex optimization problem

[Optimization-Based Verification and Stability Characterizaation of Piecewise Affine and Hybrid Systems](https://link.springer.com/chapter/10.1007/3-540-46430-1_8)
