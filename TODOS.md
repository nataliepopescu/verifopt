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
    - if that's the case, remove it (do eager resolution first)

- [ ] how to simulate SSA assumption in Rust compiler?
    - MIR does _not_ use SSA (as of 2016...)
    - will actually need to modify prototype to more closely resemble what MIR
      does do (more like `alloca`s): https://github.com/rust-lang/rfcs/blob/master/text/1211-mir.md#alternatives
    - some contradicting evidence as there seems to be an SSA pass in the
      MIR...?

- [ ] put RewrittenSwitch + InvokeTraitFunc in different set of statements?
  (private)

- [ ] MIR pass (initial steps)
    - [ ] print to see what structs/traits look like + vtable ptrs
    - [ ] simple (with hardcoded rewrite)
    - [ ] then try to automatically identify rewrite locations
    - [ ] port analysis -> quick/dirty "complete" solution

- [ ] rewrite/transformation pass
    - [x] what instructions do dynamic dispatches get compiled into?
        - just some not very interesting calls (from registers + static 
        offsets)
        - so we will need to be more high-level than that
    - [x] revisit how we're identifying dynamic dispatch, b/c running the pass
      on the handwritten rewrite still shows that there are two locations where
      we could perform the rewrite (the `into_raw` calls take in a trait object 
      as the first argument, but they are static calls, so need to refine our 
      method of identification)
    - [ ] what instruction(s)/statement(s) access the vtable (at MIR level)
        - [ ] some instructions will inevitably be terminators, so the
          transformation will need to happen across basic blocks / create new
          basic blocks (see `add_call_guard.rs`)
    - [ ] how to get the vtable ptrs of all possible types (without "faking"
      instances of those types, like the `transmute_cp` and `vtable_struct`
      examples do)
        - maybe start with the fakes, and then do this
        - in particular, print out your own version of the fakes (rather than
          the emitted pretty code) to get an exact answer of what those types
          are + what values they contain
    - [ ] how to modify/add basic blocks to include the above
      instructions/statements

- sus implementation things to confirm
    - `mk_binder_list` which relies on using `rustc_type_ir` (need to remove
      attribute allowing this) - alternative is a slightly different type that
      might make some things more difficult? but should probably try it (using
      `rustc_middle::ty` instead, which omits the TyCtxt stuff, but this creates
      a type mismatch...)





