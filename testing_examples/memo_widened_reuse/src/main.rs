// Memo reuse under a widened key: once `take` has been called with 50
// distinct argument values, its memo keys are widened (ADT fields dropped),
// so `Holder { sink: B }` and `Holder { sink: C }` share a key. The first is
// interpreted and cached; the second must not just reuse that result: the
// dispatch in `use_last` sees a `C`, and FSA there must list C (not B, and
// not the unused D).
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

pub struct C(pub usize);
#[votrace::trace]
impl Sink for C {
    fn put(&self, n: usize) -> usize {
        n + self.0 * 300
    }
}

// Never used: CHA lists it, a precise FSA doesn't.
pub struct D(pub usize);
impl Sink for D {
    fn put(&self, n: usize) -> usize {
        n + self.0 * 4000
    }
}

pub struct Holder {
    pub tag: usize,
    pub sink: Box<dyn Sink>,
}

#[inline(never)]
fn take(h: Holder) -> Box<dyn Sink> {
    std::hint::black_box(h.tag);
    h.sink
}

// Every call reaches `take` through this one call site: with the default
// context depth (1) the memo is per (call site, arguments), so this puts all
// 52 calls under the same memo scope.
#[inline(never)]
fn via(h: Holder) -> Box<dyn Sink> {
    take(h)
}

#[inline(never)]
fn use_last(s: &dyn Sink, n: usize) -> usize {
    s.put(n)
}

// One call site per tag, each with a distinct constant: 50 distinct argument
// sets for `take`, the memo's limit before it widens its keys.
macro_rules! warm_up {
    ($acc:ident; $($t:literal)*) => {
        $( $acc += via(Holder { tag: $t, sink: Box::new(A(1)) }).put(0); )*
    };
}

fn main() {
    let mut acc = 0;
    warm_up!(acc; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24
                  25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49);
    let b = via(Holder { tag: 1000, sink: Box::new(B(1)) });
    acc += b.put(1);
    let c = via(Holder { tag: 1001, sink: Box::new(C(1)) });
    acc += use_last(&*c, 2);
    votrace::hit(&format!("acc={acc}"));
}
