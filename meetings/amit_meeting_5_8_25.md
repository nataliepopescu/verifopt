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


