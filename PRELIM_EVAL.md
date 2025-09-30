# Preliminary Performance + Code Size Numbers

- <category>:
    - <code size in bytes>
    - <LLVM-IR LOC>
    - <benchmark perf>

## `cgu = 1` (`opt-level = 3`)

unmod
- not-rw:
    - 747480 B
    - 1400 LOC
    - 28.43 ns/iter (+/- 0.39)
- mir-rw:
    - 747480 B
    - 1400 LOC
    - 27.62 ns/iter (+/- 0.08)
- src-rw: 
    - 747476 B (-4) !!
    - 1425 LOC
    - 25.49 ns/iter (+/- 0.30)

mod
- not-rw:
    - 747494 B
    - 1403 LOC
    - 31.04 ns/iter (+/- 0.23)
- mir-rw:
    - build crashes (illegal instruction)
- src-rw: 
    - 747490 B (-4) !!
    - 1428 LOC
    - 27.47 ns/iter (+/- 0.23) !!

## `cgu = 1` && `opt-level = s`

unmod
- not-rw:
    - 743376 B
    - 1400 LOC
    - 32.16 ns/iter (+/- 0.10)
- mir-rw:
    - 743376 B
    - 1400 LOC
    - 32.35 ns/iter (+/- 1.51) 
- src-rw:
    - 743384 B (+8)
    - 1424 LOC
    - 32.32 ns/iter (+/- 1.81)

mod
- not-rw:
    - 747478 B
    - 1403 LOC
    - 33.63 ns/iter (+/- 0.10) 
- mir-rw:
    - 747478 B (text + bss sections are different)
    - sigill
    - seg fault
- src-rw:
    - 747486 B (+8)
    - 1428 LOC
    - 30.58 ns/iter (+/- 0.32) !!

## `cgu = 1` && `opt-level = z`

unmod
- not-rw:
    - 743372 B
    - 1400 LOC
    - 34.50 ns/iter (+/- 0.36)
- mir-rw:
    - 743372 B
    - 1400 LOC
    - 34.20 ns/iter (+/- 0.55)
- src-rw:
    - 743380 B (+8)
    - 1424 LOC
    - 30.45 ns/iter (+/- 0.50) !!

mod
- not-rw:
    - 747490 B
    - 1403 LOC
    - 32.80 ns/iter (+/- 0.20)
- mir-rw:
    - 747490 B (text + bss sections are different)
    - sigill
    - seg fault
- src-rw:
    - 747482 B (-8) !!
    - 1428 LOC
    - 32.04 ns/iter (+/- 0.32)

