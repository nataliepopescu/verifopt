use rand::Rng;

const BIRD_TYPE: usize = 0;
const CAT_TYPE: usize = 1;
const DOG_TYPE: usize = 2;

trait Animal {
    fn speak(&self);
    fn typeid(&self) -> usize;
}

fn get_animal() -> Box<dyn Animal> {
    let num: u32 = rand::rng().random_range(..2);
    if num == 0 {
        return Box::new(Cat { typeid: CAT_TYPE });
    } else {
        return Box::new(Dog { typeid: DOG_TYPE });
    }
}

struct Bird {
    typeid: usize,
}

struct Cat {
    typeid: usize,
}

struct Dog {
    typeid: usize,
}

impl Animal for Bird {
    fn speak(&self) {
        println!("chirp");
    }
    fn typeid(&self) -> usize {
        self.typeid
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }
    fn typeid(&self) -> usize {
        self.typeid
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("woof");
    }
    fn typeid(&self) -> usize {
        self.typeid
    }
}

// if copying into godbolt, make main `pub`
fn main() {
    let animal = get_animal();

    let typeid = animal.typeid();

    let rawptr = Box::into_raw(animal) as *const ();
    if typeid == 1 {
        unsafe {
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(rawptr);
            <Cat as Animal>::speak(cat);
        }
    } else if typeid == 2 {
        unsafe {
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(rawptr);
            <Dog as Animal>::speak(dog);
        }
    }
}
