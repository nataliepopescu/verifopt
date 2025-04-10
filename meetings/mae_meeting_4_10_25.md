# mae meeting

## agenda

- [ ] akash feedback
    - good feedback about interesting motivation: source-to-source optimizations
      can't propagate proven/gained information up the compiler stack, but maybe
      verifopt could

- [ ] relwork
    - [ ] [From Verification to
      Optimization](https://kedar-namjoshi.github.io/papers/Gjomemo-Namjoshi-Phung-Venkatakrishnan-Zuck-VMCAI-2015.pdf)
        - pass assertions from Frama-C source code analysis into LLVM (i think
          by injecting + transforming these assertions?)
        - TODO does it work? encouraging?
    - [ ] [Stochastic Program
      Optimization](https://theory.stanford.edu/~aiken/publications/papers/cacm16.pdf)
        - select various rewrites to perform based on some complicated
          probability distributions
    - [ ] egraphs
        - technique for optimizing code in which rewrites are iteratively
          applied / internal state is memorized to find the "best" opt

- [ ] verif efforts
    - what / how to go next?

## notes

recap fuzzing loop direction
- caroline lemeiux

remove indirect jumps
flatten loops
inline

higher-level: replace data structures
- could require verification

maximum efficiency?
- what can you take advantage of when code becomes straightline

speed of light (base efficiency): everything is inlined

specification inference is a thing
- use for a project that does "do a better job of representing this data"
- only infers specifications that are correct
    - syntax-guided synthesis space
- distinct from SMT-based tools

dead code elimination vs data structure-sub
- former: just invariants that this branch isn't ever triggered
- latter: find data structure boundaries, find specs, then do synthesis for
  generating new data structure(s) (potentially LLM involvement)

wasm verification is not happening yet

linking in static analysis
- DCE: early return
- otherwise, inject asserts into source code if verified certain things (and
  then remove the assertion of the thing you verified (i.e. the thing you know
  is false/true/whatever))
  - verify something => safe to assume



## next steps

how often are data structures misused in the wild?

look into results / methods of From Verif to Opt paper

LLVM assumes?

try manual (up call tree) propagation of requires

- MLFQ w verus
- find things that actually are faster w verif

MLFQ: replace panic w assume(3)?

make priority list of project directions in case we reach dead ends

wasm verif paper?
- confirm tool
- how fits into absence of wasm verif?

