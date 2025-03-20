# meeting

flux doing stuff w/out annotations

if there are things flux cannot understand, flux looses a lot of info that
reduces its power to prove certain things (and ends up having to be really
conservative)

more default errors fix -> the more accurate flux's model of your code will be

flux: continuously checks certain invariants

probably need to know what the internal invariants are in order to write code
that passes them

find out: what are flux's core invariants / memory model? 



mae's verification pet theory
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
    - automatic / flexible

- searchable "entailments" (NP-complete)
- good at certain classes of logical entailment
- crazy-advanced math / decidability (VASRs)
- can take a LONG time even if decidable (verifier never returns)

- ~VASRs (w some manual stuff)
- TLA+
- Ivy


3. types / language design
- verif
    - no arbitrary properties
- user
    - no math / manual proofs
    - don't really need to write stuff differently (except sometimes wrestling
      with the borrow checker)

- ~borrow checking
- compiler has internally fixed _which_ props are important to prove
    - equally decidable for _all_ programs


compiler using verification (verifopt)
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

next steps
- what is flux's model when run without annotations?
    - anja doc
- remove panics from another system?
    - what other systems/apps contain panics?

- rust code that uses linked lists?
- performance engineered code?













