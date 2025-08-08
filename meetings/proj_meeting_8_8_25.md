# meeting

## agenda

- compiled ~general list of cases where vtables are generated (at least in the 
  LLVM IR, may eventually want to confirm in binaries): 
  https://github.com/nataliepopescu/verifopt/blob/main/PATTERNS.md

- implementation now generally has everything sketched out
    - except for (maybe): sources of uncertainty (function summaries
      simplification)

## notes

- for example 17 (patterns link above) try declaring Bird but not using and see how it changes
    - want to see if speak code changes (if it doesn't thats good bc it is generated
      unrelated to number of implementors)
    - doesn't really change, at least not the vtable part (some calculations
      before that do)

- emit results of translation (i.e. print out actual code)
    - be specific about what info we're keeping in the code at this stage
    - also have some way of _running_ the rewritten code that is not relying on all
      the infrastructure from before
        - i.e. JUST an interpreter
    - alternatively, print out real rust

- once in rust, may be easier to mock things up in prototype instead of in situ

- _can_ write the parser if you want the full, end-to-end thing
    - use the prototype as a _tool_ for when you move into rust land

- uncertainty == idk type
    - different designs (how far down to push the uncertainty)
    - real examples should guide those choices (so can wait to make them)

- once in rustc
    - if anything might change what we write in the body of the paper, go back to
      prototype

- where to put our tool in rust pipeline?
    - source
        - pros: already know the dynamics
        - cons: don't know all types
    - MIR
    - LLVM IR

- want to work as high-level as possible
    - what wouldn't work as source-to-source transformation?
        - we rely on knowing all types, but there exists lots of type inference 
          (so may need to go below the surface for this)
        - choice: 
            - add info/logic to tool
            - or go into compiler when that info is already retrieved

- next steps
    - [ ] print out code from prototype rewrite (so it can be run)
        - goal: check functionality/correctness of rewrite
    - [ ] start digging in rust

