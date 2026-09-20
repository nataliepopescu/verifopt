use visitor_decl::{Animal, AnimalVisitor, Cat, Dog};

pub struct SpeakBetterDogs;
pub struct SpeakBetterCats;

pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        return Box::new(Cat {});
    } else {
        return Box::new(Dog {});
    }
}

pub fn get_visitor(num: usize) -> Box<dyn AnimalVisitor> {
    if num == 0 {
        return Box::new(SpeakBetterCats {});
    } else {
        return Box::new(SpeakBetterDogs {});
    }
}

/*
pub fn get_cat() -> Box<Cat> {
    Box::new(Cat {})
}

pub fn get_dog() -> Box<Dog> {
    Box::new(Dog {})
}

pub fn get_sbc() -> Box<SpeakBetterCats> {
    Box::new(SpeakBetterCats {})
}

pub fn get_sbd() -> Box<SpeakBetterDogs> {
    Box::new(SpeakBetterDogs {})
}
*/

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

pub fn wrap_visit_dynanimal(a: &dyn Animal, v: &SpeakBetterDogs) -> usize {
    a.visit(v)
}

pub fn wrap_visit_dynvisitor(a: &Dog, v: &dyn AnimalVisitor) -> usize {
    a.visit(v)
}

pub fn wrap_visit_dynboth(a: &dyn Animal, v: &dyn AnimalVisitor) -> usize {
    a.visit(v)
}

// 

pub fn wrap_visit_dynanimal1(a: &dyn Animal, sbd: &SpeakBetterDogs) -> usize {
    a.visit(sbd)
}
pub fn wrap_visit_dynanimal2(a: &dyn Animal, sbd: &SpeakBetterDogs) -> usize {
    a.visit(sbd)
}
pub fn wrap_visit_dynanimal3(a: &dyn Animal, sbd: &SpeakBetterDogs) -> usize {
    a.visit(sbd)
}

// 

pub fn wrap_visit_dynvisitor_direct_sbd(d: &Dog, v: &dyn AnimalVisitor) -> usize {
    d.visit(v)
}
pub fn wrap_visit_dynvisitor_funcret_sbd(d: &Dog, v: &dyn AnimalVisitor) -> usize {
    d.visit(v)
}
pub fn wrap_visit_dynvisitor_direct_sbc(d: &Dog, v: &dyn AnimalVisitor) -> usize {
    d.visit(v)
}
pub fn wrap_visit_dynvisitor_funcret_sbc(d: &Dog, v: &dyn AnimalVisitor) -> usize {
    d.visit(v)
}

// 

pub fn wrap_visit_dynboth1(a: &dyn Animal, v: &dyn AnimalVisitor) -> usize {
    a.visit(v)
}
pub fn wrap_visit_dynboth2(a: &dyn Animal, v: &dyn AnimalVisitor) -> usize {
    a.visit(v)
}
pub fn wrap_visit_dynboth3(a: &dyn Animal, v: &dyn AnimalVisitor) -> usize {
    a.visit(v)
}
pub fn wrap_visit_dynboth4(a: &dyn Animal, v: &dyn AnimalVisitor) -> usize {
    a.visit(v)
}
pub fn wrap_visit_dynboth5(a: &dyn Animal, v: &dyn AnimalVisitor) -> usize {
    a.visit(v)
}
