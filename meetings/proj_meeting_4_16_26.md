# meeting

## agenda

Mostly coding updates today~

Implemented CHA for comparison

Also tried to implement RTA (explain what it is)
- more difficult bc need to track “inits” but that consists of a lot of different kinds of things
- But will still need to figure something out for this

Now have a working example where verifopt compiles a switch statement with fewer cases than CHA

Currently trying to compile benchmarks for this case so we can compare overheads

Will is working on getting verifopt to compile an example with Rand (crate) where we’re running into some generics tasks

Anja company stuff?

## notes

can't do rewrite for monomorph fakes until we actually have the monomorph (duplicated code)

RTA
- could also be a better thing in the fallback case (than CHA)
- maybe would be the struct init syntax (constructors)

mae: figure out vtable stuff?
- "buyer beware clauses"
    - maybe baseline is wrong
    - pieces of code that is required right now for our code to be correct but ideally don't want them 
        - require fewest modifications to real world modifications

code to generate vtables
- might wanna hack that
- to add the "stable" number
- how are vtables generated / how to modify this?
- how much goes into fat pointer?
- don't even need to replace the vtable ptr, just make sure nothing gets put into where its being used

more realistic examples
- structs w fields
- how does verifopt work on something like this?
- ideally want to be able to handle arbitrary code

code currently relies on transparently creatable/constructable structs that we can enumerate
- what about non-trivial constructors: how to get vtable ptr for those?

are there certain patterns of code that we can/can't handle? 
- think about bigger "classes"
- and have some reasoning for this

