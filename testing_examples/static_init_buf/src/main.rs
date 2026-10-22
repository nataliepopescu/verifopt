// tock's static_init!: a value written into a `static mut MaybeUninit<T>`
// buffer through `MaybeUninit::write`, which hands out the `&'static mut T`
// that the rest of the program uses. The board holds the capsules through
// these references; FSA at `Board::fire` must list A and B.
#![allow(static_mut_refs)]
use std::mem::MaybeUninit;
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

pub struct Board {
    pub first: &'static dyn Sink,
    pub second: &'static dyn Sink,
}

impl Board {
    #[inline(never)]
    fn fire(&self, which: usize) -> usize {
        let s = if which == 0 { self.first } else { self.second };
        s.put(which)
    }
}

macro_rules! static_init {
    ($T:ty, $e:expr) => {{
        static mut BUF: MaybeUninit<$T> = MaybeUninit::uninit();
        unsafe { (&mut BUF).write($e) }
    }};
}

#[inline(never)]
fn setup() -> &'static Board {
    let a: &'static A = static_init!(A, A(1));
    let b: &'static B = static_init!(B, B(2));
    static_init!(Board, Board { first: a, second: b })
}

fn main() {
    let board = setup();
    let r = board.fire(0) + board.fire(std::hint::black_box(1));
    votrace::hit(&format!("r={r}"));
}
