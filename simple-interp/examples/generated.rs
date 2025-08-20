// manually added from here...
use rand::Rng;
#[derive(Clone, PartialEq)]
enum Type {
    Cat,
    Dog,
}
// ...to here
trait Animal {
    fn typeid(&self) -> Type;
    fn speak(&self);
}
// manually modified from here...
fn get_animal() -> Box<dyn Animal> {
    let num: u32 = rand::rng().random_range(..2);
    if num == 0 {
        return Box::new(Cat { typeid: Type::Cat });
    } else {
        return Box::new(Dog { typeid: Type::Dog });
    }
}
// ...to here
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
    fn typeid(&self) -> Type {
        self.typeid.clone()
    }
    fn speak(&self) {
        println!("chirp");
    }
}
impl Animal for Cat {
    fn typeid(&self) -> Type {
        self.typeid.clone()
    }
    fn speak(&self) {
        println!("meow");
    }
}
impl Animal for Dog {
    fn typeid(&self) -> Type {
        self.typeid.clone()
    }
    fn speak(&self) {
        println!("woof");
    }
}
fn main() {
    let animal = get_animal();
    let typeid = animal.typeid();
    let rawptr = Box::into_raw(animal) as *const ();
    if typeid == Type::Cat {
        unsafe {
            let animal: &Cat = std::mem::transmute::<*const (), &Cat>(rawptr);
            <Cat as Animal>::speak(animal);
        }
    }
    if typeid == Type::Dog {
        unsafe {
            let animal: &Dog = std::mem::transmute::<*const (), &Dog>(rawptr);
            <Dog as Animal>::speak(animal);
        }
    }
}
