
fn add_one(x: i32) -> i32 {
    x + 1
}

fn sub_one(x: i32) -> i32 {
    x - 1
}

fn choose(b: bool) -> fn(i32) -> i32 {
    if b {
        add_one
    } else {
        sub_one
    }
}

fn main() {
    let f = choose(true);
    println!("output: {}", f(2));
}
