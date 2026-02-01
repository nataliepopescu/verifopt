# `rustc_driver` Hook

## Function Collection

trying to populate a table of all functions that may be reachable from some
entry point.

this includes those from non-local crate dependencies.

for the `examples/simple.rs::main` entry point, crate "deps" should only be the
local crate and std?

```sh
RUSTFLAGS="-Zalways-encode-mir=yes" cargo run examples/simple.rs -Zbuild-std
```

1 found function call
- get_animal

2 not found
- std::boxed::Box::<Cat>::new
- <dyn Animal as Animal>::speak

focusing on the Box::new() call first
- from std crate so should also add `-Zbuild-std`
  (https://doc.rust-lang.org/cargo/reference/unstable.html#build-std)
    - "Using -Z build-std will implicitly compile the stable crates core, std,
      alloc, and proc_macro"
    - "Note that for this to work you need to have the source code for the
      standard library available, and at this time the only supported method of
      doing so is to add the rust-src rust rustup component:"
    - seems generally difficult to combine rustc_driver w a custom rustc
      build...
- actually cannot get `-Zbuild-std` to work?
    - but at that call site tcx.is_mir_available() returns true! even without
      `-Zbuild-std`; maybe we can ignore this for now
    - so perhaps we can lazily populate/memoize this table?
        - the problem w doing this lazily is that now we have to worry about
          loops/check for fixpoints

note, our speak() call's DefId returns false when `is_mir_available()` is
queried
- TODO

### Lazy FuncMap Population

can eagerly populate funcmap for the current crate easily

doing this for dependent crates is harder

tried populating lazily, i.e. during interpretation, but
- will require thinking about loops/fixpoint
- harder to get function signature info during interpretation
    - for local crate, rely on LocalDefId, which doesn't exist for non-local
      crates
        - convert to HirId and then get FnDecl
    - we only have DefId
    - perhaps we don't need this information? revisit why we want function
      signatures

what are HirIds? and why do they only exist for local crate items?
- i guess thats just how they're defined

- [ ] _why_ do we want signatures?
    - enables a sort of reverse search
    - i.e. can group all functions w the same signature so when we call X that
      has signature Y we know the fixed set of what functions that may be
        - i.e. used during the rewrite step
            - for indirect calls (not necessarily dyn dispatch)
            - doesn't seem to be used for trait invokes (dyn dispatch)
        - to check if our constraint set is Top or Bottom?
- [ ] can we get all the information we _need_ by interpreting the code instead?
    - assuming we aren't addressing indirect calls
    - try in branch?
    - effectively get access to a Body
      (https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/struct.Body.html#method.basic_blocks_mut)
    - local_decls ("The first local is the return value pointer, followed by
      arg_count locals for the function arguments, followed by any user-declared
      variables and temporaries.") field
        - idx 0: ret type
        - idx [n]: arg type
        - i think arg names are just indices? (`arg_count` field can get us
          that)
        - otherwise the last thing to figure out would be the `is_method` field
            - important for determining which type this is being invoked on, but
              maybe tbd is ok for now?


### Eagerly, using CrateMetadata/CrateRoot

- [ ] would this API even work?
- [ ] if so, make API accessible by building w a modified rustc

#### Custom Rustc





## Distinguishing Dynamic Dispatches

e.g. speak() call

as in the interp-func collection, we get access to an MIR Body
- the local_decls field is what is important: "The first local is the return
  value pointer, followed by arg_count locals for the function arguments,
  followed by any user-declared variables and temporaries."
- so, the second local would be the `self` type that we can use to differentiate

- knowing _which_ speak to call would require knowing what the `self` type is of
  the current object the func is being called on (if any) + if the called func
  is a method (has a self type) and what _that_ self type is
  - i guess we can assume that, since type checking has passed, the self types
    match up
        - maybe call 
          https://doc.rust-lang.org/beta/nightly-rustc/rustc_middle/ty/struct.TyCtxt.html#method.has_typeck_results
          to double check
  - so we just need
    - current self type?
    - / if the func is a method?

    - is it possible to tell if a function is being called on an object (as
      opposed to `Self::`) at the MIR level?
        - i.e. a trait implementation (*this* is really what we want to know)





