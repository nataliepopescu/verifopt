# TODOs

## Implementation

- [x] collect all funcs of certain signature

- [x] impl negative constraints

- [x] check for/address intersections (at every merge step, i.e. branch unification)
    - [ ] write test(s) for this (existing tests don't break)

- [x] clarify/fix function scope semantics (nested funcs vs lambdas)

- [ ] simplify implementation where possible
    - [ ] add more small/helper functions to make code more readable
    - [x] panics -> errors

- [ ] think of test cases that show difference between
    - separating learned vs done
    - combining learned vs done

- [ ] CHA
    - [x] code transformation: put types on all vars
        - during interp
    - [ ] in rewrite step, use function signatures to resolve list (do this
      everywhere, for now)

- [ ] conditional scopes vs SSA
    - separate declaration from assignment -> assigning a value in a conditional 

- [ ] impl traits

- [ ] impl flow-sensitive function resolution
    - everywhere where previously using CHA, if have useful constraints (not
      top/bottom of lattice) can use those constraints to further limit the
      possible funcs

- [ ] summaries
    - [ ] functions
    - [ ] bool conditions?

- [ ] ask amit/leon about rust code that heavily uses dyn

## Motivation

- [ ] "why now"?
    - what about rust makes this problem tractable and/or bigger?
        - how are ppl writing code re dynamic trait objects?

