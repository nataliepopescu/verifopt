# Project Goals / Hypotheses

Use flow-sensitive program analysis to replace each dynamic dispatch with a 
minimal set of possible static dispatches in Rust.

## Hypotheses

1. Replacing dynamic dispatch with static dispatch will improve performance
   _and_ code size

2. Replacing dynamic dispatch with a _minimal_ set of static dispatches will
   improve performance and code size _more_ than the State-of-the-Art CHA approach
   - Hypothesis: most dynamic dispatches only end up going to 1 or 2 methods in practice

3. Performance impacts are due to enabled downstream optimizations, such as
   code motion, inlining, branch prediction, etc
   - Different workloads lead to different performance: hypothesizing this is
     branch prediction
   - Different inlining annotations also lead to different performance

4. Code size impacts are due to no longer needing vtables for dynamic dispatch,
   as they are converted to static, and thus vtables can be elided (effectively
   another downstream optimization; dead code elimination)
   - Try to start evaluating this
        - in a small example, are the vtables optimized out?
