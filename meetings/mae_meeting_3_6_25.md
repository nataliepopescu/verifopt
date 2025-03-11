# mae meeting

directions?

post-student meeting

from anja
- fuzzing loop? 
- more literally verification-driven optimization search

from nikhil/nick
- generate anti-pattern list?

from mae
- "compilers suck"
- cannot recognize high-level data structures at all
- could maybe identify, at the rust level, better data structures
- could also try to figure out what the compiler is good at optimizing or not
    - jumps?
    - dynamic Traits

profile-guided stuff
- different question from: trying to optimize something but blocked by something stupid
    - mostly just tells you where your code is spending the most time
- use profiler as proxy of what optimizer can see?

when compiler is likely to not be able to do something
- jump to dynamic target
- allocate dynamic size (on stack)

- dynamic == user intention

from potential dynamism, can get either:
- switch statement
    - always want to be here (if going to optimize)
- conditionals or arbitrary jumps

how to add compiler hints
- search-guide hint
    - always/never inline
        - can't inline everything, so applies certain heuristics
        - hint: dev telling compiler what to inline
    - struct layout hints
        - padding
    - these are dangerous: dev telling compiler to ignore some info? like intrinsics? always-dead annotations; will-not-alias; 
        - but if some fact is verified then can safely hint it to the compiler
          like this

threading?
- pthreads (process threads)
    - compiler has completely diff logic re read/write optimizations (less conservative)
    - compiler: might be concurrency!
    - a bit confused about this example...
- volatile
    - sometimes ppl use multiple processes instead of multiple threads
    - communicate via memory mapped region
    - ppl use to ~memory map some region so writes are immediately shown

whatever changes you end up doing, will need to verify that it does the same thing
- will have a verification condition

differentiate project from performance engineering
- what do ppl do to perf engineer and can we address some of these automatically?

panics in tock might not be the best example
- have enough info to know what the "best" alternative is

compiler is better on simpler code
- goal: make more straightline code

line of work
- eliminating panic/error cases
- figure out at surface level: why does this branch need to exist?
- refinements on types?
    - annotation that compiler verifies
- hyperoptimization/superoptimization space
    - compiler give dev WHAT they were just able to elide in order to perform optimization X
    - more information for developer when changing future things that may invalidate previous assumptions

rust verification tools seem to be all front-end only
- although we will still need access to the same stuff

tock panic tracing
- try compiling w full debugging symbols enabled
- run the code + trigger the panic
- GDB
    - source code lines vs assembly lines?
    - set instruction-level breakpoint in machine code
