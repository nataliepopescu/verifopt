# meeting

## agenda

- goal: gain empirical confidence in verifopt potential

- tried
    - grepping for `dyn` in crates + searching for those uses in apps: https://github.com/nataliepopescu/verifopt/blob/main/DYN_TRAIT_OBS.md

    - profiling
        - flamegraph does not differentiate dynamically vs statically dispatched 
          calls: https://github.com/nataliepopescu/verifopt/blob/main/dp-ex/flamegraph_3impl.svg
        - unsure if useful to try other profiling tools at this point, seems
          unlikely based on below findings
    
    - looking at IR to try to manually identify dynamically dispatched
      calls: https://github.com/nataliepopescu/verifopt/blob/main/PROFILING.md#inspecting-mir
        - MIR
            - indirect calls are clearer (including in release builds)
            - can potentially differentiate indirect calls using func ptrs vs 
              vtables...
        - LLVM IR
            - a bit murky, since hard to distinguish between indirect calls via
              function pointers and indirect calls via vtables
                - syntax seems the same
            - LLVM might also be doing some optimizations/devirt that remove 
              some vtable calls

- next: instrument rustc to log vtable calls?
    - have a couple pointers to where vtables are constructed in the compiler
      (from zulip)
        - [this](https://github.com/rust-lang/rust/blob/3fb1b53a9dbfcdf37a4b67d35cde373316829930/compiler/rustc_middle/src/ty/vtable.rs)
          apparently gets data from
          [this](https://github.com/rust-lang/rust/blob/3fb1b53a9dbfcdf37a4b67d35cde373316829930/compiler/rustc_trait_selection/src/traits/vtable.rs#L222)

## notes

- [x] muck around w code to get somewhere where llvm can't optimize
    - llvm seems to be doing code motion rn which looks a lot like flow 
      sensitivity (in the simple case)

- use profiler on animal_speak (which hopefully uses <Animal as Animal>...?)
    - can now hopefully ID the dyn call
    - but also maybe can stop before this

- try to find code that seems reasonable to find in the wild
    - [x] write small visitor example (given OG visitor code)

- find patterns in the wild that do x
    - e.g. visitor might be a very good match
    - key: lots of indirections (not just b/c hitting vtable a lot but b/c diff
      code units)

- just find _one_ example where this works

- we currently have evidence (double check) that the compiler is doing
  suboptimal things
  - we don't know, however, how impactful it is

- so: validate until we have a few examples of where the compiler is inefficient
  and then move on
  - [x] compile a list of ~general cases
  - amit is confident that we can find important cases where this matters
  - can also argue that we can make hypothetical code faster, where the
    hypothetical code is the code you actually wanted to write (easier to
    maintain, etc, than generics, which are awful to work with)
    - similar to i++ vs ++i (i++ is more efficient than ++i)
  - "if you build it, they will come"

