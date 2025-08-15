# meeting

## agenda

- things verifopt needs to operate
    - all types (types resolved at THIR and below)
    - vtable ptrs? (constructed in MIR _i think_)
        - potentially want to switch on these
        - currently switching on type IDs (using vtable ptrs would be easier if
          they are already generated, because otherwise we'd have to generate
          the infrastructure for type IDs)

- where to operate:
    - @ source-level
        - pros
            - already know the CFG/representation we're working with
        - cons
            - types are not all resolved
                - could use `rust-analyzer`-type tool to get all the types in source
                  code
                - don't know how much work it'll take to get this working
                - `rust-analyzer` is generally used for IDEs, and thus the implicit
                  types are meant to only appear upon "hovering" (`rust-analyzer`
                  does not modify source code, which is what I think we'd want)
    - @ THIR
        - pros
            - all types are resolved
        - cons
            - vtables not constructed
            - new CFG/representation
            - conflicting data on this, but structs/traits may not be
              represented here... dealbreaker if so
                - rustdocs say no structs/traits repr:
                  https://rustc-dev-guide.rust-lang.org/thir.html
                - could check yourself (`rustc_middle/src/thir.rs`)
    - @ MIR
        - pros
            - all types are resolved
            - vtables constructed
            - structs + traits are represented (supposedly)
            - how to write a pass: https://rustc-dev-guide.rust-lang.org/mir/passes.html
        - cons
            - new CFG/representation
                - MIR for single function:
                  https://github.com/rust-lang/rust/blob/f7ec6873ccfbf7dcdbd1908c0857c866b3e7087a/src/librustc/mir/repr.rs
                  - this file is 9 years old... where is the equivalent now?
                - what about things in the global scope of a file?
                  (structs/traits)
                  - still haven't find...
            - a superset of the rust language
                - how would this affect us? is any information lost here?
                    - generally more restrictive lang is easier to analyze
                - *could* matter (may just have to discover live)
                    - analysis benefits from knowing theres only way to get to a
                      particular place
                    - unrestricted control flow complicates this
                    - also where 2-level anlysis could help: maybe source-level
                      of thir-level analysis can help reestablish some constraints
                        - source-to-mir line correspondence tools?
                - keep an eye on this
            - not supposed to be parseable (at least the textual representation,
              so likely won't be able to dump + operate)

## notes

introducing typeid stuff is quite a lot of code

also related to type sizes
- larger size may actually prevent some optimizations that causes a big perf
  difference

potential next tasks:
- familiarize w writing mir pass
    - hardcode exact transformations?
        - try to identify places where would need to access vtable ptr

    - vs port analyses
        - automate identification of above places

    - will likely handle weird constructs, can "skip" (replace w IDK) at least
      for now
      - maybe not that useful
      - maybe too complicated

      - right after it, just to idk types for everything (can't be sure what it
        does or does not touch)
        - so can impl the stuff we know how to impl first
        - fallback

    - can create a "complete" solution (with holes)
        - then can evaluate what is needed to push through

- can even email authors if see papers that use mir?

cool thought
- if not mir, can hook in at multiple places
    - say if structs/traits are erased
    - but still can try: what does a dynamic invocation turn into (equivalent 
      thing that we can use instead)?
- say:
    - analysis at source
    - transformation at llvm (llvm line to source line)


