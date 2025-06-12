# Trait Objects, Generics, and VTables

[trait obj](https://doc.rust-lang.org/reference/types/trait-object.html)

trait object == dynamically sized type (DST)
- when size (or type) is unknown at compile time

when does ^ happen?

DSTs -> fat pointers (2x size)
- slices (ptr + slice len)
- trait objs (ptr + vtable ptr)

- `?Sized`

[dyn overview](https://quinedot.github.io/rust-learning/dyn-trait-overview.html)
- "dyn Trait is also useful in some situations where generics are undesirable"
    - so `dyn Trait` and generics are not exactly the same thing
    - generic params have an implicit `Sized` bound, which is incompatible w
      unSized trait objects

## Generics

generics and traits/trait objects are related b/c in order to have a generic
parameter, need to specify which interface that parameter must abide by, hence,
which trait that parameter must implement

## VTables

[vtable-diff-rust-c++](https://users.rust-lang.org/t/v-tables-differences-between-rust-and-c/92445/11)
- "One of the goals of Rust's design in general is to not force the runtime cost
  of a feature onto code/programs that don't use that feature. In this case,
  space is only allocated for a vtable pointer in places that are actually using
  dynamic dispatch; the C++ version, in contrast, stores a copy of the vtable
  pointer inside every instance whether or not dynamic dispatch is actually
  being used."

vs jump tables
- is this effectively the switch-case statement?
