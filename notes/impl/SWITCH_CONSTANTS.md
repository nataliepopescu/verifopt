# In effort to find stable switch constants

Currently we have been using vtable ptrs/addresses to switch on in our rewritten statements

What we switch on doesn't actually matter, as long as it has the property that
it is unique for the functionality we want to switch between

## vtable gen

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


## fat ptr gen

- oh shiz, `unsize_ptr` (above) might be the thing that creates fat pointers


## design

so statically, in a CHA-manner, we can allocate a unique ID for every individual possible concrete trait object
- organized by traits ofc
- can do this in funccollect

then when `get_vtable` gets called, we use the trait_ref to figure out which set of IDs to use
- intersect that w the concrete types in the switch statement

and when were performing the rewrite, we can use that static ID list for the
stable constants, and then `get_vtable` will later resolve to whatever we've specified

there are definitely still some holes here but seems like a good first iteration
- e.g., how will we pass the static IDs to `get_vtable`
- and how to compile verifopt w modified rustc (a problem we ran into earlier)
- also since `get_vtable` returns an LLVM Value, can we make our constants LLVM values? and how?



