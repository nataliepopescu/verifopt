# Rust Compiler

[overview](https://rustc-dev-guide.rust-lang.org/overview.html)
- queries vs passes
    - queries: between HIR and LLVM-IR
        - for incremental compilation
        - queries == methods on `TyCtxt`
        - `TyCtxt` also stores in-mem cache (for incr comp)
    - passes: lexing, parsing, name res, macro expansion
- Token Stream -> AST -> HIR -> THIR -> MIR -> LLVM-IR
    - HIR
        - contains implicit things like elided lifetimes
    - THIR
        - all method calls are now fully explicit
    - MIR == CFG
        - basic blocks + edges
        - used for borrow checking + other dataflow checks (uninit values)
        - then optimizations + constant evaluation
        - for verifopt: 
            - the explicit dataflow may be useful
            - want to try to operate before borrow checking (less we'd have to
              "verify" in our codebase), and definitely before optimizations run
- values are often "interned" (perf/mem optimization: allocate in an "arena")
    - so not double-allocating for values

`TyCtxt` potentially-relevant methods
- [reserve_and_set_vtable_alloc](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.TyCtxt.html#method.reserve_and_set_vtable_alloc)
- [return_type_impl_or_dyn_traits](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.TyCtxt.html#method.return_type_impl_or_dyn_traits)
- [return_type_impl_or_dyn_traits_with_type_alias](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.TyCtxt.html#method.return_type_impl_or_dyn_traits_with_type_alias)
- [fn_abi_of_instance](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.TyCtxt.html#method.fn_abi_of_instance)
    - `InstanceKind::Virtual`
- [new_dynamic](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/inherent/trait.Ty.html#tymethod.new_dynamic)


AST lowering
- when [trait
  solving](https://rustc-dev-guide.rust-lang.org/traits/resolution.html) happens
  - "the process of pairing up an impl with each reference to a trait"
  - i suppose this doesn't happen for `dyn` traits... CHECK

MIR
- [MIR
  optimizations](https://rustc-dev-guide.rust-lang.org/mir/optimizations.html)
- MIR is generic (not monomorphized)
- also perform "monomorphization collection" (for eventual monomorphization
  during codegen, i.e. in the MIR -> LLVM-IR step)

- [MIR](https://rustc-dev-guide.rust-lang.org/mir/index.html)
    - "It is a radically simplified form of Rust that is used for certain
      flow-sensitive safety checks – notably the borrow checker! - ..."
