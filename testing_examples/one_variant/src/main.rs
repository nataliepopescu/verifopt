#![feature(ptr_metadata)]
#![allow(dead_code)]

pub trait Animal {
    fn speak(&self) -> usize;
    fn walk(&self) -> usize;
}

pub fn get_animal(_num: usize) -> Box<dyn Animal> {
    Box::new(Cat {})
}

#[inline(always)]
pub fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {});
}

pub struct Cat;

impl Cat {
    fn meow(&self) -> usize {
        0
    }
}

impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }
    fn walk(&self) -> usize {
        33333
    }
}

#[inline(never)]
fn wrap_dyn_call(animal: &Box<dyn Animal>) -> usize {
    animal.speak()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            let animal = get_animal(args[1].parse().unwrap());
            let cat = get_cat();
            let _animal_vtable = core::ptr::metadata(&*animal);
            let _cat_vtable = core::ptr::metadata(&*cat);
            //let res = wrap_dyn_call(&animal);
            let res = animal.speak();
            println!("res: {:?}", res);
        }
    }
}
