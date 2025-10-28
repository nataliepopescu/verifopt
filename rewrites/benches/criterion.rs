#![feature(ptr_metadata)]

use core::ptr::DynMetadata;
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use std::time::Duration;

use rand::Rng;
use rewrites::{og0sf, og2sf, og5sf, vec0sf, vec2sf, visitor0sf, visitor2sf};

fn bench_og0sf(c: &mut Criterion) {
    let cat: &og0sf::Cat = &og0sf::Cat {};
    let mut group = c.benchmark_group("og0sf");

    group.bench_function("og0sf_best", |b| b.iter(|| og0sf::run_best(cat)));
    group.bench_function("og0sf_not_rw", |b| {
        b.iter_batched(
            || og0sf::get_animal(rand::rng().random_range(..2usize)),
            move |animal| og0sf::run_not_rw(animal),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("og0sf_src_rw_into_raw", |b| {
        b.iter_batched(
            || {
                let animal = og0sf::get_animal(rand::rng().random_range(..2usize));
                let cat = og0sf::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                og0sf::run_src_rw_into_raw(animal, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("og0sf_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal = og0sf::get_animal(rand::rng().random_range(..2usize));
                let cat = og0sf::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                og0sf::run_src_rw_transmutes(animal, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
    /* FIXME
    group.bench_function(
        "og0sf_mir_rw_transmutes",
        |b| b.iter(|| {
            og0sf_mir_rw::run()
        })
    );
    */
    group.finish();
}

fn bench_og2sf(c: &mut Criterion) {
    let cat: &og2sf::Cat = &og2sf::Cat {
        age: 1,
        num_siblings: 13,
    };
    let mut group = c.benchmark_group("og2sf");

    group.bench_function("og2sf_best", |b| {
        b.iter(|| og2sf::run_best(cat))
    });
    group.bench_function("og2sf_not_rw", |b| {
        b.iter_batched(
            || og2sf::get_animal(rand::rng().random_range(..2usize)),
            move |animal| og2sf::run_not_rw(animal),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("og2sf_src_rw_into_raw", |b| {
        b.iter_batched(
            || {
                let animal =
                    og2sf::get_animal(rand::rng().random_range(..2usize));
                let cat = og2sf::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                og2sf::run_src_rw_into_raw(animal, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("og2sf_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal =
                    og2sf::get_animal(rand::rng().random_range(..2usize));
                let cat = og2sf::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                og2sf::run_src_rw_transmutes(animal, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn bench_og5sf(c: &mut Criterion) {
    let cat: &og5sf::Cat = &og5sf::Cat {
        age: 1,
        num_siblings: 13,
        tmp1: 90,
        tmp2: 80,
        tmp3: 70,
    };
    let mut group = c.benchmark_group("og5sf");

    group.bench_function("og5sf_best", |b| {
        b.iter(|| og5sf::run_best(cat))
    });
    group.bench_function("og5sf_not_rw", |b| {
        b.iter_batched(
            || og5sf::get_animal(rand::rng().random_range(..2usize)),
            move |animal| og5sf::run_not_rw(animal),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("og5sf_src_rw_into_raw", |b| {
        b.iter_batched(
            || {
                let animal =
                    og5sf::get_animal(rand::rng().random_range(..2usize));
                let cat = og5sf::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                og5sf::run_src_rw_into_raw(animal, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("og5sf_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal =
                    og5sf::get_animal(rand::rng().random_range(..2usize));
                let cat = og5sf::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                og5sf::run_src_rw_transmutes(animal, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn new_0sf_hc_vec(
    n_elems: usize,
) -> Vec<(
    Box<dyn vec0sf::Animal>,
    DynMetadata<dyn vec0sf::Animal>,
)> {
    let mut vec = vec![];
    for _ in 0..n_elems {
        let cat = vec0sf::get_cat();
        let cat_vtable = core::ptr::metadata(&*cat);
        vec.insert(0, (cat, cat_vtable));
    }
    vec
}
fn new_0sf_vec(
    n_elems: usize,
) -> Vec<(
    Box<dyn vec0sf::Animal>,
    DynMetadata<dyn vec0sf::Animal>,
)> {
    let mut vec = vec![];
    for _ in 0..n_elems {
        let animal = vec0sf::get_animal(rand::rng().random_range(..2usize));
        let animal_vtable = core::ptr::metadata(&*animal);
        vec.insert(0, (animal, animal_vtable));
    }
    vec
}

fn bench_vec0sf(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec0sf");

    for n_elems in [1000, 4000, 8000, 12000].iter() {
        let vec_hc = new_0sf_hc_vec(*n_elems);
        let vec = new_0sf_vec(*n_elems);

        let cat: &vec0sf::Cat = &vec0sf::Cat {};

        group.bench_function(BenchmarkId::new("vec0sf_best", n_elems), |b| {
            b.iter(|| vec0sf::run_best(&vec_hc, cat))
        });
        group.bench_function(BenchmarkId::new("vec0sf_not_rw", n_elems), |b| {
            b.iter(|| vec0sf::run_not_rw(&vec))
        });
        //group.bench_function(
        //    BenchmarkId::new("vec0sf_src_rw_into_raw", n_elems),
        //    |b| {
        //        b.iter_batched(
        //            || {
        //                let cat = vec0sf::get_cat();
        //                let cat_vtable = core::ptr::metadata(&*cat);
        //                (new_0sf_vec(*n_elems), cat_vtable)
        //            },
        //            move |(vec, cat_vtable)| {
        //                vec0sf::run_src_rw_into_raw(&vec, cat_vtable)
        //            },
        //            BatchSize::SmallInput,
        //        )
        //    },
        //);
        group.bench_function(
            BenchmarkId::new("vec0sf_src_rw_transmutes", n_elems),
            |b| {
                b.iter_batched(
                    || {
                        let cat = vec0sf::get_cat();
                        let cat_vtable = core::ptr::metadata(&*cat);
                        (new_0sf_vec(*n_elems), cat_vtable)
                    },
                    move |(vec, cat_vtable)| {
                        vec0sf::run_src_rw_transmutes(&vec, cat_vtable)
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

fn new_2sf_hc_vec(
    n_elems: usize,
) -> Vec<(
    Box<dyn vec2sf::Animal>,
    DynMetadata<dyn vec2sf::Animal>,
)> {
    let mut vec = vec![];
    for _ in 0..n_elems {
        let cat = vec2sf::get_cat();
        let cat_vtable = core::ptr::metadata(&*cat);
        vec.insert(0, (cat, cat_vtable));
    }
    vec
}
fn new_2sf_vec(
    n_elems: usize,
) -> Vec<(
    Box<dyn vec2sf::Animal>,
    DynMetadata<dyn vec2sf::Animal>,
)> {
    let mut vec = vec![];
    for _ in 0..n_elems {
        let animal = vec2sf::get_animal(rand::rng().random_range(..2usize));
        let animal_vtable = core::ptr::metadata(&*animal);
        vec.insert(0, (animal, animal_vtable));
    }
    vec
}

fn bench_vec2sf(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec2sf");

    for n_elems in [1000, 4000, 8000, 12000].iter() {
        let vec_hc = new_2sf_hc_vec(*n_elems);
        let vec = new_2sf_vec(*n_elems);

        let cat: &vec2sf::Cat = &vec2sf::Cat {
            age: 1,
            num_siblings: 13,
        };

        group.bench_function(BenchmarkId::new("vec2sf_best", n_elems), |b| {
            b.iter(|| vec2sf::run_best(&vec_hc, cat))
        });
        group.bench_function(BenchmarkId::new("vec2sf_not_rw", n_elems), |b| {
            b.iter(|| vec2sf::run_not_rw(&vec))
        });
        //group.bench_function(
        //    BenchmarkId::new("vec2sf_src_rw_into_raw", n_elems),
        //    |b| {
        //        b.iter_batched(
        //            || {
        //                let cat = vec2sf::get_cat();
        //                let cat_vtable = core::ptr::metadata(&*cat);
        //                (new_2sf_vec(*n_elems), cat_vtable)
        //            },
        //            move |(vec, cat_vtable)| {
        //                vec2sf::run_src_rw_into_raw(&vec, cat_vtable)
        //            },
        //            BatchSize::SmallInput,
        //        )
        //    },
        //);
        group.bench_function(
            BenchmarkId::new("vec2sf_src_rw_transmutes", n_elems),
            |b| {
                b.iter_batched(
                    || {
                        let cat = vec2sf::get_cat();
                        let cat_vtable = core::ptr::metadata(&*cat);
                        (new_2sf_vec(*n_elems), cat_vtable)
                    },
                    move |(vec, cat_vtable)| {
                        vec2sf::run_src_rw_transmutes(&vec, cat_vtable)
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

fn bench_visitor0sf(c: &mut Criterion) {
    let cat: &visitor0sf::Cat = &visitor0sf::Cat {};
    let sbd: &visitor0sf::SpeakBetterDogs = &visitor0sf::SpeakBetterDogs {};
    let mut group = c.benchmark_group("visitor0sf");

    group.bench_function("visitor0sf_best", |b| b.iter(|| visitor0sf::run_best(cat, sbd)));
    group.bench_function("visitor0sf_not_rw", |b| {
        b.iter_batched(
            || visitor0sf::get_animal(rand::rng().random_range(..2usize)),
            move |animal| visitor0sf::run_not_rw(animal, sbd),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_src_rw_into_raw", |b| {
        b.iter_batched(
            || {
                let animal = visitor0sf::get_animal(rand::rng().random_range(..2usize));
                let cat = visitor0sf::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                visitor0sf::run_src_rw_into_raw(animal, sbd, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal = visitor0sf::get_animal(rand::rng().random_range(..2usize));
                let cat = visitor0sf::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                visitor0sf::run_src_rw_transmutes(animal, sbd, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_visitor2sf(c: &mut Criterion) {
    let cat: &visitor2sf::Cat = &visitor2sf::Cat {
        age: 1,
        num_siblings: 13,
    };
    let sbd: &visitor2sf::SpeakBetterDogs = &visitor2sf::SpeakBetterDogs {};
    let mut group = c.benchmark_group("visitor2sf");

    group.bench_function("visitor2sf_best", |b| b.iter(|| visitor2sf::run_best(cat, sbd)));
    group.bench_function("visitor2sf_not_rw", |b| {
        b.iter_batched(
            || visitor2sf::get_animal(rand::rng().random_range(..2usize)),
            move |animal| visitor2sf::run_not_rw(animal, sbd),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor2sf_src_rw_into_raw", |b| {
        b.iter_batched(
            || {
                let animal = visitor2sf::get_animal(rand::rng().random_range(..2usize));
                let cat = visitor2sf::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                visitor2sf::run_src_rw_into_raw(animal, sbd, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor2sf_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal = visitor2sf::get_animal(rand::rng().random_range(..2usize));
                let cat = visitor2sf::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                visitor2sf::run_src_rw_transmutes(animal, sbd, animal_vtable, cat_vtable)
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
    name = og0sf_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_og0sf
}

criterion_group! {
    name = og2sf_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_og2sf
}

criterion_group! {
    name = og5sf_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_og5sf
}

criterion_group! {
    name = vec0sf_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_vec0sf
}

criterion_group! {
    name = vec2sf_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_vec2sf
}

criterion_group! {
    name = visitor0sf_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_visitor0sf
}

criterion_group! {
    name = visitor2sf_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_visitor2sf
}

criterion_group! {
    name = all_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets =
        bench_og0sf,
        bench_og2sf,
        bench_og5sf,
        bench_vec0sf,
        bench_vec2sf,
        bench_visitor0sf,
        bench_visitor2sf,
}

criterion_main!(all_benches);
