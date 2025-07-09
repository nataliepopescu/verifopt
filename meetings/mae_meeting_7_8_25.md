# notes

still too little uncertainty, would have to enum all possible Ints (for "age")
- imagine giveMeANum func
- can also have a special "don't know" value for each type
- potential: ~ranges (i.e. "idk but...") - many representations
- also: idk but i know what somefunc() on this returns


special "idk" val at every level
- is this enough to not lose precision?
- if come across it, start to add "idk but"s
- as a sys paper: heavily grounded in waht do ppl want/need


- Animal::idk
  - ^ don't need to know about Cat / Dog
  - since we have access to this, is there a good reason to use this?
- Cat::idk
- Cat::age::idk


think about the tradeoffs... may not actually change analysis precision; maybe 
impact storage (Animal ~= less complex) start less complex, expand when learn more


1. rust has typeof (not exposed) - use for func lookup (runtime
   types)
- could just stop here
2. then can opt per callsite (specific call that has same effect as
lookup)
- could potentially do CHA
- separate compilation complicates switch-statement construction
3. instead of (2), can have vtable
unclear _when_ this happens, so unclear if we will have access to
rust's version of its dynamic type table (assuming we wont't have
access)

