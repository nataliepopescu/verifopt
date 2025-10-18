# debugging

getting a core dump (memory allocation failed) when don't have print statements
in, but leaving the print statements in (w relevant MIR changes) doesn't crash?

the crash doesn't actually happen before main() runs b/c things _are_ being
printed

maybe try to replicate crash w all prints in first?
- can't
- wtf??!?!
- all i've changed in MIR are the 4 local indices at the beginning of the pass,
  and removed the printlns from the src

why is gdb not allowing me to step tho??
- try to improve core dump/figure this part out

- "Single stepping until exit from function main, which has no line number
  information."
  - https://users.rust-lang.org/t/problems-with-debugging-programs-and-gdb/7819/3

turns out need `-C debuginfo=full`

ok, now our code seems to run fine but affects something downstream...
