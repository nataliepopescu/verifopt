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

## Compiling Unmod Rust from source

Trying to compile unmodified Rust first to make sure that I can. Then will deal
with errors from any of my modifications when they arise.

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

## Running tests

`./x test`

```
failures:
    [ui] tests/ui/process/nofile-limit.rs
```

that's fine, seems irrelevant. 

double check we actually ran compiler tests though...

`./x test compiler` is compiling more things, so this likely wasn't run the
first time. maybe can even try just `./x test compiler/rustc_middle` and 
`./x test compiler/rustc_mir_transform` separately (if faster).

indeed, those individual commands may be faster (`rustc_middle` has 86 tests and
`rustc_mir_transform` has 18 tests), but maybe safer to just run all?

but all the compiler tests pass so we good. 

## Creating toolchain

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

No traits or structs, just `fn`s and `const`s: 

```
fn <impl at original.rs:26:1: 26:20>::speak(_1: &Cat) -> () {
    ...
            _4 = const <Cat as Animal>::speak::promoted[0];
    ...
}

const <impl at original.rs:26:1: 26:20>::speak::promoted[0]: &[&str; 1] = {
    ...
         _1 = [const "meow\n"];
    ...
}
```

The dynamic dispatch call:

```
         _14 = <dyn Animal as Animal>::speak(move _15) -> [return: bb12, unwind: bb15];
```

## Writing pass

Just writing a pass that does nothing but emit debug statements for now

similar/helpful existing passes:
- `add_call_guards.rs`

### Things that might be important

notice that the `speak()` call we're interested in is a _terminator_ of a basic
block
- dynamic dispatch calls will likely always be terminators, since the definition
  of a terminator (from
  [here](https://rustc-dev-guide.rust-lang.org/mir/index.html)) is: "actions
  with potentially multiple successors; always at the end of a block"
  - this is sort of the exact nature of a dynamic dispatch call: it has
    potentially multiple successors (unlike static calls which can be statically
    known to have only a single successor)

in `rustc_middle/src/terminator.rs` there is something called `SwitchTargets`
- look into as potential rewrite target

middle `syntax.rs`
- `TerminatorKind`
- `Operand` (Copy/Move/Const)

there's a query in `rustc_middle/src/query/mod.rs` called `vtable_entries`
- could try at least getting some debug info from it
- search other queries w "vtable" or "trait"

## Registering pass

in `rust/compiler/rustc_mir_transform/src/lib.rs`

add pass name to `declare_passes!` macro list

added to `run_optimization_passes`, before inlining happens

## Compiling Mod Rust

`./x check`

`./x build`: builds stage1 compiler, takes ~half the time as building stage 2

`./x build --stage 2` (takes 10 min, not great for iterating)

## Getting debug info from pass

Had to modify bootstrap.toml to set debug level, and then set `RUSTC_LOG` env
var when invoking rustc
- https://rustc-dev-guide.rust-lang.org/compiler-debugging.html#configuring-the-compiler
- https://rustc-dev-guide.rust-lang.org/tracing.html

`RUSTC_LOG=rustc_mir_transform::replace_dynamic_dispatch=debug rustc +stage1 original.rs -C opt-level=0 2> rdd-log`

ReplaceDynamicDispatch debug statements get optimized out at o3, so using o0 for
now

(can skip the `+stage1` if you've set a directory override in rustup)











