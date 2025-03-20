# Natalie using Flux
what does Flux try to verify when there aren't any annotations in your code?
- overflow
- out-of-bounds

flux is building a model so that it can work when the annotations are added

Questions:
1. what is flux's data model/memory model?

# Directions for project
(a,b) - Rocq/dependent types
  - arbitrary properties/manual proof
(b,c) - SMT/model-checking
  - arbitrary properties .... no manual proof
  - frame the properties in a logic that allows you to search the space of proofs
  - crazy-advanced math/decidability/VASR
(a,c) - Types! /language design
  - NO arbitrary props
  - NO math
  - NO manual proof
  - this is where borrow checking lives from Rust compiler


FLux:
leverage stuff from the borrow checker to make better SMT model checking
lies on edge halfway between (b,c) an (a,c)

Us:
figure out what you want to verify first... then move into the SMT world when you know what to prove
