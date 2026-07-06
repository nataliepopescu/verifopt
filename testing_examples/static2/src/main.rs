
struct Thing {
    inner: i32,
}

fn add_one(x: i32) -> i32 {
    x + 1
}

// adt
static THING: Thing = Thing { inner: 6 };

// fn ptr
static MY_FN: fn(i32) -> i32 = add_one;

// closure
static MY_CLOSURE: fn(i32) -> i32 = |x| x + 1;

fn call(f: &dyn Fn(i32) -> i32, x: i32) -> i32 { f(x) }

fn main() {
    println!("{}", call(&MY_FN, 5)); // 6
    println!("{}", call(&MY_CLOSURE, 5)); // 6
    println!("{}", THING.inner); // 6
}
