pub trait AnimalVisitor {
    fn receive(&self, a: &dyn Animal);
}

pub trait Animal {
    fn speak(&self);
    fn visit(&self, av: &dyn AnimalVisitor);
    fn get_type_id(&self) -> u32;
}

pub struct Cat;
pub struct Dog;

impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }

    fn visit(&self, av: &dyn AnimalVisitor) {
        av.receive(self);
    }

    fn get_type_id(&self) -> u32 {
        0
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("woof");
    }

    fn visit(&self, av: &dyn AnimalVisitor) {
        av.receive(self);
    }

    fn get_type_id(&self) -> u32 {
        1
    }
}

