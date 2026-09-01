#![feature(ptr_metadata)]
#![allow(dead_code)]

use std::hint::black_box;
use dep::{Animal, Cat, Dog, wrap_dyn_call};

#[inline(never)]
pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        Box::new(Cat {})
    } else {
        Box::new(Dog {})
    }
}

//#[inline(never)]
//fn noop(num: usize) {
//    println!("NOOP {:?}", num);
//}

fn main() {
    let x = 0;
    let animal = get_animal(x);
    let res = wrap_dyn_call(&animal);
    black_box(res);
}
