# Verification / Optimization Project

## Goals

use verification at source level (Rust)
- flux [flux blog](https://flux-rs.github.io/flux/blog/01-introducing-flux.html)
- verus

propagate info to compiler / optimizations
- either just leverage existing machinery to trigger existing optimization passes
- or, if needed, (for certain source code constructs/styles of info needed) may need to extend somehow (new optimization type? pull new info into the compiler?)

goal: lightweight verif
- why? b/c pinpointing very specific optimizations (bigger verification machinery is likely overkill)

will need to think about which optimizations happen where, and what info is available there (MIR vs LLVM IR)
- also where does verification happen with respect to where optimizations happen

possible applications:

- [ ] panic-free tock kernel
    - optimize out the panics for a smaller kernel binary
- [ ] safely elide bounds checks
- [ ] remove redundant reference counting (mae's paper)
- [ ] checking for nil/null for passed in parameters, etc. (can be removed if we have invariants about length)
- [ ] impl stacked/tree borrows in compiler + use for opt?

verification vs static analysis?

how should the project look at the end?
- a tool
- we took a system and optimized it via verification
- generated antipatterns
- ?

also, why are existing verified systems often not optimized/perform worse than
unverified systems?
- perhaps it is easier to verify systems that are written more simply, and
  harder to verify systems that are performance engineered (faster but more 
  complicated)?
    - a compiler that communicates its optimization conditions may be able to 
    help ease verification for perf engineered code (i.e. facts XYZ need to be 
    clear, but other stuff doesn't necessarily have to be)

## Thoughts + Potential Project Directions

### panics in tock (likely single-system paper)

mae idea: verify wasm
- verif/compiler pipeline: verif -> monomorph -> optimizations
- may get more helpful info for determining dead code after monomorph
- i.e. at either the LLVM or assembly level 
- Wasm = "massively restricted walled garden" (source: mae), so, easier
  verification target than LLVM / native code
  - how exactly is wasm a "massively restricted walled garden"?

- @ wasm level the optimizer(s) have already done a lot: monomorph, inlining, etc
- thus local-only tool may still be helpful

are there any automated verification tools for wasm?
- most seem to be built on Coq/Iris (interactive, not automated)

also, how to compile rust to webassembly?
- [Rust to
  Wasm](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm)

### verification "fuzzing" loop

verification-driven optimization search

compiler tells dev what it was able to optimize / "optimization conditions"
- devs can check that those "optimization conditions" are not invalidated during
  code maintenance/updates/etc

### generate verification conditions to verify in order to optimize code

how hard is it to automatically generate meaningful verification conditions?
- quite hard without any developer help

### antipatterns list

certain data structure substitutions that result in better perf
- to be applied at compiler level (automatically) or source level (developer
  mods)?

### iterative simplification of perf-engineered code s.t. it can be verified

sort of the opposite of optimizing code, but for verification purposes
- in a sense, de-optimizing for verification

i.e. this crazy-complicated implementation is equivalent to this longer, slower,
simpler one, which we managed to verify; therefore the high-perf version is also
verified

very specific example would be useful
- i.e. this would likely _not_ work: https://stackoverflow.com/questions/1349542/john-carmacks-unusual-fast-inverse-square-root-quake-iii
    - human cannot understand why this works
- maybe common numeric patterns?

what do verifiers do to make it easier to verify code? (two-pass approach)

### non-verification techniques? (other static analysis / intuition hints)

### potential similarities/shared goals in all of the above

pull in certain dynamic info / "user intentions" into the compiler

whatever is optimized, will need to verify the transformation preserves
semantics

maybe look into what kinds of things performance engineering successfully 
accomplishes

## Work in Progress

just _try_ flux or verus to see if the tools are capable of proving X
- sense of annoyance
- automatable? 
- user input?

First goal: look at examples of what info optimizers/compilers extract from unoptimized code in their passes.
- for panic-freeness, how to determine which panics are left in the binary vs optimized out by just the compiler on unverified code? how to track/check this? how many are left in? what's the difference between those left in vs left out?
  - for reproducability: 
  - mlfq - a queue of 3 levels of priority, each is queue of processes. current panic is in getTimeslice making sure we never return a timeslice for any priority level that isn't 1,2,3. panic eliding would be based on verifying index and array length. Can current compiler elide this panic?

Second goal(?): look at examples of what info is left to be extracted from verified code.

Third goal: understand how new info can be propogated to compiler to aid in its pass/decision making.
- ex) flux on ring buffer
- ex) ring buffer is always length greater than 1

## Look into next

### Questions:

- is flux just a verifier or both a verifier and a compiler? if just a verifier, what compiles rust code + flux annotations?
refinement == extension of type checker by adding logic specifying correctness contstraints. these add ons can be verified by the compiler --> eliminate classes of run-time problems.

flux is a refinement type checker for rust.

a plug-in to the Rust compiler

In this paper, we introduce Flux, which shows how refinements can work hand in glove with
ownership mechanisms to yield ergonomic type-based verification for imperative (safe) Rust.

the advantages of Flux’s refinement type-based verification over program logic based approaches
- Prusti’s program logic can, in general, verify
deep functional correctness specifications beyond the scope of Flux
- for lightweight verification, Flux’s refined types naturally capture invariants and heap update specifications that
must otherwise be spelled out via complex (quantified) program logic assertions.
- liquid typing for speedup

ex) normally when we write tests using assert(), we have to run the code to know if assert fails.
with flux, you can add a precondition to the implementation of assert that states the input passed in is true.
This way, the tests that pass in arguments == false (aka, cases that assert would fail on at runtime) won't type check! we can statically check our test cases.

----> so flux can allow the compiler to verify certain properties of code. and compilers can also optimize code. .... sooo we want verified code to be optimized... hmmm



- is verification just of properties user-specified via annotations? or can properties be generated/synthesized to be verified about code? "trivial/meaningless" properties to verify about code that isn't specified by user.


Flux works on compiler analyzed Rust MIR.
Rust --> MIR (flux) --> LLVM --> binary
https://llvm.org/docs/Passes.html
https://rustc-dev-guide.rust-lang.org/mir/index.html

## Completed Work

no panics in tock compiled for qemu

compiled tock for imix board
- found panics
- difficult to trace assembly panics back to particular lines of source code

## Specs / Reproducability

rust version: nightly-2024-11-16

