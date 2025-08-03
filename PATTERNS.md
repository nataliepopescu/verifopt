# Code patterns checked for flow-insensitive vtable usage

All patterns are compiled with `-C opt-level=3` (release build).

MIR generally seems to emit vtable usage regardless, so in these examples we are looking more closely at the generated LLVM IR (via [godbolt](https://godbolt.org/)).

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
```llvm
define void @speak_all(ptr noundef nonnull align 1 %animal.0, ptr noalias nocapture noundef readonly align 8 dereferenceable(32) %animal.1) unnamed_addr {
start:
  %0 = getelementptr inbounds nuw i8, ptr %animal.1, i64 24
  %1 = load ptr, ptr %0, align 8
  tail call void %1(ptr noundef nonnull align 1 %animal.0)
  ret void
}
```
- yes but nothing in scope calls `speak_all()`

### 2: add trait impls in scope

```rust
pub trait Animal {
    fn speak(&self);
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
- same IR as (1)

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
```llvm
_ZN7example6dyn_dp17hc25549bf78d82057E.exit:
  %switch.selectcmp.i = icmp eq i64 %result.sroa.0.0.i.i.i.i.i, 1
  %switch.select.i = select i1 %switch.selectcmp.i, ptr @vtable.2, ptr @vtable.3
  %switch.selectcmp1.i = icmp eq i64 %result.sroa.0.0.i.i.i.i.i, 0
  %switch.select2.i = select i1 %switch.selectcmp1.i, ptr @vtable.1, ptr %switch.select.i
  %15 = getelementptr inbounds nuw i8, ptr %switch.select2.i, i64 24
  %16 = load ptr, ptr %15, align 8
  call void %16(ptr noundef nonnull align 1 inttoptr (i64 1 to ptr))
  ret void
}
```
- maybe?
- actual `speak()` code is inlined into an indirect call with some sort of switch statement preceeding it
- switch statement switches on expected randum values to determine which vtable ptr to use

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

    speak_all(animal);
}

pub fn main() {
    dyn_dp();
}
```
- maybe?
- `speak_all()` uses vtable, but nothing calls it (although source code does)
- actual `speak()` code is inlined into an indirect call with a switch statement preceeding it
- same IR as (3)

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

    speak_all(animal);
}

pub fn main() {
    dyn_dp();
}
```
```llvm
_ZN7example6dyn_dp17hc25549bf78d82057E.exit:
  %switch.selectcmp.i = icmp eq i64 %result.sroa.0.0.i.i.i.i.i, 1
  %switch.select.i = select i1 %switch.selectcmp.i, ptr @vtable.2, ptr @vtable.3
  %switch.selectcmp1.i = icmp eq i64 %result.sroa.0.0.i.i.i.i.i, 0
  %switch.select2.i = select i1 %switch.selectcmp1.i, ptr @vtable.1, ptr %switch.select.i
  call void @speak_all(ptr noundef nonnull align 1 inttoptr (i64 1 to ptr), ptr noalias noundef nonnull readonly align 8 dereferenceable(32) %switch.select2.i)
  ret void
}
```
- maybe?
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
```llvm
_ZN7example6dyn_dp17hc25549bf78d82057E.exit:
  %animal.sroa.6.0.i = phi ptr [ @vtable.3, %bb7.i ], [ @vtable.2, %bb6.i ], [ @vtable.1, %"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hdc0c23f00f5f61f2E.exit9.i" ]
  %animal.sroa.0.0.i = phi ptr [ %dog.i, %bb7.i ], [ %cat.i, %bb6.i ], [ %bird.i, %"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hdc0c23f00f5f61f2E.exit9.i" ]
  %15 = getelementptr inbounds nuw i8, ptr %animal.sroa.6.0.i, i64 24
  %16 = load ptr, ptr %15, align 8
  call void %16(ptr noundef nonnull align 1 %animal.sroa.0.0.i)
  ret void
}
```
- maybe?
- vtable access is prefaced by two phi nodes, one for the vtable ptr, and one for the concrete struct ptr
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
```llvm
_ZN7example8dyn_dp_117hcda5620e0a50a933E.exit:
  %animal.sroa.6.0.i = phi ptr [ @vtable.3, %bb7.i ], [ @vtable.2, %bb6.i ], [ @vtable.1, %"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hdc0c23f00f5f61f2E.exit5.i" ]
  %animal.sroa.0.0.i = phi ptr [ %dog.i, %bb7.i ], [ %cat.i, %bb6.i ], [ %bird.i, %"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hdc0c23f00f5f61f2E.exit5.i" ]
  %11 = getelementptr inbounds nuw i8, ptr %animal.sroa.6.0.i, i64 24
  %12 = load ptr, ptr %11, align 8
  call void %12(ptr noundef nonnull align 1 %animal.sroa.0.0.i)
...

_ZN7example8dyn_dp_217hacede4ff08c4dd1fE.exit:
  %animal.sroa.6.0.i15 = phi ptr [ @vtable.5, %bb7.i17 ], [ @vtable.4, %bb6.i14 ], [ @vtable.2, %"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hdc0c23f00f5f61f2E.exit5.i13" ]
  %animal.sroa.0.0.i16 = phi ptr [ %frog.i, %bb7.i17 ], [ %elephant.i, %bb6.i14 ], [ %cat.i2, %"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hdc0c23f00f5f61f2E.exit5.i13" ]
  %24 = getelementptr inbounds nuw i8, ptr %animal.sroa.6.0.i15, i64 24
  %25 = load ptr, ptr %24, align 8
  call void %25(ptr noundef nonnull align 1 %animal.sroa.0.0.i16)
...
```
- maybe?
- vtable access is prefaced by two phi nodes, one for the vtable ptr, and one for the concrete animal struct ptr
- in each of `dyn_dp_1` and `dyn_dp_2`, the phi nodes only choose between the relevant subset of Animal type vtables/objects

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
  %switch.selectcmp.i = icmp eq i32 %num.i, 1
  %switch.select.i = select i1 %switch.selectcmp.i, ptr @vtable.2, ptr @vtable.3
  %switch.selectcmp1.i = icmp eq i32 %num.i, 0
  %switch.select2.i = select i1 %switch.selectcmp1.i, ptr @vtable.1, ptr %switch.select.i
  %5 = getelementptr inbounds nuw i8, ptr %switch.select2.i, i64 24
  %6 = load ptr, ptr %5, align 8
  call void %6(ptr noundef nonnull align 1 %_4.i1)
...

_ZN7example8dyn_dp_217hacede4ff08c4dd1fE.exit:
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

### 9

```rust
trait Animal {
    fn speak(&self);
}

struct Cat;
struct Dog;

impl Animal for Cat {
    #[inline(never)]
    fn speak(&self) {
        println!("Cat");
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("Dog");
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
fn foo(xs: &dyn Animal) {
    xs.speak();
}

pub fn main() {
    let xs: &dyn Animal = &Cat;
    foo(xs);
    let xs: &dyn Animal = &Dog;
    foo(xs);
}
```
```llvm
define void @foo(ptr noundef nonnull align 1 %xs.0, ptr noalias nocapture noundef readonly align 8 dereferenceable(32) %xs.1) unnamed_addr {
start:
  %0 = getelementptr inbounds nuw i8, ptr %xs.1, i64 24
  %1 = load ptr, ptr %0, align 8
  tail call void %1(ptr noundef nonnull align 1 %xs.0)
  ret void
}

define void @example::main::hf505c5b3ca9f4d81() unnamed_addr {
start:
  tail call void @foo(ptr noundef nonnull align 1 inttoptr (i64 1 to ptr), ptr noalias noundef nonnull readonly align 8 dereferenceable(32) @vtable.0)
  tail call void @foo(ptr noundef nonnull align 1 inttoptr (i64 1 to ptr), ptr noalias noundef nonnull readonly align 8 dereferenceable(32) @vtable.1)
  ret void
}
```
- maybe

### 10: 9 without `#[inline(never)]`

```llvm
define void @example::main::hf505c5b3ca9f4d81() unnamed_addr {
start:
  %_3.i = alloca [48 x i8], align 8
  tail call void @"<example::Cat as example::Animal>::speak::h0a516f4740d196fb"(ptr nonnull align 1 poison)
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3.i)
  store ptr @alloc_544006a9c9003b2f7edd4d917a4edbaf, ptr %_3.i, align 8
  %0 = getelementptr inbounds nuw i8, ptr %_3.i, i64 8
  store i64 1, ptr %0, align 8
  %1 = getelementptr inbounds nuw i8, ptr %_3.i, i64 32
  store ptr null, ptr %1, align 8
  %2 = getelementptr inbounds nuw i8, ptr %_3.i, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %2, align 8
  %3 = getelementptr inbounds nuw i8, ptr %_3.i, i64 24
  store i64 0, ptr %3, align 8
  call void @std::io::stdio::_print::h83d703bcf3ee60d9(ptr noalias nocapture noundef nonnull align 8 dereferenceable(48) %_3.i)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3.i)
  ret void
}
```
- no

### 11

```rust
use std::sync::Mutex;

#[unsafe(no_mangle)]
static my_vec: Mutex<Vec<Box<dyn Animal>>> = Mutex::new(vec![]);

trait Animal: Sync + Send {
    fn speak(&self);
}

struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("Cat");
    }
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Dog");
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
fn foo(xs: &[Box<dyn Animal>]) {
    for x in xs { x.speak() }
}

pub fn main() {
    my_vec.lock().unwrap().insert(0, Box::new(Cat));
    foo(&my_vec.lock().unwrap());
}
```
```llvm
define void @foo(ptr noalias noundef nonnull readonly align 8 %xs.0, i64 noundef %xs.1) unnamed_addr {
start:
  %_15 = getelementptr inbounds nuw %"alloc::boxed::Box<dyn Animal>", ptr %xs.0, i64 %xs.1
  %_236 = icmp eq i64 %xs.1, 0
  br i1 %_236, label %bb3, label %bb4

bb4:
  %iter.sroa.0.07 = phi ptr [ %_33, %bb4 ], [ %xs.0, %start ]
  %_33 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.07, i64 16
  %_9.0 = load ptr, ptr %iter.sroa.0.07, align 8
  %0 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.07, i64 8
  %_9.1 = load ptr, ptr %0, align 8
  %1 = getelementptr inbounds nuw i8, ptr %_9.1, i64 24
  %2 = load ptr, ptr %1, align 8
  tail call void %2(ptr noundef nonnull align 1 %_9.0)
  %_23 = icmp eq ptr %_33, %_15
  br i1 %_23, label %bb3, label %bb4

bb3:
  ret void
}

...

"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hb77972f0c771ab29E.exit":
  %_21 = load ptr, ptr getelementptr inbounds nuw (i8, ptr @my_vec, i64 16), align 8
  %len = load i64, ptr getelementptr inbounds nuw (i8, ptr @my_vec, i64 24), align 8
  invoke void @foo(ptr noalias noundef nonnull readonly align 8 %_21, i64 noundef %len)
          to label %bb8 unwind label %cleanup3
```
- maybe?

### 12: 11 without `#[inline(never)]`

```llvm
  %len.i = load i64, ptr getelementptr inbounds nuw (i8, ptr @my_vec, i64 24), align 8
...

bb6.i:
  %_25.i = load ptr, ptr getelementptr inbounds nuw (i8, ptr @my_vec, i64 16), align 8
  %_14.not.i = icmp eq i64 %len.i, 0
  br i1 %_14.not.i, label %bb4, label %bb7.i

bb7.i:
  %dst.i = getelementptr inbounds nuw i8, ptr %_25.i, i64 16
  %12 = shl nuw nsw i64 %len.i, 4
  tail call void @llvm.memmove.p0.p0.i64(ptr nonnull align 8 %dst.i, ptr nonnull align 8 %_25.i, i64 %12, i1 false)
  br label %bb4

bb4:
  store ptr inttoptr (i64 1 to ptr), ptr %_25.i, align 8
  %14 = getelementptr inbounds nuw i8, ptr %_25.i, i64 8
  store ptr @vtable.1, ptr %14, align 8
  %new_len.i = add nuw nsw i64 %len.i, 1
  store i64 %new_len.i, ptr getelementptr inbounds nuw (i8, ptr @my_vec, i64 24), align 8
  br i1 %t.1.i8, label %_ZN3std4sync6poison4Flag4done17h10a53d883c6fda20E.exit.i.i, label %bb1.i.i.i
```
- maybe?
- the `Cat` vtable is stored in %14, but never called... must be inlined somewhere but i can't identify it
