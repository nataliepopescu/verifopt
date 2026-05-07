use decl_trait::Animal;

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

