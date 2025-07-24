# meeting

## agenda

- dyn usage (mostly in log crate)

## notes

recall: profiling
- can the profiler even identify dynamic dispatch
    - dynamic vs static - that we know the answer to
    - can rust profilers even do this??

visitor pattern
- can be quite inefficient re vtables

the way that rust does vtables has pros/cons
- pros: struct looks like the declared struct
    - also for C abi compatibility
- cons: sometimes have fat pointers, which is annoying for the compiler
    - if no more vtables, maybe compiling can get more efficient
        - probs not be realistic though, b/c still have slices
        - also, a possible strategy for verifopt may depend on vtables still
          being generated (will want to switch on the vtable ptr)
