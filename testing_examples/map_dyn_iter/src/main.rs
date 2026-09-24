// The adapter half of ripgrep's `HiArgs::sort`, which boxes
// `with_timestamps.into_iter().map(|(s, _)| s)` as a `dyn Iterator`. The
// object is the `Map` adapter itself: its `iter` field (a `vec::IntoIter`)
// and `f` field (a closure) are never dispatch targets, even though the
// former implements `Iterator` too.

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

#[inline(never)]
fn make(k: u32) -> Box<dyn Iterator<Item = u32>> {
    if k == 0 {
        Box::new(Counter { n: 0, end: 3 })
    } else {
        Box::new(vec![1u32, 2, 3].into_iter().map(|x| x * 2))
    }
}

fn main() {
    for x in make(std::hint::black_box(0)) {
        std::hint::black_box(x);
    }
}
