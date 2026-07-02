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

// second path which only carries bird
#[inline(never)]
fn only_bird(n: usize, b: &dyn Animal) -> usize {
    let here = b.speak();
    match n {
        0 => here,
        _ => here + only_bird(n - 1, b),
    }
}

fn main() {
    let cat = Cat {};
    let dog = Fish {};
    let bird = Bird {};

    let a = alternate(3, &cat as &dyn Animal, &dog as &dyn Animal); // -> Cat
    let b = only_bird(2, &bird as &dyn Animal); // -> Bird

    black_box(a + b);
}
