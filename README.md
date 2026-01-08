# Verification for Compiler Optimization Project

Using verification / static analysis for optimizing code

## Repo structure

### Notes / Docs

[meetings](https://github.com/nataliepopescu/verifopt/tree/main/meetings): notes 
from various meetings on verifopt / related things

[notes](https://github.com/nataliepopescu/verifopt/tree/main/notes): project 
notes roughly organized into future paper sections

### Motivation Code

[results-test](https://github.com/nataliepopescu/verifopt/tree/main/results-test):
a python crawler that categorizes `dyn` usage in the top X cargo crates;
category explanations and results are shown
[HERE](https://github.com/nataliepopescu/verifopt/blob/main/notes/motivation/DYN_TRAIT_OBS.md)

### Tool Code

[analysis](https://github.com/nataliepopescu/verifopt/tree/main/analysis):
`Callback`-based analysis pass(es)

[simple-interp](https://github.com/nataliepopescu/verifopt/tree/main/simple-interp): 
simple interpreter for starting to think about what info we need to collect
during compilation for vtable optimization

[rewrites](https://github.com/nataliepopescu/verifopt/tree/main/rewrites):
performs various styles of dynamic-dispatch rewrites (into static-dispatch) and
benchmarks them to determine which style of rewrite would be best for our tool
to emulate

### Eval Code

[dp-ex](https://github.com/nataliepopescu/verifopt/tree/main/dp-ex): 
code that attempts to assess the overhead of dynamic dispatch vs static 
dispatch, in certain examples

[visitor-ex](https://github.com/nataliepopescu/verifopt/tree/main/visitor-ex):
a simple visitor pattern implementation to eventually evaluate our tool on
(note: from the source-level rewrites, we expected this example to have a larger
perf impact - still TODO)

### Old Code

[old-proj-code/flux-examples](https://github.com/nataliepopescu/verifopt/tree/main/flux-examples):
simplified code to use flux on, mirroring the kinds of constructs we would like
to verify in the real world

[old-proj-code/verus-examples](https://github.com/nataliepopescu/verifopt/tree/main/verus-examples):
simplified code to use verus on, mirroring the kinds of constructs we would like
to verify in the real world

