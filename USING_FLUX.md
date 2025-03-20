# Using Flux

## Installing

### Dependency: [Z3](https://github.com/Z3Prover/z3)

on Mac, naive install fails with

```sh
ld: symbol(s) not found for architecture x86_64
clang: error: linker command failed with exit code 1 (use -v to see invocation)
make: *** [z3] Error 1
```

trying

```sh
CXX=clang++ CC=clang python scripts/mk_make.py
```

dang, got same error. trying via `homebrew` now. worked.

### Dependency:
[liquid-fixpoint](https://github.com/ucsd-progsys/liquid-fixpoint)

specifically having issues with haskell/`stack`, trying to install via GHCup
now. (initially tried:
https://docs.haskellstack.org/en/stable/install_and_upgrade/#__tabbed_10_2)

## Running

trouble resolving git dependency on NixOS
- have to run `rustup update` in nix-shell to get it to work (likely a syntax 
  change in Cargo.toml according to amit)

### on [Flux-Tock](https://github.com/PLSysSec/tock)

goal: can we successfully run flux on a thing that is expected to pass
verification

trying per-package via cargo-flux

Running `cargo flux` in the kernel subdir (kernel package)
- a couple warnings but no flux errors, yay

after resolving the below errors on base tock, my guess is that this repo is
verifying the properties that preclude all of the potential errors that just 
running flux on tock produces, and possibly some other properties as well
- so flux does seem to try to prove certain things without any annotations at
  all... since i couldn't successfully run it on the base tock kernel without
  annotations

#### Scheduler/MLFQ

[queue_idx < 3](https://github.com/PLSysSec/tock/blob/master/kernel/src/scheduler/mlfq.rs#L167)
- seems to be trying to avoid a bounds check in the next line
- running flux on base tock produced an error at this line ("assertion might
  fail: possible out-of-bounds access"), so moreso for letting the compiler know
  that the bounds check will not fail...?

- same with
  [this](https://github.com/PLSysSec/tock/blob/master/kernel/src/scheduler/mlfq.rs#L171)
  line

assume impl: 

```rust
#[flux_rs::sig(fn(b:bool) ensures b)]
pub const fn assume(b: bool) {
    if !b {
        panic!("assume fails")
    }
}
```

note that certain flux constructs themselves insert panics - how does this 
affect any panic-removing goal?

### on [base Tock](https://github.com/tock/tock)

goal: 
- is flux able to prove X?
- how annoying is it to prove X?
- how automatable is proving X?
- does proving X require additional user input?

first had to manually "resolve"/ignore over 50 flux errors (some internal 
compiler ones too) by marking a bunch of unrelated things with 
`#[flux_rs::trusted]`
- errors like: "assertion might fail: possible division by zero", or "arithmetic
  operation may overflow"

#### Scheduler/MLFQ




