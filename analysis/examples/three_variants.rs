#![feature(ptr_metadata)]
#![allow(dead_code)]

use std::ptr::DynMetadata;

pub trait Animal {
    fn speak(&self) -> usize;
    fn walk(&self) -> usize;
}

#[inline(never)]
pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        Box::new(Bird {})
    } else if num == 1 {
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

pub struct Bird;
pub struct Cat;
pub struct Dog;

impl Animal for Bird {
    fn speak(&self) -> usize {
        11111
    }
    fn walk(&self) -> usize {
        44444
    }
}

impl Animal for Cat {
    fn speak(&self) -> usize {
        22222
    }
    fn walk(&self) -> usize {
        55555
    }
}

impl Animal for Dog {
    fn speak(&self) -> usize {
        33333
    }
    fn walk(&self) -> usize {
        66666
    }
}

#[inline(never)]
fn wrap_dyn_call(
    animal: &Box<dyn Animal>,
    _animal_vtable: DynMetadata<dyn Animal>,
    _bird_vtable: DynMetadata<dyn Animal>,
    _cat_vtable: DynMetadata<dyn Animal>,
) -> usize {
    animal.speak()
}

#[inline(never)]
fn noop(num: usize) {
    println!("NOOP {:?}", num);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            let x = args[1].parse().unwrap();
            let animal = get_animal(x);
            let bird = get_animal(0);
            let cat = get_animal(1);
            let animal_vtable = core::ptr::metadata(&*animal);
            let bird_vtable = core::ptr::metadata(&*bird);
            let cat_vtable = core::ptr::metadata(&*cat);
            let res = wrap_dyn_call(&animal, animal_vtable, bird_vtable, cat_vtable);
            println!("res: {:?}", res);
        }
    }
}
