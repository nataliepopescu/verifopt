#![feature(ptr_metadata)]
#![allow(dead_code)]

use std::time::Instant;

//extern crate rand;
//
//use rand::Rng;

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

//pub fn many_args(first: usize, second: usize, third: usize, fourth: usize) -> usize {
//    first + second + third + fourth
//}

pub struct Cat;
pub struct Dog;

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

impl Animal for Dog {
    fn speak(&self) -> usize {
        //println!("in dog speak!");
        22222
    }
    fn walk(&self) -> usize {
        44444
    }
}

fn wrap_dyn_call(animal: &Box<dyn Animal>) {
    let _res = animal.speak();
}

fn main() {
    // when the below line is uncommented, the speak call is resolved to
    // <Cat as Animal>::speak(), so interprocedural may indeed be the "spot"
    //let animal = Box::new(Cat {});

    // note that even when we pass in a statically-known value to `get_animal`,
    // the information from that function is not propagated, so `speak` remains
    // a dynamic dispatch
    //let animal = get_animal(rand::rng().random_range(..2usize));
    let animal_really_cat = get_animal(0);
    let cat = get_cat();
    let _animal_vtable = core::ptr::metadata(&*animal_really_cat);
    let _cat_vtable = core::ptr::metadata(&*cat);
    //let _res = animal_really_cat.speak();

    let warmup = 100000;
    let runs = 1000000;

    //println!("warmup: {:?}", warmup);
    //println!("runs: {:?}", runs);

    for _ in 0..warmup {
        wrap_dyn_call(&animal_really_cat);
    }

    let mut times = Vec::new();
    for _ in 0..runs {
        let start = Instant::now();
        wrap_dyn_call(&animal_really_cat);
        let duration = start.elapsed().as_nanos();
        times.push(duration);
    }

    // FIXME not handling overflow
    let sum: u128 = Iterator::sum(times.iter());
    //println!("sum: {:?}", sum);
    let mean = f64::from(sum as u32) / (times.len() as f64);
    println!("mean: {:?}", mean);

    //let cat = Cat {};
    //cat.meow();
}
