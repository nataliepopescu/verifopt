#![feature(ptr_metadata)]
#![allow(dead_code)]

use std::time::Instant;

use rand::RngExt;

pub trait Animal {
    fn speak(&self, ctr: &mut Ctr) -> usize;
    fn walk(&self) -> usize;
}

pub fn get_animal(_num: usize) -> Box<dyn Animal> {
    Box::new(Cat {})
}

#[inline(always)]
pub fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {});
}

pub struct Cat;
pub struct Dog;

pub struct Ctr {
    ctr: usize,
}

impl Animal for Cat {
    fn speak(&self, ctr: &mut Ctr) -> usize {
        ctr.ctr += 1;
        11111
    }
    fn walk(&self) -> usize {
        33333
    }
}

impl Animal for Dog {
    fn speak(&self, ctr: &mut Ctr) -> usize {
        ctr.ctr += 1;
        22222
    }
    fn walk(&self) -> usize {
        44444
    }
}

#[inline(never)]
fn wrap_dyn_call(animal: &Box<dyn Animal>, ctr: &mut Ctr) {
    let _res = animal.speak(ctr);
}

const WARMUP: usize = 1000000;
const RUNS: usize = 10000000;

fn main() {
    let cat = get_cat();
    let _cat_vtable = core::ptr::metadata(&*cat);

    let mut warmup_ctr: Ctr = Ctr { ctr: 0 };
    let mut ctr: Ctr = Ctr { ctr: 0 };

    //let mut animal_arr_warmup = [const { None }; WARMUP];
    //let mut animal_vtable_arr_warmup = []; //Metadata; WARMUP];
    //let mut animal_arr = [const { None }; RUNS];
    //let mut animal_vtable_arr = []; //Metadata; RUNS];

    // initialize data structures

    /*
    for i in 0..WARMUP {
        let animal = get_animal(rand::rng().random_range(..2usize));
        animal_vtable_arr_warmup[i] = core::ptr::metadata(&*animal);
        animal_arr_warmup[i] = Some(animal);
    }

    for i in 0..RUNS {
        let animal = get_animal(rand::rng().random_range(..2usize));
        animal_vtable_arr[i] = core::ptr::metadata(&*animal);
        animal_arr[i] = Some(animal);
    }
    */

    // start benchmarking

    for _ in 0..WARMUP {
        let animal = get_animal(rand::rng().random_range(..2usize));
        let _vtable = core::ptr::metadata(&*animal);
        wrap_dyn_call(&animal, &mut warmup_ctr);
    }

    let mut times = Vec::new();
    for _ in 0..RUNS {
        let animal = get_animal(rand::rng().random_range(..2usize));
        let _vtable = core::ptr::metadata(&*animal);
        let start = Instant::now();
        wrap_dyn_call(&animal, &mut ctr);
        let duration = start.elapsed().as_nanos();
        times.push(duration);
    }

    // FIXME not handling overflow
    let sum: u128 = Iterator::sum(times.iter());
    let mean = f64::from(sum as u32) / (times.len() as f64);

    println!("ctr: {:?}", ctr.ctr);
    println!("mean: {:?}", mean);
}
