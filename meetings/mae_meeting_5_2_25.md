# mae meeting

questions:
- [ ] regular functions in (interp) are also function pointers..? does this seem
  like it will become problematic?

- [ ] wdym by returning the program text?o

## notes

pure vs effects
- boolean conditions == pure
    - generally mathematical expressions
- assignments have effects

coalescing step
- single store: x is mapped to 5 OR 6 (list of vals)
- simulates vtable lookup
    - instead of jumping to arbitrary loc (jump table) 
        - blind jumps ; not in i-cache ; dynamic lookup
    - switch statement => one of five possibilities
        - local branch ; leverages branch predictor (for subsequent calls) + potential inlining

entire jump table for funcs of a particular signature

assignment of funcptr is far away
- instead, replcae funcptr usage w little conditionals/switches

helps the compiler do a better job

at every function call site can print all possible funcs one _could_ call
- now can take advantage of this info
- smaller number of potential calls than wha thte compiler thinks

conditional
- if funcptr == &foo then {...} elif funcptr == &bar then {...}










