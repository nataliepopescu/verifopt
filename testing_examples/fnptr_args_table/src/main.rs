// The shape of `core::fmt`'s argument passing, without needing std's MIR:
// `Arg` holds a value pointer plus a fn pointer transmuted from
// `<T as Show>::show` (like `fmt::rt::Argument`), `Args` is an array of them
// transmuted to `NonNull<Arg>` and walked with `.add(i)` (like
// `fmt::Arguments`), and `write` calls each one with a `Fmt` around the
// given `&mut dyn Sink` (like `fmt::write` / `Formatter`).
//
// The dispatch in `Fmt::emit` sees `Text` when `show` is called directly,
// and `Counter` only through the fn pointers. FSA should list both.
use std::ptr::NonNull;

pub trait Sink {
    fn put(&mut self, s: &str);
}

pub struct Fmt<'a> {
    buf: &'a mut dyn Sink,
}

impl Fmt<'_> {
    #[inline(never)]
    fn emit(&mut self, s: &str) {
        self.buf.put(s)
    }
}

pub trait Show {
    fn show(&self, f: &mut Fmt<'_>);
}

pub struct Lit;
impl Show for Lit {
    fn show(&self, f: &mut Fmt<'_>) {
        f.emit("lit")
    }
}

#[derive(Clone, Copy)]
struct Arg {
    value: NonNull<()>,
    formatter: unsafe fn(NonNull<()>, &mut Fmt<'_>),
}

impl Arg {
    fn new<T: Show>(x: &T) -> Arg {
        let f: fn(&T, &mut Fmt<'_>) = <T as Show>::show;
        // SAFETY: only ever called with `value`, which points at a `T`.
        Arg { value: NonNull::from(x).cast(), formatter: unsafe { std::mem::transmute(f) } }
    }

    unsafe fn call(&self, f: &mut Fmt<'_>) {
        unsafe { (self.formatter)(self.value, f) }
    }
}

struct Args {
    args: NonNull<Arg>,
    len: usize,
}

impl Args {
    fn new<const M: usize>(args: &[Arg; M]) -> Args {
        // SAFETY: a reference to an array is a non-null pointer to its first element.
        Args { args: unsafe { std::mem::transmute(args) }, len: M }
    }
}

#[inline(never)]
fn write(out: &mut dyn Sink, args: Args) {
    for i in 0..args.len {
        unsafe { args.args.add(i).as_ref().call(&mut Fmt { buf: out }) }
    }
}

pub struct Text {
    s: String,
}

#[votrace::trace]
impl Sink for Text {
    fn put(&mut self, s: &str) {
        self.s.push_str(s);
    }
}

// Laid out like `Text` (three words), so a wrong call is more likely to give
// wrong numbers than to crash.
pub struct Counter {
    bytes: usize,
    calls: usize,
    other: usize,
}

#[votrace::trace]
impl Sink for Counter {
    fn put(&mut self, s: &str) {
        self.bytes += s.len();
        self.calls += 1;
    }
}

fn main() {
    let mut text = Text { s: String::new() };
    Lit.show(&mut Fmt { buf: &mut text });

    let mut counter = Counter { bytes: 0, calls: 0, other: 0 };
    let lit = Lit;
    write(&mut counter, Args::new(&[Arg::new(&lit), Arg::new(&lit)]));
    std::hint::black_box(&mut counter.other);

    votrace::hit(&format!("text={} bytes={} calls={}", text.s, counter.bytes, counter.calls));
}
