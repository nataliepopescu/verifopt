#![feature(ptr_metadata)]
#![allow(dead_code)]

//extern crate rand;
//
//use rand::Rng;

pub trait Animal {
    fn speak(&self) -> usize;
    fn walk(&self) -> usize;
}

pub fn get_animal(_num: usize) -> Box<dyn Animal> {
    Box::new(Cat {})
}

#[inline(always)]
pub fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {});
}

//pub fn many_args(first: usize, second: usize, third: usize, fourth: usize) -> usize {
//    first + second + third + fourth
//}

pub struct Cat;
//pub struct Dog;

impl Cat {
    fn meow(&self) -> usize {
        0
    }
}

impl Animal for Cat {
    fn speak(&self) -> usize {
        //println!("in cat speak!");
        11111
    }
    fn walk(&self) -> usize {
        33333
    }
}

//impl Animal for Dog {
//    fn speak(&self) -> usize {
//        //println!("in dog speak!");
//        22222
//    }
//    fn walk(&self) -> usize {
//        44444
//    }
//}

fn main() {
    // when the below line is uncommented, the speak call is resolved to
    // <Cat as Animal>::speak(), so interprocedural may indeed be the "spot"
    //let animal = Box::new(Cat {});

    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            let animal = get_animal(args[1].parse().unwrap());
            let cat = get_cat();
            let _animal_vtable = core::ptr::metadata(&*animal);
            let _cat_vtable = core::ptr::metadata(&*cat);
            //println!("pre");
            let _res = animal.speak();
            //println!("post");
        }
    }

    // note that even when we pass in a statically-known value to `get_animal`,
    // the information from that function is not propagated, so `speak` remains
    // a dynamic dispatch
    //let animal = get_animal(rand::rng().random_range(..2usize));
    //let animal_really_cat = get_animal(0);

    //let cat = Cat {};
    //cat.meow();
}
