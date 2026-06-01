use std::cell::Cell;
//use std::hint::black_box;

fn add_one(x: i32) -> i32 {
    x + 1
}

thread_local! {
    pub static FOO: Cell<fn(i32) -> i32> = const { Cell::new(add_one) };
}

fn main() {
    let _res = FOO.get()(2);
    //black_box(res);
    //println!("res: {}", res);
}
