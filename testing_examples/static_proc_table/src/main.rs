// tock's process table: a `static mut` array of trait objects, filled through
// `&mut` to the static during setup and read by another function later. The
// static is only ever written through a pointer to it, so FSA at `run_all`
// must list A and B (not the unused D). (Read by index: iterating it with
// `.iter().flatten()` loses the elements regardless of statics - see
// slice_iter_flatten.)
#![allow(static_mut_refs)]
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

static mut PROCS: [Option<&'static dyn Sink>; 2] = [None, None];
static PA: A = A(1);
static PB: B = B(2);

#[inline(never)]
fn load(table: &mut [Option<&'static dyn Sink>; 2]) {
    table[0] = Some(&PA);
    table[1] = Some(&PB);
}

#[inline(never)]
fn run_all() -> usize {
    let mut acc = 0;
    for i in 0..std::hint::black_box(2) {
        if let Some(p) = unsafe { PROCS[i] } {
            acc += p.put(1);
        }
    }
    acc
}

fn main() {
    load(unsafe { &mut PROCS });
    let r = run_all();
    votrace::hit(&format!("r={r}"));
}
