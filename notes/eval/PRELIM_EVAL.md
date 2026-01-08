# Preliminary Performance + Code Size Numbers

mod/unmod compilers are locally built stage2 rustcs

- `category`:
    - `code size in bytes`
    - `benchmark perf`

why does the mir-rw not improve in code size but the src-rw does (w mod rustc)?

## `cgu = 1` (`opt-level = 3`)

unmod
- not-rw:
    - 747480 B
    - 28.43 ns/iter (+/- 0.39)
- mir-rw:
    - 747480 B
    - 27.62 ns/iter (+/- 0.08)
- src-rw: 
    - 747476 B (-4) ⬅️
    - 25.49 ns/iter (+/- 0.30) ⬅️

mod
- not-rw:
    - 747488 B
    - 27.58 ns/iter (+/- 0.35)
- mir-rw:
    - 747488 B
    - 26.01 ns/iter (+/- 0.49) ⬅️
- src-rw: 
    - 747484 B (-4) ⬅️
    - 25.12 ns/iter (+/- 0.44) ⬅️

## `cgu = 1` && `opt-level = s`

unmod
- not-rw:
    - 743376 B
    - 32.16 ns/iter (+/- 0.10)
- mir-rw:
    - 743376 B
    - 32.35 ns/iter (+/- 1.51) 
- src-rw:
    - 743384 B (+8)
    - 32.32 ns/iter (+/- 1.81)

mod
- not-rw:
    - 747480 B
    - 30.53 ns/iter (+/- 0.33)
- mir-rw:
    - 747480 B (text + bss sections are different)
    - SIGSEGV
- src-rw:
    - 747472 B (-8) ⬅️
    - 26.92 ns/iter (+/- 0.19) ⬅️

## `cgu = 1` && `opt-level = z`

unmod
- not-rw:
    - 743372 B
    - 34.50 ns/iter (+/- 0.36)
- mir-rw:
    - 743372 B
    - 34.20 ns/iter (+/- 0.55)
- src-rw:
    - 743380 B (+8)
    - 30.45 ns/iter (+/- 0.50) ⬅️

mod
- not-rw:
    - 747476 B
    - 32.80 ns/iter (+/- 1.79)
- mir-rw:
    - 747476 B (text + bss sections are different)
    - SIGSEGV
- src-rw:
    - 747484 B (+8)
    - 30.79 ns/iter (+/- 1.35)

