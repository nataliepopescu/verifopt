# meeting

## agenda

- high-level / perf smoke-test discussion?

- how much to generalize MIR pass before analysis? 

## notes

benchmark nits
- pre-allocate objects in setup code
- remove `to_string()` calls

- can get (static) vtable addresses from binary and then patch later, so this is
  LIKELY the worst case (meaning we can avoid measuring them during the rewrite)

revisiting paper pitch:
- needed to confirm that this technique improves performance
- b/c the research isn't that "we found an optimization that improves perf", but
  rather "we found way(s) to improve this existing kind of optimization using
  certain ~novel/un-leveraged techniques"

- prior work is useful, but there is still more to do
- in particular, does not leverage provably unused codepaths

- intro outline:
    - dynamic -> static = better perf
    - CHA exists in C++
    - naively porting CHA to Rust may still improve performance
        - e.g. due to lifetime info (more static info that can be leveraged
          during compilation)
    - namely, we (can) do things to further improve:
        - skiplists (instead of one giant inefficient if statement)
        - flow-sensitive analysis

code size smoke tests
- code size will be very vulnerable to our injections
- current numbers == worst possible case

benchmarks vs CHA (related work)
- x axis: # impls of trait
    - what about # impls used?
- y axis: perf
- lines: dyn dispatch, CHA, verifopt, maybe static oracle ideal case

CHA
- also in JVM?
- kind just how Java does dynamic dispatch
- also interacts w JITing...

possible additional experiment
- CHA in C++ vs Rust
    - in Rust: CHA could be better/more informative?
    - if we improve the CHA "baseline", then this could give applications a
      better tradeoff space to operate under?

TODOS
- [x] pre-alloc objs
- [x] remove `to_string`s
- [ ] remove vtable computations

- [ ] additional benchmarks
    - [ ] more "realistic" (ask mae what this means)
    - [ ] CHA
    - [ ] less traits used than implemented

