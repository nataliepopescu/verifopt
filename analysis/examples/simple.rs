#![feature(ptr_metadata)]

pub trait Animal {
    fn speak(&self) -> usize;
}

pub struct Cat;
pub struct Dog;

impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }
}

impl Animal for Dog {
    fn speak(&self) -> usize {
        22222
    }
}

fn main() {
    let animal = Box::new(Cat {});
    let _s = animal.speak();
}
