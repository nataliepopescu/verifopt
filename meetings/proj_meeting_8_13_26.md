# meeting

## agenda

- ripgrep improvements
    - performance
        - still hitting a CPU bottleneck
        - think proper *widening* is the problem (constraints are really big)
            - current constraint collection mech:

            - current widening mechanism = cap
                - if *too many constraints* of same kind (e.g. ADT), remove fields/variants info
                    - doesn't do anything for: closures, fndefs, fnptrs, dynamics
            - ~3 current constraint *caps*: reactive perf fixes rather than
              meaningful widening

            - TODO better widening mechanism
            - TODO widening *points*
                - [ ] control flow joins
                - [ ] loop iterations
                - [ ] updating constraints

    - now getting hundreds of dynamic dispatches (previously got to a point
      where there were thousands)
        - poses a potential validation problem

## notes

- tracking too many variables
    - prepass: which traits are used dynamically ever? narrow which variables we care
      about

- also numerics

checkpoint
- context ^, whats happening, whats in the constraints
- high / low watermarks, averages, etc

flamegraph from the perspective of ripgrep

validation
- random audits

2 tests
1. ripgrep in its own world
2. max feature test
    - will likely catch bugs here

ripgrep: AFL
- random strings are well suited for a grep/searching binary

also generally try to use binary's test suites

maybe (imHashMap vs HashMap w backptrs)
- collapse + clone
- test hashmap walks
    - use flamegraph for this

look into ImHashMap
- is garbage aware? if not, manual clone
