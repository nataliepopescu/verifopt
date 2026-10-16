// A dispatch site whose single target is only provable from CHAmax.
//
// `black_box` hides where the trait object came from: the analysis models it
// (a compiler intrinsic) as "any `&dyn Device`", so FSA can only offer what CHA
// does - every impl of `Device`. CHA has two: the generic
// `impl<C: Chip> Device for Driver<C>` (an impl, with no generic args) and
// `Unused`'s, which nothing ever calls and so is never compiled. CHAmax - the
// instances actually compiled anywhere in the program - is exactly one,
// `<Driver<Sam> as Device>::poll`, so the site must be rewritten as a plain
// direct call ("single exact" in verifopt_edit_kind_stats.txt), not a
// guarded single-candidate `Pointers` rewrite.

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

pub trait Device {
    fn poll(&self) -> u32;
}

pub struct Driver<C: Chip> {
    chip: C,
}

impl<C: Chip> Device for Driver<C> {
    fn poll(&self) -> u32 {
        self.chip.id() + 1
    }
}

pub struct Unused;
impl Device for Unused {
    fn poll(&self) -> u32 {
        0
    }
}

fn main() {
    let driver = Driver { chip: Sam };
    let dev: &dyn Device = black_box(&driver);
    let r = dev.poll();
    // Behaviour depends on the result, so a wrong target shows up in the trace.
    if r != 8 {
        black_box(Sam.id());
    }
    black_box(r);
}
