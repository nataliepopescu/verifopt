use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

use bench::{add_one, apply};

fn bench_add_one(c: &mut Criterion) {
    c.bench_function("bench_add_one", |b| b.iter(|| {
        black_box(add_one(2));
    }));
}

fn bench_apply(c: &mut Criterion) {
    c.bench_function("bench_apply", |b| b.iter(|| {
        black_box(apply(|x| x + 1, 2));
    }));
}

criterion_group!(benches, bench_add_one, bench_apply);

criterion_main!(benches);
