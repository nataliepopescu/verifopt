## notes

potential bug/feature: positive + negative constraints may intersect

1. allow and say its normal
2. disallow + maintain invariant

in branch unification, allowing intersection might be difficult
- might be easier to just maintain the invariant that they do not intersect

(branch unification whiteboard example)
- one option: drop negatives (positives override)
- but also consider order of ops (if linearly later in time, overwriite the _whole_ tuple)

- be careful when conflict: favor positive or negative?

CHA w/out flow-sensitivity
1. code transformation that puts types on all vars
2. if func call via variable => switch 
    - but don't have constraints
    - collect all funcs of certain type
    - switch over all of them

need to correctly translate betweeon EMPTY and TOP states
- in order to perform the switch rewrite

CHA == base case / fallback

amit's bet: either know exactly, or know nothing (aka CHA) => majority of cases

mae's bet: lots of switch statements w two cases

doing closures rn
- look into captures
- C++: explicit capture list
- Rust default? move + always capturing? something else?

nested funcs in rust?
- what happens / what is their behavior

- option: hoist nested func to top-level

- nested func != lambda?
    - nested funcs: don't capture, lambdas do -- confirm
    - rust lambdas: call-once (default)

    - capturing vs double-spends, etc

only visit function once + attach info to func: function summary
-> for efficiency + composability (?)
    - change a file + recompile, don't need to entirely rescan code
1. after analysis: params = constraints, ret = constraints
    - constraints wrt global names or param names only
    - don't have to step into func again

## next steps

- check for intersections (at every merge step, i.e. branch unification)
- CHA
    - code transformation: put types on all vars
    - in rewrite step, if don't have any constraints (top/bottom of lattice) use
      function signatures to resolve list
- clarify function scope semantics (nested funcs vs lambdas)
- conditional scopes vs SSA

