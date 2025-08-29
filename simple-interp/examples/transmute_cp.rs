#![feature(core_intrinsics)]

use std::any::{Any, TypeId};
use std::any::type_name_of_val;

trait Animal: Any {
    fn speak(&self);
}

fn get_animal(num: usize) -> Box<dyn Animal> { //, u128) {
    if num == 0 {
        return Box::new(Cat {}); //, core::intrinsics::type_id::<Cat>());
    } else {
        return Box::new(Dog {}); //, core::intrinsics::type_id::<Dog>());
    }
}

struct Bird {}

struct Cat {}

struct Dog {}

impl Animal for Bird {
    fn speak(&self) {
        println!("chirp");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("woof");
    }
}

// if copying into godbolt, make main `pub`
fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            let num = args[1].parse().unwrap();
            let animal = get_animal(num);
            let cat = Cat{};
            let dog = Dog{};
            println!("animal type: {:?}", type_name_of_val(&animal));
            unsafe {
                let (animal_pointer, animal_vtable) = std::mem::transmute_copy::<Box<dyn Animal>, (*const u8, *const usize)>(&animal);
                let (cat_pointer, cat_vtable) = std::mem::transmute_copy::<Cat, (*const u8, *const usize)>(&cat);
                let (dog_pointer, dog_vtable) = std::mem::transmute_copy::<Dog, (*const u8, *const usize)>(&dog);
                println!("animal_pointer: {:?}", animal_pointer);
                println!("animal_vtable: {:?}", animal_vtable);
                println!("cat_pointer: {:?}", cat_pointer);
                println!("cat_vtable: {:?}", cat_vtable);
                println!("dog_pointer: {:?}", dog_pointer);
                println!("dog_vtable: {:?}", dog_vtable);
                //println!("animal_ptr metadata: {:?}", core::intrinsics::ptr_metadata(animal_pointer));
                //println!("animal_vtable ptr metadata: {:?}", core::intrinsics::ptr_metadata(animal_vtable));
                //println!("+0: {:?}", *(animal_vtable));
                //println!("+1: {:?}", *(animal_vtable.add(1)));
                //println!("+2: {:?}", *(animal_vtable.add(2)));
                //println!("+3: {:?}", *(animal_vtable.add(3)));
                //println!("+4: {:?}", *(animal_vtable.add(4)));
                //println!("+5: {:?}", *(animal_vtable.add(5)));
                //println!("+6: {:?}", *(animal_vtable.add(6)));
                //println!("+7: {:?}", *(animal_vtable.add(7)));
            }
            let typeid = animal.type_id();
            let rawptr = Box::into_raw(animal) as *const ();
            //println!("dyn animal typeid: {:?}", typeid);
            //println!("cat typeid: {:?}", TypeId::of::<Cat>());
            //println!("dog typeid: {:?}", TypeId::of::<Dog>());

            if typeid == TypeId::of::<Cat>() { //core::intrinsics::type_id::<Cat>() {
                unsafe {
                    let cat: &Cat = std::mem::transmute::<*const (), &Cat>(rawptr);
                    <Cat as Animal>::speak(cat);
                }
            } else if typeid == TypeId::of::<Dog>() { //core::intrinsics::type_id::<Dog>() {
                unsafe {
                    let dog: &Dog = std::mem::transmute::<*const (), &Dog>(rawptr);
                    <Dog as Animal>::speak(dog);
                }
            }
        }
    }
}
