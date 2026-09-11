use std::hint::black_box;

trait Shape {
    fn area(&self) -> f32;
}

struct Circle {
    r: f32,
}

struct Rect {
    h: f32,
    w: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        3.14 * self.r.powi(2) //as u32
    }
}

impl Shape for Rect {
    fn area(&self) -> f32 {
        self.h * self.w
    }
}

// generic function that will be monomorphized in two different ways
// - one monomorph (say, String) will have one rewrite
// - the other monomorph (say, i32) will have another rewrite
#[inline(never)]
fn foo<T>(t: &T, s: &dyn Shape) -> f32 {
    black_box(t);
    s.area()
}

fn main() {
    let c = Circle { r: 2.0 };
    let r = Rect { h: 3.0, w: 4.0 };

    let i1: u32 = 8;
    let res1 = foo(&i1, &c);
    black_box(res1);

    let i2: f64 = 2.0;
    let res2 = foo(&i2, &r);
    black_box(res2);
}
