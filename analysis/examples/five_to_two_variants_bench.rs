#![feature(ptr_metadata)]
#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::ptr::DynMetadata;
use std::time::Instant;

pub trait Animal {
    fn speak(
        &self,
        b_ctr: &mut Ctr,
        c_ctr: &mut Ctr,
        d_ctr: &mut Ctr,
        e_ctr: &mut Ctr,
        f_ctr: &mut Ctr,
    ) -> usize;
}

#[inline(never)]
pub fn get_animal_small(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        Box::new(Bird {})
    } else {
        Box::new(Frog {})
    }
}

#[inline(never)]
pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        Box::new(Bird {})
    } else if num == 1 {
        Box::new(Cat {})
    } else if num == 2 {
        Box::new(Dog {})
    } else if num == 3 {
        Box::new(Elephant {})
    } else {
        Box::new(Frog {})
    }
}

//#[inline(always)]
//pub fn get_cat() -> Box<dyn Animal> {
//    return Box::new(Cat {});
//}

pub struct Bird;
pub struct Cat;
pub struct Dog;
pub struct Elephant;
pub struct Frog;

pub struct Ctr {
    ctr: usize,
}

impl Animal for Bird {
    fn speak(
        &self,
        b_ctr: &mut Ctr,
        _c_ctr: &mut Ctr,
        _d_ctr: &mut Ctr,
        _e_ctr: &mut Ctr,
        _f_ctr: &mut Ctr,
    ) -> usize {
        b_ctr.ctr += 1;
        0
    }
}

impl Animal for Cat {
    fn speak(
        &self,
        _b_ctr: &mut Ctr,
        c_ctr: &mut Ctr,
        _d_ctr: &mut Ctr,
        _e_ctr: &mut Ctr,
        _f_ctr: &mut Ctr,
    ) -> usize {
        c_ctr.ctr += 1;
        11111
    }
}

impl Animal for Dog {
    fn speak(
        &self,
        _b_ctr: &mut Ctr,
        _c_ctr: &mut Ctr,
        d_ctr: &mut Ctr,
        _e_ctr: &mut Ctr,
        _f_ctr: &mut Ctr,
    ) -> usize {
        d_ctr.ctr += 1;
        22222
    }
}

impl Animal for Elephant {
    fn speak(
        &self,
        _b_ctr: &mut Ctr,
        _c_ctr: &mut Ctr,
        _d_ctr: &mut Ctr,
        e_ctr: &mut Ctr,
        _f_ctr: &mut Ctr,
    ) -> usize {
        e_ctr.ctr += 1;
        33333
    }
}

impl Animal for Frog {
    fn speak(
        &self,
        _b_ctr: &mut Ctr,
        _c_ctr: &mut Ctr,
        _d_ctr: &mut Ctr,
        _e_ctr: &mut Ctr,
        f_ctr: &mut Ctr,
    ) -> usize {
        f_ctr.ctr += 1;
        44444
    }
}

//#[inline(always)]
fn wrap_dyn_call(
    animal: &Box<dyn Animal>,
    _animal_vtable: DynMetadata<dyn Animal>,
    _b_vtable: DynMetadata<dyn Animal>,
    _c_vtable: DynMetadata<dyn Animal>,
    _d_vtable: DynMetadata<dyn Animal>,
    _e_vtable: DynMetadata<dyn Animal>,
    b_ctr: &mut Ctr,
    c_ctr: &mut Ctr,
    d_ctr: &mut Ctr,
    e_ctr: &mut Ctr,
    f_ctr: &mut Ctr,
) -> usize {
    animal.speak(b_ctr, c_ctr, d_ctr, e_ctr, f_ctr)
}

fn bench(filename: &String, warmup: usize, runs: usize) -> std::io::Result<()> {
    let bird = get_animal(0);
    let b_vtable = core::ptr::metadata(&*bird);
    let cat = get_animal(1);
    let c_vtable = core::ptr::metadata(&*cat);
    let dog = get_animal(2);
    let d_vtable = core::ptr::metadata(&*dog);
    let elephant = get_animal(3);
    let e_vtable = core::ptr::metadata(&*elephant);

    let mut w_b_ctr: Ctr = Ctr { ctr: 0 };
    let mut w_c_ctr: Ctr = Ctr { ctr: 0 };
    let mut w_d_ctr: Ctr = Ctr { ctr: 0 };
    let mut w_e_ctr: Ctr = Ctr { ctr: 0 };
    let mut w_f_ctr: Ctr = Ctr { ctr: 0 };

    let mut b_ctr: Ctr = Ctr { ctr: 0 };
    let mut c_ctr: Ctr = Ctr { ctr: 0 };
    let mut d_ctr: Ctr = Ctr { ctr: 0 };
    let mut e_ctr: Ctr = Ctr { ctr: 0 };
    let mut f_ctr: Ctr = Ctr { ctr: 0 };

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
            b_vtable,
            c_vtable,
            d_vtable,
            e_vtable,
            &mut w_b_ctr,
            &mut w_c_ctr,
            &mut w_d_ctr,
            &mut w_e_ctr,
            &mut w_f_ctr,
        ));
    }

    // benchmark
    let start = Instant::now();
    for _ in 0..runs {
        let (animal, vtable) = animals.pop().unwrap();
        std::hint::black_box(wrap_dyn_call(
            &animal, vtable, b_vtable, c_vtable, d_vtable, e_vtable, &mut b_ctr, &mut c_ctr,
            &mut d_ctr, &mut e_ctr, &mut f_ctr,
        ));
    }
    let duration = start.elapsed().as_nanos();

    let mean = f64::from(duration as u32) / (runs as f64);

    println!("b ctr: {:?}", b_ctr.ctr);
    println!("c ctr: {:?}", c_ctr.ctr);
    println!("d ctr: {:?}", d_ctr.ctr);
    println!("e ctr: {:?}", e_ctr.ctr);
    println!("f ctr: {:?}", f_ctr.ctr);
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
            let cat = get_animal(1);
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
