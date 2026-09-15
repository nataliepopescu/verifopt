#![feature(ptr_metadata)]
#![allow(dead_code)]

use negative_ex::{inner_main, get_animal, wrap_dyn_call2};

fn main() {
    inner_main();

    let x = 0;
    let animal = get_animal(x);
    std::hint::black_box(wrap_dyn_call2(&*animal));
}
