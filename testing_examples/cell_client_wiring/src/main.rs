// tock's client wiring: a capsule is built first and gets its client later,
// through `&self`, into a `Cell` (tock's OptionalCell is a `Cell<Option<T>>`).
// The write goes through a shared reference, so only interior mutability
// makes it visible; FSA at `Capsule::fire` must list A.
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

static CLIENT: A = A(1);

fn main() {
    let cap = Capsule { client: Cell::new(None) };
    cap.set_client(&CLIENT);
    let r = cap.fire();
    votrace::hit(&format!("r={r}"));
}
