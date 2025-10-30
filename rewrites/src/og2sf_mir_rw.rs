#![feature(ptr_metadata)]

use std::ptr::DynMetadata;

pub trait Animal {
    fn kaeps(&self) -> usize;
}

pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        return Box::new(Cat {
            age: 9,
            num_siblings: 11,
        });
    } else {
        return Box::new(Dog {
            age: 7,
            num_siblings: 3,
        });
    }
}

#[inline(always)]
pub fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {
        age: 8,
        num_siblings: 10,
    });
}

#[inline(always)]
pub fn get_dog() -> Box<dyn Animal> {
    return Box::new(Dog {
        age: 4,
        num_siblings: 2,
    });
}

pub struct Cat {
    pub age: usize,
    pub num_siblings: usize,
}

pub struct Dog {
    pub age: usize,
    pub num_siblings: usize,
}

impl Animal for Cat {
    fn kaeps(&self) -> usize {
        11111
    }
}

impl Animal for Dog {
    fn kaeps(&self) -> usize {
        22222
    }
}

pub fn run(
    animal: Box<dyn Animal>,
    _animal_vtable: DynMetadata<dyn Animal>,
    _cat_vtable: DynMetadata<dyn Animal>,
) -> usize {
    animal.kaeps()
}

// if copying into godbolt, make main `pub`
fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            let animal = get_animal(args[1].parse().unwrap());
            let cat = get_cat();
            let animal_vtable = core::ptr::metadata(&*animal);
            let cat_vtable = core::ptr::metadata(&*cat);
            let r = run(animal, animal_vtable, cat_vtable);
            println!("{}", r);
        }
    }
}
