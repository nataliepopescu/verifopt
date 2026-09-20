#![feature(ptr_metadata)]
#![allow(dead_code)]

use twovariant_ex::{Animal, Cat, inner_main, wrap_dyn_call_from_outer};

fn main() {
    inner_main();

    //let x = 0;
    let animal: Box<dyn Animal> = Box::new(Cat {}); //get_animal(x);
    std::hint::black_box(wrap_dyn_call_from_outer(&*animal));
}
