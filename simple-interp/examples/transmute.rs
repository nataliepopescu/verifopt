#[derive(Clone, PartialEq)]
enum Type {
    Bird,
    Cat,
    Dog,
}

trait Animal {
    fn speak(&self);
    fn typeid(&self) -> Type;
}

fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        return Box::new(Cat { typeid: Type::Cat });
    } else {
        return Box::new(Dog { typeid: Type::Dog });
    }
}

struct Bird {
    typeid: Type,
}

struct Cat {
    typeid: Type,
}

struct Dog {
    typeid: Type,
}

impl Animal for Bird {
    fn speak(&self) {
        println!("chirp");
    }
    fn typeid(&self) -> Type {
        self.typeid.clone()
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }
    fn typeid(&self) -> Type {
        self.typeid.clone()
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("woof");
    }
    fn typeid(&self) -> Type {
        self.typeid.clone()
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
            let typeid = animal.typeid();
            let rawptr = Box::into_raw(animal) as *const ();
            if typeid == Type::Cat {
                unsafe {
                    let cat: &Cat = std::mem::transmute::<*const (), &Cat>(rawptr);
                    <Cat as Animal>::speak(cat);
                }
            } else if typeid == Type::Dog {
                unsafe {
                    let dog: &Dog = std::mem::transmute::<*const (), &Dog>(rawptr);
                    <Dog as Animal>::speak(dog);
                }
            }
        }
    }
}
