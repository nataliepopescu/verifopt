use rand::prelude::*;

pub trait Animal {
    fn speak(&self);
}

struct Dog {}

impl Animal for Dog {
    fn speak(&self) {
        println!("woof");
    }
}

struct Cat {}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }
}

fn main() {
    let a: &dyn Animal;

    let mut rng = rand::rng();
    if rng.random() {
        a = &Cat {}
    } else {
        a = &Dog {}
    }

    a.speak();
}
