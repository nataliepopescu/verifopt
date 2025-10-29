pub trait AnimalVisitor {
    fn receive_dog(&self, a: &dyn Animal) -> usize;
    fn receive_cat(&self, a: &dyn Animal) -> usize;
}

pub trait Animal {
    fn speak(&self) -> usize;
    fn visit(&self, av: &dyn AnimalVisitor) -> usize;
}

pub struct Cat;
pub struct Dog;

impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }

    fn visit(&self, av: &dyn AnimalVisitor) -> usize {
        av.receive_cat(self)
    }
}

impl Animal for Dog {
    fn speak(&self) -> usize {
        22222
    }

    fn visit(&self, av: &dyn AnimalVisitor) -> usize {
        av.receive_dog(self)
    }
}

pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        return Box::new(Cat {});
    } else {
        return Box::new(Dog {});
    }
}

#[inline(always)]
pub fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {});
}

#[inline(always)]
pub fn get_dog() -> Box<dyn Animal> {
    return Box::new(Dog {});
}

