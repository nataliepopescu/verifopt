// A write through a shared reference to interior-mutable data: `install`
// gets `&RefCell<..>` and replaces the B inside with an A through
// `borrow_mut`, so FSA at `use_slot` must list A.
pub trait Sink {
    fn put(&self, n: usize) -> usize;
}

pub struct A(pub usize);
#[votrace::trace]
impl Sink for A {
    fn put(&self, n: usize) -> usize {
        n + self.0
    }
}

pub struct B(pub usize);
#[votrace::trace]
impl Sink for B {
    fn put(&self, n: usize) -> usize {
        n + self.0 * 20
    }
}

// Never used: CHA lists it, a precise FSA doesn't.
pub struct D(pub usize);
impl Sink for D {
    fn put(&self, n: usize) -> usize {
        n + self.0 * 4000
    }
}

#[inline(never)]
fn use_slot(s: &dyn Sink, n: usize) -> usize {
    s.put(n)
}


use std::cell::RefCell;

#[inline(never)]
fn install(c: &RefCell<Box<dyn Sink>>) {
    *c.borrow_mut() = Box::new(A(1));
}

fn main() {
    let x: RefCell<Box<dyn Sink>> = RefCell::new(Box::new(B(1)));
    install(&x);
    let r = use_slot(&**x.borrow(), 2);
    votrace::hit(&format!("r={r}"));
}
