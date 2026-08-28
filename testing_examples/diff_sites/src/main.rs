#![feature(ptr_metadata)]
#![allow(dead_code)]

use std::hint::black_box;

pub trait Animal {
    fn speak(&self) -> usize;
    fn walk(&self) -> usize;
}

#[inline(never)]
pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        Box::new(Cat {})
    } else {
        Box::new(Dog {})
    }
}

#[inline(always)]
pub fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {});
}

#[inline(always)]
pub fn get_dog() -> Box<dyn Animal> {
    return Box::new(Dog {});
}

pub struct Cat;
pub struct Dog;

#[votrace::trace]
impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }
    fn walk(&self) -> usize {
        33333
    }
}

#[votrace::trace]
impl Animal for Dog {
    fn speak(&self) -> usize {
        22222
    }
    fn walk(&self) -> usize {
        44444
    }
}

#[inline(never)]
fn first_speak(a: Box<dyn Animal>) -> usize {
    a.speak()
}

#[inline(never)]
fn second_speak(a: Box<dyn Animal>) -> usize {
    a.speak()
}

fn main() {
    let x = 0;
    let animal1 = get_animal(x);
    black_box(first_speak(animal1));

    let y = 3;
    let animal2 = get_animal(y);
    black_box(second_speak(animal2));
}
