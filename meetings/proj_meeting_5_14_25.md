# meeting

## agenda

- impl updates
    - non-top-level funcs
    - traits?

- high-level

##

impl traits in toy language?
- subtle to impl this the first time

don't pre-optimizse (worry about the performance _just yet_)

reiterate
- why now?

- vtables are used differently in rust?
- verification/static analysis techniques are better/diff?

- compiler devs probably don't want to add this bc perf
- but maybe better fit for externel tool

- rust is special (more guarantees @ compiler IR)
    - at least safe subset

additional context
- frontend
    - transform to IR
- backend
    - mostly optimizations
    - static guarantees don't really exist here?
        - typed IRs are starting to appear, but this is rare/not fully adopted

- rust is different: the simple analysis is easier?
    - new class of optimizations: ones that require/use frontend knowledge

- we are well-equipped (as a group) to identify "the wins that matter"?

"template-expansion?"

vs C++
- C is a bit saner
- but really know very little at IR (about vtable)
- harder to deal w in compiler
- also lose type info
- would be REALLY hard to do this kind of opt otherwise

MLIR (LLVM replacement)

bit of a movement rn / might be a race

pitch/motivation
- be wary of "we just happened to be the first people to think of this thing"
- what is the specific combination of things (circumstances, technology,
  Rust-uniqueness, etc) that enables our solution

