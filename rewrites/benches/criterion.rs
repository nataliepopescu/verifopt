#![feature(ptr_metadata)]

use core::ptr::DynMetadata;
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use std::time::Duration;

use rand::Rng;
use rewrites::{simple, struct_fields, vec_simple, visitor};

fn bench_simple(c: &mut Criterion) {
    let cat: &simple::Cat = &simple::Cat {};
    let mut group = c.benchmark_group("simple");

    group.bench_function("simple_best", |b| b.iter(|| simple::run_best(cat)));
    group.bench_function("simple_not_rw", |b| {
        b.iter_batched(
            || simple::get_animal(rand::rng().random_range(..2usize)),
            move |animal| simple::run_not_rw(animal),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("simple_src_rw_into_raw", |b| {
        b.iter_batched(
            || {
                let animal = simple::get_animal(rand::rng().random_range(..2usize));
                let cat = simple::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                simple::run_src_rw_into_raw(animal, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("simple_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal = simple::get_animal(rand::rng().random_range(..2usize));
                let cat = simple::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                simple::run_src_rw_transmutes(animal, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
    /*group.bench_function("simple_src_rw_into_raw_fallback", |b| {
        b.iter_batched(
            || {
                let animal = simple::get_animal(rand::rng().random_range(..2usize));
                let cat = simple::get_cat();
                let dog = simple::get_dog();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                let dog_vtable = core::ptr::metadata(&*dog);
                (animal, animal_vtable, cat_vtable, dog_vtable)
            },
            move |(animal, animal_vtable, cat_vtable, dog_vtable)| {
                simple::run_src_rw_into_raw_fallback(
                    animal,
                    animal_vtable,
                    cat_vtable,
                    dog_vtable,
                )
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("simple_src_rw_transmutes_fallback", |b| {
        b.iter_batched(
            || {
                let animal = simple::get_animal(rand::rng().random_range(..2usize));
                let cat = simple::get_cat();
                let dog = simple::get_dog();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                let dog_vtable = core::ptr::metadata(&*dog);
                (animal, animal_vtable, cat_vtable, dog_vtable)
            },
            move |(animal, animal_vtable, cat_vtable, dog_vtable)| {
                simple::run_src_rw_transmutes_fallback(
                    animal,
                    animal_vtable,
                    cat_vtable,
                    dog_vtable,
                )
            },
            BatchSize::SmallInput,
        )
    });*/
    /* FIXME
    group.bench_function(
        "simple_mir_rw_transmutes",
        |b| b.iter(|| {
            simple_mir_rw::run()
        })
    );
    */
    group.finish();
}

fn bench_struct_fields(c: &mut Criterion) {
    let cat: &struct_fields::Cat = &struct_fields::Cat {
        age: 1,
        num_siblings: 13,
    };
    let mut group = c.benchmark_group("struct_fields");

    group.bench_function("struct_fields_best", |b| {
        b.iter(|| struct_fields::run_best(cat))
    });
    group.bench_function("struct_fields_not_rw", |b| {
        b.iter_batched(
            || struct_fields::get_animal(rand::rng().random_range(..2usize)),
            move |animal| struct_fields::run_not_rw(animal),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("struct_fields_src_rw_into_raw", |b| {
        b.iter_batched(
            || {
                let animal =
                    struct_fields::get_animal(rand::rng().random_range(..2usize));
                let cat = struct_fields::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                struct_fields::run_src_rw_into_raw(animal, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("struct_fields_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal =
                    struct_fields::get_animal(rand::rng().random_range(..2usize));
                let cat = struct_fields::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                struct_fields::run_src_rw_transmutes(animal, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
    /*group.bench_function("struct_fields_src_rw_into_raw_fallback", |b| {
        b.iter_batched(
            || {
                let animal =
                    struct_fields::get_animal(rand::rng().random_range(..2usize));
                let cat = struct_fields::get_cat();
                let dog = struct_fields::get_dog();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                let dog_vtable = core::ptr::metadata(&*dog);
                (animal, animal_vtable, cat_vtable, dog_vtable)
            },
            move |(animal, animal_vtable, cat_vtable, dog_vtable)| {
                struct_fields::run_src_rw_into_raw_fallback(
                    animal,
                    animal_vtable,
                    cat_vtable,
                    dog_vtable,
                )
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("struct_fields_src_rw_transmutes_fallback", |b| {
        b.iter_batched(
            || {
                let animal =
                    struct_fields::get_animal(rand::rng().random_range(..2usize));
                let cat = struct_fields::get_cat();
                let dog = struct_fields::get_dog();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                let dog_vtable = core::ptr::metadata(&*dog);
                (animal, animal_vtable, cat_vtable, dog_vtable)
            },
            move |(animal, animal_vtable, cat_vtable, dog_vtable)| {
                struct_fields::run_src_rw_transmutes_fallback(
                    animal,
                    animal_vtable,
                    cat_vtable,
                    dog_vtable,
                )
            },
            BatchSize::SmallInput,
        )
    });*/
    group.finish();
}

fn new_hc_vec(
    n_elems: usize,
) -> Vec<(
    Box<dyn vec_simple::Animal>,
    DynMetadata<dyn vec_simple::Animal>,
)> {
    let mut vec = vec![];
    for _ in 0..n_elems {
        let cat = vec_simple::get_cat();
        let cat_vtable = core::ptr::metadata(&*cat);
        vec.insert(0, (cat, cat_vtable));
    }
    vec
}
fn new_vec(
    n_elems: usize,
) -> Vec<(
    Box<dyn vec_simple::Animal>,
    DynMetadata<dyn vec_simple::Animal>,
)> {
    let mut vec = vec![];
    for _ in 0..n_elems {
        let animal = vec_simple::get_animal(rand::rng().random_range(..2usize));
        let animal_vtable = core::ptr::metadata(&*animal);
        vec.insert(0, (animal, animal_vtable));
    }
    vec
}

fn bench_vec_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_simple");

    for n_elems in [1000, 4000, 8000, 12000].iter() {
        let vec_hc = new_hc_vec(*n_elems);
        let vec = new_vec(*n_elems);

        let cat: &vec_simple::Cat = &vec_simple::Cat {};

        group.bench_function(BenchmarkId::new("vec_simple_best", n_elems), |b| {
            b.iter(|| vec_simple::run_best(&vec_hc, cat))
        });
        group.bench_function(BenchmarkId::new("vec_simple_not_rw", n_elems), |b| {
            b.iter(|| vec_simple::run_not_rw(&vec))
        });
        //group.bench_function(
        //    BenchmarkId::new("vec_simple_src_rw_into_raw", n_elems),
        //    |b| {
        //        b.iter_batched(
        //            || {
        //                let cat = vec_simple::get_cat();
        //                let cat_vtable = core::ptr::metadata(&*cat);
        //                (new_vec(*n_elems), cat_vtable)
        //            },
        //            move |(vec, cat_vtable)| {
        //                vec_simple::run_src_rw_into_raw(&vec, cat_vtable)
        //            },
        //            BatchSize::SmallInput,
        //        )
        //    },
        //);
        group.bench_function(
            BenchmarkId::new("vec_simple_src_rw_transmutes", n_elems),
            |b| {
                b.iter_batched(
                    || {
                        let cat = vec_simple::get_cat();
                        let cat_vtable = core::ptr::metadata(&*cat);
                        (new_vec(*n_elems), cat_vtable)
                    },
                    move |(vec, cat_vtable)| {
                        vec_simple::run_src_rw_transmutes(&vec, cat_vtable)
                    },
                    BatchSize::SmallInput,
                )
            },
        );
        /*group.bench_function(
            BenchmarkId::new("vec_simple_src_rw_transmutes_fallback", n_elems),
            |b| {
                b.iter_batched(
                    || {
                        let cat = vec_simple::get_cat();
                        let dog = vec_simple::get_dog();
                        let cat_vtable = core::ptr::metadata(&*cat);
                        let dog_vtable = core::ptr::metadata(&*dog);
                        (new_vec(*n_elems), cat_vtable, dog_vtable)
                    },
                    move |(vec, cat_vtable, dog_vtable)| {
                        vec_simple::run_src_rw_fallback(&vec, cat_vtable, dog_vtable)
                    },
                    BatchSize::SmallInput,
                )
            },
        );*/
    }
    group.finish();
}

fn bench_visitor(c: &mut Criterion) {
    let cat: &visitor::Cat = &visitor::Cat {};
    let sbd: &visitor::SpeakBetterDogs = &visitor::SpeakBetterDogs {};
    let mut group = c.benchmark_group("visitor");

    group.bench_function("visitor_best", |b| b.iter(|| visitor::run_best(cat, sbd)));
    group.bench_function("visitor_not_rw", |b| {
        b.iter_batched(
            || visitor::get_animal(rand::rng().random_range(..2usize)),
            move |animal| visitor::run_not_rw(animal, sbd),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor_src_rw_into_raw", |b| {
        b.iter_batched(
            || {
                let animal = visitor::get_animal(rand::rng().random_range(..2usize));
                let cat = visitor::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                visitor::run_src_rw_into_raw(animal, sbd, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal = visitor::get_animal(rand::rng().random_range(..2usize));
                let cat = visitor::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                visitor::run_src_rw_transmutes(animal, sbd, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
}

// TODO
// more than 2 trait implementations
// multiple trait methods
// different paths -> trait method call
// more traits impl than used

const SAMPLE_SIZE: usize = 200;
const WARMUP_TIME: u64 = 5;
const MEASUREMENT_TIME: u64 = 10;

criterion_group! {
    name = simple_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_simple
}

criterion_group! {
    name = struct_fields_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_struct_fields
}

criterion_group! {
    name = vec_simple_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_vec_simple
}

criterion_group! {
    name = visitor_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_visitor
}

criterion_main!(vec_simple_benches);
