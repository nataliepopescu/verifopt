# TODOs

- [x] collect all funcs of certain signature

- [x] impl negative constraints

- [x] check for/address intersections (at every merge step, i.e. branch unification)

- [ ] simplify implementation where possible
    - add more small/helper functions to make code more readable

- [ ] CHA
    - code transformation: put types on all vars
    - in rewrite step, use function signatures to resolve list (do this
      everywhere, for now)

- [ ] clarify function scope semantics (nested funcs vs lambdas)

- [ ] conditional scopes vs SSA

- [ ] traits

- [ ] impl flow-sensitive function resolution
    - everywhere where previously using CHA, if have useful constraints (not
      top/bottom of lattice) can use those constraints to further limit the
      possible funcs

- [ ] summaries
    - [ ] functions
    - [ ] bool conditions?

- [ ] ask amit/leon about rust code that heavily uses dyn
