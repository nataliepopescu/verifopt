use std::hint::black_box;

pub trait Animal {
    fn speak(&self) -> usize;
    fn legs(&self) -> usize;
}

pub struct Cat;
impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }
    fn legs(&self) -> usize {
        4
    }
}

pub struct Fish;
impl Animal for Fish {
    fn speak(&self) -> usize {
        22222
    }
    fn legs(&self) -> usize {
        0
    }
}

pub struct Bird;
impl Animal for Bird {
    fn speak(&self) -> usize {
        33333
    }
    fn legs(&self) -> usize {
        2
    }
}

// recurses on n, alternating which animal is passed down
#[inline(never)]
fn alternate(n: usize, cur: &dyn Animal, other: &dyn Animal) -> usize {
    let here = cur.speak();
    match n {
        0 => here,
        _ => here + alternate(n - 1, other, cur),
    }
}

// second path which only carries one animal
#[inline(never)]
fn retain(n: usize, fixed: &dyn Animal) -> usize {
    let here = fixed.speak();
    match n {
        0 => here,
        _ => here + retain(n - 1, fixed),
    }
}

fn main() {
    let cat = Cat {};
    let fish = Fish {};
    let bird = Bird {};

    let b = alternate(3, &fish as &dyn Animal, &cat as &dyn Animal); // -> Fish
    let a = alternate(3, &cat as &dyn Animal, &fish as &dyn Animal); // -> Cat
    let c = retain(2, &bird as &dyn Animal); // -> Bird

    black_box(a + b + c);
}
