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

#[inline(never)]
fn wrap_dyn_call(animal: &Box<dyn Animal>, ctr: &mut Ctr) {
    let _res = animal.speak(ctr);
}

fn main() {
    let animal_really_cat = get_animal(rand::rng().random_range(..2usize));
    let cat = get_cat();
    let _animal_vtable = core::ptr::metadata(&*animal_really_cat);
    let _cat_vtable = core::ptr::metadata(&*cat);

    let mut warmup_ctr: Ctr = Ctr { ctr: 0 };
    let mut ctr: Ctr = Ctr { ctr: 0 };

    let warmup = 1000000;
    let runs = 10000000;

    for _ in 0..warmup {
        wrap_dyn_call(&animal_really_cat, &mut warmup_ctr);
    }

    let mut times = Vec::new();
    for _ in 0..runs {
        let start = Instant::now();
        wrap_dyn_call(&animal_really_cat, &mut ctr);
        let duration = start.elapsed().as_nanos();
        times.push(duration);
    }

    // FIXME not handling overflow
    let sum: u128 = Iterator::sum(times.iter());
    let mean = f64::from(sum as u32) / (times.len() as f64);

    println!("ctr: {:?}", ctr.ctr);
    println!("mean: {:?}", mean);
}
