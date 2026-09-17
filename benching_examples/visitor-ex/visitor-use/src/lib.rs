use visitor_decl::{Animal, AnimalVisitor, Dog};

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

pub fn wrap_speak(d: &Dog, _dc: &SpeakBetterDogs) -> usize {
    d.speak()
}

pub fn wrap_receive_dog(d: &Dog, dc: &SpeakBetterDogs) -> usize {
    dc.receive_dog(d)
}

// 

pub fn wrap_visit_dynanimal(a: &dyn Animal, sbd: &SpeakBetterDogs) -> usize {
    a.visit(sbd)
}
//pub fn wrap_visit_dynanimal2(a: &dyn Animal, sbd: &SpeakBetterDogs) -> usize {
//    a.visit(sbd)
//}
//pub fn wrap_visit_dynanimal3(a: &dyn Animal, sbd: &SpeakBetterDogs) -> usize {
//    a.visit(sbd)
//}

// 

pub fn wrap_visit_dynvisitor_direct_sbd(d: &Dog, v: &dyn AnimalVisitor) -> usize {
    d.visit(v)
}
//pub fn wrap_visit_dynvisitor_funcret_sbd(d: &Dog, v: &dyn AnimalVisitor) -> usize {
//    d.visit(v)
//}
//pub fn wrap_visit_dynvisitor_direct_sbc(d: &Dog, v: &dyn AnimalVisitor) -> usize {
//    d.visit(v)
//}
//pub fn wrap_visit_dynvisitor_funcret_sbc(d: &Dog, v: &dyn AnimalVisitor) -> usize {
//    d.visit(v)
//}

// 

pub fn wrap_visit_dynboth(a: &dyn Animal, v: &dyn AnimalVisitor) -> usize {
    a.visit(v)
}
//pub fn wrap_visit_dynboth2(a: &dyn Animal, v: &dyn AnimalVisitor) -> usize {
//    a.visit(v)
//}
//pub fn wrap_visit_dynboth3(a: &dyn Animal, v: &dyn AnimalVisitor) -> usize {
//    a.visit(v)
//}
//pub fn wrap_visit_dynboth4(a: &dyn Animal, v: &dyn AnimalVisitor) -> usize {
//    a.visit(v)
//}
//pub fn wrap_visit_dynboth5(a: &dyn Animal, v: &dyn AnimalVisitor) -> usize {
//    a.visit(v)
//}
