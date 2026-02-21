#![feature(ptr_metadata)]
#![allow(dead_code)]

pub trait Animal {
    fn speak(&self) -> usize;
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

fn main() {
    /*
    let mut first_arg_opt: Option<String> = None;
    let mut start = false;
    for arg in std::env::args() {
        if start {
            first_arg_opt = Some(arg);
            break;
        } else {
            start = true;
        }
    }

    match first_arg_opt {
        None => println!("Pass in a number and see what happens!"),
        Some(first_arg) => {
            //println!("{:?}", first_arg);
            let animal = get_animal(first_arg.parse().unwrap());
            let cat = get_cat();
            let dog = get_dog();
            let _animal_vtable = core::ptr::metadata(&*animal);
            let _cat_vtable = core::ptr::metadata(&*cat);
            let _dog_vtable = core::ptr::metadata(&*dog);
            //println!("pre");
            let _res = animal.speak();
            //println!("post");
        }
    }
    */

    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            let animal = get_animal(args[1].parse().unwrap());
            let cat = get_cat();
            let _animal_vtable = core::ptr::metadata(&*animal);
            let _cat_vtable = core::ptr::metadata(&*cat);
            let _res = animal.speak();
        }
    }
}
