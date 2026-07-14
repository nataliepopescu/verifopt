use std::hint::black_box;

pub trait Animal {
    fn speak(&self) -> usize;
}

pub struct Cat;
impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }
}

pub struct Dog;
impl Animal for Dog {
    fn speak(&self) -> usize {
        22222
    }
}

pub struct Bird;
impl Animal for Bird {
    fn speak(&self) -> usize {
        33333
    }
}

fn two(y: &mut u32) {
    *y = 2;
}

fn main() {
    let mut x = 1;
    two(&mut x);

    let a: &dyn Animal =
        if x == 1 {
            &Cat {}
        } else {
            &Dog {}
        };

    black_box(a.speak());
}
