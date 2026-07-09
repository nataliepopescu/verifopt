use std::hint::black_box;

pub trait Animal {
    fn speak(&self) -> usize;
}

pub struct Cat;
impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }
}

pub struct Fish;
impl Animal for Fish {
    fn speak(&self) -> usize {
        22222
    }
}

pub struct Bird;
impl Animal for Bird {
    fn speak(&self) -> usize {
        33333
    }
}

#[inline(never)]
fn sum(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    let a: &dyn Animal =
        if n == 1 {
            &Cat {}
        } else {
            &Fish {}
        };

    black_box(a.speak());
    let next = n - 1;

    n + sum(next)
}

fn main() {
    let res = sum(5);
    black_box(res);

    let x: &dyn Animal =
        if res == 15 {
            &Cat {}
        } else {
            &Bird {}
        };
    black_box(x.speak());
}
