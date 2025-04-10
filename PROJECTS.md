# Project Directions

## Options

1. Specification inference -> data structure substitution
    1a. Amit's C -> Rust grant proposal

2. User verifies program -> feed this to the compiler 
    - backwards propagate (up the call tree) @ requires
    2a. feed to compiler via assumes / asserts
    2b. feed to compiler in another way?
    2c. automatically generate properties to verify, that are then fed to the
    compiler
        - fuzzing?
        - synthesis?
        - LLM?
        - other?

3. Source verification + IR / low representation verification (upward invariant
propagation)
    3a. Wasm verification
        - no tool to verify Wasm code (maybe?) - TODO confirm
        - tools to verify LLVM are gross + LLVM is gross

4. IR / low rep. verification

5. How often do developers misuse data structures?

6. Eliminating "deader" code paths
    - panics in tock
    - bounds checks
    - null/nil/none inputs

7. Categorizing patterns of code optimizations
    - performance engineering
    - inlining, loop unrolling, direct jumps

8. Adding/making (more) efficient opimizing/analyzing paths to/in compilers
themselves
    - interval analysis?
    - vtable compression?
    - other?
    8a. Why isn't interval analysis being used?

9. Anti-patterns list (more of a result of one of these other projects)

10. Take highly-optimized code (unreadable + performant) and make it readable
and verifiable while maintaining performance
    - can use verification to verify both versions are equivalent
    - show more properties can be verified on new version
    - show new version has same performance

## Priority Queue

