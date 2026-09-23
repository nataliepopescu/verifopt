// Mirrors ripgrep's hot loop: `HiArgs::sort<I: Iterator>` returns one of
// several concrete iterators boxed as `Box<dyn Iterator<Item = Haystack>>`,
// and `rg::search` consumes it with a `for` loop.

struct Counter {
    n: u32,
    end: u32,
}
#[votrace::trace]
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.n < self.end {
            self.n += 1;
            Some(self.n)
        } else {
            None
        }
    }
}

struct Countdown {
    n: u32,
}
#[votrace::trace]
impl Iterator for Countdown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.n > 0 {
            self.n -= 1;
            Some(self.n)
        } else {
            None
        }
    }
}

// Generic, like `sort<I>`: the dyn object is created from a type parameter
// that monomorphization fills in.
#[inline(never)]
fn boxed<I: Iterator<Item = u32> + 'static>(it: I) -> Box<dyn Iterator<Item = u32>> {
    Box::new(it)
}

#[inline(never)]
fn make(k: u32) -> Box<dyn Iterator<Item = u32>> {
    if k == 0 { boxed(Counter { n: 0, end: 3 }) } else { boxed(Countdown { n: 3 }) }
}

fn main() {
    for x in make(std::hint::black_box(0)) {
        std::hint::black_box(x);
    }
}
