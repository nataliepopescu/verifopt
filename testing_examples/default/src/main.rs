pub trait Thing {
    #[inline(never)]
    fn do_thing(&self) {
        println!("hello");
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


impl Thing for Concrete1 {}
impl Thing for Concrete2 {}

fn wrap(thing: &dyn Thing) {
    thing.do_thing();
}
fn main() {
    let c = Concrete2::new();
    wrap(&c);
}
