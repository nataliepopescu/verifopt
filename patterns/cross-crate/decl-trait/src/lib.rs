
pub trait Animal {
    fn speak(&self) -> usize;
    fn modify(&mut self);
}

pub struct Bird {
    num_wings: usize,
}

impl Bird {
    pub fn new() -> Bird {
        Self {
            num_wings: 0,
        }
    }

    pub fn inc_wings(&mut self) {
        self.num_wings += 1;
    }
}

impl Animal for Bird {
    fn speak(&self) -> usize {
        1111
    }

    fn modify(&mut self) {
        self.inc_wings();
    }
}

pub fn do_thing(a: &mut dyn Animal) -> usize {
    a.modify();
    a.speak()
}

pub fn do_more_thing(a: &mut dyn Animal) -> usize {
    a.modify();
    a.modify();
    a.modify();
    a.speak()
}

pub fn wrap_do_more_thing(a: &mut dyn Animal) -> usize {
    do_more_thing(a)
}

