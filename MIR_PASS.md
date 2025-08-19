# MIR Pass Notes

Notes on following [these
steps](https://rustc-dev-guide.rust-lang.org/mir/passes.html#implementing-and-registering-a-pass)
to register and implement an MIR pass. 

The previous paragraph seems to state that there are multiple points at which an
MIR pass can be hooked into, where different levels of analysis/transformation
have been performed. 

Goal: run our pass as early as possible so as many optimizations can run after
it as possible. 

Thus, I will start at the earliest point (`mir_built`) and go from there if
things aren't working for whatever reason. 

Ah, but this page does state if this is an optimization it should go into the
`optimized_mir` category. Maybe can try both. 

## Compiling Rust from source

Trying to compile unmodified Rust first to make sure that I can. Then will deal
with errors from any of my modifications when they arise.

### Unmod Rust

`nix-shell -p python3 rustup gcc`

`./x setup`

- compiler
- no githooks
- editor = None

`./x check`

was not finding `libstdc++.so.6`, just had to set `LD_LIBRARY_PATH`. you can find
your `libstdc++.so.6` location with: `sudo find / -name "libstdc++.so.6" > out 2> err`

`./x build` succeeds! (didn't seem to need `pkg-config` or `libiconv` as
[this](https://github.com/nataliepopescu/rust/blob/master/INSTALL.md#dependencies)
suggests)

ah, this only built the stage 1 compiler

`./x build --stage 2`

Then can create custom toolchain (to use custom Rust): 
[instructions](https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html#creating-a-rustup-toolchain)

Note: "The rustup toolchain points to the specified toolchain compiled in your
build directory, so the rustup toolchain will be updated whenever x build or x
test are run for that toolchain/stage."
- so, presumably, once you run both toolchain commands once, you won't need to
  run them again. that's nice.

then you can run `rustup override set stage2` to set the toolchain override for
the directory you are in. once I do this, however, and run and example, my rust
can't find `libstdc++.so.6` again, so I set the `LD_LIBRARY_PATH` once more
(this was in a different window from before). damn, that doesn't fix it this
time...

Maybe it is because I'm invoking `cargo`. Let's just try `rustc`. 

That's closer, but how do I specify external dependencies using `rustc`? Turns
out i have to download each one individually... Let's try a little longer to
make `cargo` work. 

Actually, replacing the `rand` dependency with using command-line arguments for
uncertainty. `rustc` works for that. 

## What do `impl Trait`s look like in MIR?







