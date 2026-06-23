# Design Doc

## Modeling constraints

```rust
type Constraints = Vec<Constraint>;

type Constraint = (TraitObjConstraint, RunningConstraint);

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
    Idk(...),
}
```

trade-off
- fewer coarse variants
    - less to interpret
    - loss of info
- vs
- more precise variants
    - more to interpret
    - greater info

tracking max set of types
- not applying projections to constraints is sound for our purposes b/c we will
  always overapproximate the set of possible traitobjects (i.e. applying
  projections will remove things/possibilities from the type constraints, so if 
  we do not do that, we will never miss something that is valid (?)

### High-level Structure/Interactions of `TraitObjTy`s and `TraitObjConstraint`s

TODO

#### How to get `TraitObjConstraint`s

everything can *potentially* flow into a traitobj

it is important for `RunningConstraint`s to be kept accurate, because they are
what ultimately flow into `TraitObjConstraint`s
- keep accurate without modeling/applying casts/projections

if you have accurate `RunningConstraint`s, it is relatively
straightforward to pull out `TraitObjConstraint`s
- this needs to happen whenever changing a concrete type to a dynamic type (this is visible _in the type system_)
- i.e. in the following places:
    - when a function returns a traitobj
        - the actual retval may be concrete, while the type of destination place/var is dynamic
    - when a function takes in a traitobj
        - the actual arg may be concrete, while the type of the parameter place/var is dynamic
    - during variable assignment
        - the rval may be concrete while the lval is dynamic
    - during a cast
        - the pre-cast type may be concrete, while the post-cast type is dynamic

### How to keep `RunningConstraint`s accurate

when to return empty constraints vs constraints that match the type system?

the only time that we do not want the `RunningConstraint` type to match the type
system type is where there is a traitobject (`dyn`) somewhere in the type system
type
- what about empty constraints?

tricky part: do not want to model casting/projections
- when set a type, mirror type system
- when update a type, how to mirror type system without loosing info/needing to
  model a whole bunch of shtuff

#### convert_op (operand)

- if copy/move operand, convert_place
- if constant operand, convert_const

#### convert_place

- probe cmap
    - if constraint exist, return those exact constraints
    - otherwise, initialize constraints that mirror the place type (convert_ty)

#### convert_const

- if (1) allocated const that is (2) a bool/integer type with a (3) readable inner integer value, create a scalar constraint of the allocated value
- all other allocated consts, as well as zero-sized consts, do not have directly interpretable values, so return *empty* constraints ~FIXME~
    - fixed: returning through convert_ty
    - **TODO** wrap in verifopt Const variant? (verifopt doesn't really care
      about consts though)
- have not yet run into other types of consts, TBD

#### convert_cast

- if cast operand is constant, convert_ty
- if cast operand is copy/move, convert_place
    - ~FIXME~ this keeps the constraints that were collected before the cast, so after
      the cast the constraints will not match the type system
    - difficulty: changing type may make us lose our trait object constraints
        - KEY: how to change the type in a _simple_ way without losing collected traitobj constraints? 

    - fix:
        - if casting into traitobj, search existing constraints for concrete type(s) that
          implements that trait -> putting those concrete types into the
          traitobj-constraints should preserve our collected info through the cast

#### convert_agg (aggregates)

- if ADT, ~FIXME~
    - only converting/checking genargs for traitobj/controlflow constraints
    - fixed: creating regular Adt (traitobj pulling shouldn't happen from within
      convert.rs, b/c need both lval and rval for that)
- if tuple, ~FIXME~
    - returning tuple constraints as one of the inner elem constraints, but
      really it should simultaneously be all of them
    - fixed
- if rawptr, convert inner ty ~FIXME~
    - fixed
- if array, convert inner ty ~FIXME~
    - fixed
- if closure, closure
- have not yet come across coroutines

#### convert_unop / convert_binop

straightforward

#### convert_ty

- if bool/integer, Scalar(None)
- if adt, Adt
- if tuple,
    - convert_ty for each inner elem
    - Idk([converted_inner_tys])
- if array/slice, Idk([converted_list_ty])
- if closure, Closure
- if fndef, FnDef
- if fnptr, FnPtr
- if ref/rawptr, convert_ty of ref/rawptr inner elem (essentially ignore the
  ref/rawptr)
- if char/string/never, nothing


## Getting Trait Object implementations

### Closures

Implicitly implement Fn* traits, so don't show up in our collected set of traits
in the trait store
- but when we need to execute one of the `call*` functions for the Fn* traits,
  just interpret the closure body instead

### Field Projections

Making map keys _places_ instead of _locals_ to maintain projection/field
information

When create an ADT, also store field projections

Propagate field projections when
- assign value of type w projs to another place
- passing the type w projs as an argument to a func call

Casting
- Unlikely (?) that types are cast into ADTs (current assumption)

#### Re-assigning Place w Projections (via converter)



## Default Trait Method Implementations

Only resolve to default implementation is _had_ candidate objects but none of
those have an associated method implementation of their own

Should _not_ resolve to default method implementation if there were no candidate
objects to begin with, b/c this likely means we don't have enough info to know
what to resolve to

Interpretating the default impl path
- similar to monomorphization
- if we choose to execute a default implementation, we still need to have a
  concrete type as `self`
- b/c the default implementation may call a trait method that does _not_ have a
  default implementation, in which case our analysis is stuck
- calling the default implementation should not be used as a fallback for "idk
  what concrete type this is" (unless the default method happens to not call any
  other trait methods without default impls, which requires further analysis to
  discern)
- for now, we can solve this as follows:
    - if a trait impl block does not override a trait's default method impl,
      then we essentially "copy" over that implementation into the object's
      local trait impl block space, essentially treating default impls just like
      any other impl















