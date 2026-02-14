# Analysis Notes / Hang-ups

## During Interpretation

while trying to get fine-grained information/constraints for every possible
variable/"place"

### Box: a special type?

i.e. when we get to the speak() call, the first (self) arg is a Box that happens
to be parameterized by a Cat. should Box be special such that the Cat part is
the salient piece of information?

- maybe this isn't always the case though

### Dyn Dispatch Resolution

path:
- speak
- speak is dyn: which speak impl?
- well what is self type? Cat
- so, call Cat's speak
- what actually is Cat's speak?
- Cat has impl block {impl#1}
- impl block {impl#1} implements {impl#1}::speak
- found speak impl!

have: 
- speak -> {{impl#1}::speak, {impl#2}::speak}
- Animal -> {Cat, Dog}
- {impl#1} -> Cat

need:
- 1: [x] Cat -> {impl#1}
- 2: [x] {impl#1} -> {impl#1}::speak

- how to assoc Cat w {impl#1}::speak?
    - can do some trait math, like which trait are we implementing, how many
      methods does that trait have, is this trait within that defid range?
    - jk, found a method that does step 2

### Generics

e.g. Box::new()
- takes in 1 arg: T
- when returning the Box object from this function, want to know what T is
- Box == Struct - try to track Struct generics

- can get the T generic for Box
    - how to map T to the concrete Cat that we passed in?

- Box::new locals
    - 0: Box<T, Global>
    - 1: T
    - 2: *mut u8
    - 3: *const T
    - 4: NonNull<T>
    - 5: Unique<T>

    - local 1 is only used _after_ the box is constructed
        - (*_3) = move _1
    - so this is the only place where we could theoretically resolve T
        - since at this point the type system says T while our constraints say
          Cat
        - maybe somehow we can backwards propagate?
        - / just search in our cmap scope?
            - T would have to be explicit in our map
        - this ordering is inconvenient tho :(
    - actually, no, we should be able to resolve it during argument setup
        - indeed

    - then, when the object depending on the generic is _constructed_, the
      resulting VerifoptRval should not just rely on the DefId but also any
      instantiated generics

### Const strings

not really sure how to represent
- https://doc.rust-lang.org/beta/nightly-rustc/rustc_middle/mir/enum.ConstValue.html#variant.Slice
- or how to interpret the alloc_id / meta fields
- like, is alloc_id a memory address? or some random identifier? 

maybe just "IdkString"

### Casts

if our constraints are "idk", can naively just change the VerifoptRval

but gets tricky when we actually have constraints
- may need to implement some special conversions, but we shall see if/when this
  happens!

### Place Projections

ignoring for now

### Loops

### Return Position Generics

collect() has been giving me some grief

here is the signature:

```
fn collect<B>(self) -> B
where
    B: FromIterator<Self::Item>,
    Self: Sized,
```

from the docs,
(https://doc.rust-lang.org/beta/std/iter/trait.Iterator.html#method.collect),
collect can cause issues with type inference due to its generality, so
programmers often need to specify the type to collect into. 

ok, so, we have a return type that is specified in a different way. fine. i
coded some infrastructure to handle that. but then things get a little more
complicated. 

where this shows up in our example is this (first) line in `main`: 

```rust
let args: Vec<String> = std::env::args().collect();
```

let's go through main's MIR and see what happens. 

- bb0
    - _2 = std::env::args();
        - no MIR for args()? why?
        - not intrinsic

        - source:
```rust
pub fn args() -> Args {
    Args { inner: args_os() }
}

pub fn args_os() -> ArgsOs {
    ArgsOs { inner: sys::args::args() }
}
```
        - seems system-specific, so fine that we don't have MIR for it, all we
          need to know is that we get some Args back

        - and indeed, verifopt determines the following retval: 
          `{IdkDefId(DefId(1:2229 ~ std[a5f4]::env::Args))}`

- bb1
    - _1 = <Args as Iterator>::collect::<Vec<String>>(move _2)
        - function DefId = 2:9860
        - one GenericParam: "B" (DefId 2:9861)
        - 1 arg: "Self"

        - this is already a bit confusing, b/c the function signature has two
          generic parameters, but i guess only "B" shows up in the type
          signature ("Self" parameterizes "B")

        - but also "Self" is the function argument, so in this way it does
          "show" up in the type signature...
        
        - how to get the link between "B" and "Self"?

        - i guess one way to get the link is that collect()'s retval goes into
          _1, and _1 has type "Self"
            - we may not actually even be using "B" at all?
            - but let's see
    
        - verifopt stores 
          "B": Some(Values({IdkType(std::vec::Vec<std::string::String, std::alloc::Global>)}))

// step into collect() impl

- bb0
    - _0 = <B as FromIterator<<Self as Iterator>::Item>::from_iter::<Self>(move
      _1)
        - _1 is our Args argument
        - what are B or Self?

        - mir not available for from_iter (DefId 2:9608)
            - it is a trait fn
            - many possible implementations...
            - which one do we want?

            - FIXME later resolve if needed

        - from_iter source decl:

```rust
pub trait FromIterator<A>: Sized {
    // Required method
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}
```

### Two invocations of same function have locals w different types?

specifically `DefId(2:37072 ~
core[c945]::ptr::non_null::{impl#3}::new_unchecked::precondition_check)`

first call is from defid 3:131 bb7
- DefId(3:131 ~ alloc[7a7c]::alloc::{impl#0}::alloc_impl_runtime)

second is from same defid but bb16


WTO for 3:131

bb0 -> bb1, bb2

bb2 -> *bb7*, bb9

*bb7* -> bb8 -> bb9 -> bb6 -> return


bb1 -> bb3, bb4

bb3 -> bb10 -> bb11 -> bb5

bb5 -> bb14, bb15

bb14 -> bb6 -> return

bb15 -> *bb16*, bb18

*bb16* -> bb17 -> bb18

bb18 -> bb19, bb20

bb19 -> bb20 -> bb21 -> bb6 -> return


so indeed, both paths are valid / not cleanup, but
1. we don't want to override anything in the cmap since that is precious
   information!
    - we should try to merge these things together
2. why do the constraints for Place(1) have different _types_??
    - local 1 has type `*mut ()*
    - first invocation: `std::ptr::Alignment`
        - from:
            - ...
            - _1 == first arg of two (std::alloc::Layout)
            - _15 = copy (_1.1: std::ptr::Alignment),
            - _14 = copy _15 as std::num::NonZero<usize> (Transmute)
            - _20 = copy _14 as *mut () (Transmute)
                - local14 has type std::num::NonZero<usize>
            - move _20
                - local20 has type `*mut ()`
    - second invocation: `*mut u8`
        - from: 
            - ...
            - _8 = alloc::alloc::__rust_alloc_zeroed(copy _3, move _24)...
                - returns a *mut u8
            - _36 = copy _8 as *mut () (PtrToPtr)
            - move _36
    - what is happening...
        - might have something to do with the casts/transmutes that i don't
          really know how to interpret yet


### Vec::new()

in vec::new() (defid 3:8506) (aka cur_scope)

bb0

```
_1 = alloc::raw_vec::RawVec::<T> { inner: move _2, _marker: const ZeroSized: std::marker::PhantomData<T> },
```

trying to get VerifoptRval from the above rval

aggregate kind: adt
- defid: 3:40
- fields: [move _2, _marker]
- genargs [T, std::alloc::Global]




















