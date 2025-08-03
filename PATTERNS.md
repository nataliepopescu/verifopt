# Code patterns checked for vtable usage

All patterns are compiled with `-C opt-level=3` (release build).

MIR generally seems to show vtable usage, so in these examples we are looking at the generated LLVM IR (via `godbolt`).

## Patterns

### 1: no trait impls in scope

```rust
pub trait Animal {
    fn speak(&self);
}

#[unsafe(no_mangle)]
pub fn speak_all(animal: &dyn Animal) {
    animal.speak()
}
```
- yes but nothing in scope calls `speak_all()`

### 2: add trait impls in scope

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

### 3: randomly decide which Animal subtype to be

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
- maybe?
	- actual `speak()` code is inlined into an indirect call with some sort of switch statement (switching on vtable ptr values?) preceeding it

### 4: call `speak_all()` in `dyn_dp()`

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
- maybe?
	- `speak_all()` uses vtable, but nothing calls it (although source code
      does)
	- actual `speak()` code is inlined into an indirect call with some sort of switch statement (switching on vtable ptr values?) preceeding it

### 5: annotate `speak_all()` with `#[inline(never)]`

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
- `speak_all()` is called, but prefaced by switch statement

### 6: more interesting structs for `Bird`/`Cat`/`Dog`

```rust
use rand::Rng;

pub trait Animal {
    fn speak(&self);
}

struct Bird {
    num: u32,
}

struct Cat {
    num: u32,
}

struct Dog {
    num: u32,
}

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

fn dyn_dp() {
    let animal: &dyn Animal;
    
    let bird = Bird { num: 0 };
    let cat = Cat { num: 1 };
    let dog = Dog { num: 2 };

    let num: u32 = rand::rng().random_range(..3);

    if num == 0 {
        animal = &bird;
    } else if num == 1 {
        animal = &cat;
    } else {
        animal = &dog;
    }

    animal.speak();
}

pub fn main() {
    dyn_dp();
}
```
- yes?
	- vtable access is prefaced by two phi nodes, one for the vtable ptr, and one for the concrete animal struct ptr
	- adding another field to the structs (e.g. `name: &'static str`) does not change this

### 7: calling `speak()` from different paths with different possible subsets of the Animal type

```rust
use rand::Rng;

pub trait Animal {
    fn speak(&self);
}

struct Bird {
    num: u32,
    name: &'static str,
}

struct Cat {
    num: u32,
    name: &'static str,
}

struct Dog {
    num: u32,
    name: &'static str,
}

struct Elephant {
    num: u32,
    name: &'static str,
}

struct Frog {
    num: u32,
    name: &'static str,
}

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

impl Animal for Elephant {
    fn speak(&self) {
        println!("toot");
    }
}

impl Animal for Frog {
    fn speak(&self) {
        println!("ribbit");
    }
}

fn dyn_dp_1() {
    let animal: &dyn Animal;
    
    let bird = Bird { num: 0, name: "betty" };
    let cat = Cat { num: 1, name: "cleo" };
    let dog = Dog { num: 2, name: "danny" };

    let num: u32 = rand::rng().random_range(..3);

    if num == 0 {
        animal = &bird;
    } else if num == 1 {
        animal = &cat;
    } else {
        animal = &dog;
    }

    animal.speak();
}

fn dyn_dp_2() {
    let animal: &dyn Animal;
    
    let cat = Cat { num: 1, name: "cleo" };
    let elephant = Elephant { num: 3, name: "ernie" };
    let frog = Frog { num: 4, name: "freddie" };

    let num: u32 = rand::rng().random_range(..3);

    if num == 0 {
        animal = &cat;
    } else if num == 1 {
        animal = &elephant;
    } else {
        animal = &frog;
    }

    animal.speak();
}

pub fn main() {
    dyn_dp_1();
    dyn_dp_2();
}
```
- yes?
	- vtable access is prefaced by two phi nodes, one for the vtable ptr, and one for the concrete animal struct ptr
	- in each of `dyn_dp_1` and `dyn_dp_2`, the phi nodes choose between the relevant subset of Animal type vtables/objects

### 8: make constructors less interesting again, and try different path calls

```rust
use rand::Rng;

pub trait Animal {
    fn speak(&self);
}

struct Bird {}

struct Cat {}

struct Dog {}

struct Elephant {}

struct Frog {}

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

impl Animal for Elephant {
    fn speak(&self) {
        println!("toot");
    }
}

impl Animal for Frog {
    fn speak(&self) {
        println!("ribbit");
    }
}

fn dyn_dp_1() {
    let animal: &dyn Animal;
    
    let bird = Bird {};
    let cat = Cat {};
    let dog = Dog {};

    let num: u32 = rand::rng().random_range(..3);

    if num == 0 {
        animal = &bird;
    } else if num == 1 {
        animal = &cat;
    } else {
        animal = &dog;
    }

    animal.speak();
}

fn dyn_dp_2() {
    let animal: &dyn Animal;
    
    let cat = Cat {};
    let elephant = Elephant {};
    let frog = Frog {};

    let num: u32 = rand::rng().random_range(..3);

    if num == 0 {
        animal = &cat;
    } else if num == 1 {
        animal = &elephant;
    } else {
        animal = &frog;
    }

    animal.speak();
}

pub fn main() {
    dyn_dp_1();
    dyn_dp_2();
}
```
```llvm
_ZN7example8dyn_dp_117hcda5620e0a50a933E.exit:
...
  %switch.selectcmp.i = icmp eq i32 %num.i, 1
  %switch.select.i = select i1 %switch.selectcmp.i, ptr @vtable.2, ptr @vtable.3
  %switch.selectcmp1.i = icmp eq i32 %num.i, 0
  %switch.select2.i = select i1 %switch.selectcmp1.i, ptr @vtable.1, ptr %switch.select.i
  %5 = getelementptr inbounds nuw i8, ptr %switch.select2.i, i64 24
  %6 = load ptr, ptr %5, align 8
  call void %6(ptr noundef nonnull align 1 %_4.i1)
...
_ZN7example8dyn_dp_217hacede4ff08c4dd1fE.exit:
...
  %switch.selectcmp.i13 = icmp eq i32 %num.i3, 1
  %switch.select.i14 = select i1 %switch.selectcmp.i13, ptr @vtable.4, ptr @vtable.5
  %switch.selectcmp1.i15 = icmp eq i32 %num.i3, 0
  %switch.select2.i16 = select i1 %switch.selectcmp1.i15, ptr @vtable.2, ptr %switch.select.i14
  %12 = getelementptr inbounds nuw i8, ptr %switch.select2.i16, i64 24
  %13 = load ptr, ptr %12, align 8
  call void %13(ptr noundef nonnull align 1 %_4.i1)
```
- maybe?
	- inlined `speak()` code is called, but prefaced by switch statement (across relevant Animal subsets)

