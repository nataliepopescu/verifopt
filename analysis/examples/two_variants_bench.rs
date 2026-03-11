#![feature(ptr_metadata)]
#![allow(dead_code)]

use std::time::Instant;

//use rand::RngExt;

use std::io::prelude::*;
//use std::io::BufReader;
use std::fs::File;

pub trait Animal {
    fn speak(&self, cat_ctr: &mut Ctr, dog_ctr: &mut Ctr) -> usize;
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

pub struct Ctr {
    ctr: usize,
}

impl Animal for Cat {
    fn speak(&self, cat_ctr: &mut Ctr, _dog_ctr: &mut Ctr) -> usize {
        cat_ctr.ctr += 1;
        11111
    }
    fn walk(&self) -> usize {
        33333
    }
}

impl Animal for Dog {
    fn speak(&self, _cat_ctr: &mut Ctr, dog_ctr: &mut Ctr) -> usize {
        dog_ctr.ctr += 1;
        22222
    }
    fn walk(&self) -> usize {
        44444
    }
}

// iterative mean alg (handle overflow)
fn mean(times: Vec<u128>) -> f64 {
    let mut mean: f64 = 0.0;
    let mut i = 1;
    for time in times.iter() {
        let diff = f64::from(*time as u32) - mean;
        mean += diff / (i as f64);
        i += 1;
    }
    mean
}

#[inline(never)]
fn wrap_dyn_call(animal: &Box<dyn Animal>, cat_ctr: &mut Ctr, dog_ctr: &mut Ctr) -> usize {
    animal.speak(cat_ctr, dog_ctr)
}

fn bench(filename: &String, warmup: usize, runs: usize) -> std::io::Result<()> {
    let cat = get_cat();
    let _cat_vtable = core::ptr::metadata(&*cat);

    let mut w_cat_ctr: Ctr = Ctr { ctr: 0 };
    let mut w_dog_ctr: Ctr = Ctr { ctr: 0 };
    let mut cat_ctr: Ctr = Ctr { ctr: 0 };
    let mut dog_ctr: Ctr = Ctr { ctr: 0 };

    let mut warmup_file = File::open(filename)?;
    let mut file = File::open(filename)?;

    // start benchmarking

    for _ in 0..warmup {
        // setup
        let mut buf: [u8; 1] = [0; 1];
        warmup_file.read_exact(&mut buf)?;
        let b = buf[0] & 1;
        let animal = get_animal(b.into());
        let _vtable = core::ptr::metadata(&*animal);

        // bench
        std::hint::black_box(wrap_dyn_call(&animal, &mut w_cat_ctr, &mut w_dog_ctr));
    }

    let mut times = Vec::new();
    for _ in 0..runs {
        // setup
        let mut buf: [u8; 1] = [0; 1];
        file.read_exact(&mut buf)?;
        let b = buf[0] & 1;
        let animal = get_animal(b.into());
        let _vtable = core::ptr::metadata(&*animal);

        // bench
        let start = Instant::now();
        std::hint::black_box(wrap_dyn_call(&animal, &mut cat_ctr, &mut dog_ctr));
        let duration = start.elapsed().as_nanos();

        times.push(duration);
    }

    let mean = mean(times);

    println!("cat ctr: {:?}", cat_ctr.ctr);
    println!("dog ctr: {:?}", dog_ctr.ctr);
    println!("mean: {:?}", mean);

    Ok(())
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
            let warmup = args[2].parse().unwrap();
            let runs = args[3].parse().unwrap();
            println!("filename: {:?}", filename);
            println!("num warmup runs: {:?}", warmup);
            println!("num actual runs: {:?}", runs);
            bench(filename, warmup, runs)
        }
    }
}
