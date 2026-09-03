# meeting

## agenda

- rewrite wasn't working outside of local crate before
- current problem: how to find + rewrite monomorphized code in such a way
  that sticks
    - worst case: write a new query?
    - or, rewrite at the LLVM IR level

- report some results
    - "size" of the generics problem
    - different CAP benchmarks
        - statistical significance?

## notes

two-pronged approach
- try to create a hook for monomorphized MIR / where can we modify it
- find examples where this generic issue isn't so much of a problem
