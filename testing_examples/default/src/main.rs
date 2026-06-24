use std::hint::black_box;

pub trait Thing {
    fn do_thing(&self) -> u32 {
        1
    }
}

struct Concrete1;
impl Concrete1 {
    pub fn new() -> Self { Self {} }
}
impl Thing for Concrete1 {}

struct Concrete2;
impl Concrete2 {
    pub fn new() -> Self { Self {} }
}
impl Thing for Concrete2 {}

/*
struct Concrete3;
impl Concrete3 {
    pub fn new() -> Self { Self {} }
}
impl Thing for Concrete3 {
    fn do_thing(&self) -> u32 {
        3
    }
}
*/

#[inline(never)]
fn wrap<'a>(thing: &'a impl Thing) -> u32 {
    thing.do_thing()
}

//fn call_wrap(thing: &dyn Thing) -> u32 {
//    wrap(thing)
//}

fn main() {
    let c = Concrete2::new();
    black_box(wrap(&c));
}
