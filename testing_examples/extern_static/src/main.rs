trait Animal {
    fn speak(&self) -> u32;
}

struct Cat;
#[votrace::trace]
impl Animal for Cat {
    fn speak(&self) -> u32 {
        11111
    }
}

struct Dog;
#[votrace::trace]
impl Animal for Dog {
    fn speak(&self) -> u32 {
        22222
    }
}

struct Fish;
#[votrace::trace]
impl Animal for Fish {
    fn speak(&self) -> u32 {
        33333
    }
}

// The symbol the extern declaration below resolves to at link time. On an
// embedded target this would come from a linker script (like tock's
// `_sstorage`); defining it here keeps the host test binary linkable.
#[unsafe(no_mangle)]
static VERIFOPT_EXTERN_WHICH: u32 = 0;

unsafe extern "C" {
    // A foreign static: no initializer, so its value is unknown to Rust (and
    // to the analysis) even though the definition above happens to be 0.
    #[link_name = "VERIFOPT_EXTERN_WHICH"]
    static WHICH: u32;
}

fn pick() -> &'static dyn Animal {
    match unsafe { WHICH } {
        0 => &Cat,
        1 => &Fish,
        _ => &Dog,
    }
}

fn main() {
    let a = pick();
    std::hint::black_box(a.speak());
}
