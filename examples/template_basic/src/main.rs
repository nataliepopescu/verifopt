use std::{io::Read, ops::ControlFlow};

#[derive(Debug)]
struct MyError {}
struct MyOtherError {}

pub trait Animal {
    fn speak(&self) -> u32;
}

#[derive(Debug)]
struct Cat {}

impl Animal for Cat {
    #[inline(never)]
    fn speak(&self) -> u32 {
        return 3;
    }
}

#[inline(never)]
fn cookie<T, E>(r: Result<T, E>) -> Result<T, E> {
    let t = r?;
    return Ok(t);
}

#[inline(never)]
fn foo<E, T>(v: T, e: E) -> Result<T, E> {
    // verifopttype,
    // IdkStruct(defid, vec<verifopttype>)
    let r = Err(e); // Result<i32, MyError> -> idkstruct, [idktype(i32), idktype(myerror)]
    return cookie(r);
}

#[inline(never)]
fn bar<T>(a: &dyn Animal) -> u32 {
    return a.speak();
}

#[inline(never)]
fn baz<T, E>(v: Result<T, E>) -> Result<T, E> {
    return v;
}

struct MyStruct<T: Copy> {
    data: T,
    id: u32,
}

impl<T: Copy> MyStruct<T> {
    #[inline(never)]
    fn get<E>(&self) -> Result<*const Self, E> {
        return Ok(self);
    }
}

struct MyResult<T, E> {
    data: T,
    error: E,
}

impl<T, E> MyResult<T, E> {
    fn foo<G>(&self, x: G) -> G {
        return x;
    }
}

fn trait_ex<R: Read>(buf: &mut R) {
    // genargs: {[file::Read]}
    let mut x = [0; 10];
    buf.read(&mut x);
}

fn main() {
    // let r: Result<i32, Error> = Ok(0);
    // let x = baz(r);
    let x = foo::<MyError, i32>(0, MyError {});
    let y = foo("test", MyOtherError {});
    // println!("Hello {:?}", x,);
    // let c = Cat {};
    // let x = bar::<u32>(&c);
    // let t = MyStruct { data: 0, id: 83 };
    // let x = t.get::<Error>();
}
