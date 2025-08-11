use rand::Rng;
use std::any::Any;
trait Animal : Any {
    fn speak(&self);
    //fn type_id(&self) -> usize;
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
    //fn type_id(&self) -> usize {
    //    0
    //}
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }
    //fn type_id(&self) -> usize {
    //    1
    //}
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Animal for Dog {
    fn speak(&self) {
        println!("woof");
    }
    //fn type_id(&self) -> usize {
    //    2
    //}
    fn as_any(&self) -> &dyn Any {
        self
    }
}
fn main() {
    let animal = get_animal();
    //animal.speak();
    
    //if animal.type_id() == 1 {
    //    <Cat as Animal>::speak(&*animal);
    //} else if animal.type_id() == 2 {
    //    <Dog as Animal>::speak(&*animal);
    //}

    if let Some(cat) = animal.as_any().downcast_ref::<Cat>() {
        <Cat as Animal>::speak(cat);
    }
    if let Some(dog) = animal.as_any().downcast_ref::<Dog>() {
        <Dog as Animal>::speak(dog);
    }

    //match animal {
    //    Box(Cat {}) => {
    //        <Cat as Animal>::speak(&animal);
    //    },
    //    Box(Dog {}) => {
    //        <Dog as Animal>::speak(&animal);
    //    }
    //}
}
