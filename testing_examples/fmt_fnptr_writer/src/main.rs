// `Formatter::write_fmt`'s `self.buf.write_str(s)` dispatches on whatever
// writer the Formatter wraps. `to_string()` calls `Display::fmt` directly,
// with a Formatter around a String; `write!` into another writer goes
// through `core::fmt::write`, which reaches `Display::fmt` only through the
// fn pointer stored in `fmt::rt::Argument`. If the analysis doesn't follow
// that fn pointer it sees only the String writer, and an unguarded rewrite
// of that site to `String::write_str` would run on the Counter below.
use std::fmt::{self, Write};

pub struct Lit;
impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lit")
    }
}

// Laid out like a String (three words), so a wrong call is more likely to
// produce a wrong count than to crash outright.
pub struct Counter {
    bytes: usize,
    calls: usize,
    other: usize,
}

#[votrace::trace]
impl fmt::Write for Counter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.bytes += s.len();
        self.calls += 1;
        Ok(())
    }
}

fn main() {
    let s = Lit.to_string();

    let mut counter = Counter { bytes: 0, calls: 0, other: 0 };
    write!(&mut counter, "{}", Lit).unwrap();
    std::hint::black_box(&mut counter.other);

    votrace::hit(&format!("string={s} bytes={} calls={}", counter.bytes, counter.calls));
}
