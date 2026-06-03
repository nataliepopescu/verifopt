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

#[inline(never)]
fn noop() {
    println!("NOOP");
}

#[inline(never)]
fn foo<T: Shape>(s: &T) -> f32 {
    s.area()
}

fn main() {
    let _c = Circle { r: 2.0 };
    let r = Rect { h: 3.0, w: 4.0 };

    noop();
    let res = foo(&r);
    noop();
    black_box(res);
    //let res = foo(&c);
    //println!("res: {:?}", res);

}
