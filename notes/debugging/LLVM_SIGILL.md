# Debugging an LLVM SIGILL (illegal instruction)

signal: 4

only occurs when compiling `release` builds

following stuff from here: https://rustc-dev-guide.rust-lang.org/backend/debugging.html

setting `llvm.assertions = true` in bootstrap.toml
- might need to delete "file .llvm-stamp, which should be located in
  build/<host-triple>/llvm/."
- actually did not need to delete that file (which is good b/c I couldn't find
  the `llvm` dir)
  - ah, it is now `ci-llvm`

the error is now:

```sh
rustc: /checkout/src/llvm-project/llvm/lib/IR/Instructions.cpp:3037: static CastInst *llvm::CastInst::Create(Instruction::CastOps, Value *, Type *, const Twine &, InsertPosition): Assertion `castIsValid(op, S, Ty) && "Invalid cast!"' failed.

... (signal: 6, SIGABRT: process abort signal)
```

where can i find this file? 
- running `fun / -name "Instructions.cpp"` gives me no files. 
- that's ok though, idk how useful that would;ve been anyway.

just looking at casts in generated MIR:
- Transmutes
    - _6 = copy ((_2.0: std::ptr::Unique<dyn Animal>).0: std::ptr::NonNull<dyn Animal>) as *const dyn Animal (Transmute);
    - _11 = copy ((_3.0: std::ptr::Unique<dyn Animal>).0: std::ptr::NonNull<dyn Animal>) as *const dyn Animal (Transmute);
    - _15 = copy ((_4.0: std::ptr::Unique<dyn Animal>).0: std::ptr::NonNull<dyn Animal>) as *const dyn Animal (Transmute);
    - _24 = copy _20 as &Cat (Transmute);
    - _27 = copy _20 as &Dog (Transmute);

- PtrToPtrs
    - _20 = move _18 as *const () (PtrToPtr);
    let mut _18: *mut dyn Animal;
    let mut _20: *const ();

- other "as"
    - _23 = <DynMetadata<dyn Animal> as PartialEq>::eq(move _21, move _22) -> [return: bb16, unwind: bb8];
    - _26 = <DynMetadata<dyn Animal> as PartialEq>::eq(move _21, move _25) -> [return: bb19, unwind: bb8];
    - _7 = <Cat as Animal>::speak(move _24) -> [return: bb21, unwind: bb8];
    - _7 = <Dog as Animal>::speak(move _27) -> [return: bb22, unwind: bb8];






