# meeting

## agenda

- working on vtable ptrs
    - replaced actual vtable allocation with a typeid constant
    - getting that constant still requires a pointer indirection...

    - now trying to just have the fatptr look like (data_ptr, type_id)
        - bigger lift

    - also trying to merge our pass with modified rustc

    - vtable interpretation (e.g. calling function offsets, calling drop glue,
      etc) seems to happen in codegen (MIR just passes pointers around)

    - a TODO
        - lang_start closure = dyn dispatch (before verifopt entry_point/analysis,
          so need another way to convert this)

## notes

x86 or wasm + look for lang start fatptr
- may be some dynamic dispatches that we don't convert for whatever reason (so
  keep that as option)
    - lang start call does still use a vtable

next big ambiguity: what does this look like in real rust code/what are real
rust patterns that exercise this

vtable drop/size/align
- what are they used for? and do we need them after vtables disappear?

