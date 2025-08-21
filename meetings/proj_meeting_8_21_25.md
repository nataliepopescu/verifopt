# meeting

## agenda

- catching everyone up
    - can print prototype transformation in valid rust to make sure it can run
    - moving into rust compiler
        - some decisions about _where_ to operate
            - need types to be resolved
                - THIR and below (unless use rust-analyzer or something for source-level)
            - also probably need access to vtable pointers (to switch on)
                - MIR
        - worst case could potentially grab diff info from diff levels
    - trying to write an MIR right now
      (https://github.com/nataliepopescu/verifopt/blob/main/MIR_PASS.md)
        - plan:
            - hardcode simple transformation for code we're familiar with
            - then start to port analysis
                - can skip/be extra conservative around stuff we don't know how
                  to handle yet, for the sake of quickly getting to a "complete"
                  solution/thing that we can run
        - keeping an eye on the following (not sure if/how they may impact us):
            - structs/traits look a little different than they do at the source
              level
            - MIR is a *superset* of the Rust language

## notes

LLVM would be good for other langs

higher-level: more info
lower-level: more portable

"what do we learn" - could there be a reason to do things differently (i.e.
retain type info lower level or something)
- alternatively, MIR could be _help_ w certain types of analysis (global opts
  looking at types/type heirarchy -> opt code gen)


