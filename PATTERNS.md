# Code patterns checked for vtable usage

All patterns are compiled with `-C opt-level=3` (release build).

Diffs are marked with `>`.

## Patterns

1. no trait impls in scope

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

2. add trait impls in scope

```rust
pub trait Animal {
    fn speak(&self):
}

struct Cat {}
struct Dog {}

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
```
- yes but nothing in scope calls `speak_all()`

3. randomly decide which Animal subtype to be

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

    animal.speak();
}

pub fn main() {
    dyn_dp();
}
```
- MIR: yes
- LLVM: maybe?
	- `speak_all()` uses vtable, but nothing calls it
	- actual `speak()` code is inlined into an indirect call with some sort of switch statement (switching on vtable ptr values?) preceeding it

4. call `speak_all()` in `dyn_dp()`

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

    speak_all();
}

pub fn main() {
    dyn_dp();
}
```
- MIR: yes
- LLVM: maybe?
	- `speak_all()` uses vtable, but nothing calls it (although source code
      does)
	- actual `speak()` code is inlined into an indirect call with some sort of switch statement (switching on vtable ptr values?) preceeding it

4. `#[inline(never)]`

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
#[inline(never)]
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

    speak_all();
}

pub fn main() {
    dyn_dp();
}
```
- MIR: yes
- LLVM: `speak_all()` ia called, but prefaced by switch statement
