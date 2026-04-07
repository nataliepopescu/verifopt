pub trait MyTrait {
    fn do_nothing(&self);
}

struct MyStruct;

impl MyTrait for MyStruct {
    fn do_nothing(&self) {
        println!("doing nothing");
    }
}

fn main() {
    let s: &dyn MyTrait = &MyStruct {};
    s.do_nothing();
}
