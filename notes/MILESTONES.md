# PROJECT MILESTONES

## Things Left To Do

- [ ] impl / design
    - [ ] FSA interp (DRIVEN BY EVAL)
        - [ ] generalize function summaries
        - [ ] hierarchical field support
        - [ ] inline asm
        - [ ] fn sig narrowing (scope?)
        - [ ] fn ptrs to nested fn decls
        - [ ] loops

    - [ ] FSA interp nits/improvements (ONLY IF TIME/NEED)
        - [ ] make call_stack and friends use interior mutability
        - [ ] unique-vec data structure/api

    - [ ] baselines
        - [x] CHA
            - [ ] correct?
        - [ ] RTA
        - [ ] Rupta

    - [ ] general rewrite
        - [ ] vtable ptr mod


- [ ] eval
    - [ ] may require being able to hook verifopt into non-main entry points

    - [ ] microbenchmarks
        - [ ] look into fallback rewrite - why performs worse than initial dyn
          call?

    - [ ] complete runs on 10 full binaries
        - [ ] ripgrep
        - [ ] tock
        - [ ] ?
        - [ ] ?
        - [ ] ?
        - [ ] ?
        - [ ] ?
        - [ ] ?
        - [ ] ?
        - [ ] ?

    - [ ] macrobenchmarks

- [ ] paper writing

## Tentative Milestones

by aug 1
- run verifopt on full ripgrep
- OR
- run on 1 new example

- merge ayush changes

- make progress on general rewrite / vtable ptr changes

by aug 14
- run on 4 examples total
- write summary/synthesis
- general rewrite

by aug 21
- run on 7 examples total
- write summary/synthesis
- microbenchmarks

by sept 4
- impl RTA + Rupta baselines + test on existing examples
- write summary/synthesis

by sept 9
- fill in paper

## Deadlines

ASPLOS - Sept 9

CGO - Sept 11

OOPSLA - Oct 10

CC - Nov 11
