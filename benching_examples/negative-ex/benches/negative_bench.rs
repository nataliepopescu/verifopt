use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use negative_ex::{Animal, get_animal, wrap_dyn_call1, wrap_dyn_call2};

pub fn run_inner_setup(a: &dyn Animal) -> usize {
    wrap_dyn_call1(a)
}

pub fn run_outer_setup(a: &dyn Animal) -> usize {
    wrap_dyn_call2(a)
}

fn bench_negative(c: &mut Criterion) {
    let mut group = c.benchmark_group("negative");
    let x = 0;
    group.bench_function("negative_inner", |b| {
        b.iter_batched(
            || get_animal(x),
            |animal| std::hint::black_box(run_inner_setup(&*animal)),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("negative_outer", |b| {
        b.iter_batched(
            || get_animal(x),
            |animal| std::hint::black_box(run_outer_setup(&*animal)),
            BatchSize::SmallInput,
        )
    });
    group.finish()
}

criterion_group!(benches, bench_negative);
criterion_main!(benches);
