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

- currently working on collecting trait impls into an easily accessible table, but
  UnordMap is making it difficult

## notes

