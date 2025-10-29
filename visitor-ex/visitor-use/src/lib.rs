use visitor_decl::{Animal, AnimalVisitor};

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

/*
fn main() {
    use rand::Rng;

    let a: &dyn Animal;
    let num: u32 = rand::rng().random_range(..2);
    let dc = &SpeakBetterDogs {};

    if num == 0 {
        a = &Cat {};
    } else {
        a = &Dog {};
    }

    a.visit(dc);
}
*/
