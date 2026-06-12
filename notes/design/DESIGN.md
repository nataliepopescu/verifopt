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

### How to get `TraitObjConstraint`s

everything can *potentially* flow into a traitobj

it is important for `RunningConstraint`s to be kept accurate, because they are
what ultimately flow into `TraitObjConstraint`s
- keep accurate without modeling/applying casts/projections
- this is the difficult part ^

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

convert_op (operand)
- if copy/move operand, convert_place
- if constant operand, convert_const

convert_place
- probe cmap
    - if constraint exist, return those exact constraints
    - otherwise, initialize constraints that mirror the place type (convert_ty)

convert_const
- if (1) allocated const that is (2) a bool/integer type with a (3) readable inner integer value, create a scalar constraint of the allocated value
- all other allocated consts, as well as zero-sized consts, do not have directly interpretable values, so return *empty* constraints ~FIXME~
    - fixed: returning through convert_ty
    - **FIXME** wrap in verifopt Const variant?
- have not yet run into other types of consts, TBD

convert_cast
- if cast operand is copy/move, convert_place
    - **FIXME** this keeps the constraints that were collected before the cast, so after
      the cast the constraints will not match the type system
    - difficulty: changing type may make us lose our trait object constraints - **TEST**
- if cast operand is constant, convert_ty

convert_agg (aggregate)
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

convert_unop / convert_binop = straightforward

convert_ty
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


when to return empty constraints vs constraints that match the type system?

the only time that we do not want the `RunningConstraint` type to match the type
system type is where there is a traitobject (`dyn`) somewhere in the type system
type
- what about empty constraints?

tricky part: do not want to model casting/projections
- when set a type, mirror type system
- when update a type, how to mirror type system without loosing info/needing to
  model a whole bunch of shit


















