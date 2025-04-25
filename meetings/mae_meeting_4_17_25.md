# mae meeting

summary: vtable optimization

vtable: dynamic lookup of function implementations (when have inheritance)
- (static?) list of vtable indices (i.e. the "speak" function will always be
  accessed at index 1, regardless of type?)

LTO (link time optimizations)
- considered @ launch time, because only perform linking when you're about to
  run a thing (not when you compile it, to be dynamically linked)

can have types that depend on runtime information, so you cannot statically
determine a type

abstract interpretation via an interpreter example (AST)
- recursively interp each of the statement types
- if interpreting a conditional statement, where the conditional can be EITHER
  true or false, then essentially must recursively interp each side of the
  branch
  - if the conditional statement can ONLY be true, only need to interp the true
    branch, and vice versa for if the conditional can ONLY be false

## questions

goal of abstract interpretation? 
- sound _over-approx_ of all possible states (in the form of invariants?)
- what is the benefit of abstract interpretation over symbolic exection?
    - the "over-approx" case makes me wonder whether AI would be the best tool
      for pruning compiled code (as opposed to something less over-approximated)

how does the interpreter example relate to the vtable optimization goal? 
- if "possible" for some function to be called, compile its vtable info
- if "!possible" for some function to be called, do not compile its vtable info

what do vtables look like in Rust?
- dyn traits?
- how used?

what would it mean to prune vtable code-gen in Rust?
