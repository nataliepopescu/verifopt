use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::hint::black_box;

fn main() {
    let mut hasher = DefaultHasher::new();
    42u64.hash(&mut hasher);
    let res = hasher.finish();
    black_box(res);
    //println!("{}", hasher.finish());
}
