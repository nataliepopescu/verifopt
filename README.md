# Verification for Optimization Project

Goal of the project is to use verification for optimizing code

Several paths to explore, see
[NOTES.md](https://github.com/nataliepopescu/verifopt/blob/main/NOTES.md) for 
more information

## Repo structure

`flux-example`: simplified code to play with flux on

`meetings`: notes from various meetings

`tock_results`: compiled tock binaries/elf files/object dumps for the following 
various "experiments" (for `imix` board):
- `base` = unmodified tock
- `delete_mlfq_panic` = mlfq panic replaced with `0`

`NOTES.md`: general project notes (high-level)

`PANICS.md`: notes about attempts to identify the compiled panics in tock

`RELWORK.md`: related work notes

`USING_FLUX.md`: notes on how to install / use flux, and experience using it on
certain panics in tock
