// visitor_not_rw.rs - the "not rewritten" baseline for the double-
// dispatch visitor pattern in src/main.rs. Built via
// `cargo verifopt --bench visitor_not_rw --skip-analysis --skip-rewrite`
// - --skip-rewrite unconditionally suppresses any rewrite regardless of
// what's already in verifopt_store.json, and --skip-analysis avoids
// paying for a redundant discovery pass this target was never going to
// use anyway. See visitor_mir_rw.rs for the counterpart this gets
// compared against - deliberately near-identical source, since the
// only thing that should differ between the two is whether the real
// rewrite mechanism actually gets to run.
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use visitor_decl::{Animal, AnimalVisitor, Cat, Dog};
use visitor_use::{
    SpeakBetterCats, SpeakBetterDogs,
    get_animal, get_visitor,
    //get_cat, get_dog, get_sbc, get_sbd,
    wrap_speak, wrap_receive_dog,

    wrap_visit_dynanimal,
    wrap_visit_dynanimal1,
    wrap_visit_dynanimal2,
    wrap_visit_dynanimal3,

    wrap_visit_dynvisitor,
    wrap_visit_dynvisitor_direct_sbd,
    wrap_visit_dynvisitor_funcret_sbd,
    wrap_visit_dynvisitor_direct_sbc,
    wrap_visit_dynvisitor_funcret_sbc,

    wrap_visit_dynboth,
    wrap_visit_dynboth1,
    wrap_visit_dynboth2,
    wrap_visit_dynboth3,
    wrap_visit_dynboth4,
    wrap_visit_dynboth5,
};
//use rand::Rng;
use std::hint::black_box;

/*
fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        return get_cat();
    } else {
        return get_dog();
    }
}

fn get_visitor(num: usize) -> Box<dyn AnimalVisitor> {
    if num == 0 {
        return get_sbc();
    } else {
        return get_sbd();
    }
}
*/

fn bench_visitor(c: &mut Criterion) {
    let sbc = SpeakBetterCats;
    let sbd = SpeakBetterDogs;
    let d = Dog;
    let a = get_animal(1);
    let v = get_visitor(1);

    let mut group = c.benchmark_group("visitor");

    group.bench_function("visitor_speak", |b| {
        b.iter_batched(
            || {},
            |_| black_box(wrap_speak(&d, &sbd)),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor_receive_dog", |b| {
        b.iter_batched(
            || {},
            |_| black_box(wrap_receive_dog(&d, &sbd)),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor_dynanimal", |b| {
        b.iter_batched(
            || {},
            |_| black_box(wrap_visit_dynanimal(&*a, &sbd)),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor_dynvisitor", |b| {
        b.iter_batched(
            || {},
            |_| black_box(wrap_visit_dynvisitor(&d, &*v)),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor_dynboth", |b| {
        b.iter_batched(
            || {},
            |_| black_box(wrap_visit_dynboth(&*a, &*v)),
            BatchSize::SmallInput,
        )
    });

    //group.bench_function("visitor_visit_dynanimal", |b| {
    //    b.iter_batched(
    //        || {},
    //        |_| black_box(wrap_visit_dynanimal(&d, &sbd)),
    //        BatchSize::SmallInput,
    //    )
    //});
    //group.bench_function("visitor_visit_dynanimal2", |b| {
    //    b.iter_batched(
    //        || get_animal(1),
    //        |a| black_box(wrap_visit_dynanimal2(&*a, &sbd)),
    //        BatchSize::SmallInput,
    //    )
    //});
    //group.bench_function("visitor_visit_dynanimal3", |b| {
    //    b.iter_batched(
    //        || get_animal(0),
    //        |a| black_box(wrap_visit_dynanimal3(&*a, &sbd)),
    //        BatchSize::SmallInput,
    //    )
    //});

    //group.bench_function("visitor_visit_dynvisitor_direct_sbd", |b| {
    //    b.iter_batched(
    //        || {},
    //        |_| black_box(wrap_visit_dynvisitor_direct_sbd(&d, &sbd)),
    //        BatchSize::SmallInput,
    //    )
    //});
    //group.bench_function("visitor_visit_dynvisitor_funcret_sbd", |b| {
    //    b.iter_batched(
    //        || get_visitor(1), 
    //        |v| black_box(wrap_visit_dynvisitor_funcret_sbd(&d, &*v)),
    //        BatchSize::SmallInput,
    //    )
    //});
    //group.bench_function("visitor_visit_dynvisitor_direct_sbc", |b| {
    //    b.iter_batched(
    //        || {},
    //        |_| black_box(wrap_visit_dynvisitor_direct_sbc(&d, &sbc)),
    //        BatchSize::SmallInput,
    //    )
    //});
    //group.bench_function("visitor_visit_dynvisitor_funcret_sbc", |b| {
    //    b.iter_batched(
    //        || get_visitor(0), 
    //        |v| black_box(wrap_visit_dynvisitor_funcret_sbc(&d, &*v)),
    //        BatchSize::SmallInput,
    //    )
    //});

    //group.bench_function("visitor_visit_dynboth", |b| {
    //    b.iter_batched(
    //        || {},
    //        |_| black_box(wrap_visit_dynboth(&d, &sbd)),
    //        BatchSize::SmallInput,
    //    )
    //});
    //group.bench_function("visitor_visit_dynboth2", |b| {
    //    b.iter_batched(
    //        || (
    //            get_animal(0),
    //            get_visitor(0),
    //        ),
    //        |(a, v)| black_box(wrap_visit_dynboth2(&*a, &*v)),
    //        BatchSize::SmallInput,
    //    )
    //});
    //group.bench_function("visitor_visit_dynboth3", |b| {
    //    b.iter_batched(
    //        || (
    //            get_animal(0),
    //            get_visitor(1),
    //        ),
    //        |(a, v)| black_box(wrap_visit_dynboth3(&*a, &*v)),
    //        BatchSize::SmallInput,
    //    )
    //});
    //group.bench_function("visitor_visit_dynboth4", |b| {
    //    b.iter_batched(
    //        || (
    //            get_animal(1),
    //            get_visitor(1),
    //        ),
    //        |(a, v)| black_box(wrap_visit_dynboth4(&*a, &*v)),
    //        BatchSize::SmallInput,
    //    )
    //});
    //group.bench_function("visitor_visit_dynboth5", |b| {
    //    b.iter_batched(
    //        || (
    //            get_animal(1),
    //            get_visitor(0),
    //        ),
    //        |(a, v)| black_box(wrap_visit_dynboth5(&*a, &*v)),
    //        BatchSize::SmallInput,
    //    )
    //});

    //group.bench_function("visitor_rand_both", |b| {
    //    b.iter_batched(
    //        || get_animal(rand::rng().random_range(..2usize)),
    //        |animal| std::hint::black_box(run(animal, &sbd)),
    //        BatchSize::SmallInput,
    //    )
    //});
    group.finish();
}

criterion_group!(benches, bench_visitor);
criterion_main!(benches);
