# Code patterns checked for flow-insensitive vtable usage

All patterns are compiled with `rustc 1.87.0` and 
`-C opt-level=3` (release build) via [godbolt](https://godbolt.org/).

MIR generally seems to emit vtable usage regardless, so in these examples we 
are looking more closely at the generated LLVM IR.

## List of general cases

impactful (affect generated vtable code):
- concrete type selection of single `dyn` type based on dynamic data (3-14)
- list of `dyn` type with different concrete types inserted (15-17)
- `#[inline(never)]` (6, 12, 13, 15)
- visitor pattern (18)

less impactful (don't really affect generated vtable code):
- splitting dynamic selection of concrete type from `dyn` call (5)
- different paths to dynamic dispatch (8, 10)
- adding fields to structs (7, 8, 11)
- adding functions to traits (4)

## Patterns research

### 1: no trait impls in scope, define `speak_all()` ✅

<details>

<summary>Source code</summary>

```rust
pub trait Animal {
    fn speak(&self);
}

#[unsafe(no_mangle)]
pub fn speak_all(animal: &dyn Animal) {
    animal.speak()
}
```

</details>

<details>

<summary>LLVM IR</summary>

```llvm
define void @speak_all(ptr noundef nonnull align 1 %animal.0, ptr noalias nocapture noundef readonly align 8 dereferenceable(32) %animal.1) unnamed_addr {
start:
  %0 = getelementptr inbounds nuw i8, ptr %animal.1, i64 24
  %1 = load ptr, ptr %0, align 8
  tail call void %1(ptr noundef nonnull align 1 %animal.0)
  ret void
}
```

</details>

although nothing in scope calls `speak_all()`.

rust trait object fat pointers are composed of two pointers, the first pointing
to the actual object/struct data and the second pointing to the vtable. in the
generated LLVM IR for `speak_all()`, we can see that first the vtable ptr is
loaded, and then called with the actual `animal` data pointer. 

### 2: 1 + trait impls in scope ✅

<details>

<summary>Source code</summary>

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

</details>

although nothing in scope calls `speak_all()`.

same IR as example (1).

### 3: trait impls in scope + randomly decide which Animal subtype to be ✅

<details>

<summary>Source code</summary>

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

</details>

<details>

<summary>LLVM IR</summary>

```llvm
@vtable.1 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"<example::Bird as example::Animal>::speak::h9ccf5f719d0cc105" }>, align 8
@vtable.2 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"<example::Cat as example::Animal>::speak::hd2637de08c1ab51a" }>, align 8
@vtable.3 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"<example::Dog as example::Animal>::speak::ha27122df413e6265" }>, align 8

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

</details>

a preceeding switch statement switches on the expected random values to 
determine which vtable ptr to use.

the vtable ptr then seems to be loaded into `%16`, but the argument to the
indirect call is a bit hard to parse. looking below at example (6) shows a similar
argument to the `speak_all()` function, which there is interpreted as a pointer
to the animal data. 

### 4: 3 + additional trait function ✅

<details>

<summary>Source code</summary>

```rust
use rand::Rng;

pub trait Animal {
    fn speak(&self);
    fn go(&self);
}

struct Bird {}
struct Cat {}
struct Dog {}

impl Animal for Bird {
    fn speak(&self) {
        println!("chirp");
    }
    fn go(&self) {
        println!("flying");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }
    fn go(&self) {
        println!("slowly creeping");
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("woof");
    }
    fn go(&self) {
        println!("running");
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

</details>

<details>

<summary>LLVM IR</summary>

```llvm
@vtable.1 = private unnamed_addr constant <{ [24 x i8], ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"<example::Bird as example::Animal>::speak::h9ccf5f719d0cc105", ptr @"<example::Bird as example::Animal>::go::hc9a0febe669aa53c" }>, align 8
@vtable.2 = private unnamed_addr constant <{ [24 x i8], ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"<example::Cat as example::Animal>::speak::hd2637de08c1ab51a", ptr @"<example::Cat as example::Animal>::go::h90660b822a464a59" }>, align 8
@vtable.3 = private unnamed_addr constant <{ [24 x i8], ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"<example::Dog as example::Animal>::speak::ha27122df413e6265", ptr @"<example::Dog as example::Animal>::go::hddbac414f2936661" }>, align 8

_ZN7example6dyn_dp17hc25549bf78d82057E.exit:
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_4.i)
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

the vtable pattern in the generated code is the same, just the vtables 
themselves have one more entry. 

</details>

### 5: 3 + call `speak_all()` in `dyn_dp()` ✅

<details>

<summary>Source code</summary>

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

</details>

`speak_all()` is generated and uses the vtable, but nothing in the generated IR 
calls it (although the source code does).

the actual `speak()` code is inlined into a vtable call with a switch statement 
preceeding it.

same IR as example (3).

### 6: annotate `speak_all()` with `#[inline(never)]` ✅

<details>

<summary>Source code</summary>

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

</details>

<details>

<summary>LLVM IR</summary>

```llvm
define void @speak_all(ptr noundef nonnull align 1 %animal.0, ptr noalias nocapture noundef readonly align 8 dereferenceable(32) %animal.1) unnamed_addr {
start:
  %0 = getelementptr inbounds nuw i8, ptr %animal.1, i64 24
  %1 = load ptr, ptr %0, align 8
  tail call void %1(ptr noundef nonnull align 1 %animal.0)
  ret void
}

_ZN7example6dyn_dp17hc25549bf78d82057E.exit:
  %switch.selectcmp.i = icmp eq i64 %result.sroa.0.0.i.i.i.i.i, 1
  %switch.select.i = select i1 %switch.selectcmp.i, ptr @vtable.2, ptr @vtable.3
  %switch.selectcmp1.i = icmp eq i64 %result.sroa.0.0.i.i.i.i.i, 0
  %switch.select2.i = select i1 %switch.selectcmp1.i, ptr @vtable.1, ptr %switch.select.i
  call void @speak_all(ptr noundef nonnull align 1 inttoptr (i64 1 to ptr), ptr noalias noundef nonnull readonly align 8 dereferenceable(32) %switch.select2.i)
  ret void
}
```

</details>

### 7: 3 + more interesting structs for `Bird`/`Cat`/`Dog` ✅

<details>

<summary>Source code</summary>

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

</details>

<details>

<summary>LLVM IR</summary>

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

</details>

vtable call is prefaced by two phi nodes, one to select the vtable ptr, and one 
to select the data ptr. 

adding another field to the structs (e.g. `name: &'static str`) does not change 
this.

### 8: calling `speak()` from different paths with different possible subsets of the Animal type ✅

<details>

<summary>Source code</summary>

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

</details>

<details>

<summary>LLVM IR</summary>

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

</details>

vtable call is prefaced by phi nodes as in example (7).

in each of `dyn_dp_1` and `dyn_dp_2`, the phi nodes only choose between the 
relevant subset of Animal vtable/data ptrs.

### 9: 7 + make constructors less interesting again ✅

<details>

<summary>Source code</summary>

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

</details>

<details>

<summary>LLVM IR</summary>

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

</details>

each `dyn_dp` function generates code similar to example (3), where comparisons
are only between relevant subsets of the Animal trait. 

### 10: different paths within same function ✅

<details>

<summary>Source code</summary>

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

fn dyn_dp_3() {
    let animal: &dyn Animal;

    let num: u32 = rand::rng().random_range(..2);

    if num == 0 {
        let num2: u32 = rand::rng().random_range(..3);

        if num2 == 0 {
            animal = &Bird {}
        } else if num2 == 1 {
            animal = &Cat {}
        } else {
            animal = &Dog {}
        }

        animal.speak();
    } else {
        let num2: u32 = rand::rng().random_range(..3);

        if num2 == 0 {
            animal = &Cat {}
        } else if num2 == 1 {
            animal = &Elephant {}
        } else {
            animal = &Frog {}
        }

        animal.speak();
    }
}

pub fn main() {
    dyn_dp_3();
}
```

</details>

<details>

<summary>LLVM IR</summary>

```llvm
"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hdc0c23f00f5f61f2E.exit27.i":
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7.i)
  %switch.selectcmp.i = icmp eq i32 %num2.i, 1
  %switch.select.i = select i1 %switch.selectcmp.i, ptr @vtable.2, ptr @vtable.3
  %switch.selectcmp6.i = icmp eq i32 %num2.i, 0
  %switch.select7.i = select i1 %switch.selectcmp6.i, ptr @vtable.1, ptr %switch.select.i
  br label %_ZN7example8dyn_dp_317h17329e8a324ba3dbE.exit

"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hdc0c23f00f5f61f2E.exit38.i":
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_12.i)
  %switch.selectcmp8.i = icmp eq i32 %num23.i, 1
  %switch.select9.i = select i1 %switch.selectcmp8.i, ptr @vtable.4, ptr @vtable.5
  %switch.selectcmp10.i = icmp eq i32 %num23.i, 0
  %switch.select11.i = select i1 %switch.selectcmp10.i, ptr @vtable.2, ptr %switch.select9.i
  br label %_ZN7example8dyn_dp_317h17329e8a324ba3dbE.exit

_ZN7example8dyn_dp_317h17329e8a324ba3dbE.exit:
  %switch.select11.sink.i = phi ptr [ %switch.select11.i, %"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hdc0c23f00f5f61f2E.exit38.i" ], [ %switch.select7.i, %"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hdc0c23f00f5f61f2E.exit27.i" ]
  %14 = getelementptr inbounds nuw i8, ptr %switch.select11.sink.i, i64 24
  %15 = load ptr, ptr %14, align 8
  call void %15(ptr noundef nonnull align 1 inttoptr (i64 1 to ptr))
  ret void
}
```

</details>

similar to example (9). 

### 11: 10 + make structs more interesting ✅

<details>

<summary>Source code</summary>

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

struct Elephant {
    num: u32,
}

struct Frog {
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

fn dyn_dp_3() {
    let animal: &dyn Animal;

    let bird = Bird { num: 0 };
    let cat = Cat { num: 1 };
    let dog = Dog { num: 2 };
    let elephant = Elephant { num: 3 };
    let frog = Frog { num: 4 };

    let num: u32 = rand::rng().random_range(..2);

    if num == 0 {
        let num2: u32 = rand::rng().random_range(..3);

        if num2 == 0 {
            animal = &bird;
        } else if num2 == 1 {
            animal = &cat;
        } else {
            animal = &dog;
        }

        animal.speak();
    } else {
        let num2: u32 = rand::rng().random_range(..3);

        if num2 == 0 {
            animal = &cat;
        } else if num2 == 1 {
            animal = &elephant;
        } else {
            animal = &frog;
        }

        animal.speak();
    }
}

pub fn main() {
    dyn_dp_3();
}
```

</details>

<details>

<summary>LLVM IR</summary>

```llvm
"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hdc0c23f00f5f61f2E.exit32.i":
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_20.i)
  switch i32 %num23.i, label %bb22.i [
    i32 0, label %_ZN7example8dyn_dp_317h17329e8a324ba3dbE.exit
    i32 1, label %bb21.i
  ]

bb21.i:
  br label %_ZN7example8dyn_dp_317h17329e8a324ba3dbE.exit

bb22.i:
  br label %_ZN7example8dyn_dp_317h17329e8a324ba3dbE.exit

_ZN7example8dyn_dp_317h17329e8a324ba3dbE.exit:
  %animal.sroa.10.1.sink.i = phi ptr [ @vtable.5, %bb22.i ], [ @vtable.4, %bb21.i ], [ @vtable.2, %"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hdc0c23f00f5f61f2E.exit32.i" ], [ @vtable.3, %bb11.i ], [ @vtable.2, %bb10.i ], [ @vtable.1, %"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hdc0c23f00f5f61f2E.exit21.i" ]
  %animal.sroa.0.1.sink.i = phi ptr [ %frog.i, %bb22.i ], [ %elephant.i, %bb21.i ], [ %cat.i, %"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hdc0c23f00f5f61f2E.exit32.i" ], [ %dog.i, %bb11.i ], [ %cat.i, %bb10.i ], [ %bird.i, %"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hdc0c23f00f5f61f2E.exit21.i" ]
  %14 = getelementptr inbounds nuw i8, ptr %animal.sroa.10.1.sink.i, i64 24
  %15 = load ptr, ptr %14, align 8
  call void %15(ptr noundef nonnull align 1 %animal.sroa.0.1.sink.i)
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %frog.i)
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %elephant.i)
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %dog.i)
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %cat.i)
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %bird.i)
  ret void
}
```

</details> 

### 12: call a wrapper function with two instances of Animal ✅

<details>

<summary>Source code</summary>

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

</details>

<details>

<summary>LLVM IR</summary>

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

</details>

### 13: 12 without `#[inline(never)]` on `foo` ❌

<details>

<summary>LLVM IR</summary>

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

</details>

### 14: 12 without either `#[inline(never)]`s ❌

<details>

<summary>LLVM IR</summary>

```llvm
define void @example::main::hf505c5b3ca9f4d81() unnamed_addr {
start:
  %_3.i2 = alloca [48 x i8], align 8
  %_3.i = alloca [48 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3.i)
  store ptr @alloc_23b1932c7395f328e595aeb0f19f8b75, ptr %_3.i, align 8
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
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3.i2)
  store ptr @alloc_544006a9c9003b2f7edd4d917a4edbaf, ptr %_3.i2, align 8
  %4 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 8
  store i64 1, ptr %4, align 8
  %5 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 32
  store ptr null, ptr %5, align 8
  %6 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %6, align 8
  %7 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 24
  store i64 0, ptr %7, align 8
  call void @std::io::stdio::_print::h83d703bcf3ee60d9(ptr noalias nocapture noundef nonnull align 8 dereferenceable(48) %_3.i2)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3.i2)
  ret void
}
```

makes sense that this doesn't use vtables b/c each individual call's `self` type 
can be determined statically. 

</details>

### 15: call `speak()` in a loop on a vector with only a single element (and thus a single Animal subtype) ⁇

<details>

<summary>Source code</summary>

```rust
use std::sync::Mutex;

#[unsafe(no_mangle)]
static my_vec: Mutex<Vec<Box<dyn Animal>>> = Mutex::new(vec![]);

trait Animal: Sync + Send {
    fn speak(&self);
}

struct Cat;
struct Dog;

impl Animal for Cat {
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
fn foo(xs: &[Box<dyn Animal>]) {
    for x in xs { x.speak() }
}

pub fn main() {
    my_vec.lock().unwrap().insert(0, Box::new(Cat));
    foo(&my_vec.lock().unwrap());
}
```

</details>

<details>

<summary>LLVM IR</summary>

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

</details>

### 16: 15 without `#[inline(never)]` ⁇

<details>

<summary>LLVM IR</summary>

```llvm
@vtable.1 = private constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"<example::Cat as example::Animal>::speak::h0a516f4740d196fb" }>, align 8

; still appending to vec

bb3:
  %len.i = load i64, ptr getelementptr inbounds nuw (i8, ptr @my_vec, i64 24), align 8

bb4:
  store ptr inttoptr (i64 1 to ptr), ptr %_25.i, align 8
  %14 = getelementptr inbounds nuw i8, ptr %_25.i, i64 8
  store ptr @vtable.1, ptr %14, align 8
  %new_len.i = add nuw nsw i64 %len.i, 1
  store i64 %new_len.i, ptr getelementptr inbounds nuw (i8, ptr @my_vec, i64 24), align 8
  br i1 %t.1.i8, label %_ZN3std4sync6poison4Flag4done17h10a53d883c6fda20E.exit.i.i, label %bb1.i.i.i

_ZN3std4sync6poison4Flag4done17h10a53d883c6fda20E.exit.i.i:
  %17 = atomicrmw xchg ptr @my_vec, i32 0 release, align 4
  %_8.i.i = icmp eq i32 %17, 2
  br i1 %_8.i.i, label %bb2.i.i, label %"core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<alloc::boxed::Box<dyn example::Animal>>>>::h859d8ef20401d38d.exit"

bb2.i.i:
  tail call void @std::sys::sync::mutex::futex::Mutex::wake::h0439a4c6ca014734(ptr noundef nonnull align 4 @my_vec)
  br label %"core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<alloc::boxed::Box<dyn example::Animal>>>>::h859d8ef20401d38d.exit"

"core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<alloc::boxed::Box<dyn example::Animal>>>>::h859d8ef20401d38d.exit":
  %18 = cmpxchg ptr @my_vec, i32 0, i32 1 acquire monotonic, align 4
  %19 = extractvalue { i32, i1 } %18, 1
  br i1 %19, label %bb3.i27, label %bb1.i26

bb3.i27:
  %20 = load atomic i64, ptr @std::panicking::panic_count::GLOBAL_PANIC_COUNT::h9539389daf418384 monotonic, align 8
  %_6.i.i28 = and i64 %20, 9223372036854775807
  %21 = icmp eq i64 %_6.i.i28, 0
  br i1 %21, label %"_ZN3std4sync6poison5mutex14Mutex$LT$T$GT$4lock17ha75d54c8dc20b42dE.exit32", label %bb6.i.i29

; calling unwrap code

"_ZN3std4sync6poison5mutex14Mutex$LT$T$GT$4lock17ha75d54c8dc20b42dE.exit32":
  %_5.sroa.0.0.i.i30 = phi i8 [ %24, %bb6.i.i29 ], [ 0, %bb3.i27 ]
  %25 = load atomic i8, ptr getelementptr inbounds nuw (i8, ptr @my_vec, i64 4) monotonic, align 4
  %.not49 = icmp eq i8 %25, 0
  br i1 %.not49, label %"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hb77972f0c771ab29E.exit", label %bb2.i

"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hb77972f0c771ab29E.exit":
  %t.1.i = trunc nuw i8 %_5.sroa.0.0.i.i30 to i1
  %_21 = load ptr, ptr getelementptr inbounds nuw (i8, ptr @my_vec, i64 16), align 8
  %len = load i64, ptr getelementptr inbounds nuw (i8, ptr @my_vec, i64 24), align 8
  %_15.i = getelementptr inbounds nuw %"alloc::boxed::Box<dyn Animal>", ptr %_21, i64 %len
  %_236.i = icmp eq i64 %len, 0
  br i1 %_236.i, label %bb8, label %bb4.i

; ok this seems like the loop body
; which is using something like a vtable call

bb4.i:
  %iter.sroa.0.07.i = phi ptr [ %_33.i, %.noexc ], [ %_21, %"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hb77972f0c771ab29E.exit" ]
  %_9.0.i = load ptr, ptr %iter.sroa.0.07.i, align 8
  %29 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.07.i, i64 8
  %_9.1.i = load ptr, ptr %29, align 8
  %30 = getelementptr inbounds nuw i8, ptr %_9.1.i, i64 24
  %31 = load ptr, ptr %30, align 8
  invoke void %31(ptr noundef nonnull align 1 %_9.0.i)
          to label %.noexc unwind label %cleanup3

; loop condition check

.noexc:
  %_33.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.07.i, i64 16
  %_23.i = icmp eq ptr %_33.i, %_15.i
  br i1 %_23.i, label %bb8, label %bb4.i
```

</details>

### 17: 16 + add a `Dog` to vector ✅

<details>

<summary>Source code</summary>

```rust
...

pub fn main() {
    my_vec.lock().unwrap().insert(0, Box::new(Cat));
    my_vec.lock().unwrap().insert(0, Box::new(Dog));
    foo(&my_vec.lock().unwrap());
}
```

</details>

<details>

<summary>LLVM IR</summary>

```llvm
"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hb77972f0c771ab29E.exit":
  %t.1.i = trunc nuw i8 %_5.sroa.0.0.i.i81 to i1
  %_31 = load ptr, ptr getelementptr inbounds nuw (i8, ptr @my_vec, i64 16), align 8
  %len = load i64, ptr getelementptr inbounds nuw (i8, ptr @my_vec, i64 24), align 8
  %_15.i = getelementptr inbounds nuw %"alloc::boxed::Box<dyn Animal>", ptr %_31, i64 %len
  %_236.i = icmp eq i64 %len, 0
  br i1 %_236.i, label %bb13, label %bb4.i

; loop body

bb4.i:
  %iter.sroa.0.07.i = phi ptr [ %_33.i, %.noexc ], [ %_31, %"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hb77972f0c771ab29E.exit" ]
  %_9.0.i = load ptr, ptr %iter.sroa.0.07.i, align 8
  %47 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.07.i, i64 8
  %_9.1.i = load ptr, ptr %47, align 8
  %48 = getelementptr inbounds nuw i8, ptr %_9.1.i, i64 24
  %49 = load ptr, ptr %48, align 8
  invoke void %49(ptr noundef nonnull align 1 %_9.0.i)
          to label %.noexc unwind label %cleanup5

; loop condition

.noexc:
  %_33.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.07.i, i64 16
  %_23.i = icmp eq ptr %_33.i, %_15.i
  br i1 %_23.i, label %bb13, label %bb4.i
```

</details>

### 18: visitor pattern example ✅

[Source
code](https://github.com/nataliepopescu/verifopt/blob/main/visitor-ex/visitor-use/src/main.rs)

<details>

<summary>LLVM IR</summary>

```llvm
; in visitor_decl crate

; <visitor_decl::Cat as visitor_decl::Animal>::visit
; Function Attrs: nonlazybind uwtable
define void @"_ZN58_$LT$visitor_decl..Cat$u20$as$u20$visitor_decl..Animal$GT$5visit17he353a6a328e6c372E"(ptr noalias noundef nonnull readonly align 1 %self, ptr noundef nonnull align 1 %av.0, ptr noalias nocapture noundef readonly align 8 dereferenceable(32) %av.1) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %av.1, i64 24
  %1 = load ptr, ptr %0, align 8, !invariant.load !3, !nonnull !3
  tail call void %1(ptr noundef nonnull align 1 %av.0, ptr noundef nonnull align 1 %self, ptr noalias noundef nonnull readonly align 8 dereferenceable(56) @vtable.0)
  ret void
}

; <visitor_decl::Dog as visitor_decl::Animal>::visit
; Function Attrs: nonlazybind uwtable
define void @"_ZN58_$LT$visitor_decl..Dog$u20$as$u20$visitor_decl..Animal$GT$5visit17h3fff94aa131ecb55E"(ptr noalias noundef nonnull readonly align 1 %self, ptr noundef nonnull align 1 %av.0, ptr noalias nocapture noundef readonly align 8 dereferenceable(32) %av.1) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %av.1, i64 24
  %1 = load ptr, ptr %0, align 8, !invariant.load !3, !nonnull !3
  tail call void %1(ptr noundef nonnull align 1 %av.0, ptr noundef nonnull align 1 %self, ptr noalias noundef nonnull readonly align 8 dereferenceable(56) @vtable.1)
  ret void
}

...

; in visitor_use crate

define noundef i32 @main(i32 %0, ptr %1) unnamed_addr #7 {
...

"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17haf5126768f72e27fE.exit7": ; preds = %bb2, %bb1.i.i.i6
  %9 = icmp sgt i32 %value.i.i.i9.i.i.i.i, -1
  %10 = select i1 %9, ptr @"_ZN58_$LT$visitor_decl..Cat$u20$as$u20$visitor_decl..Animal$GT$5visit17he353a6a328e6c372E", ptr @"_ZN58_$LT$visitor_decl..Dog$u20$as$u20$visitor_decl..Animal$GT$5visit17h3fff94aa131ecb55E"
  call void %10(ptr noundef nonnull align 1 inttoptr (i64 1 to ptr), ptr noundef nonnull align 1 inttoptr (i64 1 to ptr), ptr noalias noundef nonnull readonly align 8 dereferenceable(32) @vtable.4)
  ret void
```

</details>

get an indirect call to one of the `Cat` or `Dog` `visit()` methods, each of
which seems to use a vtable

