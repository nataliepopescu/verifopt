trait Animal {
    fn speak(&self) -> u32;
}

struct Cat;
struct Dog;
struct Fish;

impl Animal for Cat {
    fn speak(&self) -> u32 {
        11111
    }
}
impl Animal for Dog {
    fn speak(&self) -> u32 {
        22222
    }
}
impl Animal for Fish {
    fn speak(&self) -> u32 {
        33333
    }
}


static WHICH: u32 = 0;

fn pick() -> &'static dyn Animal {
    match WHICH {
        0 => &Cat,
        1 => &Fish,
        _ => &Dog,
    }
}

fn main() {
    let a = pick();
    std::hint::black_box(a.speak());
}
