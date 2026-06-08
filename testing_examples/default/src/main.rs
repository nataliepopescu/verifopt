use std::hint::black_box;

pub trait Thing {
    #[inline(never)]
    fn do_thing(&self) -> u32 {
        1
    }
}

struct Concrete1;
impl Concrete1 {
    pub fn new() -> Self { Self {} }
}

struct Concrete2;
impl Concrete2 {
    pub fn new() -> Self { Self {} }
}

struct Concrete3;
impl Concrete3 {
    pub fn new() -> Self { Self {} }
}

impl Thing for Concrete1 {}
impl Thing for Concrete2 {}
impl Thing for Concrete3 {
    fn do_thing(&self) -> u32 {
        3
    }
}

fn wrap(thing: &dyn Thing) -> u32 {
    thing.do_thing()
}
fn main() {
    let c = Concrete2::new();
    black_box(wrap(&c));
}
