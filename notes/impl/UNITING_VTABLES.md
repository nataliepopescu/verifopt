# Notes on uniting vtable pointers for the same type across CGUs

## CGUs

[cgus](https://doc.rust-lang.org/beta/rustc/codegen-options/index.html#codegen-units)
- specify _max_ number of cgus

default (non-incremental builds) = 16

default (incremental builds) = 256

## LTO

[lto](https://doc.rust-lang.org/beta/rustc/codegen-options/index.html#lto)o
- "If -C lto is not specified, then the compiler will attempt to perform "thin
  local LTO" which performs "thin" LTO on the local crate only across its
  codegen units."
    - so, cgus per crate
    - may need to know where local vs. global LTO is specified/differentiated

note `-C opt-level=0` also disables LTO (unless specify w LTO flag)

why is LTO an LLVM thing?
[llvm-lto](https://llvm.org/docs/LinkTimeOptimization.html)
- presumably LTOs happen once code is already in object form?
- "In this model, the linker treats LLVM bitcode files like native object files
  and allows mixing and matching among them."

steps
1. reads LLVM bitcode files + gets symbols info (-> linker's global symbol
   table)
2. symbol resolution (using global symbol table)
3. optimize bitcode files
    - which symbols do the native files need
    - invokes LLVM optimizer + codegen-ers
    - get native code
4. post-optimization symbol resolution
    - linker reads optimized native file + update global symbol table
    - + track changes re LLVM bitcode extern symbols (may strip dead code if
        option enabled)
5. then linker continues as if it never saw any LLVM bitcode

## Normal Linking

overall: combine object files into final executable

steps:
- read in .o files + check validity
- resolve cross-file (i.e. cross-object file) deps
- also link in library files (pre-compiled)
- produce output (library or executable)

