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

- [ ] function summaries
    - requires something like a program dependence graph (PDG); something to
      track not only (1) the general relationships between inputs and outputs, 
      but also (2) the precise relationship between each input/output value
      (i.e. fn foo(a, b) -> int { a + b } ==> the fact that the return value is 
      exactly a + b should be captured.)
      - note this is unlike traditional dependency analysis, which (to the best
        of my knowledge) just tracks if variables are dependent on one another,
        but not precisely _how_
    - go backwards from retval?
        - similar insight to a Brown paper i think (range analysis)

- [ ] impl traits

- [ ] conditional scopes vs SSA
    - separate declaration from assignment -> assigning a value in a conditional 

- [ ] ask amit/leon about rust code that heavily uses dyn

- [ ] fill in todo!s (cases not yet impl in code)

- [ ] impl loops

