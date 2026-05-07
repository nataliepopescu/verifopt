pub trait Shape {
    fn area(&self) -> f32;
}

pub struct Circle {
    r: f32,
}

pub struct Rectangle {
    h: f32,
    w: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        3.14 * self.r.powi(2)
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.w * self.h
    }
}

#[inline(never)]
fn noop() {
    println!("NOOP");
}

fn main() {
    let input = 3;
    let c = Circle { r: 2.0 };
    let r = Rectangle { h: 3.0, w: 4.0 };

    let s: &dyn Shape;
    if input == 3 {
        s = &c;
    } else {
        s = &r;
    }

    noop();
    let res = s.area();
    println!("res: {:?}", res);
}
