use rand::Rng;
use std::any::Any;

trait Animal: Any {
    fn speak(&self);
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
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("woof");
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// if copying into godbolt, make main `pub`
fn main() {
    let animal = get_animal();

    if let Some(cat) = animal.as_any().downcast_ref::<Cat>() {
        <Cat as Animal>::speak(cat);
    }
    if let Some(dog) = animal.as_any().downcast_ref::<Dog>() {
        <Dog as Animal>::speak(dog);
    }
}
