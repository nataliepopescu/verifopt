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

pub struct Cat {
    coat_color: usize,
}
impl Cat {
    pub fn new() -> Cat {
        Self {
            coat_color: 0,
        }
    }

    pub fn change_coat(&mut self) {
        self.coat_color += 1;
    }
}

pub struct Dog {
    eye_color: usize,
    paw_color: usize,
}
impl Dog {
    pub fn new() -> Dog {
        Self {
            eye_color: 0,
            paw_color: 0,
        }
    }

    pub fn change_eyes(&mut self) {
        self.eye_color += 1;
    }

    fn change_paws(&mut self) {
        self.paw_color += 1;
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

impl Animal for Cat {
    fn speak(&self) -> usize {
        2222
    }
    fn modify(&mut self) {
        self.change_coat();
    }
}

impl Animal for Dog {
    fn speak(&self) -> usize {
        3333
    }
    fn modify(&mut self) {
        self.change_eyes();
        self.change_paws();
    }
}

pub fn do_thing(a: &mut dyn Animal) -> usize {
    a.modify();
    a.speak()
}

/*
pub fn do_more_thing(a: &mut dyn Animal) -> usize {
    a.modify();
    a.modify();
    a.modify();
    a.speak()
}

pub fn wrap_do_more_thing(a: &mut dyn Animal) -> usize {
    do_more_thing(a)
}

pub fn decide_simple(input: usize) -> usize {
    if input == 0 {
        input + 1
    } else if input == 3 {
        input - 1
    } else {
        0
    }
}

pub fn decide_even(input: usize) -> usize {
    input * 2
}
*/

#[inline(never)]
fn noop() {
    println!("NOOP");
}

fn main() {
    //let num = decide_simple(3);
    let num = 3;

    let mut bird = Bird::new();
    let mut cat = Cat::new();
    let mut dog = Dog::new();
    bird.inc_wings();
    //dog.change_eyes();

    let a: &mut dyn Animal;
    if num == 1 {
        a = &mut cat;
    } else {
        a = &mut dog;
    }
    //a = &mut cat;

    noop();
    let res = do_thing(a);
    println!("res: {}", res);
}

