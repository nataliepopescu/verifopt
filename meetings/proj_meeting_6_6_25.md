# meeting

## agenda

- relwork / novelty table
    - zotero: a lot of the switch-case similarities == profile-guided opts
    - most of the static analyses = CHA
    - CHA vs rust prog analysis

- some code walkthrough?

- traits impl?

## notes

a lot of relwork using profiling / dynamic recompilation (JIT-like, but
ahead-of-time)

of the relwork that doesn't primarily rely on this, most use CHA (class
hierarchy analysis) -- determine all possible types that some type could be by
analyzing the inheritance tree (_not_ flow sensitive)
- flow sensitivity would further prune results

CHA
- converting to ADT match?
- similar to visitor pattern?
    - idk what this is, look into

this era of relwork seems to swap out abstract classes for (effectively) traits

important: what does relwork do after?

rust compiler already has CHA-level info

verifopt = flow-sensitive!

rust ~C++ abstract class-analogous abstractions
1. abstract class in enum (rust already emits enums)
2. trait
3. dyn trait (open trait = vtables, closed trait = switch statement)
- default = open
    - where verifopt would help
- closed: enumerate all structs that should have impls for some trait
    - used very rarely

^ none of these three are flow-sensitive

rust LTO = regular LLVM LTO

no rust notion of dynamic loading?
- rust is always statically linked
- traits don't go across the C ABI

our fallback case / baseline: source-level rewrite from open to closed traits

imagined benefit; code was written to support polymorph but can really be
monomorph'd
- in functional landscape: biggest benefit when 2 possiblities
    - compiler was inherently good for 1 possibility, but not two

todo:
- what else does relwork do after CHA? anything flow-sensitive?
    - NoVT dead code elimination - how useful/impactful is this?
        - is it just leveraging LLVM's constant prop?
        - may need to look at implementation

constraints
- added when branch/loop, merge when exit loops/conditionals
- search -> determine particularly interesting facts (constraint summaries)
    - can do manually
    - easier via SMT (simplify certain constraints)
    - can also use different decision theory
- constraints == state (e.g. x is 3, 5, or 8)
- could go a step further, record constraints and derive implications to opt
  even further (this may not be necessary)
  - if y == true then x == 7 || if y == false then x == 8 ==> propagate to get
    more optimizations

