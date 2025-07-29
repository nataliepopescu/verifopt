pub trait Animal {
    fn speak(&self);
}

struct Dog {}

impl Animal for Dog {
    fn speak(&self) {
        println!("woof");
    }
}

struct Cat {}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }
}

fn main() {
    let cat = &Cat {};

    cat.speak();
}
