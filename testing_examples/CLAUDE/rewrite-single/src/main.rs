trait Shape {
    fn area(&self) -> u32;
}

struct Circle {
    r: u32,
}

struct Square {
    side: u32,
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        3 * self.r * self.r
    }
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
}

fn dispatch_area(s: &dyn Shape) -> u32 {
    s.area()
}

fn main() {
    let c = Circle { r: 2 };
    let s = Square { side: 3 };

    let a = dispatch_area(&c);
    let b = s.area();

    println!("{} {}", a, b);
}

// `s.area()` in dispatch_area -- CHA: 2 {Circle, Square}. FSA: 1 {Circle}.
// Square is only reached by a static call, never passed to dispatch_area.
// FSA == 1 gives Edit::Single: the Call's func operand is swapped for Circle::area,
// no vtable load and no new blocks.
