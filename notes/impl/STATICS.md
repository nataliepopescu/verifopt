# statics

statics are handled similarly to normal variables, with the main caveat being that mutability may happen outside of the current analyzed scope, meaning that it has to be checked explicitly. there are two different possibilities for mutability: a `static mut` assignment or the existence of `UnsafeCell`. these are both checked for via `static_check_const`, and fulfilled statics move on to `convert_static_const`, where there is a possibility of having constraints using the constant allocated value of the static.

some of the other logic is also changed to accommodate statics. specifically, the generated MIR needs to use a deref call to access a static, so constraints are allowed to pass through those calls using the `Ptr` running constraint in `convert_place`. also, these derefs sometimes happen in-place, e.g. the discriminator in a `switchInt`, so `convert_op` is used more aggressively in that case instead of checking for a `Copy`/`Move` to handle more cases.
