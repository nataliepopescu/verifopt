use std::hint::black_box;

fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

fn main() {
    let num = 5;
    black_box(factorial(num));
}
