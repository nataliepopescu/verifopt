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

## on Tock

already verified some things [at ucsd](https://github.com/PLSysSec/tock)

### Scheduler/MLFQ

[queue_idx < 3](https://github.com/PLSysSec/tock/blob/master/kernel/src/scheduler/mlfq.rs#L167)
- what is the purpose of this line? seems to be trying to avoid a bounds check 
  maybe in the next line

what is being verified in this file/implementation, period?
