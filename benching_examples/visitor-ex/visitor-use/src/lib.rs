use visitor_decl::{Animal, AnimalVisitor, Cat, Dog};

pub struct SpeakBetterDogs;
pub struct SpeakBetterCats;

impl AnimalVisitor for SpeakBetterDogs {
    fn receive_dog(&self, _a: &dyn Animal) -> usize {
        44444
    }
    fn receive_cat(&self, a: &dyn Animal) -> usize {
        a.speak()
    }
}

impl AnimalVisitor for SpeakBetterCats {
    fn receive_dog(&self, a: &dyn Animal) -> usize {
        a.speak()
    }
    fn receive_cat(&self, _a: &dyn Animal) -> usize {
        99999
    }
}

pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        Box::new(Cat {})
    } else {
        Box::new(Dog {})
    }
}

pub fn wrap_dyn_call(a: Box<dyn Animal>, dc: &SpeakBetterDogs) -> usize {
    a.visit(dc)
}
