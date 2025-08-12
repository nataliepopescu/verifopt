use rand::Rng; // added
trait Animal: std::any::Any {
    fn as_any(&self) -> &dyn std::any::Any;
    fn speak(&self);
}
fn get_animal() -> Box<dyn Animal> {
    // rettype + body modified
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn speak(&self) {
        println!("chirp");
    }
}
impl Animal for Cat {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn speak(&self) {
        println!("meow");
    }
}
impl Animal for Dog {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn speak(&self) {
        println!("woof");
    }
}
fn main() {
    let animal = get_animal();

    if let Some(animal) = animal.as_any().downcast_ref::<Cat>() {
        <Cat as Animal>::speak(animal);
    }
    if let Some(animal) = animal.as_any().downcast_ref::<Dog>() {
        <Dog as Animal>::speak(animal);
    }
}
