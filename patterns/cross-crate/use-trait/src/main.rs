mod setup;
mod decide;

use decl_trait::{Animal, Bird};

use crate::decide::decide_simple;
use crate::setup::{Cat, Dog};

#[inline(never)]
fn noop() {
    println!("NOOP");
}

fn main() {
    //let num = decide_simple(3);
    let num = 3;

    let mut bird = Bird::new();
    let mut cat = Cat::new();
    let mut dog = Dog::new();
    dog.change_eyes();

    let a: &mut dyn Animal;
    if num == 1 {
        a = &mut cat;
    } else {
        a = &mut dog;
    }
    //a = &mut cat;

    noop();
    let res = decl_trait::do_thing(a);
    noop();
    println!("res: {}", res);
}

