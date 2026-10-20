// A widened merge must not lose a field's trait objects: `take`'s parameter
// collects every `Holder` it's called with, and with more than 50 of them
// that differ in two fields (so the lossless merge can't fold them) the
// value is widened, dropping the ADT's fields. A dropped field then reads as
// empty, so the `B` in the last `Holder` vanishes: FSA at `use_last` must
// list B, not just the A's. (Reading the dropped field as "any `Box<dyn
// Sink>`" would add D too - sound, if less precise.)
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
// context depth (1) its parameter is then one value collecting all of them.
#[inline(never)]
fn via(h: Holder) -> Box<dyn Sink> {
    take(h)
}

#[inline(never)]
fn use_last(s: &dyn Sink, n: usize) -> usize {
    s.put(n)
}

// One call site per tag; the tag and the A's contents both differ. Exactly 50:
// the memo widens its keys after 50 argument sets, and a 51st warm-up would
// make the B call a widened-key memo hit (memo_widened_reuse's bug) instead.
macro_rules! warm_up {
    ($acc:ident; $($t:literal)*) => {
        $( $acc += via(Holder { tag: $t, sink: Box::new(A($t)) }).put(0); )*
    };
}

fn main() {
    let mut acc = 0;
    warm_up!(acc; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24
                  25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49);
    let b = via(Holder { tag: 1000, sink: Box::new(B(1)) });
    acc += use_last(&*b, 1);
    votrace::hit(&format!("acc={acc}"));
}
