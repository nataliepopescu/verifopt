use std::hint::black_box;

static X: i32 = 42;

struct Inner {
    val: i32,
    r: &'static i32,
}

struct Outer {
    inner: Inner,
    flag: bool,
}

const OUTER_CONST: Outer = Outer {
    inner: Inner { val: 7, r: &X },
    flag: true,
};

fn main() {
    let o = OUTER_CONST;
    black_box(o.inner.val);
    black_box(o.inner.r);
}
