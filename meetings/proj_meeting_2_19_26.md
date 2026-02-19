# meeting

## agenda

- impl updates
    - generics

    - unevaluated constants
        - leave uneval or eval?
        - benefit of eval is that we could get more info, but depending on the
          overhead, is it worth it?

    - casts (have largely been able to avoid, but perhaps worth talking about)

    - better handling of "special" types (types w unique way of interpreting
      their contents)
        - Box
        - Result

        - are these actually unique or do we just not yet have the
          infrastructure to look into nested types?

- vtable addresses
    - anja: if we're using vtable addresses in the rewrite how will this
      affect/be affected by vtables getting optimized out?
    - mae: what was the get-vtable-address-post-compilation thing (iirc) about?

## notes

check the return value

benchmarking updates

3 versions
1. naive benchmark (might compile out b/c res isn't used, if we know it is a
   Cat)
    - otherwise the virtual func may have side effects, so must keep
    - very very best case
2. noinline + blackbox

code size improvement largely _not_ from omitting vtables
- rather from removing the dynamic calls
- + the things the vtables point to (i.e. many functions can be optimized out if
    we know we never use speak() e.g. for all animals except Cat)

- may need to check if discriminating on vtable ptr keeps the vtables around
    - if so, should discriminate on something else

