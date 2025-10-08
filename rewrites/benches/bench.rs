#![feature(test)]

extern crate test;

use test::Bencher;
use rand::Rng;
use rewrites::{not_rw, mir_rw, src_rw};

fn get_input_vec() -> Vec<usize> {
    let mut nums_vec: Vec<usize> = vec![];
    for _ in 0..1000 {
        nums_vec.push(rand::rng().random_range(..2));
    }
    nums_vec
}

#[bench]
fn bench_not_rw(b: &mut Bencher) {
    let nums_vec = get_input_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;

    b.iter(|| {
        let num = test::black_box(nums[idx % 1000]);
        idx += 1;
        not_rw::run(num)
    })
}

#[bench]
fn bench_mir_rw(b: &mut Bencher) {
    let nums_vec = get_input_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;

    b.iter(|| {
        let num = test::black_box(nums[idx % 1000]);
        idx += 1;
        mir_rw::run(num)
    })
}

#[bench]
fn bench_src_rw(b: &mut Bencher) {
    let nums_vec = get_input_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;

    b.iter(|| {
        let num = test::black_box(nums[idx % 1000]);
        idx += 1;
        src_rw::run(num)
    })
}

