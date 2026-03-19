# meeting

## agenda

- verifopt examples
    - visitor pattern
    - example w downstream opt

- dependency machinery

- will requested to start familiarizing himself w tool

## notes

find examples where: 
1. downstream opts are effective
2. compiler doesn't already do the rewrites very well

already have single-variant visitors example

mae repo: morphic lang
- https://morphic-lang.org/
    - eval on 3 examples
    - including some jank ones
- paper
    - functional lambda analog == dynamic dispatch
    - defunctionalization (what we're doing)

"cannot run on real rust projects, but _can_ run on real rust code"
-> putting example code in single lib
    - the steps translating to something our tool expects is not too important
    - janky examples ok
    - showing "science" rather than a completely finished tool

want more realistic examples, other places where double virtual call

try examples without the inline(never)s
