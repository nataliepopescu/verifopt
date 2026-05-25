/*
pub fn apply(f: fn(i32) -> i32, input: i32) -> i32 {
    f(input)
}

pub fn apply2<F>(f: F, input: i32) -> i32 where F: FnOnce(i32) -> i32 {
    f(input)
}

pub fn ret_fn() -> impl FnOnce(i32) -> i32 {
    add_one
}
*/

fn add_one(x: i32) -> i32 {
    x + 1
}

/*
pub struct TestFnPtr {
    inner: fn(i32) -> i32,
}

impl TestFnPtr {
    pub fn new(inner: fn(i32) -> i32) -> Self {
        Self { inner }
    }

    pub fn call(&self, input: i32) -> i32 {
        (self.inner)(input)
    }
}
*/

pub struct TestFnPtr<T: 'static> {
    inner: fn(Option<T>) -> T,
}

impl<T: 'static> TestFnPtr<T> {
    pub fn new(inner: fn(T) -> T) -> Self {
        Self { inner }
    }

    pub fn call(&self, input: T) -> T {
        (self.inner)(input)
    }
}

fn main() {
    //let fn_pointer: fn(i32) -> i32 = add_one;
    //let output = fn_pointer(2);
    //println!("output: {}", output);

    //let output2 = apply(|x| x+1, 2);
    //println!("output2: {}", output2);

    //let output3 = apply2(add_one, 2);
    //println!("output3: {}", output3);

    //let arr: [fn(i32) -> i32; 1] = [add_one];
    //let vec = vec![add_one];
    //for item in vec {
    //    println!("loop output: {}", item(2));
    //}

    //let rng = rand::rng();
    //black_box(rng);

    let t = TestFnPtr::new(add_one);
    let res = t.call(2);
    println!("res: {}", res);
}
