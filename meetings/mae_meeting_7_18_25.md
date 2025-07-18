# meeting notes

seeing `dyn` in return types makes sense, bc it is annoying to specify generics there

ideally want to find the concrete type in each of the dynamics
- try to trace dyn-returning funcs to something concrete
- find some simple examples of tracing back

visitor pattern solution (phrase expression problem)
- more complicated; ~AST walk
- dyn traits or enums (static types)
- dyn traits allow ppl to avoid adding enum/case statements every time the enum is modified
	- e.g. toy verifopt is currently using enums

builder/factory pattern

iterator pattern
- naturally parameterized on the thing its iterating over
- that thing should also be iterable by the iterator (circular paramaterization)

another pattern: marker type wrappers (static -> dynamic res)
- marker types that are normally static will need a wrapper trait like this that enables them to resolve dynamically 

valid rust reasons to not write down the type even though you know it (b/c of
rust restrictions, can become very difficult to do)

why dyn is being used
- is it a workaround for an annoying programming pattern?
    - goal: avoid passing crazy args around, but in reality could know the concrete type of the thing
- or simply just cannot know?
    - this would also prohibit analysis

some synthesis: 
- many good reasons to have dyn
- but most(? to confirm) are not related to actual type resolution

treesitter lib? 
- good example for insane generic type sigs
- i.e. what devs avoid when they opt for trait objects

another route: 
- mark (using a profiler) when call specific funcs that take in dyn traits
- can know more about how many times they are called (freq)
	- static numbers that count function declarations with dyn args don't really tell us too much about their usage

to start, can write small proj that heavily uses dyn dispatch
- profile it immediately
- does profiler have a static vs dynamic version for any funcs?
- i.e. better understand how profiler handles dyn

it is well-known/agreed-upon that dyn traits are bad
- eiger / treesitter: avoid dyn traits
    - these are on the other end of the spectrum
- possible eval idea: if we replace the gnarly generics w dyn traits, how much slower is it? and how much can verifopt speed back up?
	- tool: simpler + more maintainable code!

