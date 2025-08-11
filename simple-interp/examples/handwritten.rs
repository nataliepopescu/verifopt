use rand::Rng;
use std::any::Any;

trait Animal {
    fn speak(&self);
    fn type_id(&self) -> usize;
    fn as_any(&self) -> &dyn Any;
}

fn get_animal() -> Box<dyn Animal> {
    let num: u32 = rand::rng().random_range(..2);
    if num == 0 {
        return Box::new(Cat {});
    } else {
        return Box::new(Dog {});
    }
}

struct Bird {}
struct Cat {}
struct Dog {}

impl Animal for Bird {
    fn speak(&self) {
        println!("chirp");
    }
    fn type_id(&self) -> usize {
        0
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }
    fn type_id(&self) -> usize {
        1
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("woof");
    }
    fn type_id(&self) -> usize {
        2
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn any_safe() {
    let animal = get_animal();

    if let Some(cat) = animal.as_any().downcast_ref::<Cat>() {
        <Cat as Animal>::speak(cat);
    }
    if let Some(dog) = animal.as_any().downcast_ref::<Dog>() {
        <Dog as Animal>::speak(dog);
    }
}

fn typeid_transmute_unsafe() {
    let animal = get_animal();

    let ti = animal.type_id();

    let rawptr = Box::into_raw(animal) as *const ();
    if ti == 1 {
        unsafe {
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(rawptr);
            <Cat as Animal>::speak(cat);
        }
    } else if ti == 2 {
        unsafe {
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(rawptr);
            <Dog as Animal>::speak(dog);
        }
    }
}

// if copying into godbolt, make main `pub`
fn main() {
    any_safe();
    typeid_transmute_unsafe();
}
