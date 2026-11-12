pub mod bench;

pub trait Animal {
    fn speak(&self) -> usize;
    /// Per-call work that depends on its argument, for the timing harness
    /// (src/bench.rs): a constant like speak() lets a devirtualized loop
    /// fold away entirely.
    fn work(&self, x: usize) -> usize;
}

//#[inline(never)]
//pub fn get_cat() -> Cat {
//    Cat {}
//}
//
//#[inline(never)]
//pub fn get_animal(num: usize) -> Box<dyn Animal> {
//    if num == 0 {
//        Box::new(Cat {})
//    } else {
//        Box::new(Dog {})
//    }
//}

pub struct Cat;
pub struct Dog;

impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }

    fn work(&self, x: usize) -> usize {
        x.wrapping_mul(3) ^ 0x5a
    }
}

impl Animal for Dog {
    fn speak(&self) -> usize {
        22222
    }

    fn work(&self, x: usize) -> usize {
        x.rotate_left(5).wrapping_add(7)
    }
}

#[inline(never)]
pub fn wrap_dyn_call_from_inner(a: &dyn Animal) -> usize {
    a.speak()
}

#[inline(never)]
pub fn wrap_dyn_call_from_outer(a: &dyn Animal) -> usize {
    a.speak()
}

//#[inline(never)]
//pub fn wrap_dyn_call(a: &dyn Animal) -> usize {
//    a.speak()
//}

#[inline(never)]
pub fn wrap_cat_call(c: &Cat) -> usize {
    c.speak()
}

// For the harness (src/bin/bench_twovariant.rs). Constructors and wrappers
// are #[inline(never)] so the binary's dispatch sites can't see which vtable
// they get, short of LTO. One wrapper per workload pattern: each dispatch
// site only ever sees that pattern's objects. The constructors return
// references, not boxes: the harness keeps its objects in an array, because
// the analysis loses elements written into a Vec's heap buffer.

#[inline(never)]
pub fn make_cat() -> &'static dyn Animal {
    &Cat {}
}

#[inline(never)]
pub fn make_animal(sel: usize) -> &'static dyn Animal {
    if sel == 0 { &Cat {} } else { &Dog {} }
}

#[inline(never)]
pub fn wrap_mono(a: &dyn Animal, x: usize) -> usize {
    a.work(x)
}

#[inline(never)]
pub fn wrap_alt(a: &dyn Animal, x: usize) -> usize {
    a.work(x)
}

#[inline(never)]
pub fn wrap_rand(a: &dyn Animal, x: usize) -> usize {
    a.work(x)
}

pub fn inner_main() {
    //let x = 0;
    let animal: Box<dyn Animal> = Box::new(Cat {}); //get_animal(x);
    std::hint::black_box(wrap_dyn_call_from_inner(&*animal));
}

