use rand::Rng;

#[derive(Clone, PartialEq)]
enum AnimalType {
    Bird,
    Cat,
    Dog,
}

trait Animal {
    fn speak(&self);
    fn typeid(&self) -> AnimalType;
}

fn get_animal() -> Box<dyn Animal> {
    let num: u32 = rand::rng().random_range(..2);
    if num == 0 {
        return Box::new(Cat { typeid: AnimalType::Cat });
    } else {
        return Box::new(Dog { typeid: AnimalType::Dog });
    }
}

struct Bird {
    typeid: AnimalType,
}

struct Cat {
    typeid: AnimalType,
}

struct Dog {
    typeid: AnimalType,
}

impl Animal for Bird {
    fn speak(&self) {
        println!("chirp");
    }
    fn typeid(&self) -> AnimalType {
        self.typeid.clone()
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }
    fn typeid(&self) -> AnimalType {
        self.typeid.clone()
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("woof");
    }
    fn typeid(&self) -> AnimalType {
        self.typeid.clone()
    }
}

// if copying into godbolt, make main `pub`
fn main() {
    let animal = get_animal();

    let typeid = animal.typeid(); //.clone();

    let rawptr = Box::into_raw(animal) as *const ();
    if typeid == AnimalType::Cat {
        unsafe {
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(rawptr);
            <Cat as Animal>::speak(cat);
        }
    } else if typeid == AnimalType::Dog {
        unsafe {
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(rawptr);
            <Dog as Animal>::speak(dog);
        }
    }
}
