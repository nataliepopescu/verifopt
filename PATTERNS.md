# Code patterns checked for vtable usage

All patterns are compiled with `-C opt-level=3` (release build).

Diffs are marked with `>`.

## Patterns

1. 

```rust
pub trait Animal {
    fn speak(&self);
}

#[unsafe(no_mangle)]
pub fn speak_all(animal: &dyn Animal) {
    animal.speak()
}
```
- MIR: yes
- LLVM: yes but nothing in scope calls `speak_all()`

2. 

```rust
pub trait Animal {
    fn speak(&self):
}

> struct Cat {}
> struct Dog {}
> 
> impl Animal for Cat {
>     fn speak(&self)
> 		println!("meow");
>     }
> }
> 
> impl Animal for Dog {
>     fn speak(&self) {
>         println!("woof");
>     }
> }

#[unsafe(no_mangle)]
pub fn speak_all(animal: &dyn Animal) {
    animal.speak()
}
```
- yes but nothing in scope calls `speak_all()`

3.

```rust
use rand::Rng;

pub trait Animal {
    fn speak(&self);
}

struct Bird {}
struct Cat {}
struct Dog {}

impl Animal for Bird {
    fn speak(&self) {
        println!("chirp");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("woof");
    }
}

#[unsafe(no_mangle)]
pub fn speak_all(animal: &dyn Animal) {
    animal.speak()
}

fn dyn_dp() {
    let animal: &dyn Animal;

    let num: u32 = rand::rng().random_range(..3);

    if num == 0 {
        animal = &Bird {}
    } else if num == 1 {
        animal = &Cat {}
    } else {
        animal = &Dog {}
    }

    speak_all(animal);
}

pub fn main() {
    dyn_dp();
}
```
- MIR: yes
- LLVM: 
	- `speak_all()` uses vtable, but nothing calls it even though it should (dead code)
	- actual `speak_all()` code is inlined into an indirect call with some sort of switch statement (switching on vtable ptr values?) preceeding it

