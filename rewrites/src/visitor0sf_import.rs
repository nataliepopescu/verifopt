#![feature(ptr_metadata)]

use visitor_decl::{Animal, AnimalVisitor, Cat, Dog};
use visitor_use::{SpeakBetterDogs, SpeakBetterCats};
use std::ptr::DynMetadata;

pub fn run_best(animal: &dyn Animal, dc: &SpeakBetterDogs) -> usize {
    dc.receive_dog(animal)
}

pub fn run_not_rw(animal: Box<dyn Animal>, dv: &SpeakBetterDogs) -> usize {
    animal.visit(dv)
}

pub fn run_src_rw_into_raw(
    animal: Box<dyn Animal>,
    dv: &SpeakBetterDogs,
    animal_vtable: DynMetadata<dyn Animal>,
    cat_vtable: DynMetadata<dyn Animal>,
) -> usize {
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
            dv.receive_dog(dog)
        }
    }
}

pub fn run_src_rw_transmutes(
    animal: Box<dyn Animal>,
    dv: &SpeakBetterDogs,
    animal_vtable: DynMetadata<dyn Animal>,
    cat_vtable: DynMetadata<dyn Animal>,
) -> usize {
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
            dv.receive_dog(dog)
        }
    }
}

// if copying into godbolt, make main `pub`
fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            let a = visitor_decl::get_animal(args[1].parse().unwrap());
            let dv = &SpeakBetterDogs {};
            run_not_rw(a, dv);
        }
    }
}
