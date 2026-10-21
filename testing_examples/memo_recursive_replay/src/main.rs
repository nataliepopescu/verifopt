// Replayed writes of a recursion summary. `fill` stores the `v` it was given
// and passes `w` on as the next level's `v`, with an A as the next `w`. With
// context depth 1 only the first recursive call has its own context; from
// the second one on, calls are answered from a recursion summary. So `x`
// gets B (outermost call) and C (first recursive call) directly, and the A
// only through the summarized calls. FSA at `use_slot` must list A.
//
// This doesn't isolate summary replay (write_log): the summarized calls are
// re-interpreted in the scope of the first recursive call, which is still
// active, so their writes also follow that scope's alias edges to `x`
// directly. It passes with or without the replay; it guards recursion with
// writes through `&mut` in general.
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

pub struct C(pub usize);
#[votrace::trace]
impl Sink for C {
    fn put(&self, n: usize) -> usize {
        n + self.0 * 300
    }
}

#[inline(never)]
fn fill(slot: &mut Box<dyn Sink>, v: Box<dyn Sink>, w: Box<dyn Sink>, n: usize) {
    if n == 0 {
        *slot = v;
    } else {
        fill(slot, w, Box::new(A(1)), n - 1);
    }
}

#[inline(never)]
fn use_slot(s: &dyn Sink) -> usize {
    s.put(2)
}

fn main() {
    let mut x: Box<dyn Sink> = Box::new(B(1));
    fill(&mut x, Box::new(B(2)), Box::new(C(1)), std::hint::black_box(3));
    let r = use_slot(&*x);
    votrace::hit(&format!("r={r}"));
}
