#![feature(ptr_metadata)]

pub trait Animal {
    fn speak(&self) -> usize;
}

pub struct Cat;

impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }
}

fn main() {
    let animal = Box::new(Cat {});
    let _s = animal.speak();
}
