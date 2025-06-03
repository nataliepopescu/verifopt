# meeting notes

## agenda

- [x] zotero (invites)

- [x] small relwork discussion

- [ ] traits impl in toy interpreter

## notes

rust benefits
- meaningfulness of static info?
- hard part: what do you do when theres uncertainty about what something is?
    - state blowup, P-sspace complete, trivially exponential
    - compilers have a time budget

    - widening: [3, 4, 5] -> [>3]

- in relwork
    - may see analysis that looses precision quickly
        - just use funcptrs when ambiguity gets to high
        - or some things only work under restrictions -- bounded loops, etc
    - look at relwork + see when they have to fall back / don't have enough
      precision

    - bringing functional solutions to the imperative world

    - does Rust or Mae give us enough info to resolve here

- deoptimizing / hybrid between generics + trait objs
    - partially monomorphized - relwork? C++ or Rust?
        - specializing args
        - which vtable entry to call

        - ^ similar technique

next steps
- deeper understanding of relwork limitations
- more impl

motivations
- code size
- perf
- opt for opts sake
    - don't necessarily need to explore the full tradeoff space we've id'd

initial step
- monomorph: larger (switch statements)
- dynamic obj: vtables smaller

