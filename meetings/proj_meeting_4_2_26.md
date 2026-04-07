# meeting

## agenda

- perf updates

- next steps?
    - need:
        - support/bench more variant possibilities + more complex control flow
        - vtable shim
            - vtable addresses are unstable even in the same binary, how to get
              + rewrite bin?

    - will stuff
        - still working on finding examples, which is the priority
        - what additional impl-type thing would be helpful to have?
            - take from above "need"s or something else?

## notes

theories of perf diffs:
1. compiler
- code motion (could see in the machine code)
2. hardware (cannot see in the machine code, maybe try to switch features
   off/test on other hardware to isolate)
- prefetching
- diff branch predictor

read vortex paper!

two theories
1. produces better code
2. reacts better w arch/hardware
    - asplos-y

branch predictor
- popular
- sticky
- round robin

- maybe also try on different hardware
    - cortex-m: worse branch predictors, no caches? (pre-fetching) / risc-v
      (embedded)
    - NO qemu
    - need cycle-accurate simulator or FPGA / firesim?

    - goal: explain the perf diff/results
        - how to disable branch prediction/pre-fetching?
            - flush the cache?
        - but also want to run on embedded cpu

    - first double check the machine code
        - can also try to turn off those (compiler) optimizations

- does env_logger example hit any un-implemented cases?
    - test out first
    - if works great, find more examples
    - if not, there's the work

maybe write intro
- more hypothesis focused (chain of hypotheses)
- can also help w "what is next" questions

computer construction (CC) conf! ?
- call for papers


where are vtables generated?
- leon convo
    - deferred calls in tock kernel
    - effectively doing 2 things
        - vtable optimization -> where verifopt can help
        - deferring callbacks
    - will will port
- size!!
    - cache pressure?
- vtable opt (remove _some_ funcs - perhaps broken opt, though, and not used
  anymore)
- how good is the compiler at removing vtables

- also leon email, maybe him/emily can use the func_collect part of verifopt for
  statically knowing which traits are implemented by which structs
    - would be very easy to partition out
    - can maybe give them 2 options to use, 1 the simple partitioned-out
      func-collect, and then 2 the flow-sensitive interp results
    - ah, but they still need the rewrite, so might as well do the
      flow-sensitive stuff (since they don't really care about performance, it
      won't hurt -- keep in mind unless there is a reason to _not_ use
      flow-sensitive results)
