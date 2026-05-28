# meeting

## agenda

- project stable MIR still has impl TODOs
    - not really a problem

- recent impl cases (indirect function calls)
    - thread_local! -> static shim

- also type casting

## notes

- want to use cast info
    - kind of oracular assertions (if they succeed)

- always using analysis to add precision to type info
    - unless the analysis context doesn't add anything to type info, don't need
      to use it

- when type is multicomponent
- when have a compound type, don't need to retain entire thing, just the part of
  the type that could be interesting

nested trait objects exceedingly rare

options
1. only record info distinct from type system
    - optimistic that many cases won't be hit
2. implement casts + projections in verifopt land
    - lots of impl


may want to track memory use
- could be where mut/immut comes into play
- if re-used by name?
- i.e., if programer reading the code, how obvious would this be? 
    - if really complicated, out of scope
    - but if simpler, can be in scope
        - just another kind of key for the map

existential question: are there interesting examples that we can convert

