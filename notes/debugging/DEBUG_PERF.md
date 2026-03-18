# Debugging Performance

## `two_variants_bench`: no perf diff

turns out the _nonrewritten_ version has a hardcoded magic number returned??
that seems very wrong... 
- yeah, Cat was only ever returned

now that thats fixed, the perf is still the same

looking at the assembly, `wrap_dyn_call`s are identical...
- dynamic dispatch?
- try gdb, but idk what we can find out from that register

what if we are not actually modifying the MIR....
- but when i emit=mir i can see the changes?

## `one_variant_bench`: no perf diff

rewriting is not happening, why not

oh wow, i think b/c the rewrite doesn't traverse functions....

what about `one_variant`?
- this does get rewritten
- but just don't observe the perf diff (hopefully) b/c `one_variant_bench` doesn't get
  rewritten
- in the ASM, it gets optimized to the constant
- but the OG version also gets optimized to a constant
- so `two_variants` is really where we should see things

- BUT first debug the rewriter on one_variant (since that doesn't yet work w
  function nesting)



## nested `two_variants`

first val assignment (result of get_animal)
  - same for both
arg passing (for the nested example)

  - in main, _14 = wrap_dyn_call(move _15)
  - resolve arg constraints, maintains IdkStructness
  - in wrap_dyn_call() body, the animal gets cast as a couple
    types before speak()
      - NonNull
      - *const dyn Animal
      - &(*_)

      - this does also happen in the non-nested example example
        the first two types are combined into a single statement:
          - copy ((Unique) NonNull) as *const dyn Animal

      - the first statement (NonNull) erases the struct info...
          - FIXME in from_rvalue (core)
              - which branches...

              - working: Cast -> rvalue_from_cast
              - not working: Use -> rvalue_from_op -> rvalue_from_place

              - place projections look really similar


## non-nested `two_variants`

ok but wait does the rewrite (two_variants) work without nesting?
  - its really hard to see... maybe flank with something
    obvious
  - flanked with noop() funcs
  - OG: seems to be an indirect call (as expected)
  - RW: inlined magic nums!! ok thats great

## non-nested `two_variants_bench`

bench without nesting?
- for some reason, we also land in an IdkType situation when
  we don't nest functions in the benchmark
    - why?


trait obj should be at 65 (according to some math/assumptions)
- that actually looks right

then we get to resolving the first arg constraints
- IdkType(Box(Animal)) essentially
1. how did we get here?
    - probably b/c we're getting the value/type from a vec....
    - can we somehow make this an IdkStruct? (i.e. using projections?) or do we
      try to handle it as is
2. can we process this?


what do things look like coming out of a vec?

bb60 + bb65 (main)

- go from IdkType(NonNull) -> IdkType(Box(Animal))
- 242 -> animal
- 243 -> vtable

- 242 -> 61 -> 103 -> 65

- 242: IdkType(Box(Animal))
- 61: IdkType(Box(Animal))
- 103 (cast) - same
- 65: Ref(IdkType...)


might need to take a closer look at those projections...

but also before we do that, maybe try to process as IdkType?
- can't really decompose the IdkType into useful parts though, b/c need to check
  not only if box but whats inside the box + its genargs
- also, just like, philosophically, IdkType is a fallback. so i think it makes
  sense to try to get things into IdkStruct form
- but maybe not though...?

- this might be good to talk about in next meeting
    - IdkType essentially tells us static knowledge is no more precise than type
      system, _but_ can still use this in analysis/rewrite
    - whereas IdkStruct may or may not be more precise than type system (if it
      is not more precise, it might just be an enumeration of e.g. Cat and Dog
      instead of Animal)


next steps: 
- pinpoint first IdkType wrap of this type (where we were just looking essentially, but maybe a bit before local242)
- look into the projections


start with local 192
- assignment has like 4 field projections


actually was able to make IdkType path work


now the rewrite seems a bit messed up though, seem to have two into_raw()
chains?
- oh wait no yeah this makes sense b/c we're in bench, which invokes speak() in
  two distinct loops

but also the vtable locals are def wrong

they do happen to be right for the two_variants (no bench) case, but this needs
to be automated (currently hardcoded)


automation
- need a DynMetadata local (specifically `std::ptr::DynMetadata<dyn [Binder { value: Trait(Animal), bound_vars: [] }] + '{erased}>,`)


in bench
- locals: 35, 58, 243, 246 (think its the last two)

- which is cat_vtable?
    - ty source info!! it is 35

- warmup vtable: 243

- run vtable: 246


b/c of how the rewrite is setup, can only currently specify one variant vtable
- but need to specify one PER rewrite / dyn dispatch instance
- (and then eventually potentially multiple per rewrite)

- it might affect the benchmark performance if we never correctly specify the
  warmup vtable - so keep this in mind when looking at results, before you get
  to fixing it

currently panicking (slice index error somewhere in the compiler)

maybe try to implement the fix first, but i don't think thats whats causing the
slice index panic...

maybe while at beach think about how to fix :)

### slice index error

trying to debug just with calling speak() in a loop

what if no separate setup

what about no loop.. just a single call

what about no blackbox...

might be the reading from the file?

apparently its the counters.........
- TODO WHAT IS HAPPENING

lets try benching without counters then
- need to get the right vtable ptr locals
    - 35, 54, 207, 210

#### debugging counters

try to not even use the counters (but still pass them around) and see if that
has the same effect

- it does, meaning the bug is not about how the counters are used (which makes
  sense since that is isolated in speak()), but rather how they are propagated
  - makes a bit less sense because how is the rewrite affecting speak()
    arguments..... oh
  - there we go, the rewrite is hardcoding speak args (and not passing the
    counters)

- when transforming/adding back the speak call, only the FIRST argument should
  change (the rest should NOT)

### rewrite might be messing up vec push/pop()?

when trying to bench, after setup, the first pop() returns a None value (while
the non-rewritten binary runs fine :') )

das is so veird

for some reason when 0 warmup runs it works....
- ~1.5 ns (vs 2ns OG)

somehow flanking w the Instant blocks also fixes things
- veiiiird

- TODO WHAT IS HAPPENING

try flanking speak() w noops, the latter noop doesn't get printed (in fact it
doesn't seem to even end up in the asm)

fixing the rewrite (vtable_loc) constants seems to fix this...

#### should probably automate getting the vtable_loc

when are the actual vtable ptrs calculated?

well first they are calculated, then those are added to a Vec, and then popped
from the Vec

calc
- PtrMetdata call
- _24 = PtrMetadata(copy _52);
- _5 = PtrMetadata(copy _51);
- _97 = PtrMetadata(copy _99);
- _54 = PtrMetadata(copy _82);
- _35 = PtrMetadata(copy _81);


some res locals are DynMetadata (overlap), but what are the others?
- let mut _24: &usize;
- let mut _5: std::fmt::Arguments<'_>;
- let mut _97: usize;

- let _35: std::ptr::DynMetadata<dyn Animal>;
- let _54: std::ptr::DynMetadata<dyn Animal>;

so one step could be finding the overlap of 
1. calling PtrMetadata
2. into a DynMetadata<dyn Trait>
- this would give us _35 (cat_vtable) and _54 (the vtable loc used in the setup loop)
- so not an exact match

what about getting all DynMetadata types?


















