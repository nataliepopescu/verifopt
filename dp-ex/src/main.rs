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

fn dyn_dp() {
    let a: &dyn Animal;

    let mut rng = rand::rng();
    if rng.random() {
        a = &Cat {}
    } else {
        a = &Dog {}
    }

    a.speak();
}

fn static_dp_rand() {
    let a: &dyn Animal;
    
    let mut rng = rand::rng();
    if rng.random() {
        a = &Cat {}
    } else {
        a = &Cat {}
    }

    a.speak();
}

fn static_dp_rand_dummy() {
    let cat;

    let mut rng = rand::rng();
    if rng.random() {}

    cat = &Cat {};

    cat.speak();
}

fn static_dp() {
    let cat = &Cat {};
    cat.speak();
}

fn main() {
    let iter = 10000;

    for _ in 0..iter {
        dyn_dp();
    }

    for _ in 0..iter {
        static_dp_rand();
    }

    for _ in 0..iter {
        static_dp_rand_dummy();
    }

    for _ in 0..iter {
        static_dp();
    }
}
