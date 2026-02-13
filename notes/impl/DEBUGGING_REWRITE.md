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

