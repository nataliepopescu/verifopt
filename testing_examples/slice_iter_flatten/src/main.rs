// Iterating a slice of `Option<&dyn Sink>` with `.iter().flatten()`: the
// elements are lost on the way through the slice iterator and Flatten, so
// FSA at `run_all` falls back to CHA (the never-used D included). Red
// until that path is followed; FSA must list A and B.
pub trait Sink { fn put(&self, n: usize) -> usize; }
pub struct A(pub usize);
#[votrace::trace]
impl Sink for A { fn put(&self, n: usize) -> usize { n + self.0 } }
pub struct B(pub usize);
#[votrace::trace]
impl Sink for B { fn put(&self, n: usize) -> usize { n + self.0 * 20 } }
pub struct D(pub usize);
impl Sink for D { fn put(&self, n: usize) -> usize { n + self.0 * 4000 } }

#[inline(never)]
fn run_all(procs: &[Option<&dyn Sink>; 2]) -> usize {
    let mut acc = 0;
    for p in procs.iter().flatten() {
        acc += p.put(1);
    }
    acc
}

fn main() {
    let a = A(1);
    let b = B(2);
    let procs: [Option<&dyn Sink>; 2] = [Some(&a), Some(&b)];
    let r = run_all(&procs);
    votrace::hit(&format!("r={r}"));
}
