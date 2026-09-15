use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use negative_ex::{Animal, get_animal, wrap_dyn_call};

pub fn run(a: Box<dyn Animal>) -> usize {
    wrap_dyn_call(a)
}

fn bench_negative(c: &mut Criterion) {
    let mut group = c.benchmark_group("negative");
    let x = 0;
    group.bench_function("negative_mir_rw", |b| {
        b.iter_batched(
            || get_animal(x),
            |animal| std::hint::black_box(run(animal)),
            BatchSize::SmallInput,
        )
    });
    group.finish()
}

criterion_group!(benches, bench_negative);
criterion_main!(benches);
