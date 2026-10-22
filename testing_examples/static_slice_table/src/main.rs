// tock's process table as the kernel uses it: a `static mut` array filled
// through a `&mut [T]` slice made from it (an unsizing cast), then walked by
// a `process_until`-style helper that calls a closure on each process. The
// closure's dispatch must be reached - with the table read as its initial
// `[None, None]` only, the closure is never called - and FSA there must list
// A and B (not the unused D).
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
fn load(table: &mut [Option<&'static dyn Sink>]) {
    table[0] = Some(&PA);
    table[1] = Some(&PB);
}

#[inline(never)]
fn process_until<F: Fn(&dyn Sink) -> Option<usize>>(f: F) -> Option<usize> {
    for i in 0..std::hint::black_box(2) {
        if let Some(p) = unsafe { PROCS[i] } {
            if let Some(r) = f(p) {
                return Some(r);
            }
        }
    }
    None
}

fn main() {
    load(unsafe { &mut PROCS });
    let r = process_until(|p| {
        let v = p.put(1);
        if v > 30 { Some(v) } else { None }
    });
    votrace::hit(&format!("r={r:?}"));
}
