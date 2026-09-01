#![feature(ptr_metadata)]
#![allow(dead_code)]

pub trait Animal {
    fn speak(&self) -> usize;
    fn walk(&self) -> usize;
}

pub struct Cat;
pub struct Dog;

//#[votrace::trace]
impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }
    fn walk(&self) -> usize {
        33333
    }
}

//#[votrace::trace]
impl Animal for Dog {
    fn speak(&self) -> usize {
        22222
    }
    fn walk(&self) -> usize {
        44444
    }
}

#[inline(never)]
pub fn wrap_dyn_call(
    animal: &Box<dyn Animal>,
) -> usize {
    animal.speak()
}
