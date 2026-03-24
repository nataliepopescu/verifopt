#![feature(ptr_metadata)]
#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::ptr::DynMetadata;
use std::time::Instant;

pub trait Animal {
    fn speak(&self) -> usize;
    fn visit(
        &self,
        av: &Box<dyn AnimalVisitor>,
        av_vtable: DynMetadata<dyn AnimalVisitor>,
        vis1_vtable: DynMetadata<dyn AnimalVisitor>,
        cat_ctr: &mut Ctr,
        dog_ctr: &mut Ctr,
    ) -> usize;
}

pub struct Ctr {
    ctr: usize,
}

/* Helpers */

pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        Box::new(Cat {})
    } else {
        Box::new(Dog {})
    }
}

//#[inline(always)]
pub fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {});
}

pub fn get_visitor(num: usize) -> Box<dyn AnimalVisitor> {
    if num == 0 {
        return Box::new(Visitor1 {});
    } else {
        return Box::new(Visitor2 {});
    }
}

pub fn get_visitor1() -> Box<dyn AnimalVisitor> {
    return Box::new(Visitor1 {});
}

/* Cat methods */

pub struct Cat;

impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }
    #[inline(always)]
    fn visit(
        &self,
        av: &Box<dyn AnimalVisitor>,
        _av_vtable: DynMetadata<dyn AnimalVisitor>,
        _vis1_vtable: DynMetadata<dyn AnimalVisitor>,
        cat_ctr: &mut Ctr,
        dog_ctr: &mut Ctr,
    ) -> usize {
        av.visit_cat(self, cat_ctr, dog_ctr)
    }
}

impl Cat {
    pub fn purr(&self, cat_ctr: &mut Ctr, _dog_ctr: &mut Ctr) -> usize {
        cat_ctr.ctr += 1;
        999
    }
    pub fn pounce(&self, cat_ctr: &mut Ctr, _dog_ctr: &mut Ctr) -> usize {
        cat_ctr.ctr += 1;
        888
    }
}

/* Dog methods */

pub struct Dog;

impl Animal for Dog {
    fn speak(&self) -> usize {
        22222
    }
    #[inline(always)]
    fn visit(
        &self,
        av: &Box<dyn AnimalVisitor>,
        _av_vtable: DynMetadata<dyn AnimalVisitor>,
        _vis1_vtable: DynMetadata<dyn AnimalVisitor>,
        cat_ctr: &mut Ctr,
        dog_ctr: &mut Ctr,
    ) -> usize {
        av.visit_dog(self, cat_ctr, dog_ctr)
    }
}

impl Dog {
    pub fn fetch(&self, _cat_ctr: &mut Ctr, dog_ctr: &mut Ctr) -> usize {
        dog_ctr.ctr += 1;
        555
    }
    pub fn dig(&self, _cat_ctr: &mut Ctr, dog_ctr: &mut Ctr) -> usize {
        dog_ctr.ctr += 1;
        666
    }
}

/* VISITOR */

pub trait AnimalVisitor {
    fn visit_cat(&self, a: &Cat, cat_ctr: &mut Ctr, dog_ctr: &mut Ctr) -> usize;
    fn visit_dog(&self, a: &Dog, cat_ctr: &mut Ctr, dog_ctr: &mut Ctr) -> usize;
}

pub struct Visitor1;
pub struct Visitor2;

impl AnimalVisitor for Visitor1 {
    fn visit_dog(&self, d: &Dog, cat_ctr: &mut Ctr, dog_ctr: &mut Ctr) -> usize {
        d.fetch(cat_ctr, dog_ctr)
    }
    fn visit_cat(&self, c: &Cat, cat_ctr: &mut Ctr, dog_ctr: &mut Ctr) -> usize {
        c.purr(cat_ctr, dog_ctr)
    }
}

impl AnimalVisitor for Visitor2 {
    fn visit_dog(&self, d: &Dog, cat_ctr: &mut Ctr, dog_ctr: &mut Ctr) -> usize {
        d.dig(cat_ctr, dog_ctr)
    }
    fn visit_cat(&self, c: &Cat, cat_ctr: &mut Ctr, dog_ctr: &mut Ctr) -> usize {
        c.pounce(cat_ctr, dog_ctr)
    }
}

/* benchmarking */

#[inline(always)]
fn wrap_dyn_call(
    animal: &Box<dyn Animal>,
    _animal_vtable: DynMetadata<dyn Animal>,
    _cat_vtable: DynMetadata<dyn Animal>,
    av: &Box<dyn AnimalVisitor>,
    av_vtable: DynMetadata<dyn AnimalVisitor>,
    vis1_vtable: DynMetadata<dyn AnimalVisitor>,
    cat_ctr: &mut Ctr,
    dog_ctr: &mut Ctr,
) -> usize {
    animal.visit(av, av_vtable, vis1_vtable, cat_ctr, dog_ctr)
}

fn bench(filename: &String, warmup: usize, runs: usize) -> std::io::Result<()> {
    let cat = get_cat();
    let cat_vtable = core::ptr::metadata(&*cat);

    let vis1 = get_visitor1();
    let vis1_vtable = core::ptr::metadata(&*vis1);

    let mut w_cat_ctr: Ctr = Ctr { ctr: 0 };
    let mut w_dog_ctr: Ctr = Ctr { ctr: 0 };
    let mut cat_ctr: Ctr = Ctr { ctr: 0 };
    let mut dog_ctr: Ctr = Ctr { ctr: 0 };

    let mut file = File::open(filename)?;
    let mut animals = Vec::new();

    // setup
    for _ in 0..(warmup + runs) {
        let mut buf: [u8; 1] = [0; 1];

        // read animal input, construct obj + get vtable
        file.read_exact(&mut buf)?;
        let b_a = buf[0] & 1;
        //println!("b_a: {:?}", b_a);
        let animal = get_animal(b_a.into());
        let vtable = core::ptr::metadata(&*animal);

        // read visitor input, construct obj + get vtable
        file.read_exact(&mut buf)?;
        let b_av = buf[0] & 1;
        //println!("b_av: {:?}", b_av);
        let av = get_visitor(b_av.into());
        let av_vtable = core::ptr::metadata(&*av);

        animals.push((animal, vtable, av, av_vtable));
    }

    // warmup
    for _ in 0..warmup {
        let (animal, vtable, av, av_vtable) = animals.pop().unwrap();
        std::hint::black_box(wrap_dyn_call(
            &animal,
            vtable,
            cat_vtable,
            &av,
            av_vtable,
            vis1_vtable,
            &mut w_cat_ctr,
            &mut w_dog_ctr,
        ));
    }

    // benchmark
    let start = Instant::now();
    for _ in 0..runs {
        let (animal, vtable, av, av_vtable) = animals.pop().unwrap();
        std::hint::black_box(wrap_dyn_call(
            &animal,
            vtable,
            cat_vtable,
            &av,
            av_vtable,
            vis1_vtable,
            &mut cat_ctr,
            &mut dog_ctr,
        ));
    }
    let duration = start.elapsed().as_nanos();

    let mean = f64::from(duration as u32) / (runs as f64);

    println!("cat ctr: {:?}", cat_ctr.ctr);
    println!("dog ctr: {:?}", dog_ctr.ctr);
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
            let warmup: usize = args[2].parse().unwrap();
            let runs: usize = args[3].parse().unwrap();
            println!("filename: {:?}", filename);
            println!("num warmup runs: {:?}", warmup);
            println!("num actual runs: {:?}", runs);
            bench(filename, warmup, runs)

            // bench without nested functions

            /*
            let cat = get_cat();
            let _cat_vtable = core::ptr::metadata(&*cat);

            let mut w_cat_ctr: Ctr = Ctr { ctr: 0 };
            let mut w_dog_ctr: Ctr = Ctr { ctr: 0 };
            let mut cat_ctr: Ctr = Ctr { ctr: 0 };
            let mut dog_ctr: Ctr = Ctr { ctr: 0 };

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
            //for _ in 0..warmup {
            //    let (animal, _vtable) = animals.pop().unwrap();
            //    std::hint::black_box(animal.speak(&mut w_cat_ctr, &mut w_dog_ctr));
            //    //std::hint::black_box(wrap_dyn_call(
            //    //    &animal,
            //    //    vtable,
            //    //    cat_vtable,
            //    //    &mut w_cat_ctr,
            //    //    &mut w_dog_ctr,
            //    //));
            //}

            // benchmark
            let start = Instant::now();
            for _ in 0..runs {
                let (animal, _vtable) = animals.pop().unwrap();
                std::hint::black_box(animal.speak(&mut cat_ctr, &mut dog_ctr));
                //std::hint::black_box(wrap_dyn_call(
                //    &animal,
                //    vtable,
                //    cat_vtable,
                //    &mut cat_ctr,
                //    &mut dog_ctr,
                //));
            }
            let duration = start.elapsed().as_nanos();

            let mean = f64::from(duration as u32) / (runs as f64);

            println!("cat ctr: {:?}", cat_ctr.ctr);
            println!("dog ctr: {:?}", dog_ctr.ctr);
            println!("mean (ns): {:?}", mean);

            Ok(())
            */
        }
    }
}
