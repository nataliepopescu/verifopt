#![feature(ptr_metadata)]

use std::ptr::DynMetadata;

/* DATA TO VISIT */

pub trait Animal {
    fn speak(&self) -> usize;
    fn visit(
        &self,
        av: Box<dyn AnimalVisitor>,
        av_vtable: DynMetadata<dyn AnimalVisitor>,
        vis1_vtable: DynMetadata<dyn AnimalVisitor>,
    ) -> usize;
}

pub struct Cat;
impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }

    //#[inline(never)]
    fn visit(
        &self,
        av: Box<dyn AnimalVisitor>,
        _av_vtable: DynMetadata<dyn AnimalVisitor>,
        _vis1_vtable: DynMetadata<dyn AnimalVisitor>,
    ) -> usize {
        av.visit_cat(self)
    }
}
// Cat specific methods
impl Cat {
    pub fn purr(&self) -> usize {
        999
    }
}

pub struct Dog;
impl Animal for Dog {
    fn speak(&self) -> usize {
        22222
    }

    //#[inline(never)]
    fn visit(
        &self,
        av: Box<dyn AnimalVisitor>,
        _av_vtable: DynMetadata<dyn AnimalVisitor>,
        _vis1_vtable: DynMetadata<dyn AnimalVisitor>,
    ) -> usize {
        av.visit_dog(self)
    }
}
// Dog specific methods
impl Dog {
    pub fn fetch(&self) -> usize {
        555
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
        d.fetch()
    }
    fn visit_cat(&self, c: &Cat) -> usize {
        c.purr()
    }
}

/* HELPERS */

pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        return Box::new(Cat {});
    } else {
        return Box::new(Dog {});
    }
}

pub fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {});
}

pub fn get_visitor() -> Box<dyn AnimalVisitor> {
    return Box::new(Visitor1 {});
}

pub fn get_visitor1() -> Box<dyn AnimalVisitor> {
    return Box::new(Visitor1 {});
}

/* Optimized Functions to Test */

#[inline(never)]
pub fn run_full_not_rw(
    animal: Box<dyn Animal>,
    _animal_vtable: DynMetadata<dyn Animal>,
    _cat_vtable: DynMetadata<dyn Animal>,
    av: Box<dyn AnimalVisitor>,
    av_vtable: DynMetadata<dyn AnimalVisitor>,
    vis1_vtable: DynMetadata<dyn AnimalVisitor>,
) -> usize {
    animal.visit(av, av_vtable, vis1_vtable)
}

// if copying into godbolt, make main `pub`
fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            println!("Running with argument: {}", args[1]);
            if 0 == args[1].parse().unwrap() {
                println!("so we're getting cat");
            } else {
                println!("so we're getting dog");
            }

            let animal = get_animal(args[1].parse().unwrap());
            let cat = get_cat();
            let animal_vtable = core::ptr::metadata(&*animal);
            let cat_vtable = core::ptr::metadata(&*cat);

            let av = get_visitor();
            let vis1 = get_visitor1();
            let av_vtable = core::ptr::metadata(&*av);
            let vis1_vtable = core::ptr::metadata(&*vis1);

            let res = run_full_not_rw(animal, animal_vtable, cat_vtable, av, av_vtable, vis1_vtable);
            println!("res: {:?}", res);
        }
    }
}
