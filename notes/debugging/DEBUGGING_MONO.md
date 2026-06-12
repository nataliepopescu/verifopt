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

### when dyn call, constraints do not contain traitobj

trying to trace backwards


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




















