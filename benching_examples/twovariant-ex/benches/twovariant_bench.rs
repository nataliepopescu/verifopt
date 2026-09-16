use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use twovariant_ex::{Animal, Cat, get_cat, get_animal, wrap_dyn_call_from_inner, wrap_dyn_call_from_outer, wrap_cat_call};

pub fn run_inner_setup(a: &dyn Animal) -> usize {
    wrap_dyn_call_from_inner(a)
}

pub fn run_outer_setup(a: &dyn Animal) -> usize {
    wrap_dyn_call_from_outer(a)
}

//pub fn run_no_setup(a: &dyn Animal) -> usize {
//    wrap_dyn_call(a)
//}

pub fn run_cat(c: &Cat) -> usize {
    wrap_cat_call(c)
}

fn bench_twovariant(c: &mut Criterion) {
    let mut group = c.benchmark_group("twovariant");
    let x = 0;
    group.bench_function("twovariant_inner", |b| {
        b.iter_batched(
            || get_animal(x),
            |animal| std::hint::black_box(run_inner_setup(&*animal)),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("twovariant_outer", |b| {
        b.iter_batched(
            || get_animal(x),
            |animal| std::hint::black_box(run_outer_setup(&*animal)),
            BatchSize::SmallInput,
        )
    });
    //group.bench_function("twovariant_dynamic", |b| {
    //    b.iter_batched(
    //        || get_animal(x),
    //        |animal| std::hint::black_box(run_no_setup(&*animal)),
    //        BatchSize::SmallInput,
    //    )
    //});
    group.bench_function("twovariant_static", |b| {
        b.iter_batched(
            || get_cat(),
            |cat| std::hint::black_box(run_cat(&cat)),
            BatchSize::SmallInput,
        )
    });
    group.finish()
}

criterion_group!(benches, bench_twovariant);
criterion_main!(benches);
