#![feature(core_intrinsics)]

trait Animal {
    fn speak(&self);
}

fn get_animal(num: usize) -> (Box<dyn Animal>, u128) {
    if num == 0 {
        return (Box::new(Cat {}), core::intrinsics::type_id::<Cat>());
    } else {
        return (Box::new(Dog {}), core::intrinsics::type_id::<Dog>());
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
            let (animal, typeid) = get_animal(num);
            let rawptr = Box::into_raw(animal) as *const ();

            if typeid == core::intrinsics::type_id::<Cat>() {
                unsafe {
                    let cat: &Cat = std::mem::transmute::<*const (), &Cat>(rawptr);
                    <Cat as Animal>::speak(cat);
                }
            } else if typeid == core::intrinsics::type_id::<Dog>() {
                unsafe {
                    let dog: &Dog = std::mem::transmute::<*const (), &Dog>(rawptr);
                    <Dog as Animal>::speak(dog);
                }
            }
        }
    }
}
