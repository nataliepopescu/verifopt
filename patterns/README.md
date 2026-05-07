# PATTERNS (for motivation)

Re-attempting pattern checks b/c previous documentation is a bit sparse

Running all of these as `RUSTFLAGS="-C opt-level=3" cargo build --release`

Then `objdump -d target/release/binary > crate_name.S`

## Cross Crate

Also cross-CGU.

### Naive Rustc

dynamic calls are inlined into main when using
- `let num = decide_simple(3);`
- `let num = 3;`
- `a = &mut cat;`
- dog and bird options removed

### Rupta

seems to be able to resolve things to a single target
- `a = &mut cat;`
- dog and bird options removed

imprecision when:
- at least `let num = 3;`
    - tracks things for two separate speak() instances
    - one in use_trait (cat?), other in decl_trait (bird?)
    - confirmed! is tracking something for Cat::speak() even when statically can
      know it will never be called (via const prop)!

### VerifOpt


## Same Crate

Animal::Bird, Cat, Dog

### Naive Rustc

can optimize (constant prop) into simple constant op

### Rupta

seems to be able to resolve things to a single target
- `a = &mut cat;`
- dog and bird options removed

imprecision when:
- at least `let num = 3;`
    - tracks things for two separate speak() instances
    - one in use_trait (cat?), other in decl_trait (bird?)
    - confirmed! is tracking something for Cat::speak() even when statically can
      know it will never be called (via const prop)!

### VerifOpt


## Inter-proc flow-sensitive stuff

