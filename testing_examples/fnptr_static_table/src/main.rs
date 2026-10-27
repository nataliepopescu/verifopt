// The shape of tock's deferred calls (kernel/deferred_call.rs): `register`
// stores a callback - a closure cast to `fn(*const ())` that calls
// `T::handle` on the data pointer - in a static table behind interior
// mutability; `run_all` reads the entries back and calls them. FSA at the
// dispatch inside `Holder::handle` must list A and B (never D): reaching it
// needs the callback's value to survive the static table.
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

pub trait Client {
    fn handle(&self) -> usize;
}

pub struct Holder {
    sink: Box<dyn Sink + Sync>,
}

impl Client for Holder {
    fn handle(&self) -> usize {
        self.sink.put(3)
    }
}

#[derive(Clone, Copy)]
struct Ref {
    data: *const (),
    callback: fn(*const ()) -> usize,
}

impl Ref {
    fn new<T: Client>(x: &'static T) -> Self {
        Ref {
            data: x as *const T as *const (),
            callback: |p| unsafe { (*(p as *const T)).handle() },
        }
    }
}

struct Table {
    slots: [Cell<Option<Ref>>; 4],
    n: Cell<usize>,
}

unsafe impl Sync for Table {}

static TABLE: Table = Table { slots: [const { Cell::new(None) }; 4], n: Cell::new(0) };

fn register<T: Client>(x: &'static T) {
    let i = TABLE.n.get();
    TABLE.slots[i].set(Some(Ref::new(x)));
    TABLE.n.set(i + 1);
}

#[inline(never)]
fn run_all() -> usize {
    let mut s = 0;
    for i in 0..TABLE.n.get() {
        if let Some(r) = TABLE.slots[i].get() {
            s += (r.callback)(r.data);
        }
    }
    s
}

fn main() {
    let h1: &'static Holder = Box::leak(Box::new(Holder { sink: Box::new(A(1)) }));
    let h2: &'static Holder = Box::leak(Box::new(Holder { sink: Box::new(B(2)) }));
    register(h1);
    register(h2);
    let r = run_all();
    votrace::hit(&format!("r={r}"));
}
