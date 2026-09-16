use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use twovariant_ex::{Animal, Cat, wrap_dyn_call_from_inner, wrap_dyn_call_from_outer, wrap_cat_call};

fn bench_twovariant(c: &mut Criterion) {
    let mut group = c.benchmark_group("twovariant");
    group.bench_function("twovariant_inner", |b| {
        b.iter_batched(
            || Box::new(Cat {}),
            |animal| std::hint::black_box(wrap_dyn_call_from_inner(&*animal)),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("twovariant_outer", |b| {
        b.iter_batched(
            || Box::new(Cat {}),
            |animal| std::hint::black_box(wrap_dyn_call_from_outer(&*animal)),
            BatchSize::SmallInput,
        )
    });
    //group.bench_function("twovariant_dynamic", |b| {
    //    b.iter_batched(
    //        || get_animal(x),
    //        |animal| std::hint::black_box(wrap_dyn_call(&*animal)),
    //        BatchSize::SmallInput,
    //    )
    //});
    group.bench_function("twovariant_static", |b| {
        b.iter_batched(
            || Cat {},
            |cat| std::hint::black_box(wrap_cat_call(&cat)),
            BatchSize::SmallInput,
        )
    });
    group.finish()
}

criterion_group!(benches, bench_twovariant);
criterion_main!(benches);
