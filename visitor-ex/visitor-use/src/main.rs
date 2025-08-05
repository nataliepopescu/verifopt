use rand::Rng;

use visitor_decl::{Animal, AnimalVisitor, Cat, Dog};

struct SpeakBetterDogs;

impl AnimalVisitor for SpeakBetterDogs {
    fn receive(&self, a: &dyn Animal) {
        if a.get_type_id() == 1 {
            println!("grrr");
        } else {
            a.speak();
        }
    }
}

fn main() {
    let a: &dyn Animal;

    let num: u32 = rand::rng().random_range(..2);

    if num == 0 {
        a = &Cat {};
    } else {
        a = &Dog {};
    }

    let dc = &SpeakBetterDogs {};

    a.visit(dc);
}
