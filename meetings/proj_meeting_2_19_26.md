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

