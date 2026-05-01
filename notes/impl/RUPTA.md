# Rupta

Trying to run on NixOs, fails

ugh fix:

`export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH`

installed `cargo pta` command only works if using the same rustc version
- added a rust-toolchain.toml file which also includes components like
  `rustc-dev`
