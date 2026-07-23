trait Codec {
    fn encode(&self, x: u32) -> u32;
}

struct Hex;
struct Rot13;
struct Base64;

impl Codec for Hex {
    fn encode(&self, x: u32) -> u32 {
        x * 16
    }
}

impl Codec for Rot13 {
    fn encode(&self, x: u32) -> u32 {
        x + 13
    }
}

impl Codec for Base64 {
    fn encode(&self, x: u32) -> u32 {
        x * 64
    }
}

fn run(c: &dyn Codec, x: u32) -> u32 {
    c.encode(x)
}

fn main() {
    let h = Hex;
    let r = Rot13;
    let b = Base64;

    let a = run(&h, 3);
    let c = run(&r, 3);
    let d = b.encode(3);

    println!("{} {} {}", a, c, d);
}

// `c.encode(x)` in run -- CHA: 3 {Hex, Rot13, Base64}. FSA: 2 {Hex, Rot13}.
// Base64 is only reached by a static call, never passed to run.
// FSA == 2 gives Edit::Multi: the receiver's vtable slot is loaded and compared against
// Hex::encode's reified fn ptr, with Rot13::encode as the fallback block.
