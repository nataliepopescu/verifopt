use std::hint::black_box;
use rand::RngExt;

fn main() {
    let x = rand::rng().random_range(..2usize);
    black_box(x);
}
