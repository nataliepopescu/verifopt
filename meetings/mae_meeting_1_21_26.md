# meeting

## agenda

- populating functions: solution
    - iterating through `crates_used`, and for each crate all possible DefIds
      until the code panics (still need to figure out how to handle this)
    - for each forged DefId, if it is a function, add it to our function map

- moving on to interpretation...
    - MIR is not generated for intrinsics, so when our interpreter tries to
      execute an intrinsic function call, it doesn't see anything to execute
        - https://doc.rust-lang.org/beta/nightly-rustc/rustc_middle/ty/enum.InstanceKind.html#variant.Intrinsic
    - currently effectively using summaries, but we could theoretically have
      like an intrinsic library that we pull from to augment interpreter
      knowledge - is this something we may want?

- currently working on collecting trait impls into an easily accessible table

## notes

next steps
- more type info for vars
- what is the "self" / struct type of potential dynamic calls

- concretely
    - at least just get Idk(Ty)

- want to know more for specifically the potential dynamic dispatch calls

- since going forward, want to know as much as we can about types, always

- will want to store _struct_ as well (in trait_impls map)
    - i.e. "what are the Animals"


