#![feature(ptr_metadata)]

use std::{os::linux::raw, ptr::DynMetadata};
use rand::Rng;

pub trait AnimalVisitor {
    fn receive_dog(&self, a: &dyn Animal) -> usize;
    fn receive_cat(&self, a: &dyn Animal) -> usize;
}

pub trait Animal {
    fn speak(&self) -> usize;
    fn visit(&self, av: &dyn AnimalVisitor) -> usize;
}

pub fn get_animal(num: usize) -> Box<dyn Animal> {
    // println!("Calling get_animal with num = {}. ", num);
    if num == 0 {
        return Box::new(Cat {});
    } else {
        return Box::new(Dog {});
    }
}

pub struct Cat;
pub struct Dog;

#[inline(always)]
pub fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {});
}

#[inline(always)]
pub fn get_dog() -> Box<dyn Animal> {
    return Box::new(Dog {});
}

impl Animal for Cat {
    fn speak(&self) -> usize {
        // println!("Calling speak in Animal for Cat");
        11111
    }

    fn visit(&self, av: &dyn AnimalVisitor) -> usize {
        // println!("Calling visit in Animal for Cat");
        av.receive_cat(self)
    }
}

impl Animal for Dog {
    fn speak(&self) -> usize {
        // println!("Calling speak in Animal for Dog");
        22222
    }

    fn visit(&self, av: &dyn AnimalVisitor) -> usize {
        // println!("Calling visit in Animal for Dog");
        av.receive_dog(self)
    }
}

pub struct SpeakBetterDogs;

impl AnimalVisitor for SpeakBetterDogs {
    fn receive_dog(&self, _a: &dyn Animal) -> usize {
        // println!("Calling receive_dog in AnimalVisitor for SpeakBetterDogs");
        44444
    }
    fn receive_cat(&self, a: &dyn Animal) -> usize {
        // println!("Calling receive_cat in AnimalVisitor for SpeakBetterDogs");
        a.speak()
    }
}

pub fn run_best(animal: &dyn Animal, dc: &SpeakBetterDogs) -> usize {
    // println!("Calling run_best");
    dc.receive_dog(animal)
}

pub fn run_best_cat(animal: &Cat, dc: &SpeakBetterDogs) -> usize {
    // println!("Calling run_best_cat");
    animal.speak()
}

pub fn run_not_rw(animal: Box<dyn Animal>, dc: &SpeakBetterDogs) -> usize {
    // println!("Calling run_not_rw");
    animal.visit(dc)
}

pub fn run_full_not_rw(animal: Box<dyn Animal>, dc: &dyn AnimalVisitor) -> usize {
    // println!("Calling run_full_not_rw");
    animal.visit(dc)
}

pub fn run_src_rw_into_raw(
    animal: Box<dyn Animal>,
    dc: &SpeakBetterDogs,
    animal_vtable: DynMetadata<dyn Animal>,
    cat_vtable: DynMetadata<dyn Animal>,
) -> usize {
    // println!("Calling run_src_rw_into_raw");
    let raw_animal = Box::into_raw(animal) as *const ();
    if animal_vtable == cat_vtable {
        unsafe {
            // FIXME right transformation?
            // 1: which visit method to call
            // 2: which receive method to call
            // 3?: some receive methods may also have dynamic dispatch, so here we start
            // all over again
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            <Cat as Animal>::speak(cat)
        }
    } else {
        unsafe {
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            dc.receive_dog(dog)
        }
    }
}

pub fn run_src_rw_transmutes(
    animal: Box<dyn Animal>,
    dc: &SpeakBetterDogs,
    animal_vtable: DynMetadata<dyn Animal>,
    cat_vtable: DynMetadata<dyn Animal>,
) -> usize {
    // println!("Calling run_src_rw_transmutes");
    if animal_vtable == cat_vtable {
        unsafe {
            let raw_animal: *const () =
                std::mem::transmute::<&Box<dyn Animal>, *const ()>(&animal);
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            <Cat as Animal>::speak(cat)
        }
    } else {
        unsafe {
            let raw_animal: *const () =
                std::mem::transmute::<&Box<dyn Animal>, *const ()>(&animal);
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            dc.receive_dog(dog)
        }
    }
}

// if copying into godbolt, make main `pub`
fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            println!("Running with argument: {}", args[1]);
            // print cat if arg is 0, dog otherwise
            if (0 == args[1].parse().unwrap()) {
                println!("so we're getting cat");
            } else {
                println!("so we're getting dog");
            }

            /* Setup to run `run_best` and `run_not_rw` */
            // let a = get_animal(args[1].parse().unwrap());
            // let dc = &SpeakBetterDogs {};
            // run_best(&*a, dc);
            // run_not_rw(a, dc);

            /* Setup to run `run_src_rw_into_raw` and `run_src_rw_transmutes` */
            let animal = get_animal(rand::rng().random_range(..2usize));
            let sbd = &SpeakBetterDogs {};
            let cat = get_cat();
            let animal_vtable = core::ptr::metadata(&*animal);
            let cat_vtable = core::ptr::metadata(&*cat);
            run_src_rw_into_raw(animal, sbd, animal_vtable, cat_vtable);
            // run_src_rw_transmutes(animal, sbd, animal_vtable, cat_vtable);


            /* Setup to run `run_best_cat` */
            // let cat = Box::new(Cat {});
            // let dc = &SpeakBetterDogs {};
            // run_best_cat(&cat, dc);
        }
    }
}
