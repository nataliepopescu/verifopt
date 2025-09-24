# meeting

## agenda

- impl updates
    - hardcoded MIR rewrite
        - effectively know how to generate all of the central logic (locals and 
          statements) we'll need
        - currently combing through carefully to check/add:
            - correct locals are being used
            - storage instructions (`StorageLive`/`StorageDead`) for locals
            - ensure all edges funnel to the right place
            - etc

- impl todos
    - vtable lookup
        - currently "faking" vtable lookups by creating `dyn Animal`s of a
          single concrete kind, so we know exactly which vtable said vtable 
          address refers to

    - currently the "original" source code is using the `ptr_metadata` feature
        - what does this do exactly?
        - if introduces runtime overhead, does operating at the MIR level alleviate this?
        - if just introduces unsafety, maybe we can operate at the source-Rust level

    - explore analysis tradeoffs in MIR
        - MIR has more flexible semantics, which may make analysis harder
        - source-Rust has stricter semantics but many more "cases" to handle

- big picture discussion

- `StorageLive` / `StorageDead` instructions
    - https://internals.rust-lang.org/t/semantics-of-storagelive-storagedead-in-mir/16184/10
    - allegedly used for 2 things
        - 1. borrowck input
        - 2. llvm input (for `llvm.lifetime.start` and `llvm.lifetime.end`
             intrinsics)
    - however, they seem to be elided from the MIR once we get to the
      `runtime-optimized` phase...
        - so, are they really being used for reason #2? if not, we don't need to
          insert them
        - *this part could use some investigation*
        - jk, they reappear for opt-level=3 (previously used opt-level=0)
    - other refs:
        - https://doc.rust-lang.org/beta/nightly-rustc/rustc_middle/mir/enum.StatementKind.html#variant.StorageLive
            - "Using a local in any way (not only reading/writing from it) while
              it is unallocated is UB."
            - "Some locals have no StorageLive or StorageDead statements within
              the entire MIR body. These locals are implicitly allocated for the
              full duration of the function. There is a convenience method at
              rustc_mir_dataflow::storage::always_storage_live_locals for
              computing these locals."
            - "If the local is already allocated, calling StorageLive again will
              implicitly free the local and then allocate fresh uninitialized
              memory. If a local is already deallocated, calling StorageDead
              again is a NOP."

- sub-proj for anja
    - test effects of Storage events
        - run valgrind on a tight loop w lots of storage events
        - would answer the question: how important are storage events for
          performance?
    - something else?

## notes

_some_ baseline: synthetic examples are ok (use patterns)
- just want some promise
- logic through (synthetic -> real projs)
- build hypotheses

ideal (i.e. for paper): run on big real proj

lines: 
- transform code as much as possible -> code size diff (~speed of light
  baseline, won't get this)
    - no vtable discriminant, just like an oracle transformation
    - could be way faster than what we think
- manual vtable rewrite (doing now)
    - closer to what the tool will actually achieve
    - sort of in the middle of above/below baselines
- sloppy version of tool (ignores things it doesn't know)
    - conservative lower bound

