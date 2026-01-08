# Notes on [MirChecker](https://dl.acm.org/doi/pdf/10.1145/3460120.3484541)

[github](https://github.com/lizhuohua/rust-mir-checker/tree/master)

Summary:

Analyze Rust's MIR for certain bugs (e.g. numerical analysis)

Analysis style (program analysis) is very similar, albeit of a subset of the 
language. Notably, they essentially ignore dynamic dispatch. But the way that
they _implement_ their analysis is of use to us. 

WTO
- CFG generally has a topological ordering
- but WTO is for when there are loops apparently

## Implementation

Question:
- how does their implementation interact with `MirPass`? i.e., in what order are
  bodies evaluated?
    - most tests _only_ have a main function, so interprocedural analysis isn't
      really exercised
    - at least one test's main _does_ call another function, so interprocedural
      analysis does exist
    - but does it only exist because that called function is inlined?
        - https://github.com/lizhuohua/rust-mir-checker/blob/master/tests/safe-bugs/incorrect-cast/src/main.rs
    - oh but the trophy_case tests are more realistic (interproc)
    - so how does this happen????

    - there is a call_visitor (CallVisitor) implementation that analyzes a
      called function based on the caller's current state!
        - sometimes MIR is not available... why would this be? some default
          state is returned in this case


hooks into rust compiler via `run_compiler`
- https://github.com/lizhuohua/rust-mir-checker/blob/master/src/bin/mir-checker.rs#L76

implements `Callbacks` trait, specifically the `after_analysis` function
- https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/callback.rs#L35

`after_analysis` impl calls `run_analysis`
- https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/callback.rs#L50

`run_analysis`
- initializes `GlobalContext`
  https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/callback.rs#L69
  https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/global_context.rs#L74
    - `entry_point` field dictates the entry function of the analysis
    - found at https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/global_context.rs#L115
        - via arguments (manual spec of entry point could be good for tool perf)
            - `entry_def_id_index` arg...
            - or `entry` arg...

- initializes `NumericalAnalysis` pass
  https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/callback.rs#L73
- runs pass
  https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/analyzer/numerical_analysis.rs#L89
  which calls `analyze_function` https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/analyzer/numerical_analysis.rs#L149
  with a `DefId`! the `DefId` is gotten from `self.context.entry_point;` https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/analyzer/numerical_analysis.rs#L107

`analyze_function` 
- initializes/computes a WTO (weak-topological order) visitor https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/mir_visitor/body_visitor.rs#L112
- gets the WTO from the GlobalContext (_order_ in which to perform analysis)
    - https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/mir_visitor/body_visitor.rs#L119
    - https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/global_context.rs#L169
        - runs the `optimized_mir` query for the entry point `DefId`
        - get cached/computes WTO
            - https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/wto.rs#L162
            - computes WTO for _this_ body (ordering of basicblocks within one
              body)
                - by _visiting_ the blocks starting from the start block (bb0 i
                  guess)
                    - https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/wto.rs#L213
- and inits a `TypeVisitor`

- calls `wto_visitor.init_promote_constants()` - ?
    - https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/mir_visitor/body_visitor.rs#L196
    - what is this doing????

- calls `wto_visitor.run()` - runs analysis / i.e. visits all the MIR components/basic blocks in WTO-specified order
    - depending if a `Vertex` or a `Circle`
    - populates (inserts) `wto_nesting_map` (vertex -> nesting AKA vertex ->
      [vertex])
        - kind of like our `ConstraintMap` type

- calls `wto_visitor.run_checker()` - bug checker

`create_function_post_state`
- https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/mir_visitor/call_visitor.rs#L125
    - visits body doing the same WTO stuff done before
        - https://github.com/lizhuohua/rust-mir-checker/blob/master/src/analysis/mir_visitor/call_visitor.rs#L156
    - also resolves arguments/arg types
    - populates data structure using `body_visitor.run()`
    - and join state at end (~similar to simple-interp)




