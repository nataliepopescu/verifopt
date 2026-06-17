# Debugging VerifOpt, the Monomorphized Version

## When we were collecting max constraints

### Getting a Scalar/Array when want a callable item

scope "getrandom::backends::linux_android_with_fallback::fill_inner::{closure#0}"
- terminator of BB2
- local3
    - [Scalar(0), Array(Ty { id: 11, kind: RigidTy(Uint(U8)) })]
- local8: Assign(_3, Use(Copy((*_8))))
- local1 (arg): Assign(_8, Use(Copy(((*_1).0: &unsafe extern "C" fn(*mut std::ffi::c_void, usize, u32) -> isize))))
    - deref
    - field 0

type of local1 should be a closure

the constraints we pass in are way off
- the next arg is a closure?
- why is the first arg an array?

https://doc.servo.org/src/getrandom/backends/linux_android_with_fallback.rs.html#79-101

- are the scopes correct? that might be messed up...

- when/where is call_stack/cur_scope updated/changed?
    - run (very beginning)
    - interp_closure
    - interp_static_call
    - simulate_static_calls

second arg is array but seems to be interp as ptr?
- found use of slice_as_uninit_mut fn
    - actually, not a ptr yet
- aha: std::ptr::from_mut::<[u8]>

### Casting

#### Transmute

Cast(
    Transmute,
    Copy(_6),
    Ty: { .. kind: {
        RigidTy(Uint(Usize))
    }}
) -> _14

this is a pointer to integer cast
- getting the address of a pointer

what is _6?
- Ty: RawPtr(Adt(Rc), [UnsafeCell(BlockRng(ReseedingCore)), Global])
- set: Call(_7, const args)

- indirect call: no candidates (maybe a shim) -> retty fallback

- reinterpret this as a usize...?


## Only collecting Adt (mostly) constraints

### one_variant never getting to get_animal call

main bbs ordering: [0, 1, 3, 26, 18, 19, 20, 22, 23, 24, 21]

visually: 
- bb0
    - args()

- bb4
    - get_animal()

- why filtered?

CLEANUP filter was wrong apparently (idk why)

new order: [0, 16, 17, 2, 3, 1, 18, 19, 4, 5, 20, 6, 7, 8, 9, 10, 12, 13, 14,
11]

visiting: 0, 16, 17, 2, 3, *10, 14, 11*

switch statement weirdness

### Getting a Scalar when want a callable item

in LocalKey::try_with
- triggered via minimal example in shims/
- and rand_/ example (same func)

how did we get here (last -> first)
- term func: Copy _7
- assign: Copy *_1.0



### default impls

`impl<E> StdError for E` ...
`let backtrace = backtrace_if_absent!(&self); `

might be default

but also may not have collected constraints b/c macro stuff makes this weird?
- or some other reason, i think casting
- fixed casting but want to make sure haven't erased any important information
  about which precise error this is

tracing backwards:
- in parse()
- in context()
    - move(_5) -> first arg of ext_context
    - currently: Constraint { toc, None, cfc: Some(Adt(Result,
      Genargs(Option(lexopt::Args), lexopt::Error))) }
      - log.md line 222205
- in ext_context()
    - move(_5) -> first arg of request_ref_backtrace
    - currently: also dyn (see below)
      - log.md line 222360
- in request_ref_backtrace
    - move(_1) -> first arg of request_by_type_tag
    - currently: Constraint { toc: None, cfc: Some(Dyn(Error)) }
        - info was lost here
        - when do we set this var / when is the non-dynamic value overridden?
- dynamic dispatch site (provide()) in request_by_type_tag
    - self should be in _1,

casting op
- log.md line 222334
- in ext_context, but really the backtrace! macro
- /home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/backtrace.rs: 41:54 41:85
- yep, `$err as &dyn core::error::Error`
- explicit, source level cast






### traitobj generic args

testing-examples/generic_traitobj

main: [0, 7, 6, 8, 2, 5, 1]

bb7





### fixed "default" impl in ripgrep / wrong Self arg (first dyn dispatch)

get FSA constraints
- traitobj constraints seem correct
- TraitObjTy: Error
- TraitObjConstraints: Result<Option<lexopt::Arg>, lexopt::Error>

resolve defid / ADT helper for Result
- ADT: Result
    - genarg 1: Option
        - resolve defid / ADT helper for Option
            - ADT: Option
                - genarg 1: lexopt::Arg
                    - resolve defid / ADT helper for lexopt::Arg
                        - ADT: lexopt::Arg
                            - genarg 1: Lifetime (skip)

calling default provide is correct, but now know that got the correct self
constraints (lexopt::Error) and when those constraints impl the trait Error they
do not override the provide impl

### FnMut genargs mismatch?

fallback to CHA

which genargs are correct? where to get them from?

actually, why are we falling back to CHA here?
- why can we not retain the precise eq info? isn't that assigned somewhere?

TRACE:

find_short ~/hack/verifopt-examples/ripgrep/crates/core/flags/parse.rs:298
- find flag corresponding to command-line arg byte in map (FlagMap =
  HashMap<Vec<u8>, usize>) populated from static FLAGS array
- `self.map.find(&[byte])`
    - what is `&[byte]`'s constraint?
        - byte = Result<u8, TryFromCharError> (b/c didn't apply unwrap on constraints/type)
        - TODO why can't handle unwrap / other funccalls like casts, in terms of
          preserving traitobj constraints across type changes?
        - TBD if this is a problem we need to solve now or not

        - then [byte] = List<Scalar(None)>

        - then pointer coercion (Array -> Slice)

        - then call find()
            - arg 0 (self.map)
                - Result<Parser<Never>>,
                - but arg ty is a FlagMap, so how did we get here?
                    - self == Parser
                    - self.map == FlagMap (I guess we can just trust that this
                      is here)
            - arg 1 (&[byte])
                - Idk(Scalar(None)) (fine)
                - arg ty is a Slice(u8)

FlagMap::find: ~/hack/verifopt-examples/ripgrep/crates/core/flags/parse.rs:394
- `self.map.get(name)`
- same args

HashMap::get:
/home/np/.rustup/toolchains/nightly-2026-01-13-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:1002
- `self.base.get(k)`
- (inlined into find, same args)

HashMap::get: /rust/deps/hashbrown-0.16.1/src/map.rs:1311
- if table is not empty, make hash, then `match self.table.get(hash, equivalent_key(k)) { ... }`
- after return from hash_one... [skipping hash fns here]
- hash = Scalar(None)

equivalent_key() (same file as above, line 224, inlined apparently via debugging
info)
- retval = impl Fn(..) -> bool
- returns a big ol closure
    - ClosureDef(closure#0)
    - GenericArgs(
        Slice(u8),
        Vec(u8, Global),
        usize,
        i8,
        FnPtr(
          inputs: Tuple(
            Ref(Tuple(
              Vec(u8, Global),
              usize
            ))
          ),
          output: Bool,
        ),
      )
    - ok, sure

RawTable::get: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:1251

RawTable::find: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:1234
- args
    - arg 0 (self)
        - Result(Parser)
    - arg 1 (hash)
        - Scalar
    - arg 2 (eq)
        - Closure(...) (above)
- self.table should be RawTableInner type but is Result<Parser<..>> still

- creating another closure  (_8)
    - arg (old closure) == eq
    - new closure == &mut |index| eq(self.bucket(index).as_ref())

- ref
- cast into FnMut!! finally
    - this cast fails! no resulting constraints
    - *FIXME*
    - problem: implicit Fn* trait implementations are not registered in our tool

RawTableInner::find_inner: /rust/deps/hashbrown-0.16.1/src/raw/mod.rs:2075
- args
    - arg 0 (self)
        - Result(Parser) still
    - arg 1 (hash)
        - Scalar(None)
    - arg 2 (eq, traitobj!)
        - [] :(
- some low level instructions [skipping]
- binary ops => Scalar(None) (line 2073)
- line 2075: `if likely(eq(index)) { ... }`
    - FnDef("call_mut")
    - GenericArgs(
        dyn FnMut(GenericArgs(
          usize,
        )),
        FnOnce::Output(usize),
        term=Bool,
      )

- which call_mut do we actually want?
    - many FnMut implementors are implicit - are we collecting all the ones we
      should be??
    - also, why are FSA constraints empty??
        - b/c the arg constraints were empty!! need to fix :)

implicit closure Fn* trait impls
- either find where the compiler generates these impls/shims or simulate them
  ourselves
- i guess we're rerouting the call_mut to the closure body? no need to simulate














