#![feature(ptr_metadata)]
#![allow(dead_code)]

use std::ptr::DynMetadata;

pub trait Shape {
    fn area(&self) -> f32;
}

#[inline(never)]
pub fn get_shape(num: usize) -> Box<dyn Shape> {
    if num == 0 {
        Box::new(Circle { r: 2.87 })
    } else {
        Box::new(Triangle { h: 4.34, w: 8.45 })
    }
}

pub struct Circle {
    r: f32
}
pub struct Rectangle {
    h: f32,
    w: f32,
}
pub struct Triangle {
    h: f32,
    w: f32,
}
impl Shape for Circle {
    fn area(&self) -> f32 {
        3.14 * self.r.powi(2)
    }
}
impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.h * self.w
    }
}
impl Shape for Triangle {
    fn area(&self) -> f32 {
        self.h * self.w / 2.0
    }
}

#[inline(never)]
pub fn foo(
    s: &Box<dyn Shape>,
    _s_vtable: DynMetadata<dyn Shape>,
    _circle_vtable: DynMetadata<dyn Shape>,
) -> f32 {
    s.area()
}

#[inline(never)]
pub fn bar(c: &Circle) -> f32 {
    c.area()
}

#[inline(never)]
pub fn baz<T: Shape>(s: &T) -> f32 {
    s.area()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            let x = args[1].parse().unwrap();
            let shape = get_shape(x);
            let circle = get_shape(0);
            let shape_vtable = core::ptr::metadata(&*shape);
            let circle_vtable = core::ptr::metadata(&*circle);
            let res_dyn = foo(&shape, shape_vtable, circle_vtable);

            println!("res_dyn: {}", res_dyn);

            let circle_static = Circle { r: 2.87 };
            let res_bar = bar(&circle_static);
            let res_baz = baz(&circle_static);

            println!("res_bar: {}", res_bar);
            println!("res_baz: {}", res_baz);

            //let bits64: u64 = 0x4039DD2640000000;
            //let value64 = f64::from_bits(bits64);
            //println!("{}", value64);
        }
    }
}
