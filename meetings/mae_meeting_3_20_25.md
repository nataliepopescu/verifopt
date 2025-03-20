figure out what you want to verify first... then move into the SMT world when you know what to prove
=======
# Natalie using Flux

flux doing stuff w/out annotations

What does Flu try to verify when there aren't any annotations in your code?
- overflow
- out-of-bounds

flux is buildinga model so that it can work when the annotations are added in.
if there are things flux cannot understand, flux loses a lot of info that
reduces its power to prove certain things (and ends up having to be really
conservative)

more default errors fix -> the more accurate flux's model of your code will be

flux: continuously checks certain invariants

probably need to know what the internal invariants are in order to write code
that passes them

TODO/find out: what is flux's datamodel/ memory model? core invariants 


# mae's verification pet theory
- 3 primary axes
    - how think about verification / what users do

1. Rocq / dependent types
- verif
    - arbitrary properties
- user
    - manual proof

- takes forever

2. SMT / model-checking
- verif
    - arbitrary properties
- user
    - no manual proof. automatic / flexible
- frame the properties in a logic that allows you to search the space of proofs
- searchable "entailments" (NP-complete)
- good at certain classes of logical entailment
- crazy-advanced math / decidability (VASRs)
- can take a LONG time even if decidable (verifier never returns)

- ~VASRs (w some manual stuff)
- TLA+
- Ivy


3. types / language design
- verif
    - NO arbitrary properties
- user
    - NO math 
    - NO manual proofs
    - don't really need to write stuff differently (except sometimes wrestling
      with the borrow checker)

- Rust borrow checking
- compiler has internally fixed _which_ props are important to prove
    - equally decidable for _all_ programs


## Some example tools
### Flux
- leverage stuff from the borrow checker to make better SMT model checking
- lies on edge halfway between 2. and 3.

### compiler using verification (verifopt)
- verif
    - not arbitrary properties
- user
    - relatively simple to write but user-guided

what properties do we think are important to prove?
- code patterns w some structural basis?
    - linked lists but random index


use flux: what can / cannot verify
- automatically?

types of things that we'd want to verify for perf are very different from things
ppl want to verify for other reasons, so will naturally be targetting different
things

tighter scope = more automatic
- the more you can constrain your scope, the more likely you can automate verifying

# Next steps
next steps
- what is flux's model when run without annotations?
    - anja doc
- remove panics from another system?
    - what other systems/apps contain panics? (anja)
- continue verifying panic reachability in mlfq with Flux (natalie)


- rust code that uses linked lists?
- performance engineered code?













>>>>>>> 6fb04b2e8ff519d625bb80b3fb884a672fee2454
