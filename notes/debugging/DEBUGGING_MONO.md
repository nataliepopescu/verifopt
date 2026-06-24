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

### std::fmt::Write::write_str()

no generic args in signature

but missing a lifetime generic arg
- Formatter has a lifetime generic arg...

TRACE

Formatter::write_fmt
- signature: `pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result { ... }`
    - has an elided lifetime arg
    - also from body:
        - `impl<'a> Arguments<'a> { .. pub fn as_statically_known_str(&self) -> Option<&'static str> { ... } }`
        - same lifetime arg
- `self.buf.write_str`
- Fomatter { options, buf: &'a mut (dyn Write + 'a) }
    - one generic arg: 'a

however, we don't modify the generic args at all... so we wouldn't be removing
that lifetime arg from anyway

Write::write_str

spec_to_string (Display impl for SpecToString)
- buf = String::new
- formatter = Formatter::new(buf, options..)
    - when we create formatter, we lose the buf:String association
    - the eventual `self.buf.write_str` should run on String's write_str impl
        - DefId 40456
- Display::fmt(&self, &mut formatter)

when get to dynamic call, traitobj local is _41 which just seems to have the
following constraint: (toc: None, cfc: Some(Dynamic(Write)))
- maybe we can look at fields once hit a traitobj like this?
- how is _41 set? is it correctly Formatter, or did we mis-assign it and it
  should be Formatter.buf?
  - previous statement: `stmt: Assign(_41, Use(Copy(((*_2).1: &mut dyn
    std::fmt::Write))))`
        - scope: fmt
  - how is _2 set?
    - _2 refers to the second (Formatter) arg: `[Constraint { toc: None, cfc: Some((Location, Adt(AdtDef(DefId { id: 37903, name: "std::fmt::Formatter" }), GenericArgs([Lifetime(Region { kind: ReErased })])))) }]`

    - getting projection wrong? (the ty might be wrong?)
        - PROJ: Deref
        - PROJ: Field(1, 
            Ty { id: 24921, kind: RigidTy(Ref(Region { kind: ReErased }, Ty { id: 113266, kind: RigidTy(Dynamic([Binder { value: Trait(ExistentialTraitRef { def_ id: TraitDef(DefId { id: 229, name: "std::fmt::Write" }), generic_args: GenericArgs([]) }), bound_vars: [] }], Region { kind: ReErased })) }, Mut)) }
          ) 
        
    - made:
        - PROJ: Deref, 
        - PROJ: Field(1, 
            Ty { id: 24921, kind: RigidTy(Ref(Region { kind: ReErased }, Ty { id: 113266, kind: RigidTy(Dynamic([Binder { value: Trait(ExistentialTraitRef { def_id: TraitDef(DefId { id: 229, name: "std::fmt::Write" }), generic_args: GenericArgs([]) }), bound_vars: [] }], Region { kind: ReErased })) }, Mut)) }
          )

    - the projection looks correct??

    - expecting these constraints: `[Constraint { toc: Some((TraitObjTy { def: TraitDef(DefId { id: 229, name: "std::fmt::Write" }), genargs: GenericArgs([]) }, Adt(AdtDef(DefId { id: 29272, name: "std::string::String" }), GenericArgs([])))), cfc: Some((Location, Adt(AdtDef(DefId { id: 29272, name: "std::string::String" }), GenericArgs([])))) }]`

    - ah, we do not carry over projections when resolving args!

### Understanding/Tracking FnDef generic args

interp_fn_def()
- called in 2 places
    - interp_indirect_call -> interp_constraint_as_fn
        - uses RunningConstraintInner::FnDef(fndef, genargs)
    - interp_direct_call
        - uses RigidTy::FnDef(fndef, genargs)

- calls 3 fns
    - interp_static_call
    - interp_virtual_call
    - retty_from_polysig

For FnMut traitobj

For Write traitobj
- using genargs from write_fmt func?


### Debugging ripgrep `try_find_fwd` instance resolution error

"aho_corasick::automaton::Automaton::match_kind"
`let earliest = aut.match_kind().is_standard() || input.get_earliest();`

what is aut?
- TraitDef(Automaton)
- arg was `&self` in the `trait Automaton` block
- does that mean it is dyn, or that this is a static call?

Automaton is a trait

match_kind is a method on the Automaton trait
- but it does not have a default implementation
- so we cannot call it without a concrete Automaton type

however, there is a concrete impl of it in this block:
`unsafe impl<'a, A: Automaton + ?Sized> Automaton for &'a A { ... }`
- this seems like the impl we should be using, but how do we get there?

try_find_fwd has a generic param: `A: Automaton + ?Sized` 
- this dictates the type of `aut`

coming from the default implementation, `aut` is just `Automaton`, but when we
call `try_find_fwd` that generic type constraint makes it an `&A`, i.e. an
`&(Automaton + ?Sized)`
- check arg resolution!
- arg is an AcAutomaton - why?
    - _3

unsafe trait Automaton { ... }

pub struct AhoCorasick {
  aut: Arc<dyn AcAutomaton>,
  kind, 
  start_kind,
}

trait AcAutomaton: Automaton + Debug + ... {}

impl AcAutomaton for A where A: Automaton + Debug + ... {}

- according to comments, AcAutomaton == means for "practical" dynamic dispatch
  for Automaton objects
- wraps Automatons w helpful bounds

unsafe impl Automaton for Arc<dyn AcAutomaton> { ... }
- redirect to inside Arc, which should redirect to one of below impls

unsafe impl Automaton for NFA { ... } (contiguous + noncontiguous)
unsafe impl Automaton for DFA { ... }
- all 3 impls amount to: self.match_kind (field)


- from claude: "The blanket impl<A> AcAutomaton for A where A: Automaton + Debug
  + Send + Sync + ... means any concrete automaton struct automatically gets
    AcAutomaton for free."


think this is another default impl problem
- if we go down the default impl path for a type, we are still restricted to
  that type

either that, OR, we are not monomorphizing a default impl's instance


the problem in getting to the correct match_kind() impl is that we first pass
through the default impl of try_find() which erases our info
- well, we actually don't have any info to erase
- we actually just don't have a way to get to _any_ of the match_kind() impls
  from here it seems


### PrefilterI::find() Dyn Dispatch

/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/regex-automata-0.4.13/src/util/prefilter/mod.rs
find()

pub struct Prefilter {
  pre: Arc<dyn PrefilterI>,
  ...
}

pub(crate) trait PrefilterI {
  fn find(..);
  ...
}

impl PrefilterI for [ByteSet, Memchr, Memchr2, Memchr3, Memmem, Teddy, AhoCorasick] { ... }

when is the field set?
- when is Prefilter created?

- WHITESPACE_ANCHORED_FWD = Lazy<DFA>
    - closure uses DFA::from_bytes
        - dfa/dense.rs: 2340
    - static slice, too

- when have WHITESPACE_ANCHORED_FWD.method()
    - go through Lazy::get
    - which calls an atomic_load (via self.data.load())
            - self.data = AtomicPtr<_>
            - @ new() -> data: AtomicPtr::new(core::ptr::null_mut()),
        - this checks initialization
        - if not init, ret None
        - if init, ret Some(init_val)

    - so, first time called, will return None and thus will need to set by
      calling the closure (containing DFA::from_bytes)

    - the closure doesn't seem to be getting called correctly?
        - yeah from_bytes is never called
        - somehow the relevant instance doesn't have a body?
        - where/how is it resolved?

        - first arg is _4
            - _4 = Ref(Region { kind: ReErased }, Shared, ((*_1).1: fn() -> regex_automata::dfa::dense::DFA<&[u32]>))

            - _1 = resolved arg constraints: [Constraint { toc: None, cfc: Some(Adt(AdtDef(DefId { id: 47597, name: "regex_automata::dfa::dense::DFA" }), GenericArgs([Type(Ty { id: 84111, kind: RigidTy(Ref(Region { kind: ReErased }, Ty { id: 2091, kind: RigidTy(Slice(Ty { id: 589, kind: RigidTy(Uint(U32)) })) }, Not)) })]))) }]
                - no field projections...
                - even though it definitely has fields

                - why??
                - from _14 in enclosing

        - in Lazy::get()
            - _14 = Cast(PtrToPtr, Copy(_15), Ty { id: 114425, kind: RigidTy(RawPtr(Ty { id: 114428, kind: RigidTy(RawPtr(Ty { id: 114420, kind: RigidTy(Adt(AdtDef(DefId { id: 47597, name: "regex_automata::dfa::dense::DFA" }), GenericArgs([Type(Ty { id: 84111, kind: RigidTy(Ref(Region { kind: ReErased }, Ty { id: 2091, kind: RigidTy(Slice(Ty { id: 589, kind: RigidTy(Uint(U32)) })) }, Not)) })]))) }, Mut)) }, Not)) })

            - _15 = AddressOf(Const, (((*_1).0: std::sync::atomic::AtomicPtr<regex_automata::dfa::dense::DFA<&[u32]>>).0: std::cell::UnsafeCell<*mut regex_automata::dfa::dense::DFA<&[u32]>>))

            - _1 = resolved arg constraints: [Constraint { toc: None, cfc: Some(Adt(AdtDef(DefId { id: 48197, name: "regex_automata::util::lazy::lazy::Lazy" }), GenericArgs([Type(Ty { id: 114420, kind: RigidTy(Adt(AdtDef(DefId { id: 47597, name: "r        egex_automata::dfa::dense::DFA" }), GenericArgs([Type(Ty { id: 84111, kind: RigidTy(Ref(Region { kind: ReErased }, Ty { id: 2091, kind: RigidTy(Slice(Ty { id: 589, kind: RigidTy(Uint(U32)) })) }, Not)) })]))) }), Type(Ty { id: 11        4421, kind: RigidTy(FnPtr(Binder { value: FnSig { inputs_and_output: [Ty { id: 114420, kind: RigidTy(Adt(AdtDef(DefId { id: 47597, name: "regex_automata::dfa::dense::DFA" }), GenericArgs([Type(Ty { id: 84111, kind: RigidTy(Ref(Re        gion { kind: ReErased }, Ty { id: 2091, kind: RigidTy(Slice(Ty { id: 589, kind: RigidTy(Uint(U32)) })) }, Not)) })]))) }], c_variadic: false, safety: Safe, abi: Rust }, bound_vars: [] })) })]))) }]
                - from _11 in enclosing

        - in whitespace_len_fwd
            - _11 = Ref(Region { kind: ReErased }, Shared, ((*_7).0: regex_automata::util::lazy::lazy::Lazy<regex_automata::dfa::dense::DFA<&[u32]>, fn() -> regex_automata::dfa::dense::DFA<&[u32]>>))

            - _7 = Use(Constant(ConstOperand { span: Span { id: 6, repr: "no-location" }, user_ty: None, const_: MirConst { kind: Allocated(Allocation { bytes: [Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)], provenance: ProvenanceMap { ptrs: [(0, Prov(AllocId(357, ThreadLocalIndex)))] }, align: 8, mutability: Mut }), ty: Ty { id: 114410, kind: RigidTy(Ref(Region { kind: ReErased }, Ty { id: 114419, kind: RigidTy(Adt(AdtDef(DefId { id: 48195, name: "regex_automata::util::lazy::Lazy" }), GenericArgs([Type(Ty { id: 114420, kind: RigidTy(Adt(AdtDef(DefId { id: 47597, name: "regex_automata::dfa::dense::DFA" }), GenericArgs([Type(Ty { id: 84111, kind: RigidTy(Ref(Region { kind: ReErased }, Ty { id: 2091, kind: RigidTy(Slice(Ty { id: 589, kind: RigidTy(Uint(U32)) })) }, Not)) })]))) }), Type(Ty { id: 114421, kind: RigidTy(FnPtr(Binder { value: FnSig { inputs_and_output: [Ty { id: 114420, kind: RigidTy(Adt(AdtDef(DefId { id: 47597, name: "regex_automata::dfa::dense::DFA" }), GenericArgs([Type(Ty { id: 84111, kind: RigidTy(Ref(Region { kind: ReErased }, Ty { id: 2091, kind: RigidTy(Slice(Ty { id: 589, kind: RigidTy(Uint(U32)) })) }, Not)) })]))) }], c_variadic: false, safety: Safe, abi: Rust }, bound_vars: [] })) })]))) }, Not)) }, id: MirConstId(1630, ThreadLocalIndex) } }))
                - Lazy<DFA<[u32]>, FnPtr<DFA<[u32]>>
                    - FnPtr has no inputs


going through:
    - Ref
    - AddressOf
    - Cast














