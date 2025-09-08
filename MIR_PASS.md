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
`optimized_mir` category. Maybe can try both. (Note that if we do try both, the
MIR that each pass be operating on will contain different sets of constructs,
since optimizations may add/remove things. So it won't be just a simple change
of which list we're registering our pass in. Just something to be aware of).

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
your `libstdc++.so.6` location with: `sudo find / -name "libstdc++.so.6" > out
2> err`. the command for me was:

```sh
export LD_LIBRARY_PATH="/nix/store/qksd2mz9f5iasbsh398akdb58fx9kx6d-gcc-13.2.0-lib/lib/"
```

since i am using gcc 13.2.

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

`RUSTC_LOG=rustc_mir_transform::replace_dynamic_dispatch=debug rustc +stage1 original.rs -C opt-level=0 2> log`

ReplaceDynamicDispatch debug statements get optimized out at o3, so using o0 for
now

(can skip the `+stage1` if you've set a directory override in rustup)

## Writing pass

similar/helpful existing passes:
- `add_call_guards` (identifies calls + modifies code around them)
- `check_call_recursion` (gets trait/impl method args)

### Progress/plan

Just writing a pass that does nothing but emit debug statements for terminators
of kind `Call` or `TailCall` first

Now have that working, can access the `func` that each of those terms is
calling, as well as its `args`

`args` are Operands, i.e. a `Copy` or `Move` of a `Place` _or_ a constant value
(not interested in consts)

a `Place` is ~a location in memory
- `Place` struct has `local` and `projection` fields (projection is empty array 
  if place is local)

want to get vtable from Place
- get _type_ of Place arg => get vtable
- also finding out that the mir we're operating on is _not_ the one that gets
  emitted (which makes sense for o3 since we're operating before many of the
  optimization passes, but less so for o0, although i could still imagine o0
  makes some minimal changes)

- type: `PlaceTy { ty: &'{erased} dyn [Binder { value: Trait(Animal),
  bound_vars: [] }] + '{erased}, variant_index: None }`
  - what is a binder? https://rustc-dev-guide.rust-lang.org/ty_module/binders.html ?
  - not sure if ^ is what `Binder` means here
  - maybe it is referring to a boxed type? nope..
  - `is_box` = false
  - `is_trait` = false
  - `is_any_ptr` = true, oh!
    - `is_ref` = true
    - deref...
    - `is_trait` = true!

From Zulip convo
- apparently vtables are opaque until after monomorphization
    - bjorn3 says we should really only access them at codegen or const eval
    - trying to clarify if const eval happens before or after monomorphization
      (order of sections in rustc dev guide is confusing)
        - if const eval happens before monomorph, then can only access the
          vtable at codegen time
- const eval: compute values at compile time
- why are vtables only not-opaque after monomorph?

- can try UnOp::PtrMetadata but apparently can't do much with this (confirm what
  this means)
    - UnOp defined in `rustc_middle/mir/syntax.rs`
    - how to _add_ a statement to a basic block? (to maybe print what info we
      can get from PtrMetadata)
        - modify `BasicBlockData` (struct def in `rustc_middle/src/mir/mod.rs`,
          has `statements` field)
    - this is also apparently an intrinsic, so maybe we can play with the
      intrinsic first: https://doc.rust-lang.org/std/intrinsics/fn.ptr_metadata.html
    - cool, so I just get `()` from that...
    - what is this intrinsic actually doing?

- trying to access vtables from source code
    - https://www.reddit.com/r/rust/comments/11okz75/vtable_layout_documentation/
      "to the extent that any of this is reliable, we can only “rely” on the
      drop/size/align members, since they are explicitly marked as being in
      slots 0/1/2, respectively"
        - `rustc_middle/ty/vtable.rs` line 84: `vtable_allocation_provider()` 
        (constructs vtable)
    - also https://users.rust-lang.org/t/vtable-method-search-technique/120424/7
      "One implementation detail: Since every codegen unit is independent
      there might be multiple vtables for the same type's trait implementation.
      This is definitely true for dynamic linking. I do not know what the
      current status is regarding deduplication with static linking."
      and
      "in practice, the compiler is allowed to duplicate vtables for its
      convenience, or to combine ones that in fact have all the same functions
      when looked at at the machine-code level. So actually comparing vtable
      pointers can be misleading; the compiler even warns against doing this."

this is what the dynamic dispatch looks like in assembly: 
```
.LBB62_32:
        mov     rdi, qword ptr [rsp + 296]
        mov     rax, qword ptr [rsp + 304]
        mov     rax, qword ptr [rax + 24]
        call    rax
        jmp     .LBB62_34
```
(from godbolt vimdiff)

### pre-rewrite MIR

```
bb10: {
    _13 = get_animal(copy _8) -> [return: bb_animal_speak, unwind: bbcleanup2];
}

bb_animal_speak: {
    _16 = copy ((_13.0: std::ptr::Unique<dyn Animal>).0: std::ptr::NonNull<dyn Animal>) as *const dyn Animal (Transmute);
    _15 = &(*_16);
    _14 = <dyn Animal as Animal>::speak(move _15) -> [return: bbdrop, unwind: bbcleanup];
}

// cleanup mess below

bbdrop: {
    drop(_13) -> ...
}

bbcleanup (cleanup): {
    drop(_13) -> [return: _, unwind terminate(cleanup)];
}

bbcleanup2 (cleanup): {
    drop(_1) -> [return: _, unwind terminate(cleanup)];
}
```

### post-rewrite MIR

emitted MIR (from `--emit=mir`):

```
bb_transmute {
    _18 = &_13 (rhs = box dyn trait thing)
    _17 = transmute_copy::Box<dyn Animal, (*const u8, *const usize)>(copy _18) -> [return: bb_get_vtable_ptrs, unwind: bbunwind];
}

bb_get_vtable_ptrs {
    // TODO how to get possible vtable ptrs
    ... -> [return: bb_first_compare, unwind: bbunwind];
}

bb_first_compare {
    _25 = Eq(copy _16, copy _19); (compare trait vptr to one type's, i.e. Cat's, vptr)
    // https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/struct.SwitchTargets.html
    // 0: they're equal, otherwise: they're not equal
    switchInt(move _25) -> [0: bb_second_compare, otherwise: bb_cat_into_raw];
}

bb_cat_into_raw {
    _28 = move _13 (idk why)
    _27 = Box::<dyn Animal>::into_raw(move _28) -> [return: bb_cat_speak, unwind: bbunwind];
}

bb_cat_speak {
    _26 = move _27 as *const () (PtrToPtr);
    _29 = copy _26 as &Cat (Transmute);
    _30 = <Cat as Animal>::speak(copy _29) -> [return: bbgotodrop, unwind: bbunwind];
}

bb_second_compare {
    _31 = Eq(copy _16, _22); (compare trait vptr to Dog's)
    switchInt(move _31) -> [0: bbdrop, otherwise: bb_dog_into_raw];
}

bb_dog_into_raw {
    _34 = move _13 (idk why)
    _33 = Box::<dyn Animal>::into_raw(move _34) -> [return: bb_dog_speak, unwind: bbunwind];
}

bb_dog_speak {
    _32 = move _33 as *const () (PtrToPtr);
    _35 = copy _32 as &Dog (Transmute);
    _36 = <Dog as Animal>::speak(copy _35) -> [return: bbgotodrop2, unwind: bbunwind];
}

// cleanup mess below

bbunwind (cleanup) {
    drop(_) -> [return: bbunwind2, unwind terminate(cleanup)];
}

bbunwind2 (cleanup) {
    drop(_) -> [return: _, unwind terminate(cleanup)];
}

bbgotodrop {
    goto bbdrop
}

bbgotodrop2 {
    goto bbdrop
}

bbdrop {
    drop ...
}
                                
```

omitting a lot of StorageLive/Dead statements, and some copies appear as moves
in the earlier state of the compiler (might be worth looking at the MIR dump
before we run our pass for a more accurate assessment)



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
- i actually think this is related to `SwitchInt` (and not what a general switch
  statement): 
    - https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html#variant.SwitchInt
    - https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/struct.SwitchTargets.html

middle `syntax.rs`
- `TerminatorKind`
- `Operand` (Copy/Move/Const)

there's a query in `rustc_middle/src/query/mod.rs` called `vtable_entries`
- could try at least getting some debug info from it
- search other queries w "vtable" or "trait"

`rustc_middle/ty/vtable.rs`
- line 84: `vtable_allocation_provider()` (constructs vtable)
    - calls `vtable_entries()` (diff argnum from below func, but the types match
      if do dot notation desugaring)
    - gets data from `rustc_trait_selection/src/traits/vtable.rs`
        - line 222: `vtable_entries()`

`rustc_middle/ty/trait_def.rs`
- `pub struct TraitDef { pub def_id: DefId, .. }`
- line 135: `for_each_relevant_impl()`
- line 192: `all_impls()` (iterator over all trait impls)

`rustc_mir_transform/src/ssa.rs`
- is this enforcing SSA?












