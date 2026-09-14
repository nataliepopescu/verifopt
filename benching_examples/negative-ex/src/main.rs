#![feature(ptr_metadata)]
#![allow(dead_code)]

use std::hint::black_box;
use negative_ex::get_animal;

fn main() {
    let x = 0;
    let animal = get_animal(x);
    //let _cat = get_animal(0);
    //let _animal_vtable = core::ptr::metadata(&*animal);
    //let _cat_vtable = core::ptr::metadata(&*cat);
    black_box(animal.speak());
    //println!("res: {:?}", res);
}
