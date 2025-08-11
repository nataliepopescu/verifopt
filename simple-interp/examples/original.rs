use rand::Rng;

trait Animal {
    fn speak(&self);
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
    let animal = get_animal();
    animal.speak();
}
