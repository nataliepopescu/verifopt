# meeting

## agenda

- tried
    - [ ] profiling
        - flamegraph does not differentiate dynamically vs statically dispatched 
          calls: https://github.com/nataliepopescu/verifopt/blob/main/dp-ex/flamegraph_3impl.svg
        - unsure if useful to try other profiling tools at this point, seems
          unlikely based on below findings
    
    - [ ] looking at IR to try to manually identify dynamically dispatched calls
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

- [ ] next: instrument rustc to log vtable calls?
    - have a couple pointers to where vtables are constructed in the compiler
      (from zulip)

## notes


