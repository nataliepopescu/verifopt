// Companion to `box_dyn_iter`, pinning down the other side of its fix: when
// the value unsized to `Box<dyn Iterator>` is itself a Box, the *inner* Box
// is the dyn object's concrete type, and its forwarding
// `<Box<Countdown> as Iterator>::next` really is a dispatch target. Only the
// outermost Box - the fat pointer - must be skipped.

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

#[inline(never)]
fn make(k: u32) -> Box<dyn Iterator<Item = u32>> {
    if k == 0 {
        Box::new(Counter { n: 0, end: 3 })
    } else {
        // Box<Box<Countdown>> -> Box<dyn Iterator>: the object is a Box.
        let inner: Box<Countdown> = Box::new(Countdown { n: 3 });
        Box::new(inner)
    }
}

fn main() {
    for x in make(std::hint::black_box(1)) {
        std::hint::black_box(x);
    }
}
