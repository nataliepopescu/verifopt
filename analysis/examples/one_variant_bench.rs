#![feature(ptr_metadata)]
#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::ptr::DynMetadata;
use std::time::Instant;

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
fn wrap_dyn_call(
    animal: &Box<dyn Animal>,
    _animal_vtable: DynMetadata<dyn Animal>,
    _cat_vtable: DynMetadata<dyn Animal>,
    ctr: &mut Ctr,
) -> usize {
    animal.speak(ctr)
}

fn bench(filename: &String, warmup: usize, runs: usize) -> std::io::Result<()> {
    let cat = get_cat();
    let cat_vtable = core::ptr::metadata(&*cat);

    let mut w_ctr: Ctr = Ctr { ctr: 0 };
    let mut ctr: Ctr = Ctr { ctr: 0 };

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

    for _ in 0..warmup {
        let (animal, vtable) = animals.pop().unwrap();
        std::hint::black_box(wrap_dyn_call(&animal, vtable, cat_vtable, &mut w_ctr));
    }

    let start = Instant::now();
    for _ in 0..runs {
        let (animal, vtable) = animals.pop().unwrap();
        std::hint::black_box(wrap_dyn_call(&animal, vtable, cat_vtable, &mut ctr));
    }
    let duration = start.elapsed().as_nanos();

    let mean = f64::from(duration as u32) / (runs as f64);

    println!("ctr: {:?}", ctr.ctr);
    println!("mean (ns): {:?}", mean);

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

            //bench(filename, warmup, runs)

            // attempt to rewrite w/out traversing callee fns

            let cat = get_cat();
            let _cat_vtable = core::ptr::metadata(&*cat);

            let mut w_ctr: Ctr = Ctr { ctr: 0 };
            let mut ctr: Ctr = Ctr { ctr: 0 };

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

            for _ in 0..warmup {
                let (animal, _vtable) = animals.pop().unwrap();
                std::hint::black_box(animal.speak(&mut w_ctr)); //wrap_dyn_call(&animal, vtable, cat_vtable, &mut w_ctr));
            }

            let start = Instant::now();
            for _ in 0..runs {
                let (animal, _vtable) = animals.pop().unwrap();
                std::hint::black_box(animal.speak(&mut ctr)); //wrap_dyn_call(&animal, vtable, cat_vtable, &mut ctr));
            }
            let duration = start.elapsed().as_nanos();

            let mean = f64::from(duration as u32) / (runs as f64);

            println!("ctr: {:?}", ctr.ctr);
            println!("mean (ns): {:?}", mean);

            Ok(())
        }
    }
}
