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

    //#[inline(always)]
    fn visit(
        &self,
        av: Box<dyn AnimalVisitor>,
        _av_vtable: DynMetadata<dyn AnimalVisitor>,
        _vis1_vtable: DynMetadata<dyn AnimalVisitor>,
    ) -> usize {
        println!("cat's visit()");
        av.visit_cat(self)
    }
}
// Cat specific methods
impl Cat {
    pub fn purr(&self) -> usize {
        println!("purr");
        999
    }
    pub fn pounce(&self) -> usize {
        println!("pounce");
        888
    }
}

pub struct Dog;
impl Animal for Dog {
    fn speak(&self) -> usize {
        22222
    }

    //#[inline(always)]
    fn visit(
        &self,
        av: Box<dyn AnimalVisitor>,
        _av_vtable: DynMetadata<dyn AnimalVisitor>,
        _vis1_vtable: DynMetadata<dyn AnimalVisitor>,
    ) -> usize {
        println!("dog's visit()");
        av.visit_dog(self)
    }
}
// Dog specific methods
impl Dog {
    pub fn fetch(&self) -> usize {
        println!("fetch");
        555
    }
    pub fn dig(&self) -> usize {
        println!("dig");
        666
    }
}

/* VISITOR */

pub trait AnimalVisitor {
    fn visit_dog(&self, a: &Dog) -> usize;
    fn visit_cat(&self, a: &Cat) -> usize;
}
pub struct Visitor1;
pub struct Visitor2;

impl AnimalVisitor for Visitor1 {
    fn visit_dog(&self, d: &Dog) -> usize {
        println!("v1's visit_dog()");
        d.fetch()
    }
    fn visit_cat(&self, c: &Cat) -> usize {
        println!("v1's visit_cat()");
        c.purr()
    }
}

impl AnimalVisitor for Visitor2 {
    fn visit_dog(&self, d: &Dog) -> usize {
        println!("v2's visit_dog()");
        d.dig()
    }
    fn visit_cat(&self, c: &Cat) -> usize {
        println!("v2's visit_cat()");
        c.pounce()
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

/* Optimized Functions to Test */

//#[inline(always)]
pub fn run_full_not_rw(
    //_dummy: usize,
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
        1 | 2 => println!("Pass in two numbers and see what happens!"),
        _ => {
            println!("Running with arguments: {}, {}", args[1], args[2]);
            let animal_arg = args[1].parse().unwrap();
            let visitor_arg = args[2].parse().unwrap();

            if 0 == animal_arg {
                println!("so we're getting cat");
            } else {
                println!("so we're getting dog");
            }
            if 0 == visitor_arg {
                println!("so we're using visitor1");
            } else {
                println!("so we're using visitor2");
            }

            let animal = get_animal(animal_arg);
            let animal_vtable = core::ptr::metadata(&*animal);
            let cat: &dyn Animal = &Cat {}; //get_cat();
            let cat_vtable = core::ptr::metadata(&*cat);

            let av = get_visitor(visitor_arg);
            let vis1 = get_visitor1();
            let av_vtable = core::ptr::metadata(&*av);
            let vis1_vtable = core::ptr::metadata(&*vis1);

            println!("cat_vtable: {:?}", cat_vtable);
            println!("vis1_vtable: {:?}", vis1_vtable);

            let res = run_full_not_rw(
                //0,
                animal,
                animal_vtable,
                cat_vtable,
                av,
                av_vtable,
                vis1_vtable,
            );
            println!("res: {:?}", res);
        }
    }
}
