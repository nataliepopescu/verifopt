// (Not zero-sized: the impls read a field, and ZST values are modelled
// differently.)
// One dispatch site (`emit`) reached three ways: directly with `A`, from a
// closure passed by generic type that captures a `&mut dyn Sink` to `B`, and
// from a closure called through `&mut dyn FnMut()` that captures one to `C`.
// FSA at `emit` must list all three (and not `D`, which is never used): the last two are only visible if the
// closures' captured values are modelled.
pub trait Sink {
    fn put(&mut self, n: usize) -> usize;
}

pub struct A(usize);
#[votrace::trace]
impl Sink for A {
    fn put(&mut self, n: usize) -> usize {
        n + self.0
    }
}

pub struct B(usize);
#[votrace::trace]
impl Sink for B {
    fn put(&mut self, n: usize) -> usize {
        n + self.0 * 20
    }
}

pub struct C(usize);
#[votrace::trace]
impl Sink for C {
    fn put(&mut self, n: usize) -> usize {
        n + self.0 * 300
    }
}

// Never used: CHA lists it, a precise FSA doesn't.
pub struct D(usize);
impl Sink for D {
    fn put(&mut self, n: usize) -> usize {
        n + self.0 * 4000
    }
}

#[inline(never)]
fn emit(s: &mut dyn Sink, n: usize) -> usize {
    s.put(n)
}

#[inline(never)]
fn run<F: FnMut() -> usize>(mut f: F) -> usize {
    f()
}

#[inline(never)]
fn run_dyn(f: &mut dyn FnMut() -> usize) -> usize {
    f()
}

fn main() {
    let mut a = A(1);
    let mut b = B(1);
    let mut c = C(1);

    let x = emit(&mut a, 0);

    let sb: &mut dyn Sink = &mut b;
    let y = run(|| emit(&mut *sb, x));

    let sc: &mut dyn Sink = &mut c;
    let z = run_dyn(&mut || emit(&mut *sc, y));

    votrace::hit(&format!("x={x} y={y} z={z}"));
}
