pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn apply(f: fn(i32) -> i32, input: i32) -> i32 {
    f(input)
}
