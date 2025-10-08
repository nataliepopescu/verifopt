use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rand::Rng;
use rewrites::{not_rw, mir_rw, src_rw};

fn get_input_vec() -> Vec<usize> {
    let mut nums_vec: Vec<usize> = vec![];
    for _ in 0..1000 {
        nums_vec.push(rand::rng().random_range(..2));
    }
    nums_vec
}

fn bench_not_rw(c: &mut Criterion) {
    let nums_vec = get_input_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;

    c.bench_function(
        "not_rw",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            not_rw::run(num)
        })
    );
}

fn bench_mir_rw(c: &mut Criterion) {
    let nums_vec = get_input_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;

    c.bench_function(
        "mir_rw",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            mir_rw::run(num)
        })
    );
}

fn bench_src_rw(c: &mut Criterion) {
    let nums_vec = get_input_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;

    c.bench_function(
        "src_rw",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            src_rw::run(num)
        })
    );
}

criterion_group!(benches, bench_not_rw, bench_mir_rw, bench_src_rw);
criterion_main!(benches);

