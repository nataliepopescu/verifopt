trait Animal {
    fn legs(&self) -> u32;
}

struct Cat;
struct Bird;

impl Animal for Cat {
    fn legs(&self) -> u32 {
        4
    }
}

impl Animal for Bird {
    fn legs(&self) -> u32 {
        2
    }
}

fn describe(a: &dyn Animal, depth: u32) -> u32 {
    if depth == 0 {
        a.legs()
    } else {
        describe(a, depth - 1)
    }
}

fn main() {
    let cat = Cat;
    let bird = Bird;

    let a = describe(&cat, 3);
    let b = bird.legs();

    println!("{} {}", a, b);
}

// `a.legs()` in describe -- CHA: 2 {Cat, Bird}. FSA: 1 {Cat}.
// Bird is only reached by a static call, never passed to describe.
