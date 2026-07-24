pub trait Animal {
    fn speak(&self, base: usize) -> usize;
}

pub struct Cat;
impl Animal for Cat {
    fn speak(&self, base: usize) -> usize {
        base + 11111
    }
}

pub struct Fish;
impl Animal for Fish {
    fn speak(&self, base: usize) -> usize {
        base + 22222
    }
}

fn main() {
    let x = 0;

    let animal: &dyn Animal =
        if x == 0 {
            &Cat {}
        } else {
            &Fish {}
        };

    let res = animal.speak(100);

    println!("{}", res);
}
