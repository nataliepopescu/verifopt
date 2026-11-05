# meeting

## agenda

- running tock on renode
    - instrumentation for speed of light baseline in progress for nrf boards
    - nrf boards only have drivers for LED + alarm, though, which limits the
      number of apps we can use for our workload data

- imix results
    - 10 places where verifopt analysis does better than CHAmax (compiled instances)
        - 2: 154 -> 1 (Iterator::next)
        - 2: 2 -> 1
        - 5: 10 -> 9
        - 1: 83 -> 38
    - but none of these sites end up as single (unguarded) rewrites
        - 4 _could_ if we unsoundly removed the guard, and assume verifopt
          analysis is sound
    - when single rewrites only (unguarded) are applied:
        - if currently "provable" rewrites only, verifopt does no better than 
          CHAmax in terms of size (this excludes guarded direct call rewrites)
        - if we assume verifopt analysis is sound, _then_ verifopt can convert 4
          more sites to single direct calls:
            - -16 B if VFE+elim or just elim
            - -36 B if just VFE
    - so, *verifopt still reaps some bytes on tock* relative to CHAmax
        - still waiting on speed of light baseline + other related work
          comparisons (Rupta + RTA)

- nrf results
    - tried k=2 context sensitivity, didn't change anything

- ripgrep analysis is now running in 25 minutes
    - various optimizations, the biggest ones being:
        - statics "updates" (initialized immut statics, and ?) were invalidating
          memoized cache so many things were repeatedly executing
        - drop unreferenced temporaries -> help cheapen state merges
            - return local + args are not dropped
            - if a local is at all referenced in the body, not dropped
            - if a body creates a closure/coroutine, is a coroutine, or 
              contains inline asm: no temps are dropped
            - drops happens _after_ a body has been walked
                - including outstanding recursive calls

- ripgrep results
    - caveat: missing some precision changes we made for the tock builds
    - n=1 rewrites are mostly in panic handling code
    - when max candidates threshold == 1
        - 13 rewrites
            - 2 single unguarded: ignore::WalkParallel::visit
                - might be useful for perf [TBD]
            - 2 single guarded:
              std::small_c_string::run_with_cstr_(stack/allocating)
                - used in path-taking syscalls
            - 3 single guarded: panic handling
    - TBD larger max candidates thresholds

## notes

