# VerifOpt Design

## State

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
    // TODO
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

## Interpretation












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

