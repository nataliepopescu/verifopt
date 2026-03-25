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
        vis1_ctr: &mut Ctr,
        vis2_ctr: &mut Ctr,
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
        vis1_ctr: &mut Ctr,
        vis2_ctr: &mut Ctr,
    ) -> usize {
        println!("cat's visit()");
        av.visit_cat(self, cat_ctr, dog_ctr, vis1_ctr, vis2_ctr)
    }
}

impl Cat {
    pub fn purr(
        &self,
        cat_ctr: &mut Ctr,
        _dog_ctr: &mut Ctr,
        vis1_ctr: &mut Ctr,
        _vis2_ctr: &mut Ctr,
    ) -> usize {
        cat_ctr.ctr += 1;
        vis1_ctr.ctr += 1;
        999
    }
    pub fn pounce(
        &self,
        cat_ctr: &mut Ctr,
        _dog_ctr: &mut Ctr,
        _vis1_ctr: &mut Ctr,
        vis2_ctr: &mut Ctr,
    ) -> usize {
        cat_ctr.ctr += 1;
        vis2_ctr.ctr += 1;
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
        vis1_ctr: &mut Ctr,
        vis2_ctr: &mut Ctr,
    ) -> usize {
        println!("dog's visit()");
        av.visit_dog(self, cat_ctr, dog_ctr, vis1_ctr, vis2_ctr)
    }
}

impl Dog {
    pub fn fetch(
        &self,
        _cat_ctr: &mut Ctr,
        dog_ctr: &mut Ctr,
        vis1_ctr: &mut Ctr,
        _vis2_ctr: &mut Ctr,
    ) -> usize {
        dog_ctr.ctr += 1;
        vis1_ctr.ctr += 1;
        555
    }
    pub fn dig(
        &self,
        _cat_ctr: &mut Ctr,
        dog_ctr: &mut Ctr,
        _vis1_ctr: &mut Ctr,
        vis2_ctr: &mut Ctr,
    ) -> usize {
        dog_ctr.ctr += 1;
        vis2_ctr.ctr += 1;
        666
    }
}

/* VISITOR */

pub trait AnimalVisitor {
    fn visit_cat(
        &self,
        a: &Cat,
        cat_ctr: &mut Ctr,
        dog_ctr: &mut Ctr,
        vis1_ctr: &mut Ctr,
        vis2_ctr: &mut Ctr,
    ) -> usize;
    fn visit_dog(
        &self,
        a: &Dog,
        cat_ctr: &mut Ctr,
        dog_ctr: &mut Ctr,
        vis1_ctr: &mut Ctr,
        vis2_ctr: &mut Ctr,
    ) -> usize;
}

pub struct Visitor1;
pub struct Visitor2;

impl AnimalVisitor for Visitor1 {
    fn visit_dog(
        &self,
        d: &Dog,
        cat_ctr: &mut Ctr,
        dog_ctr: &mut Ctr,
        vis1_ctr: &mut Ctr,
        vis2_ctr: &mut Ctr,
    ) -> usize {
        println!("vis1's visit_dog()");
        d.fetch(cat_ctr, dog_ctr, vis1_ctr, vis2_ctr)
    }
    fn visit_cat(
        &self,
        c: &Cat,
        cat_ctr: &mut Ctr,
        dog_ctr: &mut Ctr,
        vis1_ctr: &mut Ctr,
        vis2_ctr: &mut Ctr,
    ) -> usize {
        println!("vis1's visit_cat()");
        c.purr(cat_ctr, dog_ctr, vis1_ctr, vis2_ctr)
    }
}

impl AnimalVisitor for Visitor2 {
    fn visit_dog(
        &self,
        d: &Dog,
        cat_ctr: &mut Ctr,
        dog_ctr: &mut Ctr,
        vis1_ctr: &mut Ctr,
        vis2_ctr: &mut Ctr,
    ) -> usize {
        println!("vis2's visit_dog()");
        d.dig(cat_ctr, dog_ctr, vis1_ctr, vis2_ctr)
    }
    fn visit_cat(
        &self,
        c: &Cat,
        cat_ctr: &mut Ctr,
        dog_ctr: &mut Ctr,
        vis1_ctr: &mut Ctr,
        vis2_ctr: &mut Ctr,
    ) -> usize {
        println!("vis2's visit_cat()");
        c.pounce(cat_ctr, dog_ctr, vis1_ctr, vis2_ctr)
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
    vis1_ctr: &mut Ctr,
    vis2_ctr: &mut Ctr,
) -> usize {
    println!("--wrap_dyn_call");
    animal.visit(
        av,
        av_vtable,
        vis1_vtable,
        cat_ctr,
        dog_ctr,
        vis1_ctr,
        vis2_ctr,
    )
}

fn bench(filename: &String, warmup: usize, runs: usize) -> std::io::Result<()> {
    let cat = get_cat();
    let cat_vtable = core::ptr::metadata(&*cat);

    let vis1 = get_visitor1();
    let vis1_vtable = core::ptr::metadata(&*vis1);

    let mut w_cat_ctr: Ctr = Ctr { ctr: 0 };
    let mut w_dog_ctr: Ctr = Ctr { ctr: 0 };
    let mut w_vis1_ctr: Ctr = Ctr { ctr: 0 };
    let mut w_vis2_ctr: Ctr = Ctr { ctr: 0 };

    let mut cat_ctr: Ctr = Ctr { ctr: 0 };
    let mut dog_ctr: Ctr = Ctr { ctr: 0 };
    let mut vis1_ctr: Ctr = Ctr { ctr: 0 };
    let mut vis2_ctr: Ctr = Ctr { ctr: 0 };

    let mut file = File::open(filename)?;
    let mut animals = Vec::new();

    // setup
    for _ in 0..(warmup + runs) {
        let mut buf: [u8; 1] = [0; 1];

        // read animal input, construct obj + get vtable
        file.read_exact(&mut buf)?;
        let b_a = buf[0] & 1;
        let animal = get_animal(b_a.into());
        let vtable = core::ptr::metadata(&*animal);
        println!("b_a: {:?}", b_a);
        if b_a == 0 {
            println!("should be CAT");
        } else {
            println!("should be DOG");
        }

        // read visitor input, construct obj + get vtable
        file.read_exact(&mut buf)?;
        let b_av = buf[0] & 1;
        let av = get_visitor(b_av.into());
        let av_vtable = core::ptr::metadata(&*av);
        println!("b_av: {:?}", b_av);
        if b_av == 0 {
            println!("should be VIS1");
        } else {
            println!("should be VIS2");
        }

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
            &mut w_vis1_ctr,
            &mut w_vis2_ctr,
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
            &mut vis1_ctr,
            &mut vis2_ctr,
        ));
    }
    let duration = start.elapsed().as_nanos();

    let mean = f64::from(duration as u32) / (runs as f64);

    println!("cat ctr: {:?}", cat_ctr.ctr);
    println!("dog ctr: {:?}", dog_ctr.ctr);
    println!("vis1 ctr: {:?}", vis1_ctr.ctr);
    println!("vis2 ctr: {:?}", vis2_ctr.ctr);
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
