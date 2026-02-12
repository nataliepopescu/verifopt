# meeting

## agenda/updates

- completed a rewrite for extremely simple example (statically know there will
  only be one single dyn dispatch variant)
- benchmarked, but no significant perf difference
    - actually, not properly microbenchmarking, so will still need to do this
    - tips on how to decompile rust to double check what was compiled?

- working on another example where we do _not_ statically know which variant
  will be called (but that there is only one possible anyway)
- mainly doing this to help debug code changes between the first and third
  examples
- while working on this I ran into a lot of trouble with `collect()`
    - `collect()` internally calls `FromIterator::from_iter()`, but things got
      quite messy/complicated w finding the exact `from_iter()` implementation
      to use (during analysis, not transformation)
    - sunk a couple days into this but ultimately rewrote the source code to not
      use `collect()`, and can complete analysis for that
    - still working on the transformation for this second example
- transformation for this second example just went through (w the help of some
  hardcoding)

- now working on third example where we do not statically know which of _two_
  variants will be called

## notes

still kinda shooting for oopsla

asplos is two weeks later

systems might be difficult to get eval things for

