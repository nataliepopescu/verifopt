# meeting

## agenda/updates

- completed a rewrite for extremely simple example (statically know there will
  only be one single dyn dispatch variant)
- benchmarked, but no significant perf difference
- tips on how to decompile rust to double check what was compiled?

- working on another example where we do _not_ statically know which variant
  will be called
    - while working on this I ran into a lot of trouble with `collect()`
    - `collect()` internally calls `FromIterator::from_iter()`, but things got
      quite messy/complicated w finding the exact `from_iter()` implementation
      to use (during analysis, not transformation)
    - sunk a couple days into this but ultimately rewrote the source code to not
      use `collect()`, and can complete analysis for that
    - still working on the transformation for this second example

## notes


