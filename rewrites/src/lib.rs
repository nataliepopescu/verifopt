#![feature(ptr_metadata)]

pub mod not_rw;
pub mod mir_rw;
pub mod src_rw;

pub fn add_two(a: i32) -> i32 {
    a + 2
}
