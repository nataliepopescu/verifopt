#![feature(ptr_metadata)]
#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::ptr::DynMetadata;
use std::time::Instant;

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

// iterative mean alg (handle overflow)
//fn mean(times: Vec<u128>) -> f64 {
//    let mut mean: f64 = 0.0;
//    let mut i = 1;
//    for time in times.iter() {
//        let diff = f64::from(*time as u32) - mean;
//        mean += diff / (i as f64);
//        i += 1;
//    }
//    mean
//}

#[inline(never)]
fn wrap_dyn_call(
    animal: &Box<dyn Animal>,
    _animal_vtable: DynMetadata<dyn Animal>,
    _cat_vtable: DynMetadata<dyn Animal>,
) -> usize {
    animal.speak()
}

fn bench(filename: &String, warmup: usize, runs: usize) -> std::io::Result<()> {
    let cat = get_cat();
    let cat_vtable = core::ptr::metadata(&*cat);

    //let mut w_cat_ctr: Ctr = Ctr { ctr: 0 };
    //let mut w_dog_ctr: Ctr = Ctr { ctr: 0 };
    //let mut cat_ctr: Ctr = Ctr { ctr: 0 };
    //let mut dog_ctr: Ctr = Ctr { ctr: 0 };

    let mut file = File::open(filename)?;
    let mut animals = Vec::new();

    // setup
    for _ in 0..(warmup + runs) {
        let mut buf: [u8; 1] = [0; 1];
        file.read_exact(&mut buf)?;
        let b = buf[0] & 1;
        let animal = get_animal(b.into());
        let vtable = core::ptr::metadata(&*animal);
        animals.push((animal, vtable));
    }

    // warmup
    for _ in 0..warmup {
        let (animal, vtable) = animals.pop().unwrap();
        std::hint::black_box(wrap_dyn_call(
            &animal,
            vtable,
            cat_vtable,
        ));
    }

    // benchmark
    let start = Instant::now();
    for _ in 0..warmup {
        let (animal, vtable) = animals.pop().unwrap();
        std::hint::black_box(wrap_dyn_call(
            &animal,
            vtable,
            cat_vtable,
        ));
    }
    let duration = start.elapsed().as_nanos();

    let mean = f64::from(duration as u32) / (runs as f64);

    println!("mean (ns): {:?}", mean);

    Ok(())
}

#[inline(never)]
fn noop(num: usize) {
    println!("NOOP {:?}", num);
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            println!(
                "USAGE \nPass in:
                     \n\t(1) a filename to read from for bench input,
                     \n\t(2) a number of warmup runs,
                     \n\t(3) a number of actual runs"
            );
            Ok(())
        }
        _ => {
            let filename = &args[1];
            let warmup: usize = args[2].parse().unwrap();
            let runs: usize = args[3].parse().unwrap();
            println!("filename: {:?}", filename);
            println!("num warmup runs: {:?}", warmup);
            println!("num actual runs: {:?}", runs);
            //bench(filename, warmup, runs)

            // bench without nested functions

            let cat = get_cat();
            let _cat_vtable = core::ptr::metadata(&*cat);

            let mut file = File::open(filename)?;
            let mut animals = Vec::new();

            // setup
            for _ in 0..(warmup + runs) {
                let mut buf: [u8; 1] = [0; 1];
                file.read_exact(&mut buf)?;
                let b = buf[0] & 1;
                let animal = get_animal(b.into());
                let vtable = core::ptr::metadata(&*animal);
                animals.push((animal, vtable));
            }

            //println!("len: {:?}", animals.len());
            //noop();

            // warmup
            //let start_ = Instant::now();
            for _ in 0..warmup {
                //noop(69905);
                let (animal, _vtable) = animals.pop().unwrap();
                //noop(139810);
                std::hint::black_box(animal.speak());
                //noop(209715);
            }
            //let _duration_ = start_.elapsed().as_nanos();

            // benchmark
            let start = Instant::now();
            for _ in 0..runs {
                let (animal, _vtable) = animals.pop().unwrap();
                std::hint::black_box(animal.speak());
            }
            let duration = start.elapsed().as_nanos();

            let mean = f64::from(duration as u32) / (runs as f64);
            println!("mean (ns): {:?}", mean);

            Ok(())
        }
    }
}
