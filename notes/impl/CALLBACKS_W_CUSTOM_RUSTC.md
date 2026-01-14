# Compiling Callbacks-based code (rustc_driver) w our custom rustc

attempting to follow
[these](https://rustc-dev-guide.rust-lang.org/rustc-driver/remarks-on-perma-unstable-features.html#using-rustc-private-with-custom-toolchains)
notes (but they are not very detailed...)

- [ ] make LLVM libs available to system's lib search paths
    - [ ] does this mean i need to _download_ the LLVM libs locally?
    - [ ] or are they already on my system and i just need to point PATH to
      them?
- [ ] ensure LLVM version matches that used to build Rust toolchain
    - [x] get rustc LLVM version
        - how to check this?
        - `bootstrap.toml` doesn't have anything
        - nor does `src/bootstrap/defaults/bootstrap.compiler.toml`
        - `rustc -vV` prints: LLVM version: 21.1.2
    - [ ] what LLVM version is on the system?
        - since I am using NixOS, i'm assuming I can just start a shell w a
          certain set (+ specific version) of LLVM packages
            - first of all, don't know _which_ LLVM package set we'd need
            - second of all, NixOS doesn't seem to have LLVM packages w version
              21.1.2, so might have to change the version of LLVM that our rustc
              was compiled with?

## Which LLVM packages does NixOS (25.11) support? 

`llvmPackages_20.libllvm`
- version = 20.1.8

`libllvm`
- version = 21.1.7

are these versions just the versions of the respective packages (their dev) or
of the LLVM they install?
- try!

- whoa, when using `libllvm` the resulting LLVM version is only 17.0.6 :(
- not ideal
- maybe we need an unstable package...

are we on NixOS 25.11?
- NixOS version: 24.05.7376.b134951a4c9f (Uakari)

on the package search page, 25.11 corresponds to the nix _channel_
- how does this relate to NixOS versions?

- `nix-channel --list` prints nothing
- `nix-channel --list-generations` prints
```sh
   1   2025-08-16 12:29:36   (current)
```
    - don't really know what this means in terms of which channel is being used
- `sudo -i nix-channel --list` says the root user is subscribed to nixos-24.05
  which is very outdated! so i should upgrade to 25.11

"According to the NixOS wiki’s Nix channels entry, “a channel is a name for the
latest “verified” git commits in Nixpkgs”."

updated channel to 25.11
- doesn't change llvm version in nixshell w `libllvm` (still 17.0.6)

what about the unstable channel?

how to control which channel is being used if subscribed to multiple?

maybe don't use channels actually... https://github.com/NixOS/nix.dev/issues/16#issuecomment-713701415
- but then how to control where you're getting your packages from?
- maybe just make a shell.nix that controls what nixpkgs is

using shell.nix
- if we point nixpkgs to the unstable branch/channel, libllvm gets us llvm
  version 21.1.7
- if we point nixpkgs to 25.11 branch/channel, libllvm get us llvm version
  21.1.7 as well
- so, maybe stick with the stable channel for now

## Controlling which LLVM version Rustc is built with

i've been building our custom rust from inside a temporary nix-shell that i 
started a long time ago, so let's recall what i put in there

thanks to my documentation, looks like that shell was started as:

```sh
nix-shell -p python3 rustup gcc
```

no llvm specified.

if we naively add `libllvm` into the mix i don't think anything will
change/break.

i'm going to try this with the unmod comparison branch of rustc to make sure
that we can build it without our changes. then if we fail to build with our
changes we know where to look.

ok now that our env is set up let's figure out what to change in the rustc
compilation process. 
- there is a section in the `bootstrap.example.toml`  titled "Tweaking how LLVM
  is compiled" - this seems like our guy

- set `llvm.download-ci-llvm = false` (default = true) to _not_ download Rust's
  CI built LLVM (want to use locally-built one instead)

- running `./x build` errors out w: 

```sh
Couldn't find required command: cmake

You should install cmake, or set `download-ci-llvm = true` in the
`[llvm]` section of `bootstrap.toml` to download LLVM rather
than building it.

Build completed unsuccessfully in 0:00:00
```
- which is good in the sense that we've successfully prevented the build from
  using Rust's CI built llvm, but we haven't yet gotten it to use our own

installing (via shell.nix)
- cmake
- ninja

got
```sh
cmake: /nix/store/qksd2mz9f5iasbsh398akdb58fx9kx6d-gcc-13.2.0-lib/lib/libstdc++.so.6: version `CXXABI_1.3.15' not found (required by cmake)
```
- this is the gcc we manually pointed our LD_LIBRARY_PATH var to

- let's try removing this export (to find a better solution that covers both
  this and the initial error that had us make the export in the first place)
    - the initial error hasn't popped back up (yet; and i think it happened
      really early)
    - so this tells me that maybe some of the llvm/cmake packages fixed
      something?
    - do we still need gcc?

    - whatever we end up doing, we should have as similar a setup as possible
      when running our rust tool (the Callback stuff), maybe even the same
      shell.nix
    - dang, not fully

```sh
Building stage1 lld-wrapper (stage0 -> stage1, x86_64-unknown-linux-gnu)
   Compiling lld-wrapper v0.1.0 (/home/np/hack/rust_unmod_locllvm/src/tools/lld-wrapper)
    Finished `release` profile [optimized + debuginfo] target(s) in 0.36s
Building stage1 library artifacts{alloc, compiler_builtins, core, panic_abort, panic_unwind, proc_macro, rustc-std-workspace-core, std, std_detect, sysroot, test, unwind} (stage1 -> stage1, x86_64-unknown-linux-gnu)
error: process didn't exit successfully: `/home/np/hack/rust_unmod_locllvm/build/bootstrap/debug/rustc /home/np/hack/rust_unmod_locllvm/build/bootstrap/debug/rustc -vV` (exit status: 127)
--- stderr
/home/np/hack/rust_unmod_locllvm/build/x86_64-unknown-linux-gnu/stage1/bin/rustc: error while loading shared libraries: libz.so.1: cannot open shared object file: No such file or directory
```
- that's just in the zlib package but I also had to point LD_LIBRARY_PATH to the
  right dir

- and the libstdc++.so.6 error came back so brought back the gcc path export

and it works!

when trying to compile my mod locllvm build that initial cmake error came back
up, i think because i was pointing LD_LIBRARY_PATH to an old gcc version so i
updated that pointer/replaced the shell.nix package import from `gcc` to
`libgcc` (idk why i had `gcc` in there in the first place that isn't a valid
full package? how did that work? anyway)

## Building the Callback code

works w the new unmod_locllvm toolchain!

however, llvm-config says LLVM version is 21.1.7, but rustc -vV says 21.1.2

perhaps it is lucky that things work, or perhaps the rustc -vV isn't acurate?


build also works w w mod_locllvm toolchain :)

## Try testing out some change

made a modification to the compiler to make a certain function pub (vs
pub(crate) ), and now the original error is back...
















