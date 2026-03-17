#![feature(ptr_metadata)]
#![allow(dead_code)]

//use std::ptr::DynMetadata;

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

impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }
    fn walk(&self) -> usize {
        33333
    }
}

impl Animal for Dog {
    fn speak(&self) -> usize {
        22222
    }
    fn walk(&self) -> usize {
        44444
    }
}

//#[inline(never)]
//fn wrap_dyn_call(animal: &Box<dyn Animal>) -> usize {
//    animal.speak()
//}

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
            let cat = get_animal(0);
            let _animal_vtable = core::ptr::metadata(&*animal);
            let _cat_vtable = core::ptr::metadata(&*cat);
            //let res = wrap_dyn_call(&animal);
            //noop(01234);
            let res = animal.speak();
            //noop(56789);
            println!("res: {:?}", res);
        }
    }
}
