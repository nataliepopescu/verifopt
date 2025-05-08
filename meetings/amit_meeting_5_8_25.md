# amit meeting

## agenda

- remind of the vtable efficiency pitch

- general stuff we've been working on so far / the future plan
    - writing a toy interpreter to get a feeling for control flow analysis
        - apparently "abstract interpretation" but haven't looked into that a ton
    - interp goal: track possible funcptr info statically such that can
      rewrite blind jumps into conditional/switch statements that let the
      compiler choose from a smaller/fixed number of jump targets (and thus
      better leverage the branch predictor, inlining, etc)
    - eventually do this in the Rust compiler

- foresee any difficulties about doing this in the Rust compiler?
    - presumably would do at MIR level, but not sure

## notes

no traits or anything in toy lang yet

lots of rust complexity that is unnecessary to model

worth looking into what rust complexities will actually impact the vtable stuff

everything the compiler does is static analysis / pruning branches
- diff == tracking
- but compilers typically avoid propagating global info
    - more local
    - at least until LTO
- does Rust compiler do optimization across crates?

- why not? "mega-slow"
    - are we doing anything about this?
    - options
        - more directed approach
        - hints
        - marker type

- dyn traits => meant to be stored as vtables
    - mess with developers intuition about how vtables work

think about: why is this not already happening?
- some new tool we can leverage?
- likely more about the performance
- semantic practicality?
- _can_ the compiler even do this on its own?

potentially an argument for embedded systems (saving memory)
    - don't run tool during dev cycle but yes for shipping

think about potential pitfalls
- are Rust vtables special in some way that enables us to do this?
    - how theyre specialized, or used, or something else?

return to relwork search

more code size or perf benefit?
- code size is a newly important thing... (embedded sys / security chips)
    - until recently, wouldn't even use C++ on these things -> no vtables on them

zotero
- reference management
- useful paper writing time
- typst.app has the ability to sync w zotero collection
- desktop client
    - can upload pdfs
    - tracking + taking notes?

