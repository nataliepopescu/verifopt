trait Animal {
    fn legs(&self) -> u32;
}

struct Cat {
    id: u32,
}

struct Bird {
    id: u32,
}

impl Animal for Cat {
    fn legs(&self) -> u32 {
        self.id + 4
    }
}

impl Animal for Bird {
    fn legs(&self) -> u32 {
        self.id + 2
    }
}

fn shared() -> u32 {
    let pet = Cat { id: 1 };
    let r: &dyn Animal = &pet;
    r.legs()
}

fn mut_read() -> u32 {
    let mut pet = Cat { id: 1 };
    let r: &mut dyn Animal = &mut pet;
    r.legs()
}

fn set_one(n: &mut u32) {
    *n = 1;
}

fn mut_write() -> u32 {
    let mut n = 0;
    set_one(&mut n);
    let a: &dyn Animal = if n == 0 { &Cat { id: 1 } } else { &Bird { id: 1 } };
    a.legs()
}

fn main() {
    let a = shared();
    let b = mut_read();
    let c = mut_write();

    std::hint::black_box(a + b + c);
}

// `r.legs()` in shared    -- CHA: 2 {Cat, Bird}. FSA: 1 {Cat}.
// `r.legs()` in mut_read  -- CHA: 2 {Cat, Bird}. FSA: 1 {Cat}.
// `a.legs()` in mut_write -- CHA: 2 {Cat, Bird}. FSA: 2 {Cat, Bird}.
//
// Cat and Bird carry a field on purpose. As unit structs they are ZSTs, `let pet = Cat;`
// emits no assignment at all, and _1 never gets constraints -- the ref resolves to an
// unset place and the site falls back to CHA.
//
// shared and mut_read are the win: reads through the ref redirect to pet's constraints.
//
// mut_write is a control, not a win. scoped_update merges instead of overwriting, so
// *n = 1 leaves n as {Scalar(0), Scalar(1)} and set_bytemap keeps both branches. FSA: 1
// there means the write never reached the caller's n, which is a bug reported as a win.
