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

### Casts


### Place Projections

ignoring for now

### Loops


