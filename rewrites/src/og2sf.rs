#![feature(ptr_metadata)]

use std::ptr::DynMetadata;

pub trait Animal {
    fn speak(&self) -> usize;
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
}

impl Animal for Dog {
    fn speak(&self) -> usize {
        22222
    }
}

pub fn run_best(cat: &Cat) -> usize {
    <Cat as Animal>::speak(cat)
}

pub fn run_not_rw(animal: Box<dyn Animal>) -> usize {
    animal.speak()
}

pub fn run_src_rw_into_raw(
    animal: Box<dyn Animal>,
    animal_vtable: DynMetadata<dyn Animal>,
    cat_vtable: DynMetadata<dyn Animal>,
) -> usize {
    let raw_animal = Box::into_raw(animal) as *const ();

    if animal_vtable == cat_vtable {
        unsafe {
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            <Cat as Animal>::speak(cat)
        }
    } else {
        unsafe {
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            <Dog as Animal>::speak(dog)
        }
    }
}

pub fn run_src_rw_transmutes(
    animal: Box<dyn Animal>,
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
            <Dog as Animal>::speak(dog)
        }
    }
}

pub fn run_src_rw_into_raw_fallback(
    animal: Box<dyn Animal>,
    animal_vtable: DynMetadata<dyn Animal>,
    cat_vtable: DynMetadata<dyn Animal>,
    dog_vtable: DynMetadata<dyn Animal>,
) -> usize {
    if animal_vtable == cat_vtable {
        let raw_animal = Box::into_raw(animal) as *const ();
        unsafe {
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            <Cat as Animal>::speak(cat)
        }
    } else if animal_vtable == dog_vtable {
        let raw_animal = Box::into_raw(animal) as *const ();
        unsafe {
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            <Dog as Animal>::speak(dog)
        }
    } else {
        animal.speak()
    }
}

pub fn run_src_rw_transmutes_fallback(
    animal: Box<dyn Animal>,
    animal_vtable: DynMetadata<dyn Animal>,
    cat_vtable: DynMetadata<dyn Animal>,
    dog_vtable: DynMetadata<dyn Animal>,
) -> usize {
    if animal_vtable == cat_vtable {
        unsafe {
            let raw_animal: *const () =
                std::mem::transmute::<&Box<dyn Animal>, *const ()>(&animal);
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            <Cat as Animal>::speak(cat)
        }
    } else if animal_vtable == dog_vtable {
        unsafe {
            let raw_animal: *const () =
                std::mem::transmute::<&Box<dyn Animal>, *const ()>(&animal);
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            <Dog as Animal>::speak(dog)
        }
    } else {
        animal.speak()
    }
}

// if copying into godbolt, make main `pub`
fn main() {
    let args: Vec<String> = std::env::args().collect();
    //let cat: &Cat = &Cat {};

    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            let animal = get_animal(args[1].parse().unwrap());
            let cat = get_cat();
            let animal_vtable = core::ptr::metadata(&*animal);
            let cat_vtable = core::ptr::metadata(&*cat);
            let s = run_src_rw_into_raw(animal, animal_vtable, cat_vtable);
            println!("{}", s);
        },
    }
}
