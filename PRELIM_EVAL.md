# Preliminary Performance + Code Size Numbers

code sizes in bytes.

## only `lto = true`

no diff to slack

## only `cgu = 1`

unmod
- not-rw:
    - 718812
- mir-rw:
    - 718812
- src-rw: 
    - 718808 (less!)

mod
- not-rw:
    - 747494
- mir-rw:
    - build crashes (illegal instruction)
- src-rw: 
    - 747490 (less!)

now a 30K code size diff though

## `cgu = 1` && `opt-level = s`

unmod
- not-rw:
    - 718804
- mir-rw:
    - 718804
- src-rw:
    - 718796 (less!)

mod
- not-rw:
    - 747478
- mir-rw:
    - 747478 (text + bss sections are different)
- src-rw:
    - 747486

still a 30K code size diff

## `cgu = 1` && `opt-level = z`

unmod
- not-rw:
    - 718800
    - 40.08 ns/iter (+/- 1.01)
    - 1407 LLVM-IR LOC
- mir-rw:
    - 718800
    - 39.04 ns/iter (+/- 0.52)
    - 1407 LLVM-IR LOC
- src-rw:
    - 718792 (less!)
    - 34.92 ns/iter (+/- 0.58)
    - 1432 LLVM-IR LOC

mod
- not-rw:
    - 747490
    - 32.80 ns/iter (+/- 0.20)
    - 1403 LLVM-IR LOC
- mir-rw:
    - 747490 (text + bss sections are different)
    - seg fault :) 
    - sigill :)
- src-rw:
    - 747482 (less!)
    - 32.04 ns/iter (+/- 0.32)
    - 1428 LLVM-IR LOC

still a 30K code size diff

