// A pointer into a static returned by a call: `capsule()` hands out a
// `&'static Capsule` into a static buffer each time it's called. The client
// is set through the first result and used through the second - the write
// must reach the static, not stay in the first caller's copy. FSA at
// `Capsule::fire` must list A.
#![allow(static_mut_refs)]
use std::cell::Cell;
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

pub struct Capsule {
    pub client: Cell<Option<&'static dyn Sink>>,
}

impl Capsule {
    #[inline(never)]
    fn set_client(&self, c: &'static dyn Sink) {
        self.client.set(Some(c));
    }

    #[inline(never)]
    fn fire(&self) -> usize {
        self.client.get().map_or(0, |c| c.put(3))
    }
}

static mut CAPSULE: Capsule = Capsule { client: Cell::new(None) };
static CLIENT: A = A(1);

#[inline(never)]
fn capsule() -> &'static Capsule {
    unsafe { &*(&raw const CAPSULE) }
}

fn main() {
    capsule().set_client(&CLIENT);
    let r = capsule().fire();
    votrace::hit(&format!("r={r}"));
}
