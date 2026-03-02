#![feature(ptr_metadata)]

use std::{os::linux::raw, ptr::DynMetadata};
use rand::Rng;

/* DATA TO VISIT */
pub trait Animal {
    fn speak(&self) -> usize;
    fn visit(&self, av: Box<dyn AnimalVisitor>) -> usize;
}

pub struct Cat;
impl Animal for Cat {
    fn speak(&self) -> usize {
        // println!("Calling speak in Animal for Cat");
        11111
    }

    fn visit(&self, av: Box<dyn AnimalVisitor>) -> usize {
        // println!("Calling visit in Animal for Cat");
        av.visit_cat(self)
    }
}
// Cat specific methods
impl Cat {
    pub fn purr(&self) -> usize {
        // println!("Calling purr in Cat");
        999
    }
    pub fn pounce(&self) -> usize {
        // println!("Calling pounce in Cat");
        333
    }
}

pub struct Dog;
impl Animal for Dog {
    fn speak(&self) -> usize {
        // println!("Calling speak in Animal for Dog");
        22222
    }

    fn visit(&self, av: Box<dyn AnimalVisitor>) -> usize {
        // println!("Calling visit in Animal for Dog");
        av.visit_dog(self)
    }
}
// Dog specific methods
impl Dog {
    pub fn fetch(&self) -> usize {
        // println!("Calling fetch in Dog");
        555
    }
    pub fn dig(&self) -> usize {
        // println!("Calling dig in Dog");
        777
    }
}

/* VISITOR */
pub trait AnimalVisitor {
    fn visit_dog(&self, a: &Dog) -> usize;
    fn visit_cat(&self, a: &Cat) -> usize;
}
pub struct Visitor1;

impl AnimalVisitor for Visitor1 {
    fn visit_dog(&self, d: &Dog) -> usize {
        // println!("Calling visit_dog in AnimalVisitor for Visitor1");
       d.fetch() 
    }
    fn visit_cat(&self, c: &Cat) -> usize {
        // println!("Calling visit_cat in AnimalVisitor for Visitor1");
        c.purr()
    }
}

pub struct Visitor2;

impl AnimalVisitor for Visitor2 {
    fn visit_dog(&self, d: &Dog) -> usize {
        // println!("Calling visit_dog in AnimalVisitor for Visitor1");
       d.dig() 
    }
    fn visit_cat(&self, c: &Cat) -> usize {
        // println!("Calling visit_cat in AnimalVisitor for Visitor1");
        c.pounce()
    }
}

pub fn run_no_dispatch(animal: &Dog, v: &Visitor1) -> usize {
    // println!("Calling run_best");
    <Visitor1 as AnimalVisitor>::visit_dog(v, animal)
}

pub fn run_animal_dispatch(animal: Box<dyn Animal>, v: Box<Visitor1>) -> usize {
    // println!("Calling run_animal_dispatch");
    animal.visit(v)
}

pub fn run_visitor_dispatch(animal: &Dog, v: Box<dyn AnimalVisitor>) -> usize {
    // println!("Calling run_visitor_dispatch");
    v.visit_dog(animal)
}

/* HELPERS */
pub fn get_animal(num: usize) -> Box<dyn Animal> {
    // println!("Calling get_animal with num = {}. ", num);
    if num == 0 {
        return Box::new(Cat {});
    } else {
        return Box::new(Dog {});
    }
}

pub fn get_visitor(num: usize) -> Box<dyn AnimalVisitor> {
    // println!("Calling get_visitor with num = {}. ", num);
    if num == 0 {
        return Box::new(Visitor1 {});
    } else {
        return Box::new(Visitor2 {});
    }
}

#[inline(always)]
pub fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {});
}

#[inline(always)]
pub fn get_dog() -> Box<dyn Animal> {
    return Box::new(Dog {});
}

/* Optimized Functions to Test */

pub fn run_best_cat(animal: &Cat, dc: &Visitor1) -> usize {
    // println!("Calling run_best_cat");
    animal.purr()
}

pub fn run_full_not_rw(animal: Box<dyn Animal>, dc: Box<dyn AnimalVisitor>) -> usize {
    // println!("Calling run_full_not_rw");
    animal.visit(dc)
}

pub fn run_src_rw_into_raw(
    animal: Box<dyn Animal>,
    visitor: Box<dyn AnimalVisitor>,
    animal_vtable: DynMetadata<dyn Animal>,
    visitor_vtable: DynMetadata<dyn AnimalVisitor>,
    cat_vtable: DynMetadata<dyn Animal>,
    visitor1_vtable: DynMetadata<dyn AnimalVisitor>,
) -> usize {
    // println!("Calling run_src_rw_into_raw");
    let raw_animal = Box::into_raw(animal) as *const ();
    let raw_visitor = Box::into_raw(visitor) as *const ();
    if animal_vtable == cat_vtable {
        if visitor_vtable == visitor1_vtable {
            unsafe {
                let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
                let visitor1: &Visitor1 = std::mem::transmute::<*const (), &Visitor1>(raw_visitor);
                // cat.purr()
                <Visitor1 as AnimalVisitor>::visit_cat(visitor1, cat)
            }
        } else {
            unsafe {
                let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
                let visitor2: &Visitor2 = std::mem::transmute::<*const (), &Visitor2>(raw_visitor);
                // cat.pounce()
                <Visitor2 as AnimalVisitor>::visit_cat(visitor2, cat)
            }
        }
    } else {
        if visitor_vtable == visitor1_vtable {
            unsafe {
                let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
                let visitor1: &Visitor1 = std::mem::transmute::<*const (), &Visitor1>(raw_visitor);
                <Visitor1 as AnimalVisitor>::visit_dog(visitor1, dog)
                // dog.fetch()
            }
        } else {
            unsafe {
                let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
                let visitor2: &Visitor2 = std::mem::transmute::<*const (), &Visitor2>(raw_visitor);
                <Visitor2 as AnimalVisitor>::visit_dog(visitor2, dog)
                // dog.dig()
            }
        }
    }
}

pub fn run_src_rw_transmutes(
    animal: Box<dyn Animal>,
    visitor: Box<dyn AnimalVisitor>,
    animal_vtable: DynMetadata<dyn Animal>,
    visitor_vtable: DynMetadata<dyn AnimalVisitor>,
    cat_vtable: DynMetadata<dyn Animal>,
    visitor1_vtable: DynMetadata<dyn AnimalVisitor>,
) -> usize {
    // println!("Calling run_src_rw_transmutes");
    if animal_vtable == cat_vtable {
        if visitor_vtable == visitor1_vtable {
            unsafe {
                let raw_animal: *const () =
                    std::mem::transmute::<&Box<dyn Animal>, *const ()>(&animal);
                let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
                let raw_visitor: *const () =
                    std::mem::transmute::<&Box<dyn AnimalVisitor>, *const ()>(&visitor);
                let visitor1: &Visitor1 = std::mem::transmute::<*const (), &Visitor1>(raw_visitor);
                // cat.purr()
                <Visitor1 as AnimalVisitor>::visit_cat(visitor1, cat)
            }
        } else {
            unsafe {
                let raw_animal: *const () =
                    std::mem::transmute::<&Box<dyn Animal>, *const ()>(&animal);
                let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
                let raw_visitor: *const () =
                    std::mem::transmute::<&Box<dyn AnimalVisitor>, *const ()>(&visitor);
                let visitor2: &Visitor2 = std::mem::transmute::<*const (), &Visitor2>(raw_visitor);
                // cat.pounce()
                <Visitor2 as AnimalVisitor>::visit_cat(visitor2, cat)
            }
        }
    } else {
        if visitor_vtable == visitor1_vtable {
            unsafe {
                let raw_animal: *const () =
                    std::mem::transmute::<&Box<dyn Animal>, *const ()>(&animal);
                let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
                let raw_visitor: *const () =
                    std::mem::transmute::<&Box<dyn AnimalVisitor>, *const ()>(&visitor);
                let visitor1: &Visitor1 = std::mem::transmute::<*const (), &Visitor1>(raw_visitor);
                // dog.fetch()
                <Visitor1 as AnimalVisitor>::visit_dog(visitor1, dog)
            }
        } else {
            unsafe {
                let raw_animal: *const () =
                    std::mem::transmute::<&Box<dyn Animal>, *const ()>(&animal);
                let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
                let raw_visitor: *const () =
                    std::mem::transmute::<&Box<dyn AnimalVisitor>, *const ()>(&visitor);
                let visitor2: &Visitor2 = std::mem::transmute::<*const (), &Visitor2>(raw_visitor);
                // dog.dig()
                <Visitor2 as AnimalVisitor>::visit_dog(visitor2, dog)
            }
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
            // let dc = &Visitor1{};
            // // run_best(&*a, dc);
            // run_not_rw(a, dc);

            /* Setup to run `run_src_rw_into_raw` and `run_src_rw_transmutes` */
            // let animal = get_animal(rand::rng().random_range(..2usize));
            let animal = get_animal(args[1].parse().unwrap());
            let sbd = get_visitor(args[1].parse().unwrap());
            let cat = get_cat();
            let visitor1 = get_visitor(0);
            let animal_vtable = core::ptr::metadata(&*animal);
            let sbd_vtable = core::ptr::metadata(&*sbd);
            let cat_vtable = core::ptr::metadata(&*cat);
            let visitor1_vtable = core::ptr::metadata(&*visitor1);
            // run_src_rw_into_raw(animal, sbd, animal_vtable, cat_vtable);
            run_src_rw_transmutes(animal, sbd, animal_vtable, sbd_vtable, cat_vtable, visitor1_vtable);


            /* Setup to run `run_best_cat` */
            // let cat = Box::new(Cat {});
            // let dc = &Visitor1{};
            // run_best_cat(&cat, dc);
        }
    }
}