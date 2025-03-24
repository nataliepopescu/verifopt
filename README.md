# Verification for Optimization Project

Using verification / static analysis for optimizing code

## Repo structure

[flux-examples](https://github.com/nataliepopescu/verifopt/tree/main/flux-examples):
simplified code to use flux on, mirroring the kinds of constructs we would like
to verify in real world code

[meetings](https://github.com/nataliepopescu/verifopt/tree/main/meetings): notes 
from various meetings on verifopt / related things

[tock_results](https://github.com/nataliepopescu/verifopt/tree/main/tock_results): 
[compiled
binaries](https://github.com/nataliepopescu/verifopt/tree/main/tock_results/bins)/[elf
files](https://github.com/nataliepopescu/verifopt/tree/main/tock_results/elfs)/[object
dumps](https://github.com/nataliepopescu/verifopt/tree/main/tock_results/objdumps) for the following various 
"experiments" (board = `imix`):
- `base_<board>` = unmodified tock
- `delete_mlfq_panic_<board>` = mlfq panic replaced with `0`

[NOTES.md](https://github.com/nataliepopescu/verifopt/blob/main/NOTES.md): general 
project notes (high-level)

[PANICS.md](https://github.com/nataliepopescu/verifopt/blob/main/PANICS.md): notes 
about attempts to identify the compiled panics in tock

[RELWORK.md](https://github.com/nataliepopescu/verifopt/blob/main/RELWORK.md): 
related work notes

[USING_FLUX.md](https://github.com/nataliepopescu/verifopt/blob/main/USING_FLUX.md): 
notes on how to install / use flux, and experience using it on certain panics in 
tock

