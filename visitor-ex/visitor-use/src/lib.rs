use visitor_decl::{Animal, AnimalVisitor};

pub struct SpeakBetterDogs;

impl AnimalVisitor for SpeakBetterDogs {
    fn receive_dog(&self, _a: &dyn Animal) -> usize {
        44444
    }
    fn receive_cat(&self, a: &dyn Animal) -> usize {
        a.speak()
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
