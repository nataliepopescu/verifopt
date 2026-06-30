# VerifOpt Design

## Overview

Dynamic dispatch exists to solve the problem of not knowing which concrete type
a trait object might be at runtime, and thus not knowing which concrete
implementation of a function to call. 

Abstractly interpreting a program allows us to gather sets of possible
concrete type constraints for each (trait)object, which enables transforming
a dynamic dispatch call into a set of one of more static calls.

VerifOpt operates in largely three phases: 
- preprocess
    - gather information needed in later phases
- interpret
    - flow-sensitive abstract interpretation -> collect all _reachable_ type
      constraints for any given object
- rewrite
    - transform dynamic dispatch sites into a switch-table of static calls,
      using constraints from the interpret phase

## Interpreter State

```rust
use rustc_public::mir::mono::Instance;
use rustc_public::ty::GenericArgs;

// VOID = VerifOpt ID
type VOID = (Instance, GenericArgs);

// Callers
type EnclosingScopes = Option<Vec<VOID>>;

struct InterpStore {
  constraint_map: HashMap<MapKey, Box<MapValue>>,
  field_map: HashMap<(Place, VOID), Vec<Place>>,
  memoized_bb_orders: HashMap<VOID, Vec<BasicBlock>>,
}

enum MapKey {
  Var(Place),
  Scope(VOID),
}

enum MapValue {
  Store(InterpStore, EnclosingScopes),
  Constraints(Constraints),
}

impl InterpStore {
    fn link_adt_fields(&mut self, adt_scoped_place: &(Place, VOID), field: &Place) {
        // if adt_scoped_place exists in field_map:
        // - update w field place
        // else:
        // - initialize w field place
    }

    fn scoped_get(&self, scope: &VOID, key: &MapKey, is_closure: bool) -> Option<MapValue> {
        // get store @ scope
        // in store, get constraints @ key
        // if closure, get in enclosing scope
    }

    fn scoped_update(&mut self, scope: &VOID, key: MapKey, value: Box<MapValue>) {
        // get store @ scope
        // in store, merge existing key's value with new value + set
    }
}
```

InterpStore contains maps for
1. variables -> constraints
2. variables -> fields (other variables)
as well as the memoized WTO order of basic blocks for every interpreted function body

InterpStore constraint map
- keys are either variables (MIR places) or scopes (represented by special verifopt ids)
- values are either constraints (for variables) or sub-stores (for scopes)
- sub-stores are also associated with their calling scopes
- constraints are explained more below

## Constraints

```rust
type Constraints = Vec<Constraint>;

struct Constraint = {
    traitobj: Option<TraitObj>, 
    running: Option<RunningConstraint>,
}

type TraitObj = (TraitObjTy, TraitObjConstraint);

type TraitObjTy = (TraitDef, GenericArgs);

enum TraitObjConstraint {
    Adt(...),
    Closure(...),
}

enum RunningConstraint {
    // primitive types
    Scalar(...),
    // adt types
    Adt(...),
    // callable types
    Closure(...),
    FnDef(...),
    FnPtr(...),
    // other
    Other(...),
}
```

Constraints model the possible concrete types of any (trait) object at any point in time

Since we do forward analysis, we do not know a priori which objects will
eventually flow into trait objects. Thus we must treat every non-trait object as potentially flowing into some future trait object

This necessitates storing two kinds of information
- The current flow-sensitive type constraints of the object, whether it is a trait object or not (RunningConstraint)
- If this is a trait object, the flow-sensitive type constraints for that particular
  trait type (TraitObj)

Thus, a `Box<dyn Animal>` that is either a `Cat` or a `Dog` would look like: 

```rust
[Constraint {
    traitobj: Some((Animal, Cat)),
    running: Some(Box<Cat>),
},
Constraint {
    traitobj: Some((Animal, Dog)),
    running: Some(Box<Dog>),
}]
```

The point at which a non-trait object becomes a trait object is
interesting. This generally happens when, at the type system-level, an rvalue
of a concrete type gets assigned to an lvalue of a dynamic type. This largely
happens in one of four kinds of places:
- during a variable assignment
- when a function returns a traitobj type
    - the actual returned object type may be concrete, while the type of destination place/var is dynamic
    - implicit cast
- when a function takes in a traitobj argument 
    - the actual object argument type may be concrete, while the type of the
      parameter is dynamic
    - implicit cast
- explicit casts to traitobjects

Note that a non-trait object does not _always_ flow into a trait object at these
kinds of places, but that these are the *only* places in which it might. 

Also note that all of these places are observable via the type system, i.e. the
type system can tell us when a non-trait object type is flowing into a
trait-object type. 

### Example

Say we have a `Box<Cat>` that flows into a `Box<dyn Animal>` across these two operations:

```mir
_2 = Box::<Cat>::new(const Cat);
...
_0 = move _2 as std::boxed::Box<dyn Animal>  (PointerCoercion(Unsize, Implicit));
```

The process of pulling out the relevant trait object information is as follows. 

Before the second operation, our constraint for _2 looks like:

```rust
[Constraint {
    traitobj: None,
    running: Some(Box<Cat>),
}]
```

We inspect the second operation's destination for any dynamic types. We find the
following type for _0: `let mut _0: std::boxed::Box<dyn Animal>;`, which
contains a `dyn Animal` type. 

Note, the second operation is a cast.

VerifOpt looks through the `running` constraints for any type that implements
the `Animal` trait, finding `Cat`. It then populates the `traitobj` field of the
constraints like so: 

```rust
[Constraint {
    traitobj: Some((Animal, Cat)),
    running: Some(Box<Cat>),
}]
```

These constraints are stored in the constraint map for MapKey::Var(Place { 0 }),
and VerifOpt then proceeds to interpret the next statement or block. 

## Interpretation

### General VerifOpt Flow

Starting from the entry function specified by `rustc_public::entry_fn()`,
VerifOpt traverses/interprets the MIR body. The blocks in each MIR body are
interpreted in reverse-postorder, such that blocks are interpreted before any
of their successor blocks are interpreted (TODO except for loops). This ensures
that all variable dependencies are fulfilled, i.e. that an operation requiring
constraints for some variable will have at least *some* constraints to work with
(TODO loops will likely iteratively update these constraints).

Each block in an MIR body has zero-or-more statements and a terminator.
Statements can be of varioius kinds (e.g. storage markers, nops). VerifOpt only
concerns itself with assignment statements, which affect the store in the
currently-interpreting scope.

motivate/explain why ignoring other statement kinds
- setdiscriminant?
    - might be useful if static info to reap
- intrinsic?
    - i.e. copy_nonoverlapping + assume
- place mention?
    - might need to interp (e.g. `let _ = expr` - don't want to ignore expr)
- fakeread - seems to just be for the borrow checker
- retag - seems to just be for memory model checkers
- ascribe user type - used alongside type checking
- coverage - for instrumentation
- constevalcounter - for the compiler

Similarly, there are several kinds of terminators. VerifOpt examines the
following:
- switchint
    - can prune basic blocks based on statically-known information
- call
- return
    - set retval (constraints) from callee scope back into caller scope
- inlineasm? (coarsely)
    - TODO
- assert?
    - TODO could give us more info

Control flow in terminators are taken into account when prescribing the MIR body reverse-postorder. 

Terminators that signal an error path (resume, abort, unreachable) are ignored as those
will not return to a non-error path (execution/interpretation would diverge) and thus will not affect any subsequent dynamic dispatches in the non-error path.

Storage-relevant terminators (drop) are also ignored as VerifOpt does not
track/model (stack or heap) memory.

### Function Calls

Function calls can be:
- direct (via constant)
- indirect (via variable/constraint)
- static
- dynamic
- via closures
- via fn ptrs
- intrinsics
- recursive

[Note: not sure the direct/indirect distinction is useful right now, but thats
how the implementation is set up]

All function calls largely follow this pattern:
- get the (monomorphized) target MIR function body
- resolve argument constraints (copy from caller to callee scope)
- interpret body in new scope

The difference is how each path decides which target MIR to get (i.e. how call graphs are resolved): 
- static
    - known constant, straightforward resolution to a monomorphized instance
- dynamic
    - direct only
    - unknown targets, use constraints (explained more below)
- closure 
    - indirect only (via variable/constraint)
- fnptr
    - can be direct or indirect
- intrinsics
    - no MIR to interpret, set retval constraints to the return type of the intrinsic function

#### Resolving Dynamic Function Call Targets

Dynamic call targets are resolved largely by getting the relevant trait for the 
current trait object, getting the concrete type constraints collected for the
current trait object (located at the first argument to the virtual function)
and collating each concrete type's implementation of the virtual function. This 
makes the set of concrete call targets that FSA has resolved the virtual call to. 

More precisely, dynamic call targets are resolved via the following (pseudo-code) 
algorithm:

```rust
fn resolve_dynamic_targets(&self, traitobj: &Place, virt_func_id: &DefId) -> Vec<DefId> {
    let impl_ids = Vec::new();

    // get associated trait
    let trait_id = self.trait_store.fns_to_traits.get(virt_func_id).unwrap();

    // get traitobj current type constraints
    let traitobj_constraints = self.constraint_store.get(traitobj).unwrap();

    // get concrete implementation for each type constraint
    for toc in traitobj_constraints {
        impl_ids.push(self.trait_store.concrete_impls((toc, virt_func_id)).unwrap();
    }

    impl_ids
}
```

TODO excluding GenericArgs, leave out or include?

If FSA does not have enough information to determine any concrete
implementations, then VerifOpt falls back to TODO technique for getting the
target set. 

Once the concrete/static call targets have been determined, VerifOpt loops
through static invocation of each of them, as if the dynamic call was already
transformed into a switch statement of static calls. 

After this loop, both the InterpStores and result constraints should be merged
into singular objects, so the separately-updated stores can exist in a single
place and the virtual function can simulate returning a single result. Then
execution proceeds as normal. 

#### Recursive Calls

TODO








## Rewriting

TODO

## In-Flux Design Decisions

What to store for RunningConstraints
- trade-offs:
    - fewer coarse variants (only store things that don't match/refine type system info)
        - less to track/interpret
        - potential(?) loss of info
    - vs
    - more precise variants (store even if match type system info)
        - more to track/interpret
        - potentially(?) greater info

How do loops affect WTO/reverse post-order interpretation of blocks?

Other statement kinds?

