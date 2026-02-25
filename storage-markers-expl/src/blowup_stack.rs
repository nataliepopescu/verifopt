//@ run-pass
//@ ignore-backends: gcc

#![allow(dead_code)]
// check that we don't have linear stack usage with multiple calls to `push`

use std::mem;
use std::hint::black_box;

fn meal() -> Big {
    if black_box(false) {
        panic!()
    }
    Big { drop_me: [
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
    ]}
}

pub struct Big {
    drop_me: [Option<Box<u8>>; 48],
}

// THIS TEST WON'T WORK IF THIS INLINE IS MISSING!!!
#[inline]
fn push(out: &mut Vec<Big>) {
    out.push(meal());
}

#[inline(never)]
pub fn supersize_me(out: &mut Vec<Big>) {
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out); // 16 calls to `push`

    verify_stack_usage(out);

    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out);
    push(out); // 16 calls to `push`
}

#[inline(never)]
fn verify_stack_usage(before_ptr: *mut Vec<Big>) {
    // To check stack usage, create locals before and after
    // and check the difference in addresses between them.
    let mut stack_var: Vec<Big> = vec![];
    black_box(&mut stack_var);
    let stack_usage = isize::abs(
        (&mut stack_var as *mut _ as isize) -
            (before_ptr as isize)) as usize;
    // Give space for 2 copies of `Big` + 272 "misc" bytes
    // (value observed on x86_64-pc-windows-gnu).
    // if stack_usage > mem::size_of::<Big>() * 2 + 272 {
    //     panic!("used {} bytes of stack, but `struct Big` is only {} bytes",
    //            stack_usage, mem::size_of::<Big>());
    // }
    // println!("used {} bytes of stack, should at most be {}", stack_usage, mem::size_of::<Big>() * 2 + 272);
}

pub fn main() {
    let mut v = vec![];
    black_box(&mut v);
    supersize_me(&mut v);
}