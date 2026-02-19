#![feature(ptr_metadata)]
#![allow(dead_code)]

pub trait Animal {
    fn speak(&self) -> usize;
    fn walk(&self) -> usize;
}

pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        Box::new(Cat {})
    } else {
        Box::new(Dog {})
    }
}

#[inline(always)]
pub fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {})
}

pub struct Cat {
    //speak_ctr: u64,
}

pub struct Dog {
    //speak_ctr: u64,
}

//impl Cat {
//    fn meow(&self) -> usize {
//        0
//    }
//}

impl Animal for Cat {
    fn speak(&self) -> usize {
        //self.speak_ctr += 1;
        11111
    }
    fn walk(&self) -> usize {
        33333
    }
}

impl Animal for Dog {
    fn speak(&self) -> usize {
        //self.speak_ctr += 1;
        22222
    }
    fn walk(&self) -> usize {
        44444
    }
}

fn main() {
    // when the below line is uncommented, the speak call is resolved to
    // <Cat as Animal>::speak(), so interprocedural may indeed be the "spot"
    //let animal = Box::new(Cat {});

    // note that even when we pass in a statically-known value to `get_animal`,
    // the information from that function is not propagated, so `speak` remains
    // a dynamic dispatch
    //let animal = get_animal(rand::rng().random_range(..2usize));
    let animal_really_cat = get_animal(0);
    let cat = get_cat();
    let _animal_vtable = core::ptr::metadata(&*animal_really_cat);
    let _cat_vtable = core::ptr::metadata(&*cat);
    let _res = animal_really_cat.speak();
}

