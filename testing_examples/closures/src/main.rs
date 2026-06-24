use std::hint::black_box;

pub fn apply(f: fn(i32) -> i32, input: i32) -> i32 {
    f(input)
}

#[inline(never)]
pub fn apply3<F>(f: F, input: u32) -> u32
where
    F: FnOnce(u32) -> u32,
{
    f(input)
}

/*
pub fn apply2<F>(f: F, input: i32) -> i32 where F: FnOnce(i32) -> i32 {
    f(input)
}

fn add_one(x: i32) -> i32 {
    x + 1
}
*/

fn main() {
    //let res = apply(|x| x + 1, 2);
    //let res = apply2(add_one, 2);
    let res = apply3(|x| x + 1, 2);
    black_box(res);
}
