# meeting

## agenda

- weird perf (non)diffs

## notes

check benchmarking setup
- benchmark the whole loop (may not have small enough time granularity)
    - figure out a setup loop

try inlining vs not inlining  perf diff (maybe also in the criterion benchmarks)

is MIR even getting modified?

crazy motivation:
- rewrite static-full program to be dynamic-full to see the relative slowdown

load-time rewriting
- ASLR
- for rewriting vtable comp

should figure out vtable ptr overlap
