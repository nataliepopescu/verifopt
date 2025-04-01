# akash / mike feedback

look into interval analysis in copmilers
- how often / aggressively does this happen?
    - apparently not used very much b/c expensive... any other reasons? and why
      is it so expensive?
- how does it work?

e-graphs / egg
- may help find a good optimization pass order?
- what else to use for?
- => reachability analysis, trims paths as it goes

mike: e-graphs are generally used to find best opt pass order / or most
optimized code
- memorizes "internal" states of rewrite rules instead of throwing them away

think about loops

also maybe optimizing for GPUs

source-to-source optimizations: can't propagate information upwards

if doing verification in wasm, probably need to track provenance
