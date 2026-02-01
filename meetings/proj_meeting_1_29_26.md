# meeting

## notes

transf @ source level vs mir
- wouldn't want source level transf to be permanent in case the programmer
  changes something that violates our assumptions
- so source-level transf would be still be a temp compilation artifact (can
  "clean")
- also, would be using unsafe anyway, so not benefitting THAT much from
  downstream rust checks
- and might be harder to code, since need to handle more syntax?

undergrad (will)
- starting w rust generals/identifying dynamic dispatch

conf deadline?
- maybe OOPSLA 26 (deadline mid march)
- or compilers conf CGO 27 (deadline end of may)
    - HPC-oriented
- maybe ASPLOS
- SOSP not a priority (more of a community/audience mismatch)

next steps:

- after actually calling dyn dispatch
- do the transformation for this example! (now having the analysis info that i
  think we were missing)
- and then start expanding to more examples

