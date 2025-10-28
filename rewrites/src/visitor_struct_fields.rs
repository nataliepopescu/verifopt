#![feature(ptr_metadata)]

use std::ptr::DynMetadata;

pub trait AnimalVisitor {
    fn receive_dog(&self, a: &dyn Animal) -> usize;
    fn receive_cat(&self, a: &dyn Animal) -> usize;
}

pub trait Animal {
    fn speak(&self) -> usize;
    fn visit(&self, av: &dyn AnimalVisitor) -> usize;
}

pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        return Box::new(Cat {
            age: 9,
            num_siblings: 11,
        });
    } else {
        return Box::new(Dog {
            age: 7,
            num_siblings: 3,
        });
    }
}

#[inline(always)]
pub fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {
        age: 8,
        num_siblings: 10,
    });
}

#[inline(always)]
pub fn get_dog() -> Box<dyn Animal> {
    return Box::new(Dog {
        age: 4,
        num_siblings: 2,
    });
}

pub struct Cat {
    pub age: usize,
    pub num_siblings: usize,
}

pub struct Dog {
    pub age: usize,
    pub num_siblings: usize,
}

impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }

    fn visit(&self, av: &dyn AnimalVisitor) -> usize {
        av.receive_cat(self)
    }
}

impl Animal for Dog {
    fn speak(&self) -> usize {
        22222
    }

    fn visit(&self, av: &dyn AnimalVisitor) -> usize {
        av.receive_dog(self)
    }
}

pub struct SpeakBetterDogs;

impl AnimalVisitor for SpeakBetterDogs {
    fn receive_dog(&self, _a: &dyn Animal) -> usize {
        44444
    }
    fn receive_cat(&self, a: &dyn Animal) -> usize {
        a.speak()
    }
}

pub fn run_best(animal: &dyn Animal, dc: &SpeakBetterDogs) -> usize {
    dc.receive_dog(animal)
}

pub fn run_not_rw(animal: Box<dyn Animal>, dc: &SpeakBetterDogs) -> usize {
    animal.visit(dc)
}

pub fn run_src_rw_into_raw(
    animal: Box<dyn Animal>,
    dc: &SpeakBetterDogs,
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
            let a = get_animal(args[1].parse().unwrap());
            let dc = &SpeakBetterDogs {};
            run_not_rw(a, dc);
        }
    }
}
