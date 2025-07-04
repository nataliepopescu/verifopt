# meeting

## agenda

- implementing traits
    - when calling methods, will need to use fully qualified syntax

- function summaries
    - where to start? generally seems to be a hard problem

## notes

function resolution/dispatch modes
1. standard (static) mode
    - compiler simply looks up which function to call
    - fully qualified thing tells you _exactly_ this
    - like function overloading
2. dynamic mode
    - intersection between subtyping + ?
    - when vtables are used
......

rust: chooses most specific impl

the diamond problem

when impl traits: the "interesting" part == `dyn traits`
- non-dyn traits are just statically resolved like overloading (mode 1)
- dyn keyword => dynamic resolution (mode 2)

type tag
- for each struct: save some num of bits => which traits this struct impls
- the "discriminator"

once in rust compiler: will lose access to type names
- will need to match on (and, thus, store) what we've generated in the switch 
  statement
    - which is..?
- think Collection<Animal>

current strategy: C++ style
- saving type names

another strategy
- rust-style enums (will need to impl this)
    - we can assume closed traits, Rust traits are open by default
- generate enum w all types that impl a trait; CHA-equiv for traits
- probably better than names/C++ strat
- preface to type tags/discriminator

