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

#[inline(never)]
fn wrap_dyn_call(animal: &dyn Animal) -> usize {
    animal.speak()
}
fn main() {
    let cat = Cat{};
    let res = wrap_dyn_call(&cat as &dyn Animal);
    black_box(res);
}
