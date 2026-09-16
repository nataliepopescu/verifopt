use visitor_decl::{Animal, AnimalVisitor, Dog, Cat};
use visitor_use::{
    SpeakBetterDogs, SpeakBetterCats,
    //wrap_speak, wrap_receive_dog,
    wrap_visit_dynanimal,
    //wrap_visit_dynanimal2,
    //wrap_visit_dynanimal3,
    //wrap_visit_dynvisitor_direct_sbd,
    //wrap_visit_dynvisitor_funcret_sbd,
    //wrap_visit_dynvisitor_direct_sbc,
    //wrap_visit_dynvisitor_funcret_sbc,
    //wrap_visit_dynboth,
    //wrap_visit_dynboth2,
    //wrap_visit_dynboth3,
    //wrap_visit_dynboth4,
    //wrap_visit_dynboth5,
};
use std::hint::black_box;
//use rand::Rng;

/*
fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        return Box::new(Cat {});
    } else {
        return Box::new(Dog {});
    }
}

fn get_visitor(num: usize) -> Box<dyn AnimalVisitor> {
    if num == 0 {
        return Box::new(SpeakBetterCats {});
    } else {
        return Box::new(SpeakBetterDogs {});
    }
}
*/

fn main() {
    let d = Dog;
    let sbd = SpeakBetterDogs;

    // wrap_speak
    //black_box(wrap_speak(&d, &sbd));

    // wrap_receive_dog
    //black_box(wrap_receive_dog(&d, &sbd));

    // visit dyn animal
    black_box(wrap_visit_dynanimal(&d, &sbd));
    //let a = get_animal(1);
    //black_box(wrap_visit_dynanimal2(&*a, &sbd));
    //let a = get_animal(0);
    //black_box(wrap_visit_dynanimal3(&*a, &sbd));

    // visit dyn visitor
    //black_box(wrap_visit_dynvisitor_direct_sbd(&d, &sbd));
    //let v = get_visitor(1);
    //black_box(wrap_visit_dynvisitor_funcret_sbd(&d, &*v));
    //let sbc = SpeakBetterCats;
    //black_box(wrap_visit_dynvisitor_direct_sbc(&d, &sbc));
    //let v = get_visitor(0);
    //black_box(wrap_visit_dynvisitor_funcret_sbc(&d, &*v));

    // visit dyn both
    //black_box(wrap_visit_dynboth(&d, &sbd));
    //let a = get_animal(0);
    //let v = get_visitor(0);
    //black_box(wrap_visit_dynboth2(&*a, &*v));
    //let v = get_visitor(1);
    //black_box(wrap_visit_dynboth3(&*a, &*v));
    //let a = get_animal(1);
    //black_box(wrap_visit_dynboth4(&*a, &*v));
    //let v = get_visitor(0);
    //black_box(wrap_visit_dynboth5(&*a, &*v));
}
