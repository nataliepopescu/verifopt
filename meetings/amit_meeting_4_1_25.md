# amit meeting

unwraps()
- option fields

verif fuzzing loop: solves "hard to know what to verify" problem
- hard part: generating plausible invariants
- assume "hard to prove" invariants are proven, if optimize code well, then tell programmer about them
- check if useful first, then put in effort to verify

code size
- vtables (code = general but uses = specific)
- compile whole vtable if dynamic dispatch
- prove only certain subset of functions are called

DARPA tractor
- C-to-Rust thing?
    - convert C into idiomatic Rust automatically
        - https://homes.cs.washington.edu/~djg/papers/exists_imp.pdf : linear types in C-like language
    - LLMs
    - take C code and guess high-level props of the code (via LLMs)
        - can we also do this via PL/heuristics?
    - translate into Rust (via LLMs vs PL)
    - also automatically generate equivalence* tests (except when C code is buggy/unsafe)
- efficient hacky data structures
- more of an equivalence thing than a verification thing

- other languages have a generic "Map" type w many diff implementations
- write properties about your thing, try plugging in diff implementations and see with which the properties hold

alex aiken line of work
- generating more optimal code + testing equivalence
- some randomness involved?
- also similar to verification fuzzing loop

function that returns result (possibly lower hanging fruit)
- checks a bunch of things in input
- verify inputs never have certain values (null, none, etc)
- replace if statements with preconditions
- propagate across libs too?

range analysis
- maybe allowing user input will allow a verifier (/compiler) to extract certain info more easily

toy examples
- i.e. assume_true macro
- code that, if verified, will make compilation faster
- perhaps start with this

next steps
- try to verify simple things!
