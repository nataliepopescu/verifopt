# MIR Analysis Notes

Goal: traverse/visit MIR to collect information

## MIR Visitor

Notes on 
[this](https://rustc-dev-guide.rust-lang.org/mir/visitor.html)

Note, a visitor could _also_ be used to make changes to the MIR
- i.e. could help us identify specific dynamic dispatch spots (instead of the
  hardcoded way we are doing it now)

Two visitors
- one immutable (focus here for now)
- one mutable
    - use `visit_terminator()` method (function calls will always be
      terminators)

- so ultimately, can implement our "phases" in this way: first visitor(s)
  operate on immutable MIR, then last visitor operates on mutable MIR

`make_mir_visitor` macro?

unclear how the `Visitor` trait and `MirPass` traits interact... why have both?

also how are visitors registered? is this automatic? if so, then how to control
in which order they run?
- ah they are used from within `MirPass` implementations

is it ok for SSA checks to happen _after_ our analysis/transformation?
- why do we rely on SSA again?




how are errors handled in the rust compiler?




an `MirPass` operates on a `Body`, i.e. a single function
- but we need to analyze cross-function data
- how does/can this happen in the compiler?

- maybe look at inlining as an example

`cross_crate_inline.rs`
- does not implement `MirPass`
- a `Visitor` is instantiated in the main function
- where is this function registered?
    - used in `provide` method
    - in `lib.rs`: `cross_crate_inline::provide(providers);`
- when does this run then? / when is `provide` called?

[Queries](https://rustc-dev-guide.rust-lang.org/query.html#queries-demand-driven-compilation)
- what kinds of things are computed by queries?
    - well, in theory, everything
    - example: type_of (given some DefId)
    - `TyCtxt` has methods for each query
- specifically,
  [how queries are invoked](https://rustc-dev-guide.rust-lang.org/query.html#how-the-compiler-executes-a-query)
  (if not in the cache) seems to rely on
  [providers](https://rustc-dev-guide.rust-lang.org/query.html#providers)
- Providers struct: function table (contains function pointers)
    - local vs external crate, depending what crate the query arguments refer to
        - for now can implement a local provider only, but will eventually need
          both external + local
            - `provide()` + `provide_extern()`
            - _unless_ single crate == single codegen unit, which is a
              requirement that we will be enforcing anyway
                - if this is the case, only need local provider(s)
    - [Key](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/query/keys/trait.Key.html)
      = which types can be used as query args
        - generally some type of `DefId`

- Provider signatures
    - args: `tcx` and `key`
    - query result

- still don't know _when_ these things run. i guess they need to be called
  explicitly? but why are their "keys" only valid if ~DefIds? this implies some
  assumed granularity
- indeed, need to be explicitly called (I guess registering them just allows
  them to be callable?)

- example: inline.rs calls `cross_crate_inlinable`
    - arg: a callee's `def_id()`
    - in a function called `check_callee_mir_body()`
    - in impl block (`impl Inliner for NormalInliner`)

    - ultimately called from `run_pass` in `MirPass` impl block

    - `try_inline` does actual inlining (i guess cross-function) - what does
      this look like?
        - gets a callee (`Instance` ?) def
            - remember
              [InstanceKind](https://doc.rust-lang.org/beta/nightly-rustc/rustc_middle/ty/enum.InstanceKind.html)
              ? (e.g. `VTableShim`)
            - needs to make sure the callee's MIR is available
            - also checking for query cycles apparently

i still don't think this achieves the cross-function analysis we're looking for

quick google search
- AI overview
    - per-function, flow-sensitive analysis: `rustc_mir_dataflow`
    - external tools for cross-function analysis
        - MirChecker?
            - flow-sensitive security analysis
        - FFIChecker
            - analysis at LLVM IR level
    - Miri
        - how does Miri work?




### Things that might be important

https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/enum.InstanceKind.html
- `VTableShim`
- `ReifyShim`
- `FnPtrShim`
- `Virtual`

- from
  [MirSource](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/struct.MirSource.html)


`type_of` query given some `DefId`
- how to get `DefIds` from the Mir body?

since query results are memoized, can the cache ever be invalidated? i.e. if you
change the MIR in some way that requires re-computation? would _would_ vs 
_wouldn't_ require re-computation?


"we clone the return value out of the cache and return it (therefore, you should
try to ensure that the return types of queries are cheaply cloneable; insert an
Rc if necessary)."
- https://rustc-dev-guide.rust-lang.org/query.html#how-the-compiler-executes-a-query
