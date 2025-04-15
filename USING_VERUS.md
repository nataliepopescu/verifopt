# Using Verus

[verus docs](https://verus-lang.github.io/verus/guide/overview.html)
- "preconditions, postconditions, assertions, and loop invariants" = GHOST code
- TODO is the `flux_assume` "ghost" code?

verifying `get_array_elems` just works, but `get_slice_elems` is more
complicated

[vstd](https://verus-lang.github.io/verus/verusdoc/vstd/)

when trying forall (`forall|n: nat| 0 <= n < slice.len() ==> n < 5,`) for 
`get_slice_elems` get:

```sh
error: Could not automatically infer triggers for this quantifer.  Use
#[trigger] annotations to manually mark trigger terms instead.
```

goal of forall in this context is to say something like "for all possible input
slices, their length will always be less than X"; perhaps this is not the
intended use of forall though
- looking into intended use + what a trigger is

[Verus Quantifiers](https://verus-lang.github.io/verus/guide/quants.html)
- example starts w more like _contents_ of a collection obey some invariant/fact

[Spec Libraries](https://verus-lang.github.io/verus/guide/spec_lib.html)
- Seq, Set, Map: collections of arbitrary size
    - `len()` returns `nat` (unbounded)
- Seq == finite, but Set / Map can be infinite

- seems like Seq is a good abstraction for a slice

- "Proofs about set cardinality (Set::len) and set finiteness (Set::finite)
  often require inductive proofs."
  - specific to sets?

- now i'm wondering _when_ we even need proofs?

[Modes](https://verus-lang.github.io/verus/guide/modes.html)
- spec: describes properties about program
- proof: proves program satisfies certain properties
- exec: rust code (compiles + runs)

- spec + proof = ghost code
    - "Verus erases all ghost code before compilation so that it imposes no
      run-time overhead."

- all can contain/call SPEC
- proof/exec can contain/call PROOF
- exec can contain/call EXEC

- more concrete can use more abstract, but not other way around

- SPEC
    - body is _optional_
        - if no body, [uninterpreted
          function](https://microsoft.github.io/z3guide/docs/logic/Uninterpreted-functions-and-constants/)
    - if body, _can_ be visible to other (modules?)
    - open / closed ~= public / private (implementation, not declaration)
    - pure functional code style
    - deterministic
        - stronger assumptions about determinism (i.e. no side effects, I/O, RNG)
    - pre-/post-conditions == `recommends`
        - no `requires`/`ensures` => keep similar to
          [Boogie](https://github.com/boogie-org/boogie) style
        - i.e. keep spec lang close to SMT solver's mathematical language ->
          better solving efficiency

- PROOF
    - use if have an abstract function definition only? (i.e. when spec ==
      closed?)
      - does this mean marking spec w open will remove the need for a proof?
    - exec code can contain proof blocks (`proof { ... }`)

    - may need PROOF to show that:
        - calls to function satisfy preconditions (`requires` clause)
            - may need a lil proof at each callsite for certain functions
        - function satisfies postconditions (`ensures` clause)

    - strengths / limitations of SMT solving
        - i.e. SMT cannot prove by induction, "but you can write a proof by
          induction simply by writing a recursive Rust function whose `ensures`
          clause expresses the induction hypothesis"

[Loops](https://verus-lang.github.io/verus/guide/while.html)
- "In fact, internally, Verus verifies the loop as if it were its own function,
  separate from the enclosing ... function."
- "This means that the loop does not automatically inherit preconditions ...
  from the surrounding function — if the loop relies on these preconditions,
  they must be listed explicitly in the loop invariants"
  => more efficient verification when larger loops
- can opt-out! via `#[verifier::loop_isolation(false)]`


i cannot figure out how to import a crate!!
- multi-crate support is limited
- https://verus-lang.zulipchat.com/#narrow/channel/399078-help/topic/.E2.9C.94.20Support.20for.20separate.20verification.20of.20multi-crate.20projects/near/467686900
    - might only be able to import verified things? but im confused by the
      following example from the verus docs...
      - https://verus-lang.github.io/verus/guide/calling-unverified-from-verified.html#applying-specifications-to-existing-library-functions

given these limitations, tock (and things like it) make even more sense to
verify at the moment because theyre very unlikely to use third-party libraries
and generally either depend on `core` or things they've written themselves

MLFQ simplification could still be verifiable, but may need to statically decide
an index which defeats the purpose
- unless you want to write/verify your own rand lib....












