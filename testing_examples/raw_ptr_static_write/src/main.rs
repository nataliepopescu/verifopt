// Writes to a static through raw pointers: one from `&raw mut STATIC`, one
// from `as_mut_ptr()` on a `MaybeUninit` static, one through a raw pointer
// kept in a struct field. Each replaces a B with an A; FSA at each `use_*`
// must list A.
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

static PA: A = A(1);
static PB: B = B(2);
static mut SLOT1: &'static dyn Sink = &PB;
static mut SLOT2: MaybeUninit<&'static dyn Sink> = MaybeUninit::uninit();
static mut SLOT3: &'static dyn Sink = &PB;

pub struct Holder {
    pub tag: usize,
    pub ptr: *mut &'static dyn Sink,
}

#[inline(never)]
fn write_via_field(h: &Holder) {
    std::hint::black_box(h.tag);
    unsafe { *h.ptr = &PA };
}

#[inline(never)]
fn use_1() -> usize {
    unsafe { SLOT1.put(1) }
}

#[inline(never)]
fn use_2() -> usize {
    unsafe { SLOT2.assume_init_ref().put(2) }
}

#[inline(never)]
fn use_3() -> usize {
    unsafe { SLOT3.put(3) }
}

fn main() {
    unsafe {
        let p1 = &raw mut SLOT1;
        *p1 = &PA;
        let p2 = SLOT2.as_mut_ptr();
        p2.write(&PA);
    }
    let h = Holder { tag: 7, ptr: &raw mut SLOT3 };
    write_via_field(&h);
    let r = use_1() + use_2() + use_3();
    votrace::hit(&format!("r={r}"));
}
