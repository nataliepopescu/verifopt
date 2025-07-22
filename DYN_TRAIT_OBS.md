# Dyn Trait Observations

Goal: gain empirical confidence for the potential effectiveness of verifopt's
technique on real world code

## grep/regex tool(s)

flow-insensitive
1. grep for all `dyn` + remember which trait name the `dyn` is bound to
2. then grep for all `impl <trait-name> for...` instances to get a CHA count
(total number of possible implementations)
3. gets more complicated when usages are exported to external apps/libraries
	- currently doing this by hand just to get a better understanding

flow-sensitive
- would need to happen in the compiler, i'd think...
- essentially very similar to the structure of the interpreter we are building,
  except no rewriting, just accumulating info

### initial results

from flow-insensitive tool, in the top 50 (most downloaded) crates, we see one of:

Group 1: (23) no `dyn Trait`s

Group 2: (15) `dyn Trait`s where the `Trait`s are _not_ implemented in the current crate
- would probably make more sense to look at full _projects_ rather than 
  individual crates

Group 3: (5) `dyn Trait`s where each `Trait` is only implemented _once_ in the current crate

Group 4: (7) `dyn Trait`s where at least one `Trait` is implemented more than once in
  the current crate

### categories / some synthesis

note some intersections between want/don't want filings

valid arg types (excluding generic constraints/closure args):
- Group 2: 5
- Group 3: 37 (all cc crate)
- Group 4: 27

valid return types: 
- Group 2: 21
- Group 3: 15
- Group 4: 17

compiled crates list (to search app deps for):
- serde_json (2-4; 3 total)
- once_cell (1-2; 2 total)
- cc (1-37; 37 total) (build-time dep, though)
- log (1, 6-7, 9, 15-26, 30-32, 34-39, 42-43; 27 total)

- log (4, 8, 10; 3 "unsure")
    - `key_values` func
    - `set_logger` func
    - `set_boxed_logger` func
- aho-corasick (6; 1 "unsure")
    - `build` func
- time (1-13; 13 "unsure")
    - `shrink` func
    - `std::error::Error::source` func
- base64 (1 "unsure")
    - `std::error::Error::source` func
- thiserror (all but really just 2, 4, 6, 8, 10; 5 "unsure")
    - `as_dyn_error` func
- rand_core (1)
    - `std::error::Error::source` func

#### want

- struct method arg type
    - Group 2 (8 - 7 = 1)
        - hashbrown 1-5 (unsafe)
        - parking_lot (cold)
        - *serde_json 4*, 6 (6=cold)
    - Group 3 (4)
        - cc 1-3, 6
    - Group 4 (3)
        - *log 5, 30-32*

- assoc func arg type (might have put some of these in above category)
    - Group 2
    - Group 3
    - Group 4 (2)
        - *log 42-43*

- function arg type
    - Group 2 (2)
        - *once_cell 1-2*
    - Group 3 (33)
        - cc 4-5, 7-37
    - Group 4 (4)
        - *log 1, 6-7, 9*

- trait decl/def arg types
    - Group 2 (3 - 1 = 2)
        - *serde_json 2-3*, 5 (5=cold)
    - Group 3
    - Group 4 (18)
        - *log 15-26, 34-39*

#### unsure

- struct method return types
    - Group 2 (1)
        - serde_derive
    - Group 3
    - Group 4 (4)
        - aho-corasick 6
        - log 4, 33

- function return type
    - Group 2 (2)
        - thiserror-impl 2-3
    - Group 3
    - Group 4 (1)
        - log 10

- trait decl/impl return types
    - Group 2 (18 - 1 = 17)
        - digest 1-2
        - time 1-13
        - serde_json 1 (unsafe)
        - base64 1
    - Group 3 (11)
        - thiserror 1-2, 4, 6, 8, 10 (old: 5-6, 8, 10, 12, 14)
        - rand_core 1
        - syn 2-3 (old: 8-9)
        - either 1-2
    - Group 4 (12)
        - indexmap 3-4
        - regex-automata 13, 16, 18, 27, 29-30, 32-33, 36-37

- trait impl for dyn trait (top-level line)
    - Group 2 (1)
        - digest 3
    - Group 3 (8)
        - thiserror 3, 5, 7, 9, 11-14 (old: 7, 9, 11, 13, 15-18)
    - Group 4 (2 - 1 = 1)
        - aho-corasick 7-8 (8=unsafe)

- generic type constraint, arg type
    - Group 2
    - Group 3
    - Group 4 (5)
        - itertools 6-7, 10-11
        - log 8

- closure arg type
    - Group 2
    - Group 3
    - Group 4 (2)
        - itertools 8-9

#### don't want

- in unsafe struct method
    - Group 2 (6)
        - hashbrown 1-5
        - serde_json 1
    - Group 3
    - Group 4

- `#[cold]`
    - Group 2 (3)
        - parking_lot
        - serde_json 5-6
    - Group 3
    - Group 4

- as a type / type cast / type assertion
    - Group 2 (3)
        - proc-macro2
        - rand_chacha
    - Group 3 (2)
        - syn 1, 4 (old: 7, 10)
    - Group 4 (22)
        - aho-corasick 1-5, 9-12
        - regex-automata 14-15, 17, 25, 28, 31, 34-35
        - log 2-3, 14, 40-41

- in tests

- unused funcs?

- `#[inline(always)]`?

------------------------------------------

### notes on more complicated uses ("unsure" category above)




#### serde_derive [unsolved]

```rust
pub fn all_fields(&'a self) -> Box<dyn Iterator<Item = &'a Field<'a>> + 'a> {...}
```

- getting all fields of `Data` (either a struct or enum)
- `iter()` on a `Vec` probably returns a `dyn` type
    - [iter](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter)
      returns an [Iter](https://doc.rust-lang.org/std/slice/struct.Iter.html)
      struct instance
    - `Iter` implements the `Iterator` trait
    - but why not just use a return type of `Iter` without `dyn`?
- `all_fields` is then used with iterator methods
    - `any`, a for loop, `all`, and `filter_map` in-crate

TODO how does verifopt help here?




#### aho-corasick

##### 6 [solved]

```rust
fn build_auto(
    &self,
    nfa: noncontiguous::NFA,
) -> (Arc<dyn AcAutomaton>, AhoCorasickKind) {
```

- builder pattern
- is `dyn` because the `Arc` can contain one of two (three) types: `DFA` or
  `NFA` (contiguous or noncontiguous)

- where is retval used (/ where is `build_auto` called)?
    - in `build` to construct an instance of aho-corasick automaton when
      self.kind == None (`new()` calls `default()`, from the derived `Default`
      trait, which, for `Option` types, turns out to be None, so the common case
      may indeed fall in this category)
        - triggers some automatic decision based on input patterns/other
          configurations (impl prioritizes DFA, then contig NFA, then noncontig NFA)
            - this indeed sounds like it hinges on dynamic information (config), and is 
              not a developer workaround for big generics? although I'm not
              completely sure
    - TODO who uses the aho-corasick crate??




#### log

##### 4 [solved]

```rust
pub fn key_values(&self) -> &dyn kv::Source {
    self.key_values.0
}
```

- `self` == `Record` with `key_values: KeyValues<'a>,`
- `struct KeyValues<'a>(&'a dyn kv::Source);`
- thus, returning a `dyn` value
    - but why does `KeyValues` need to be `dyn`?
    - `Source` is a trait, with several in-crate implementations ('BTreeMap', 
      'HashMap', 'Rc', 'Arc', 'Vec', 'Box', 'Option', 'OnePair')
        - visitor pattern
        - `fn visit<'kvs>(&'kvs self, visitor: &mut dyn VisitSource<'kvs>) -> Result<(), Error>;`

- where is `key_values()` used?
    - in tests :( - TODO who uses the `log` crate??
    - chained w `visit(&mut visitor)`, which has a `dyn` arg that we've already
      caught

##### 8 [solved]

```rust
fn set_logger_inner<F>(make_logger: F) -> Result<(), SetLoggerError>
where
    F: FnOnce() -> &'static dyn Log,
{
```

- `set_logger_inner` used in `set_logger` or `set_boxed_logger`, where `make_logger` is a
  closure that takes no arguments and returns some `dyn Log` thing
- see below (log 10) for implementors of trait `Log`

##### 10 [solved]

```rust
pub fn logger() -> &'static dyn Log {
    if STATE.load(Ordering::Acquire) != INITIALIZED {
        static NOP: NopLogger = NopLogger;
        &NOP
    } else {
        unsafe { LOGGER }
    }
}
```

- `static mut LOGGER: &dyn Log = &NopLogger;`
	- static muts are unsafe...
	- should be using a sync primitive
- if LOGGER was not mutated, the two branches are the same
	- possible LOGGER types: 
	- &'static dyn Log
	- Box::leak(Box<dyn Log>)
- how is `set_logger` used? will determine what kinds of `dyn Log` are used
    - various logging structs that impl the `Log` trait
    - in-crate we have `NopLogger`, `GlobalLogger`, `std::boxed::Box<T>`, and `std::sync::Arc<T>`
    - TODO who uses the log crate??

##### 33 [unsolved]

```rust
 pub fn to_borrowed_error(&self) -> Option<&(dyn std::error::Error + 'static)> {
     self.inner.to_borrowed_error()
 }
```

- in `impl Value` block
- `pub struct Value<'v> { inner: inner::Inner<'v> }`
	- inner = submod, Inner = enum
- cannot find `to_borrowed_error()` on Inner type...




#### thiserror-impl

##### 2 [unsolved]

```rust
let source_method = source_body.map(|body| {
    quote! {
        fn source(&self) -> ::core::option::Option<&(dyn ::thiserror::__private::Error + 'static)> {
            use ::thiserror::__private::AsDynError as _;
            #body
        }
    }
});
```

- a lot of macro stuff happening here, but `#body` is one of the following:

(A)

```rust
Some(quote_spanned! {transparent_attr.span=>
    ::thiserror::__private::Error::source(self.#member.as_dyn_error())
})
```

- where `#member` ~= `&input.fields[0].member`

(B)

```rust
Some(quote! {
    ::core::option::Option::Some(#dyn_error)
})
```

- where `#dyn_error` ~= `&source_field.member.as_dyn_error()`

(C)

```rust
None
```

(END)

- similarities: `Option< something.as_dyn_error() >` (from `thiserror::__private::AsDynError`)
	- trait func that returns `dyn Error`

```rust
pub trait AsDynError<'a>: Sealed {
    fn as_dyn_error(&self) -> &(dyn Error + 'a);
}
```

- why is it defined this way? TODO
	- in `thiserror-2.0.12/src/aserror.rs`

- TODO who uses thiserror-impl crate?

##### 3 [unsolved / ref above]

- similar to above, while above was for structs this one is for enums
	- constructing a match statement/arms

- arms result in one of following: 

(A)

`::thiserror::__private::Error::source(transparent.as_dyn_error())`

(B)

`::core::option::Option::Some(&source_field.member.as_dyn_error())`

(C)

`None`

(END)




#### digest [unsolved]

##### 1

```rust
pub trait DynDigest {
    fn box_clone(&self) -> Box<dyn DynDigest>;
}
```

- trait declaration

##### 2

```rust
fn box_clone(&self) -> Box<dyn DynDigest> {
    Box::new(self.clone())
}
```
calls `Box::new(self.clone())`, which digest 3 implements

##### 3

```rust
impl Clone for Box<dyn DynDigest> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
```

- how is this not circular...?
	- `self.clone()` (digest 2) calls `self.box_clone()` (digest 2), which calls `self.clone()` (digest 2) again





#### time

##### 1-9 [solved]

```rust
fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {...}
```

- impl `shrink` function from `Arbitrary` trait declared in [quickcheck crate](https://crates.io/crates/quickcheck)
	- default impl returns an empty iterator

return types
- 1: `FlatMap`
- 2-7: `Map`
- 8-9: `empty_shrinker()` => `Box::new(empty())` (boxed `Empty`), OR 
  `single_shrinker(value)` => `Box::new(once(value))` (boxed `Once`)

- TODO where is `shrink` used??

##### 10-13 [solved]

- all implementing `std::error::Error` trait
    - `fn source(&self) -> Option<&(dyn std::error::Error + 'static)>;`

- 10: for `Format` enum
    - `Box<error::ComponentRange>` or `io::Error`
- 11: for `TryFromParsed` enum
    - `error::ComponentRange`
- 12: for `Error` enum (wraps `Format` and `TryFromParsed` e.g.)
- 13: for `Parsed` enum (wraps `TryFromParsed`)

- TODO where is `std::error::Error::source` used??





#### base64 [solved]

- also implementing `std::error::Error` trait
    - `fn source(&self) -> Option<&(dyn std::error::Error + 'static)>;`

- TODO where is `std::error::Error::source` used??





#### thiserror [solved]

##### 1

```rust
pub trait AsDynError<'a>: Sealed {                                                                                                                                      
    fn as_dyn_error(&self) -> &(dyn Error + 'a);                                                                                                                        
}
```

- trait declaration

##### 2, 4, 6, 8, 10

- impl above `as_dyn_error` func for `Error` + various other trait + lifetime bounds
  (Send, Sync, UnwindSafe, and 'a)

- TODO who uses `as_dyn_error` ?
    - thiserror-impl at least

##### the rest

- impl Trait (`AsDynError` and `Sealed`) for `dyn Error` (+ various other trait 
  + lifetime bounds)





#### rand_core [solved]

##### 1

- also implementing `std::error::Error` trait
    - `fn source(&self) -> Option<&(dyn std::error::Error + 'static)>;`

- TODO where is `std::error::Error::source` used??





#### syn

##### 2

```rust
trait IterTrait<'a, T: 'a>: Iterator<Item = &'a T> + DoubleEndedIterator + ExactSizeIterator {                                                                         
    fn clone_box(&self) -> Box<NoDrop<dyn IterTrait<'a, T> + 'a>>;                                                                                                     
}
```

##### 3

- impl above trait func `clone_box`, body = `Box::new(NoDrop::new(self.clone()))`









------------------------------------------

### patterns / solutions

[rust design
patterns](https://rust-unofficial.github.io/patterns/patterns/index.html)
- strategy pattern (in rust) can be simply replaced by traits

visitor pattern
- i.e. traversing AST (as the toy interpreter does)
- for heterogenous data (vs iterator which operates on homogenous data)
- good for separating collection traversal from the operations performed on each object

builder pattern

iterator pattern

wrapper pattern

------------------------------------------

### individual use notes

TODO actually don't want `dyn` return types (unless double counting `dyn` arg
types)?

#### Group 2

valid: 
- [x] serde_derive (1)
    - return type
    - `dyn Iterator`
    - retval is chained w iterator methods (.all(), .filter_map(), etc)
    - `attrs` field is also accessed on the retval
        - i believe this is irrelevant? since not going through vtable ptr, but
          instead going through struct pointer
- [x] hashbrown (5)
    - are these irrelevant/"invalid" for us since in unsafe funcs? TODO
    - 1-2
        - arg type
        - `dyn FnMut`
        - unsafe method
        - struct impl block (`RawTableInner`)
    - 3-5
        - arg type
        - `dyn Fn`
        - unsafe method
        - struct impl block (`RawTableInner`)
- [x] proc-macro2 (1)
    - in function body
    - `type PanicHook = dyn Fn(&PanicInfo) + Sync + Send + 'static;`
    - does not seem to be called, but rather used in pointer comparison?
- [x] digest (3)
    - 1
        - return type
        - `Box<dyn DynDigest>`
        - in `DynDigest` trait decl block
    - 2
        - return type
        - `Box<dyn DynDigest>`
        - in `DynDigest` trait impl block
    - 3
        - trait impl for dyn trait (`impl Clone for Box<dyn DynDigest> {...}`)
- [x] time (13)
    - 1-9
        - return type
        - `Box<dyn Iterator<Item = Self>>`
        - in impl trait for struct
    - 10-13
        - return type
        - `Option<&(dyn std::error::Error + 'static)>`
        - in impl trait for enum
- [x] rand_chacha (1)
    - type cast (`let rng2 = &mut rng1.clone() as &mut dyn CryptoRng;`)
- [x] once_cell (2)
    - 1-2
        - function arg
- [x] thiserror-impl (3 -> 2)
    - 1
        - in an error message (can maybe update tool for this)
    - 2-3
        - return type
        - in `quote!` block
        - in (non-assoc) function
- [x] parking_lot (1)
    - method arg
    - `#[cold]` attribute
    - in struct impl block
- [x] serde_json (6)
    - 1
        - return type
        - impl Trait for Struct
    - 2-3
        - arg type
        - impl Trait for Struct
    - 4
        - struct method arg type
        - struct impl block
    - 5
        - arg type
        - impl Trait for Struct
        - `#[cold]` attribute
    - 6
        - arg type
        - struct impl block
        - `#[cold]` attribute
- [x] base64 (1)
    - 1
        - return type
        - in impl Trait for Enum block
- [x] clap (1 -> 0)
    - func return type
	- in examples

invalid: 
- [x] regex-syntax (1 -> 0)
    - line starts with '#' / is an example, not part of the implementation
- [x] regex (1 -> 0)
    - line starts with '#' / is an example, not part of the implementation
- [x] memchr (3 -> 0)
    - 1-3
        - in `src/tests`

#### Group 3

- [x] thiserror (18 -> 14)
    - 1-4
        - tests
    - 5-6, 8, 10, 12, 14
        - trait method ret type
    - 7, 9, 11, 13, 15-18
        - impl trait for dyn trait (`Error` == trait)
- [x] rand_core (3 -> 1)
    - 1
        - ret type
        - impl trait for struct
    - 2-3
        - test nested funcs
        - arg type
- [x] syn (10 -> 4)
    - 1-6
        - in tests
    - 7, 10
        - struct field type
    - 8
        - in trait decl
        - trait method return type
    - 9
        - in impl trait for type block
        - trait method return type
- [x] cc (37) - note, cc is a _build-time_ dep
    - 1-3
        - struct impl block
        - method arg type
    - 4-5
        - function arg type
    - 6
        - in struct impl block (nested in cfg module)
        - method arg type
    - 7-37
        - function arg type (nest in cfg module)
- [x] either (2)
    - 1-2
        - in impl trait for enum block
        - trait method return type

#### Group 4

- [x] bytes (10 -> 0)
    - 1-5
        - tests
    - 6-8
        - benchmarks
    - 9-10
        - unused funcs?
        - comment: The existence of this function makes the compiler catch if
          the Buf trait is "object-safe" or not.
- [x] rand (3 -> 0)
    - 1-3
        - in tests
- [x] indexmap (4 -> 2)
    - 1-2
        - in tests
    - 3-4
        - impl trait for type
        - method return type
- [x] aho-corasick (12)
    - 1-4, 9, 12
        - struct field type
    - 5, 10-11
        - match statement return type
    - 6
        - struct impl block
        - method return type
    - 7
        - impl trait for dyn trait
    - 8
        - impl trait for dyn trait
        - unsafe
- [x] regex-automata (39 -> 18)
    - 1-7, 12 (8)
        - in tests
    - 8-11, 19-24, 26, 38-39 (13)
        - line starts with "#"
    - 13, 16, 18, 27, 33, 36-37
        - impl trait for type
        - method ret type
    - 14, 35
        - field type
    - 15
        - match statement return type
    - 17
        - in macro def
        - func ret type
    - 25, 31
        - type assertion
    - 28
        - function ret type
    - 34
        - type
        - boxed func type where arg == `dyn Fn`
    - 29-30, 32
        - in struct impl block
        - assoc func ret type
- [x] itertools (11 -> 6)
    - 1-5
        - in tests
    - 6-7, 10-11
        - generic type constraints
        - dyn in the arg type portion
        - in function signature
    - 8-9
        - closure argument
- [x] log (43 -> 37)
    - 11-13, 27-29
        - in tests
    - 1, 6-7, 9
        - function arg type
    - 15, 34-35
        - trait decl func arg
    - 16-26, 36-37
        - trait def func arg
    - 5, 30-32
        - struct impl block
        - method arg type
    - 38-39
        - trait def func arg
        - nested
    - 42-43
        - nested mod
        - assoc func arg type
    - 10
        - function ret type
    - 4, 33
        - struct impl block
        - method return type
    - 8
        - generic type constraints
        - dyn in the ret type portion
        - in function signature
    - 2
        - type assertion
    - 3
        - struct field type
    - 14
        - type
    - 40-41
        - enum type

