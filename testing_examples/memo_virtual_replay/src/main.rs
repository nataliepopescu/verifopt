// Replayed writes of a virtual-call memo hit. `call` is reached from two
// call sites (two contexts), but virtual_call_memo is keyed by the dispatch
// site and its argument values only, so the second `i.install(slot)` is a
// hit: it must replay the write into `y`. FSA at `use_y` must list A.
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

pub trait Installer {
    fn install(&self, slot: &mut Box<dyn Sink>);
}

pub struct I(pub usize);
#[votrace::trace]
impl Installer for I {
    fn install(&self, slot: &mut Box<dyn Sink>) {
        *slot = Box::new(A(self.0));
    }
}

#[inline(never)]
fn call(i: &dyn Installer, slot: &mut Box<dyn Sink>) {
    i.install(slot)
}

#[inline(never)]
fn use_y(s: &dyn Sink) -> usize {
    s.put(1)
}

fn main() {
    let i = I(1);
    let mut x: Box<dyn Sink> = Box::new(B(1));
    let mut y: Box<dyn Sink> = Box::new(B(1));
    call(&i, &mut x);
    call(&i, &mut y);
    let r = x.put(0) + use_y(&*y);
    votrace::hit(&format!("r={r}"));
}
