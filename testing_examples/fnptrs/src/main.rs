fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let fn_pointer: fn(i32) -> i32 = add_one;
    let output = fn_pointer(2);
    println!("output: {}", output);
}
