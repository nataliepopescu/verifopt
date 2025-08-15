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
            - structs + traits are represented
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
            - not supposed to be parseable (at least the textual representation,
              so likely won't be able to dump + operate)

## notes


