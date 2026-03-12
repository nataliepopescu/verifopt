# Miri and the Rustc Interpreter


## [miri](https://github.com/rust-lang/miri/tree/c12096bae69a1c531cc4f995a01174170230cc1c)

seems to hook into the rust interpreter

## [interpreter](https://rustc-dev-guide.rust-lang.org/const-eval/interpret.html)

used for Miri and CTFE (compilie-time function evaluation)

it is a virtual machine
- i.e. _no_ machine code produced

- i.e. might be good for analysis, but not for the rewrite?


## Monomorphization

important for, e.g., getting the precise `call_once` implementations for
closures.......




