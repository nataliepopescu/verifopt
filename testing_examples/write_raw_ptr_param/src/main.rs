// A write through a `*mut` parameter: `install` replaces the B in `x` with
// an A through a raw pointer, so FSA at `use_slot` must list A.
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


#[inline(never)]
unsafe fn install(slot: *mut Box<dyn Sink>) {
    unsafe { *slot = Box::new(A(1)) };
}

fn main() {
    let mut x: Box<dyn Sink> = Box::new(B(1));
    unsafe { install(&raw mut x) };
    let r = use_slot(&*x, 2);
    votrace::hit(&format!("r={r}"));
}
