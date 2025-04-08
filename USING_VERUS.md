# Using Verus

[verus docs](https://verus-lang.github.io/verus/guide/overview.html)
- "preconditions, postconditions, assertions, and loop invariants" = GHOST code
- TODO is the `flux_assume` "ghost" code?

verifying `get_array_elems` just works, but `get_slice_elems` is more
complicated

general specificity/reality (from least to most): spec -> proof -> exec

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


















