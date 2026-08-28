# PROJECT MILESTONES

## Things Left To Do

- [ ] impl / design
    - [ ] FSA interp (DRIVEN BY EVAL)
        - [x] hierarchical field support
        - [x] exact function memoization
        - [x] generalize function summaries
        - [ ] inline asm

        - [ ] widening vs stubbing
        - [ ] loops
        - [ ] fn ptrs to nested fn decls
        - [ ] ~fn sig narrowing (scope?)~

    - [ ] baselines
        - [x] naive Rust
            - already implemented (ofc), but still need machinery to check against
            - [x] votrace!
        - [x] CHA
            - fixed one bug already
            - [ ] correct?
        - [ ] Rupta
        - [ ] RTA
            - either skip or do _after_ Rupta

    - [ ] general rewrite
        - [x] tag-based rewrite (doesn't use vtables)
        - [ ] vtable ptr mod?

    - [ ] FSA interp nits/improvements (ONLY IF TIME/NEED)
        - [ ] make call_stack and friends use interior mutability
        - [x] unique-vec data structure/api


- [ ] eval
    - [x] perf bottlenecks / tool benchmarks (verifopt performance)

    - [ ] how to validate FSA
        - run binary tests
        - sample dynamic dispatches

    - [x] hook verifopt into non-main entry points (for benchmarks)

    - [ ] microbenchmarks
        - [ ] look into fallback rewrite - why performs worse than initial dyn call?
        - [ ] whats the expected speedup amount for different cases?

    - [ ] complete runs on _at least_ 10 full binaries
        - [x] ripgrep
        - [ ] dynolog cli
        - [ ] quixote
        - [ ] zellij
        - [ ] tock
        - [ ] ?
        - [ ] ?
        - [ ] ?
        - [ ] ?
        - [ ] ?

    - [ ] macrobenchmarks

- [ ] paper writing

## Tentative Milestones

as of aug 27
- [x] run verifopt on full ripgrep
    - verifopt runs on ripgrep runs in just over an hour
- [ ] rewrite effectiveness
    - perf
    - code size
- [ ] run on 4 examples total
    - starting to run on other examples
    - need to impl inline asm
- [ ] impl RTA + Rupta baselines
- [ ] write

## Deadlines

ASPLOS - Sept 9

CGO - Sept 11

OOPSLA - Oct 10

CC - Nov 11
