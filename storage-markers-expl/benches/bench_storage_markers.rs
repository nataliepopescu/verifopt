#![feature(ptr_metadata)]

use core::ptr::DynMetadata;
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use std::time::Duration;

use rand::Rng;
use storage_markers_expl::{heap_dep_stack, recurse_large_array, cereal, cereal_200, blowup_stack, stack_overflow};

fn bench_recurse_large_array(c: &mut Criterion) {
    let mut group = c.benchmark_group("rla");

    group.bench_function("main", |b| b.iter(|| std::hint::black_box(recurse_large_array::rla())));
    // group.bench_function("og0sf_mir_rw", |b| {
    //     b.iter_batched(
    //         || {
    //             let animal = og0sf_mir_rw::get_animal(rand::rng().random_range(..2usize));
    //             let cat = og0sf_mir_rw::get_cat();
    //             let animal_vtable = core::ptr::metadata(&*animal);
    //             let cat_vtable = core::ptr::metadata(&*cat);
    //             (animal, animal_vtable, cat_vtable)
    //         },
    //         move |(animal, animal_vtable, cat_vtable)| {
    //             std::hint::black_box(og0sf_mir_rw::run(animal, animal_vtable, cat_vtable))
    //         },
    //         BatchSize::SmallInput,
    //     )
    // });
    group.finish();
}

fn bench_heap_dep_stack(c: &mut Criterion) {
    let mut group = c.benchmark_group("hds");

    group.bench_function("main", |b| b.iter(|| std::hint::black_box(heap_dep_stack::hds())));
    group.finish();
}

fn bench_cereal(c: &mut Criterion) {
    let mut group = c.benchmark_group("cereal");

    group.bench_function("main", |b| b.iter(|| std::hint::black_box(cereal::c())));
    group.finish();
}

fn bench_cereal_200(c: &mut Criterion) {
    let mut group = c.benchmark_group("cereal_200");

    group.bench_function("main", |b| b.iter(|| std::hint::black_box(cereal_200::c_200())));
    group.finish();
}

fn bench_blowup_stack(c: &mut Criterion) {
    let mut group = c.benchmark_group("blowup_bench");

    group.bench_function("bb", |b| b.iter(|| std::hint::black_box(blowup_stack::main())));
    group.finish();
}

fn bench_stack_overflow(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_overflow");

    group.bench_function("so", |b| b.iter(|| std::hint::black_box(stack_overflow::main())));
    group.finish();
}

const SAMPLE_SIZE: usize = 200;
const WARMUP_TIME: u64 = 10;
const MEASUREMENT_TIME: u64 = 30;

criterion_group! {
    name = recurse_large_array_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_recurse_large_array
}

criterion_group! {
    name = heap_dep_stack_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_heap_dep_stack
}

criterion_group! {
    name = cereal_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_cereal
}

criterion_group! {
    name = cereal_200_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_cereal_200
}


criterion_group! {
    name = blowup_stack_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_blowup_stack
}

criterion_group! {
    name = all_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets =
        // bench_recurse_large_array,
        // bench_heap_dep_stack,
        // bench_cereal,
        // bench_cereal_200,
        bench_blowup_stack,
        // bench_stack_overflow,
}


criterion_main!(all_benches);

