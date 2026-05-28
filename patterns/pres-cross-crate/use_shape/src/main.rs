use decl_shape::{Shape, area_wrapper};

struct Circle {
    r: f32,
}
impl Circle {
    fn new() -> Circle {
        Self { r: 2.0 }
    }
}

struct Rectangle {
    h: f32,
    w: f32,
}
impl Rectangle {
    fn new() -> Rectangle {
        Self { h: 3.0, w: 4.0 }
    }
}

struct Triangle {
    h: f32,
    w: f32,
}
impl Triangle {
    fn new() -> Triangle {
        Self { h: 3.0, w: 4.0 }
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        3.14 * self.r.powi(2)
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.h * self.w
    }
}

impl Shape for Triangle {
    fn area(&self) -> f32 {
        self.h * self.w / 2.0
    }
}

#[inline(never)]
fn noop() {
    println!("NOOP");
}

fn main() {
    let input = 3;
    let c = Circle::new();
    let r = Rectangle::new();
    let s: &dyn Shape;

    if input == 1 {
        s = &r;
    } else {
        s = &c;
    }

    noop();
    let res = area_wrapper(s);
    noop();
    println!("res: {:?}", res);
    noop();
}
