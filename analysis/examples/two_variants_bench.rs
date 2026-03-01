#![feature(ptr_metadata)]
#![allow(dead_code)]

use std::time::Instant;

//use rand::RngExt;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

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

fn bench(filename: &String, warmup: usize, runs: usize) -> std::io::Result<()> {
    let cat = get_cat();
    let _cat_vtable = core::ptr::metadata(&*cat);

    let mut warmup_ctr: Ctr = Ctr { ctr: 0 };
    let mut ctr: Ctr = Ctr { ctr: 0 };

    let warmup_file = File::open(filename)?;
    let mut warmup_reader = BufReader::new(warmup_file);
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    // start benchmarking

    for _ in 0..warmup {
        let mut buf: [u8; 1] = [0; 1];
        warmup_reader.read_exact(&mut buf)?;
        let animal = get_animal(buf[0].into());
        let _vtable = core::ptr::metadata(&*animal);
        wrap_dyn_call(&animal, &mut warmup_ctr);
    }

    let mut times = Vec::new();
    for _ in 0..runs {
        let mut buf: [u8; 1] = [0; 1];
        reader.read_exact(&mut buf)?;
        let animal = get_animal(buf[0].into());
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
