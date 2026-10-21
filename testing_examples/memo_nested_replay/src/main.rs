// Replayed writes of nested memo hits. `install` is reached through `set`
// with one context (k = 1), so in the first `both` the second `install` -
// through `set(b)` - is a memo hit, which must replay its write into `b`
// (through `set`'s and `both`'s operands: an alias chain). The second `go`
// then hits `both`'s memo entry, whose log must hold both writes, including
// the replayed one. FSA at `use_v` (on `v`, written only by replays) must
// list A.
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
fn install(slot: &mut Box<dyn Sink>) {
    *slot = Box::new(A(1));
}

#[inline(never)]
fn set(slot: &mut Box<dyn Sink>) {
    install(slot)
}

#[inline(never)]
fn both(a: &mut Box<dyn Sink>, b: &mut Box<dyn Sink>) {
    set(a);
    set(b);
}

#[inline(never)]
fn go(a: &mut Box<dyn Sink>, b: &mut Box<dyn Sink>) {
    both(a, b)
}

#[inline(never)]
fn use_y(s: &dyn Sink) -> usize {
    s.put(1)
}

#[inline(never)]
fn use_v(s: &dyn Sink) -> usize {
    s.put(2)
}

fn main() {
    let mut x: Box<dyn Sink> = Box::new(B(1));
    let mut y: Box<dyn Sink> = Box::new(B(1));
    let mut u: Box<dyn Sink> = Box::new(B(1));
    let mut v: Box<dyn Sink> = Box::new(B(1));
    go(&mut x, &mut y);
    go(&mut u, &mut v);
    let r = x.put(0) + use_y(&*y) + u.put(3) + use_v(&*v);
    votrace::hit(&format!("r={r}"));
}
