// Side effects of a memoized call: `set` is called on two different slots
// that hold equal values (a B each). A memo key that sees a `&mut` argument
// only as its pointee's value makes the two calls identical, so the second
// can be answered from the memo - but it must still write its A into `y`,
// not just return. FSA at `use_slot`'s dispatch (on `y`) must list A.
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
fn install(slot: &mut Box<dyn Sink>, s: Box<dyn Sink>) {
    *slot = s;
}

// Both calls reach `install` (and the A's allocation) through this one call
// site: with the default
// context depth (1) the memo is per (call site, arguments).
#[inline(never)]
fn set(slot: &mut Box<dyn Sink>) {
    install(slot, Box::new(A(1)))
}

#[inline(never)]
fn use_slot(s: &dyn Sink, n: usize) -> usize {
    s.put(n)
}

fn main() {
    let mut x: Box<dyn Sink> = Box::new(B(1));
    let mut y: Box<dyn Sink> = Box::new(B(1));
    set(&mut x);
    set(&mut y);
    let r = x.put(1) + use_slot(&*y, 2);
    votrace::hit(&format!("r={r}"));
}
