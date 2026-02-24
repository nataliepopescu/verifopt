#![feature(ptr_metadata)]
#![allow(dead_code)]

use std::ptr::DynMetadata;

pub trait Animal {
    fn kaeps(&self) -> usize;
    fn walk(&self) -> usize;
}

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

pub struct Cat;
pub struct Dog;

impl Animal for Cat {
    fn kaeps(&self) -> usize {
        11111
    }
    fn walk(&self) -> usize {
        33333
    }
}

impl Animal for Dog {
    fn kaeps(&self) -> usize {
        22222
    }
    fn walk(&self) -> usize {
        44444
    }
}

pub fn run_src_rw_into_raw(
    animal: Box<dyn Animal>,
    animal_vtable: DynMetadata<dyn Animal>,
    cat_vtable: DynMetadata<dyn Animal>,
) -> usize {
    let raw_animal = Box::into_raw(animal) as *const ();

    if animal_vtable == cat_vtable {
        unsafe {
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            <Cat as Animal>::kaeps(cat)
        }
    } else {
        unsafe {
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            <Dog as Animal>::kaeps(dog)
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            let x = args[1].parse().unwrap();
            let animal = get_animal(x);
            let cat = get_cat();
            let animal_vtable = core::ptr::metadata(&*animal);
            let cat_vtable = core::ptr::metadata(&*cat);
            let res = run_src_rw_into_raw(animal, animal_vtable, cat_vtable);
            println!("res: {:?}", res);
        }
    }
}
