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
    } else if num == 2 {
        Box::new(Dog {})
    } else if num == 3 {
        Box::new(Elephant {})
    } else {
        Box::new(Frog {})
    }
}

//#[inline(always)]
//pub fn get_cat() -> Box<dyn Animal> {
//    return Box::new(Cat {});
//}
//
//#[inline(always)]
//pub fn get_dog() -> Box<dyn Animal> {
//    return Box::new(Dog {});
//}

pub struct Bird;
pub struct Cat;
pub struct Dog;
pub struct Elephant;
pub struct Frog;

impl Animal for Bird {
    fn speak(&self) -> usize {
        11111
    }
    fn walk(&self) -> usize {
        22222
    }
}

impl Animal for Cat {
    fn speak(&self) -> usize {
        33333
    }
    fn walk(&self) -> usize {
        44444
    }
}

impl Animal for Dog {
    fn speak(&self) -> usize {
        55555
    }
    fn walk(&self) -> usize {
        66666
    }
}

impl Animal for Elephant {
    fn speak(&self) -> usize {
        77777
    }
    fn walk(&self) -> usize {
        88888
    }
}

impl Animal for Frog {
    fn speak(&self) -> usize {
        99999
    }
    fn walk(&self) -> usize {
        0
    }
}

#[inline(never)]
fn wrap_dyn_call(
    animal: &Box<dyn Animal>,
    _animal_vtable: DynMetadata<dyn Animal>,
    _bird_vtable: DynMetadata<dyn Animal>,
    _cat_vtable: DynMetadata<dyn Animal>,
    _dog_vtable: DynMetadata<dyn Animal>,
    _elephant_vtable: DynMetadata<dyn Animal>,
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
            let dog = get_animal(2);
            let elephant = get_animal(3);

            let animal_vtable = core::ptr::metadata(&*animal);
            let bird_vtable = core::ptr::metadata(&*bird);
            let cat_vtable = core::ptr::metadata(&*cat);
            let dog_vtable = core::ptr::metadata(&*dog);
            let elephant_vtable = core::ptr::metadata(&*elephant);

            let res = wrap_dyn_call(
                &animal,
                animal_vtable,
                bird_vtable,
                cat_vtable,
                dog_vtable,
                elephant_vtable,
            );
            println!("res: {:?}", res);
        }
    }
}
