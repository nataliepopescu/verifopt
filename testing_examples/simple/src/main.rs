pub trait Animal {
    fn speak(&self) -> usize;
}

pub struct Dog;
impl Animal for Dog {
    fn speak(&self) -> usize {
        11111
    }
}

pub struct Cat;
impl Animal for Cat {
    fn speak(&self) -> usize {
        22222
    }
}

pub struct Fish;
impl Animal for Fish {
    fn speak(&self) -> usize {
        33333
    }
}

pub struct Bird;
impl Animal for Bird {
    fn speak(&self) -> usize {
        44444
    }
}

fn main() {
    let x = std::hint::black_box(0);

    let animal: &dyn Animal =
        if x == 1 {
            &Cat {}
        } else if x == 0 {
            &Fish {}
        } else {
            &Bird {}
        };

    let res = animal.speak();

    println!("{}", res);
}
