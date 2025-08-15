# TODOs

## Motivation

- [ ] "why now"?
    - what about rust makes this problem tractable and/or bigger?
        - how are ppl writing code re dynamic trait objects?

## Implementation

- [x] collect all funcs of certain signature

- [x] impl negative constraints

- [x] check for/address intersections (at every merge step, i.e. branch unification)
    - [ ] write test(s) for this (existing tests don't break)
        - TODO not breaking b/c remove things that cause intersections beforehand

- [ ] think of test cases that show difference between (i.e. learning facts from
  conditions or function calls)
    - separating learned vs done
    - combining learned vs done
        - TODO as long as SSA, shouldn't be any different...

- [x] clarify/fix function scope semantics (nested funcs vs lambdas)

- [ ] simplify implementation where possible
    - [x] panics -> errors
    - [ ] assign types during interp? or separate pass?

- [x] CHA
    - [x] code transformation: put types on all vars
        - during interp
    - [x] in rewrite step, use function signatures to resolve list (do this
      everywhere, for now)
    - [ ] write specific test(s) for this
        - TODO how to simulate bottom constraints?

- [x] impl flow-sensitive function resolution
    - everywhere where previously using CHA, if have useful constraints (not
      top/bottom of lattice) can use those constraints to further limit the
      possible funcs

- [x] impl traits (structs that impl traits)
    - need to have some notion of `self`, or what type the current object is, in
      order to call a trait-impl method on the current object
    - either that or must pass in the `self` object as an argument, but then
      have to deal with argument polymorphism
    - [x] impl methods (vs functions) / dot notation
    - [x] impl using trait objects
    - [ ] CHA vs fewer-constraints

- [ ] more code org/cleanup
    - [ ] indirect func calls vs trait func calls (code path)
    - [ ] interpreter.rs is 5k LOC - how best to split up
        - mostly tests...

- [x] empirical confidence
    - [x] ask amit/leon about rust code that heavily uses dyn

- [ ] function summaries?

- [ ] reuse trait pruning logic from intepret step in rewrite step or store
  results somewhere

- [x] make FuncDef (stmt) variant contain FuncVal struct, and (new) FuncDecl (stmt) variant contain FuncDecl
  struct

- [ ] conditional scopes vs SSA
    - separate declaration from assignment -> assigning a value in a conditional 

- [ ] when is the negative part of constraints useful?
    - check, currently don't think it is
    - if that's the case, remove it

- [ ] how to simulate SSA assumption in Rust compiler?
    - MIR does _not_ use SSA (as of 2016...)
    - will actually need to modify prototype to more closely resemble what MIR
      does do (more like `alloca`s): https://github.com/rust-lang/rfcs/blob/master/text/1211-mir.md#alternatives

- [ ] put RewrittenSwitch + InvokeTraitFunc in different set of statements?
  (private)

