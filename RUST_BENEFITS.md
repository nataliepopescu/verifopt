# notes on when/where Rust aids in interpreter/program analysis reasoning

## `func_collector.rs`

`Env`
- `funcs` is technically immutable but in reality is in fact expected to mutate
  by way of being passed as an argument to each function
    - current mechanism does not guarantee that `funcs` will only undergo
      exclusionary mutation...

- `funcs` hashmap values are strictly `FuncVals`

## `interpreter.rs`

`Store`?
- `funcs` is immutable
    - how does relying on this make reasoning easier?
    - still technically on the stack
- `vars` like `funcs` in previous step (`Env`)
    - current mechanism does not guarantee that `vars` will only undergo
      exclusionary mutation...

`funcs` hashmap values are strictly `FuncVals`
- when interpreter is running, know that everything in this hashmap is a
      function

the `Box<Statement>`s are immutable


## `rewriter.rs`

`store` field of `Rewriter` is immutable
- both `funcs` and `vars`

`funcs` hashmap values are strictly `FuncVals`


## general

`Box`ed values
- uniquely named, in heap => never have an alias


## todos

add interior mutability to make struct fields effectively `mut`?
- unfortunately, this does not leverage the type system
- on the other hand, making the entire struct `mut` (which _uses_ the type
  system) would be overly conservative

- might want to somehow split up the data structure in order to be able to
  achieve this behavior...
- but just splitting into substructs won't work b/c field mutability is
  inherited
- could wrap field in a Box and use
  [`as_mut_ptr()`](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.as_mut_ptr)
  when want to mutate?
    - maintains some aliasing guarantees BUT requires unsafe to perform the
      mutation...
    - and using unsafe (unless necessary) sort of defeats the purpose of using
      rust

- do ^ for not only symbols table/variables (`funcs` and `vars` but also maybe
  the AST (`Statement`s))

- what needs to be _shared_ vs what needs _exclusive access_

did a high-level pass over code structure/data layout, but reason about the 
implementation a bit more

