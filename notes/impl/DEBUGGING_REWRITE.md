# Debugging Rewrite Pass

## on `one_variant.rs`

finding the correct speak() call to replace

`get_trait_impl_dids()` calls `get_struct_dids()`, which attempts to get all the
structs that implement the trait pointed to by the first function argument

- this actually seems to be a hardcoded artifact from the `simple.rs` example
- where the `dyn Animal` object was stored in Place(_1)
- but in `one_variant.rs`, Place(_1) does not hold the thing we are looking for
- in `one_variant.rs` this happens to be Place(_17) (i think)
- how do we get the correct place dynamically?

- well, in the block to speak() we have the following statements:

```
_35 = copy ((_17.0: std::ptr::Unique<dyn Animal>).0: std::ptr::NonNull<dyn Animal>) as *const dyn Animal (Transmute),

_30 = &(*_35),

(term)
_29 = <dyn Animal as Animal>::speak(move _30) -> [return: bb22, unwind: bb26],
```

finished

## crash with `range start index 2 out of range for slice of length 1` from
inside compiler

everything in rewrite (up to and past applying the patch) succeeds without
crashing

manually going through the patch to see if anything looks weird
- next_block is correct
- but next_local is 32, while it _should_ be 27
- new code uses locals 27 -> 31

- maybe somehow the locals are modifying the actual MIR instead of being added
  to the patch?

- ah, no, the new_temp() call increments this value

- weird that new_block() doesn't have the same behavior but ok

