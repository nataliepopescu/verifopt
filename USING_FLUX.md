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
- [commit](https://github.com/nataliepopescu/tock/commit/361f058792272d8375682b907f87f38d35684d7a)

#### Scheduler/MLFQ

can get flux to pass on tock if simply add

```rust
#[flux_rs::sig(fn(&MLFQSched<A>[@m], i:usize{i < 3}) -> u32{r: r > 0})]
```

on `get_timeslice_us`, but not when i add an equivalent annotation to the
simplified analogous function in our example/simplified code. this makes me
think that there are some `#[flux_rs::trusted]` annotations somewhere making
this "easier" to verify than it should be. 

next TODO: try to track these down



### on [example
code](https://github.com/nataliepopescu/verifopt/blob/main/flux-example/src/lib.rs)

using `flux_assume` probably inserts a panic into the compiled output, because 
it is not a flux-intrinsic function (proof-carrying code) but rather a 
flux-verified "normal" function. 
- however, `flux_assume` is actually a `const` function, which means "When
  called from a const context, the function is interpreted by the compiler at
  compile time"
  - what does calling a const function from a const context mean? does the
    calling function also need to be const? 
    - this is not the case in the flux-tock MLFQ usage...
    - how difficult / obstructive is it to make a function const? probably
      similar to async (i.e. depends on what calls _it_)...
  - [const
    context](https://doc.rust-lang.org/reference/const_eval.html#const-context)
  - calling from a const context is difficult, even in a simple example
  - if not called from a const context, still panicking in the LLVM IR (unsure
    about the assembly/machine code)


not flux, but alternatively trying to use
[this](https://www.reddit.com/r/rust/comments/1jafvbe/how_to_inform_the_rust_compiler_of_an_enforced/) 
to affect compilation / aid flux, but it is very unclear and also seems to 
require `unsafe`. was not able to successfully help flux with it
- finally an example: https://github.com/rust-lang/rust/issues/59524

try [contracts](https://docs.rs/contracts/latest/contracts/) next
- contracts will likely add the same the flux does
    - the current problem is that `idx` is not in the function signature/API, so
      it is hard to say anything about it in pre-/post-conditions

### Simple Bounds Checks

b/c cannot verify bounds checks without `assume`, 
looking for places in tock where flux reports `assertion might fail: possible 
out-of-bounds access`
- `scheduler/mlfq.rs` ; `result()`
  - uses `assume`
- `deferred_call.rs` ; `service_next_pending()`
  - flux-tock doesn't change this code directly, maybe some invariants elsewhere
    make this possible? 
  - TODO track!
- `processbuffer.rs` ; `impl ProcessSliceIndex<ReadableProcessSlice> for usize`
  => `index()` && `impl ProcessSliceIndex<WriteableProcessSlice> for usize` =>
  `index()`
  - former: `impl Index<usize> for ReadableProcessSlice`
    - `#[flux_rs::trusted_impl]` TODO why need?
    - `#[flux_rs::sig(fn(self: &ReadableProcessSlice[@len], idx: usize ) ->
      &Self::Output requires len > idx)]`

```rust
#[repr(transparent)]
#[flux_rs::refined_by(len: int)]
pub struct ReadableProcessSlice {
    #[field([ReadableProcessByte][len])]
    slice: [ReadableProcessByte],
}
```


  - latter: `impl Index<usize> for WriteableProcessSlice`
    - `#[flux_rs::trusted_impl]` TODO why need?
    - `#[flux_rs::sig(fn(self: &WriteableProcessSlice[@len], idx: usize) ->
      &Self::Output requires len > idx)]`






