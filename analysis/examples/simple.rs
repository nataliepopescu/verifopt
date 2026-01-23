#![feature(ptr_metadata)]
#![allow(dead_code)]

pub trait Animal {
    fn speak(&self) -> usize;
}

pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        Box::new(Cat {})
    } else {
        Box::new(Dog {})
    }
}

pub fn many_args(first: usize, second: usize, third: usize, fourth: usize) -> usize {
    first + second + third + fourth
}

pub struct Cat;
pub struct Dog;

impl Cat {
    fn meow(&self) -> usize {
        0
    }
}

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
    // when the below line is uncommented, the speak call is resolved to
    // <Cat as Animal>::speak(), so interprocedural may indeed be the "spot"
    //let animal = Box::new(Cat {});

    // note that even when we pass in a statically-known value to `get_animal`,
    // the information from that function is not propagated, so `speak` remains
    // a dynamic dispatch
    let animal = get_animal(0);

    let _s = animal.speak();

    //let cat = Cat {};
    //cat.meow();
}
