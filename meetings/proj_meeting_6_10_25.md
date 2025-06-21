# meeting

## agenda

- code walkthrough
    - things in interpreter that are easier w Rust guarantees (prepare in advance)
    - idiomatic rust
        - i.e. passing store around vs mutating it
    - code org
        - declaring things in lib.rs as opposed to other modules

- traits discussion

## notes

------ part 1

SSA check of funcdef: separate all vars in foo/bar

improvement over research: switch statement has _no_ impossible branches (unlike
CHA)

hard part: exponential divergence => lose precision
- can be up to thousands of calls
- when does the high preciison thing take too long? 
- don't need to think/worry about this yet though

eval metrics
- e2e benchmark perf (real code)
- microbench of dynamic call overhead
- oracle calls
- show: verifopt is very close to oracle

pathological use cases are not realistic...

classification of code that performs better/worse
- maybe don't rely on eval for this (risk overfitting solution)
- q: how does looking at this before (vs after) eval not over-fit?

lookup github dataset on dyn?
- but first ask amit/leon

------ part 2

went over Rust trait syntax in C++
- static traits (impl Trait)
- dynamic traits (dyn Trait)

add negative constraints
- if Dog -> panic, value can not be Dog from this point onwards

also consider `instanceof`s

adding traits
- add vtable simulacrum via function types

add func params
- list of param types + output

collect all funcs of certain signature

impl CHA first

traits come in handy w negative constraints
- more to eliminate (unlike positive constraints, if already know Cat or Dog,
  knowing Animal won't help)
- CHA takes more advantage of traits

single-compilation unit assumption: 
- reasonable b/c the field is moving in this direction anyway

style 1: accumulating constraints, no resolution (set of possible vals)
style 2: res on spot (set of constraints about possible vals)
- simpler
- no accepted perf diff between the two

## next steps

- [x] clone bug in invocation interp
- [x] SSA pass: separate funcs in body
- [x] add function args
- [x] collect all funcs of certain signature
- [x] add functional function retvals
- [ ] impl negative constraints
- [ ] traits (before/after CHA)
- [ ] impl CHA
- [ ] eventually impl flow-sensitive stuff (for function res)

- [ ] ask amit/leon about rust code that heavily uses dyn

