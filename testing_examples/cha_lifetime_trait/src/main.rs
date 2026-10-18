// CHA for traits with their own generic parameters.
//
// CHA filters each impl by comparing the call site's trait args with the
// impl's. Those must be compared loosely: at the call site lifetimes are
// erased while an impl names `'a` or `'static`, and an impl's args may be
// generic (`impl<F: Flash> Sink<F> for Store`). Comparing them with `==`
// rejected every impl, so CHA came back empty - like tock's
// `uart::Transmit<'a>` and `flash::Client<F>` dispatches.
//
// `black_box` hides where each trait object came from, so FSA falls back to
// CHA. Each trait has an impl that's used and one that never is (never
// compiled), so CHAmax is exactly one instance per site and both sites must be
// rewritten as "single exact". With an empty CHA neither site is rewritten.

use std::hint::black_box;

pub trait Chip {
    fn id(&self) -> u32;
}

pub struct Sam;
#[votrace::trace]
impl Chip for Sam {
    fn id(&self) -> u32 {
        7
    }
}

// A trait with a lifetime parameter.
pub trait Client<'a> {
    fn fired(&self) -> u32;
}

pub struct Pin<'a> {
    chip: &'a Sam,
}

impl<'a> Client<'a> for Pin<'a> {
    fn fired(&self) -> u32 {
        self.chip.id() + 1
    }
}

pub struct UnusedClient;
impl Client<'static> for UnusedClient {
    fn fired(&self) -> u32 {
        0
    }
}

// A trait with a type parameter, implemented generically over it.
pub trait Flash {}
pub struct Calw;
impl Flash for Calw {}

pub trait Sink<F: Flash> {
    fn done(&self) -> u32;
}

pub struct Store {
    chip: Sam,
}

impl<F: Flash> Sink<F> for Store {
    fn done(&self) -> u32 {
        self.chip.id() + 2
    }
}

pub struct UnusedSink;
impl Sink<Calw> for UnusedSink {
    fn done(&self) -> u32 {
        0
    }
}

fn main() {
    let sam = Sam;
    let pin = Pin { chip: &sam };
    let client: &dyn Client<'_> = black_box(&pin);
    let a = client.fired();

    let store = Store { chip: Sam };
    let sink: &dyn Sink<Calw> = black_box(&store);
    let b = sink.done();

    // Behaviour depends on the results, so a wrong target shows up in the trace.
    if a != 8 || b != 9 {
        black_box(Sam.id());
    }
    black_box((a, b));
}
