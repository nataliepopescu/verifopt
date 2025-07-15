# meeting

## notes

serde
- functions that use dyn traits
- search github for specific functions
- can search github for projs that depend on serde (/other crates)

"what conditions need to be true about the real world in order to keep going
with the project?"

library author vs app dev (style/design choices/etc)

where are the dyn traits appearing?
- most interesting:
    - code that uses dyn traits, but no impl of trait within that trait (expect
      proj to do the impl)
    - the pattern would normally be generics
        - motivation: generics == too hairy
    - iterables
        - very complicated library (definition)
        - but used very simply
- good:
    - bare func w dyn param
- bad:
    - collection data structure?
- worst:
    - set w generic "choose" func (dynamically) from that set


func summaries
- just as good: each input == "idk" type


analysis vs interp
- pass: context w accumulating info
- also something that runs the code

- analysis:
    - positions w ambiguity
    - more time available
- exec:
    - actual values instead of ambiguity

tester
- want to make sure analyzer is not lying
- build a realistic "running" interpreter that models runtime exec
    - generates random values for every possible ambiguity in analysis
    - don't want to exec more possibilities than analysis takes care of
    - also can have diff probabilities of the actual uncertainty being triggered
        - relevant for forgetting/widening

step after analysis: print out as well-formatted rust + then compile w rust
compiler
- i.e. transpiler
- use this for the exec part
- when hit an "idk", use a function that adds more certainty (maybe calls rand)

cha vs flow-sens (traits)
- think about iterator stuff (dyn trait -> support all possible collections vs
  just the one called with)

func ptr vs dyn trait
- func ptr
    - the value we are switching on _is_ the func ptr value
- dyn trait
    - fat ptr: one field is a struct ptr, other is vtable?
        - struct may contain funcptrs?
    - statically know vtable index for whatever function
        - ideally will be constant propagated
    - the value we are switching on _is_ either
        - the vtable ptr (one less deref)
        - OR, the (followed) func ptrs

