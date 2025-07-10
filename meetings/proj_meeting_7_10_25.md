# meeting

## agenda

- [x] high-level
    - current goal: eliminate indirect function calls via flow-sensitive
      analysis
        - get rid of vtables
        - CHA vs flow-sensitive

- [x] what is implemented?
    - general
        - simplified program CFG
        - types: `Int`, `Struct`, `Func`, `DynTrait`, `Idk`
        - values: `Num`, `IdkNum`, `Struct`, `IdkStruct`, `Var`, `IdkVar`

    - indirect calls
        - CHA
        - flow-sensitive analysis
        - func rewriting

    - trait calls
        - CHA

- [x] currently implementing
    - trait calls
        - flow-sensitive analysis
        - func rewriting

- [x] what implementation is left before moving into Rust compiler?
    - function summaries

## notes

- param == dyntrait, arg == struct

- fn foo(self: &dyn Trait Animal) {}... cat = Cat{}, cat.foo()

- fn foo(self: &Struct Cat) {}... animal = DynTrait(Animal), animal.foo()

- OR...

```
Animal { fn speak }
Cat { fn speak }
Dog { fn speak }

fn callSpeak(x: &dyn Trait Animal) { x.speak() }

y = Cat{}

callSpeak(y)
```

- CHA enums for Cat/Dog/...
- vs monomorph

```
fn CallThespeak(x ) { switch(x) { case Cat : Cat::speak(x);  case Dog: Dog::speak(x); };
y = Cat{}, CallTheSpeak(y)
fn CallThespeak'(x : Cat) {Cat::speak(x);};
y = Cat{}, CallTheSpeak'(y)
```

- want some sort of type ID
- won't be switching on vtable address bc ideally that won't be emitted at all

add strong barrier
- raw exec (no context)
    - keeps function table (i.e. symbols table)
    - exec context: pared down (only has what is 

- analysis step (context)
    - put constraint context (what we're switching on) into program text
    - e.g. add extra field to structs
    - instead of (address, vtable ptr) => (address, extra data)


- func summaries
    - purpose: test cases for ambiguity
        - if not using func summaries, need another method of ambiguity
          resolution (for test cases)
          - i.e. reading from stdin
    - would _also_ help w analysis efficiency
        - highly tied to utility of tool (development cycle)

think about empirical confidence before moving to rust
- option: take some binaries, can we reasonably do both the flow-[in]sensitive
  analysis
    - no rewrite
    - just counting possible set/length of switch statement

- grep based tool
    - find dyn traits


- probably fine to impl widening once in rust (within own data structure)
- natural barrier @ function boundaries
    - choice:
        - can use function summaries (lose precision)
        - or do full analysis

