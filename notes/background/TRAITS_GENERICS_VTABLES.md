# Trait Objects, Generics, and VTables

## Traits

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

[advanced traits](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html)
- disambiguating...
    - "Nothing in Rust prevents a trait from having a method with the same name as
      another trait’s method, nor does Rust prevent you from implementing both
      traits on one type. It’s also possible to implement a method directly on the
      type with the same name as methods from traits."
    - `Dog::baby_name()` calls non-trait `baby_name()` method on `Dog`
    - `Animal::baby_name()` does not compile
    - `<Dog as Animal>::baby_name()` calls `Animal` trait `baby_name()` method
      on `Dog`
    - in general: `<Type as Trait>::function(receiver_if_method, next_arg, ...);`

[trait objs](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)
- "In a struct or enum, the data in the struct fields and the behavior in impl
  blocks are separated, whereas in other languages, the data and behavior
  combined into one concept is often labeled an object... trait objects
  differ from traditional objects in that we can’t add data to a trait object."

### Function vs Associated Functions vs Methods

[assoc funcs vs
methods](https://www.reddit.com/r/learnrust/comments/ums8uw/difference_between_associated_function_and_method/)
- "Note that methods can be called as associated functions, you just need to
  pass an argument for the receiver as well. For example foo.bar() could be
  Foo::bar(&foo)"
    - any difference when compiled/run between these two syntaxes?

[method
syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#associated-functions)
- "The &self is actually short for self: &Self"
- "The main reason for using methods instead of functions, in addition to
  providing method syntax and not having to repeat the type of self in every
  method’s signature, is for organization."

[dot operator](https://doc.rust-lang.org/nomicon/dot-operator.html)
- "The dot operator will perform a lot of magic to convert types. It will perform
  auto-referencing, auto-dereferencing, and coercion until types match."

[detailed method lookup 
mechanics](https://rustc-dev-guide.rust-lang.org/method-lookup.html)
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
