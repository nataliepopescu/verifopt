# mae meeting

questions: 

- [ ] interp should eventually spit out 3 things
    - [x] store
    - [ ] expression returned
        - if anything (at this point, nothing i think)
    - [x] string of program text
        - presumably with the funcptr -> conditional/switch rewrite?

- [x] is a funcptr the same as a funcdef? (at least in my code?)
    - i think right now they are the same thing
    - what would it mean/look like for them to be different?

## notes

conflating inlining + calling

top-level funcnames -> funcbodies (i.e. symbol table, actual names) table
- scan FIRST to get this table
- additional level of indirection

cleverness of extraction (do them at different times / passes)
- braindead extraction (funcnames)
- ~what interpreter is doing

stages
1. collect simple stats (funcnames)
2. step through abstract eval (more specific var info)
3. translation (print out new program)

- separating (into diff files) makes more debuggable
    - doing the simple thing might break the complex thing if done at same time
- more extensible, too

compiler research area for how to do this efficiently
- but calling the SMT solver is the real perf bottleneck, so a few extra passes
  won't really matter

will largely get stage 1 from existing compiler, but stages 2 and 3 we'll be
adding

how to get funcptr vals out of hashmap + use to do something useful w program

between step 2 and 3: print out hashmap

in tests
- not capturing potential overwrites
- can just crash if ever overwrite (presume input is SSA, otherwise malformed)
    - want historic information about what "x" used to be
    - SSA
- unifying branches == only interesting case anyway

"easy" stuff = often more than one pass

so far, passes are:
- func-symbols pass
- SSA-only pass
- interp pass
- rewrite pass

