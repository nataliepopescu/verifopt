# In effort to find stable switch constants

Currently we have been using vtable ptrs/addresses to switch on in our rewritten statements

What we switch on doesn't actually matter, as long as it has the property that
it is unique for the functionality we want to switch between

Two things this document is trying to address:
1. Get a constant value to use in the switch statement
    - more in verifopt code
2. Removing vtable allocation (since it won't be needed after verifopt pass)
    - more in rustc code

## Explore

### generating vtable (general/control flow)

One route is to modify vtable generation so vtables are never generated, and
then we can just use their addresses as regular constants

so where are vtables generated?

chatgpt pointed me to `compiler/rustc_codegen_ssa/src/meth.rs::get_vtable()`

it does seem like the vtable address is gotten after the vtable has been
allocated, so don't want to do that

what if, instead, we just assign each new vtable "construction" an ID from a
monotonously-inc ID space (or whatever)

this `get_vtable` method takes in a `CodegenMethods` type
- what is this?

- specifically b/c the cached vtable check is this:

```rust
// Check the cache.
if let Some(&val) = cx.vtables().borrow().get(&(ty, trait_ref)) {
    return val;
}
```

- and then that data structure is updated like: 

```rust
cx.vtables().borrow_mut().insert((ty, trait_ref), vtable);
```

- `CodegenMethods` seems like a trait...
	- it is a trait alias in `rustc_codegen_ssa/src/traits/mod.rs`
    - combines: LayoutOf, FnAbiOf, TypeCodegenMethods, StaticCodegenMethods,
      ConstCodegenMethods, DebugInfoCodegenMethods, AsmCodegenMethods,
      PreDefineCodegenMethods

- what is `CodegenMethods::vtables()`?
    - appears in `MiscCodegenMethods` decl
    - no args (except for &self)
    - returns `&RefCell<FxHashMap<(Ty<'tcx>, Option<ty::ExistentialTraitRef<'tcx>>), Self::Value>>`

- impls:
    - ./rustc_codegen_gcc/src/context.rs:387
        - impl block: `impl<'gcc, 'tcx> MiscCodegenMethods<'tcx> for CodegenCx<'gcc, 'tcx> {`
    - ./rustc_codegen_llvm/src/context.rs:803
        - impl block: `impl<'ll, 'tcx> MiscCodegenMethods<'tcx> for CodegenCx<'ll, 'tcx> {`
        - body: `&self.vtables`
        - `pub(crate) type CodegenCx<'ll, 'tcx> = GenericCx<'ll, FullCx<'ll, 'tcx>>;`
            - one per CGU
        - `GenericCx` is a generic wrapper
        - `FullCx` is the concrete type
            - struct
            - `vtables` field: `pub vtables: RefCell<FxHashMap<(Ty<'tcx>, Option<ty::ExistentialTraitRef<'tcx>>), &'ll Value>>,`

- its funny that this function call is itself a dynamic dispatch...

also note that `get_vtable` returns a `Cx::Value`
- seems to be an llvm::Value
    - `pub(crate) use crate::llvm::Value;`

```rust
unsafe extern "C" {
    ...
    pub(crate) type Value;
    ...
}
```

could try to figure out how to construct such a value, or could change the type
sig and figure out who calls `get_vtable`
- seems to be called by
    - `./rustc_codegen_ssa/src/base.rs:209`
        - called by `unsized_info` when target kind() == dynamic
            - returns `get_vtable` res
        - called by `unsize_ptr`
            - returns `unsized_info` res
        - called by `coerce_unsized_into`
            - uses `unsize_ptr` res (`info` var)
            - stores some info
                - `OperandValue::Pair(base, info).store(bx, dst);`
                    - `OperandValue` == enum
            - this is called by other crates it looks like
    - `./rustc_codegen_cranelift/src/unsize.rs:62`
        - similar usage here


### fat ptr gen

- oh shiz, `unsize_ptr` (above) might be the thing that creates fat pointers

### allocating vtables

vtable allocation = Tcx query
- could try to modify this to not actually allocate anything
    - but would need to make sure that not doing an empty alloc
    - which might be what would happen in order to get an actual allocID
- where impl??

`vtable_allocation_provider`
- in `compiler/rustc_middle/src/ty/vtable.rs`
- calls `tcx.vtable_allocation` >:(
- description: gets a (possibly cached) vtable allocation
    - returned AllocId should not be used
    - use the GlobalAlloc::VTable id (see usage in `get_vtable`)

- impl:
    - gets vtable_entries
        - how is the `tcx.vtable_entries` query impl??
    - `let mut vtable = Allocation::new(vtable_size, ptr_align, AllocInit::Uninit, ());`
        - the raw memory buffer for the vtable!
    - loop to populate the vtable memory buffer
        - scalar is one of: fn ptr, int (size/align), drop glue ptr
        - write scalar into mem buff
    - intern alloc (_actual_ allocation) via `tcx.reserve_and_set_memory_alloc(alloc)`
        - maybe THIS is where we can just "fake" allocate
        - but this still doesn't solve the problem of getting constant 

        - `reserve_and_set_memory_alloc` calls:
            - `reserve_alloc_id`
                - gets `AllocMap`'s `next_id`
            - `set_alloc_id_memory`
                - alloc_map.to_alloc = ShardHashMap AllocId -> GlobalAlloc
                - tries to set GlobalAlloc::Memory(mem), where mem is a
                  ConstAllocation (from vtable_allocation_provider)
                - GlobalAlloc == enum

            - perhaps important for the sake of our vtable consts:
              "[reserve_and_set_memory_alloc] Interns the
              `Allocation` and return a new `AllocId`, even if there's already
              an identical `Allocation` with a different `AllocId`."
                - this is in the comment above method

                - i guess i'd be interested in knowing why this was chosen?
                - i suppose itd be hard to search through all of the things
                  you've already stored to make sure you're not duplicating,
                  which now puts into perspective all the dedup methods i'm
                  seeing here
                - `reserve_and_set_dedup` fn exists: why aren't we using that?
                    - oh, well it takes in a GlobalAlloc so thats one potential problem (need to have the GlobalAlloc first, which we'd only get by setting the AllocId the first time)
                - i think i might actually be misinterpreting the above quote...

                - ooooh, so apparently b/c queries are cached, we will never get
                  to this point again for the same vtable, so no dedup actually
                  has to happen (thanks chatgpt)


### accessing vtables

maybe in tcx.global_alloc(alloc_id)?
- returns: GlobalAlloc::Memory(Allocation)



### rustc constant interning

vtables stored as constants? understand this more
- compile-time constant

from chatgpt: "vtables are just another interned constant allocation"
- rustc global memory table
    - statics, consts, promoted MIR constants (idk how this is different from
      regular consts), vtables, const-eval results

constant promotion
- ""Promotion" is the act of splicing a part of a MIR computation out into a
  separate self-contained MIR body which is evaluated at compile-time like a
  constant." from https://github.com/rust-lang/const-eval/blob/master/promotion.md
    - i.e. `&3` (a reference to a constant)
    - get a `'static` lifetime

interning
- "If Rust needs the same vtable again, it reuses the same allocation."
  (chatgpt)
- but may have multiple AllocIds that point to that memory?
    - when in the same crate, vtable allocs are cached by
      `tcx.vtable_allocation`, so no
    - but if in a different crate/cgu, have a different cache, so yes


### relocations??

how the compiler represents ptrs inside constant memory
- b/c ptrs cannot just be stored as raw bytes during compilation
    - why? b/c don't actually know the concrete ptr value statically?
    - yep, so essentially a stand-in for real ptr addresses

### const eval

seems like const eval happens _in_ miri
- "The Rust compiler runs the MIR in the MIR interpreter (miri), which sort of
  is a virtual machine using MIR as "bytecode"." from https://github.com/rust-lang/const-eval




## Design

so statically, in a CHA-manner, we can ascribe a unique ID for every individual possible concrete trait object
- organized by traits ofc
- can do this in funccollect

- top-down or bottom-up? i.e. do we let verifopt determine IDs, or do we use something that already happens to be generated by rustc (e.g. vtable ptrs)?

then when `get_vtable` gets called, we use the trait_ref to figure out which set of IDs to use
- intersect that w the concrete types in the switch statement
- at this point we can also skip actual vtable allocation

and when were performing the rewrite, we can use that static ID list for the
stable constants, and then `get_vtable` will later resolve to whatever we've specified

there are definitely still some holes here but seems like a good first iteration
- e.g., how will we pass the static IDs to `get_vtable`
- and how to compile verifopt w modified rustc (a problem we ran into earlier)
- also since `get_vtable` returns an LLVM Value, can we make our constants LLVM values? and how?
    - `rustc_codegen_llvm/src/llvm/ffi.rs`

### Switch Constants

if a struct impls two traits, it will have two vtables

so constants should be assigned to each (type, trait) _pair)_

use FuncCollect `trait_to_struct_impls` map to get all possible pairs for said
library/binary
- then assign them unique (monotonically increasing) IDs

maybe we can also address the cross-cgu vtable ptr duplication here too
- since multiple AllocIds may point to the same vtable contents (if
  cross-crate/cgu), then probably do not want to use AllocIds as our "constant"
- apparently this can also happen due to monomorphization

"AllocIds are not globally unique across crates." (chatgpt)
- only meaningful within a TyCtxt (crate/cgu)

mental model: "rustc prefers: duplicate constants > global coordination"

actually, constants don't really need to be unique for each (type, trait) pair
- really just need a unique constant for each type
- then dynamic dispatch sites for different traits can switch across the relevant
  struct constants
    - since we're only switching over types that impl that trait anyway, we
      don't need to differentiate struct-for-traitX vs struct-for-traitY

chatgpt also suggested that constants don't even need to be globally-unique,
just need type tags for every dynamic dispatch site
- this is true but also seems weirder to impl?
- like we'd have to somehow add/generate code that assigns the ids to each type i think

another option: hashing
- this seems so obvious idk why we haven't thought about it before
- also wouldn't need to store the mappings anywhere since the hashing is
  deterministic
    - although maybe would need to check for collisions? just to be REALLY sure?
- chat recommended Fingerprint datastructure, but thats `pub struct
  Fingerprint(u64, u64);` -> why two u64s?

- i think we would want a hash result that is `usize` large (or just `u32`?)
    - FxHasher: `write` and `finish`
        - have to create a new hasher every time want clean slate
    - hash struct DefIds
        - what is our discriminant?
        - this is dynamic, so i think we'd need to generate code that does this
          hashing at runtime
        - in which case, now we're adding runtime overhead
        - so this is actually probably not the best thing to do




### Modify/Remove Vtable Allocation


### Modify Vtable Access Methods




## Impl

compiling rust without any changes first (again) to make sure we're starting from a working baseline
- [x] check
- [x] build

### LLVM Value

defined in unsafe extern "C" block
- `type Value`
- the thing is that this is an opaque pointer type, not necessarily an actual value
- ah, seems like the fact that these are pointer types means that they are
  UNSIZED and thus must be used via pointer

### LLVM Constant Int

`type ConstantInt`
- still opaque, but we'd be using constants so what is `get_vtable` returned one
  of these instead of a `Value`?

### Value -> ConstantInt












