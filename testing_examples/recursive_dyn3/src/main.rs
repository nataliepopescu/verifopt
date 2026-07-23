use std::hint::black_box;

pub trait Animal {
    fn speak(&self) -> usize;
    fn legs(&self) -> usize;
}

pub struct Cat;
impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }
    fn legs(&self) -> usize {
        4
    }
}

pub struct Fish;
impl Animal for Fish {
    fn speak(&self) -> usize {
        22222
    }
    fn legs(&self) -> usize {
        0
    }
}

pub struct Bird;
impl Animal for Bird {
    fn speak(&self) -> usize {
        33333
    }
    fn legs(&self) -> usize {
        2
    }
}

#[inline(never)]
fn a(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    n + b(n - 1)
}

#[inline(never)]
fn b(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    n + c(n - 1)
}

#[inline(never)]
fn c(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    n + a(n - 1)
}

fn main() {
    let res = a(10);
    black_box(res);

    let x: &dyn Animal =
        if res == 55 {
            &Cat {}
        } else {
            &Bird {}
        };
    black_box(x.speak());
}
