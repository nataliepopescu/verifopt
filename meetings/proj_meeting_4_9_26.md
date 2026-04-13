# meeting

## agenda

- ~intro / hypotheses
    - mostly go over hypotheses - not sure if the level of detail is enough

- vortex paper
    - data showing that dynamic dispatches, in many benchmarks + across 4
      languages (C++, Cecil, Java, Modula-3), are dominated by a small number of
      "receiver classes"/applicable methods (but not necessarily just a single
      one)

- will helped w adding dependencies to verifopt

## notes

combine two pitches
- lots of ppl who _should_ be using dyn dispatch
- and when they do, there aren't many variants to choose from

- would require a running example

hyp 1
- core
- replacing dyn (ptr indirection + vtable) w switch == good for perf/code size
- one baseline (other is SOTA)

hyp 2
- maybe core?
- given the switch statement, reducing the size of the switch stmt (via
  flow-sensitive), gives you more perf/code size improvements

TODO
- need to isolate flow-sensitivity somehow
- add crisp research questions for eval

emily
- re-entrancy bugs
    - calling something that you are already in the stack frame of
    - not uniformly a bad thing, but can be bad if you claim a resource
- emily is doing this in a place where there is actual dyn dispatch in tock
- but then also leon's thing



