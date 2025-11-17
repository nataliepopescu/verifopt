# meeting

entry point
- how to automatically find?
- don't want to spend a lot of time analyzing stuff you won't need

- MirChecker "cheats" - command line argument == entry point (default: main)

- find: where does dynamic dispatch happen?
- then: reverse search

- proxy: functions that take in dynamic dispatch
    - revisit the stuff we found during the summer - is this indeed an accurate
      proxy?
    - also, how "easy" is it to resolve the chain of functionality to the dyn
      dispatch?

- operating from within the compiler infrastructure could make finding dyn 
  dispatch easier (as opposed to gnarly regexing)


techinques
- forward analysis (MirChecker)
    - everytime reach func, know its reachable

- random analysis (MirPass)
    - don't know if this func is reachable when we are analyzing it
    - can set upperbound on num times go through scratchpad - so find things out
      quickly?
    - but also potentially analyze unused information? - why need timebound (esp
      if analyzing for richer information)


- program analysis typically used for like integer analysis / numerical things
- but with types, our search space is much smaller
    - could be motivation/justification - this use case is actually even better
      for program analysis type things because the set of "all" things is much
      smaller (can make more progress/gain more information within this smaller
      set, potentially)

- is the rust compiler actually good at doing this within a single function
  body? / when theres a single invariant?
    - confirm this though
    - is our actual "space" interprodecural? i.e. is this where we are improving
      upon existing work/infrastructure


maybe just start at main for now and think about pruning later, if it is needed!
- don't pre-opt! >:(

don't focus on evaluation/interpreation functionality rn
- figure out exactly what we care about first
- will _not_ be a general interpreter - this is not the goal
- _may_ become an interpreter for certain relevant things, but again, don't
  pre-optimize -> figure out what we need to track first, and then if that needs
  simplification/evaluation then do it, otherwise we're fine

using Rvalue as storage will prevent us from using what we've learning
- b/c doesn't reference the previous value(s) we learned
- so what _should_ we use? ref into a data structure? i.e. track/map places ->
  values/types?


storage markers
- big weird complicated graph
- want storage markers to trigger heap usage


TODOS
- start analysis using default entry point/main _first_
    - later maybe figure out how to automatically identify entry points

    - are function boundaries (arg = dyn dispatch) good proxy for dyn dispatch to
      target?
        - confirm over-the-summer findings: is the rust compiler good at resolving
          dyn dispatch -> static dispatch within a single function?
        - i.e. are we just targeting interproc use-cases?

- storage markers: benchmark big weird complicated graph

- alternative analysis location: add new hook to Callbacks trait called
  `after_transformation`...
    - where are the various `dyn Callbacks` used?
    - in `rustc_driver_impl`, but codegen + linker are combined...
    - what would `after_transformation` imply?
    - for `after_analysis`, there is a root `analysis` query that is triggered,
      after which the `after_analysis` hook is invoked
        - is there a root `tranform` query? / what exactly does the `analysis`
          query quick off?
        - _could_ potentially call one of the later queries (i.e.
          `optimized_mir`, although this takes in a `DefId` so would have to
          write an `analysis`-like wrapper that takes in the unit type...)
            - how to find the implementation for the `analysis` query?
            - several other queries that take in `key: ()` which could also help
        - i'm also realizing that queries like this don't return anything
            - how, then, are their query results used??
            - ex: there is also an `entry_fn` query that would be nice to
              repurpose
