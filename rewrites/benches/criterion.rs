#![feature(ptr_metadata)]

use core::ptr::DynMetadata;
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use std::time::Duration;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use rand::Rng;
use rewrites::{og0sf, og2sf, og5sf, og0sf_mir_rw, og2sf_mir_rw, vec0sf, vec2sf, visitor0sf, visitor0sf_import, visitor2sf, prime2sf, visitor0sf_2, double_visitor0sf, double_visitor0sf_ref};

fn bench_og0sf(c: &mut Criterion) {
    let cat: &og0sf::Cat = &og0sf::Cat {};
    let mut group = c.benchmark_group("og0sf");

    group.bench_function("og0sf_best", |b| b.iter(|| std::hint::black_box(og0sf::run_best(cat))));
    group.bench_function("og0sf_not_rw", |b| {
        b.iter_batched(
            || og0sf::get_animal(rand::rng().random_range(..2usize)),
            move |animal| std::hint::black_box(og0sf::run_not_rw(animal)),
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
                std::hint::black_box(og0sf::run_src_rw_into_raw(animal, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    /*group.bench_function("og0sf_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal = og0sf::get_animal(rand::rng().random_range(..2usize));
                let cat = og0sf::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(og0sf::run_src_rw_transmutes(animal, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });*/
    group.bench_function("og0sf_mir_rw", |b| {
        b.iter_batched(
            || {
                let animal = og0sf_mir_rw::get_animal(rand::rng().random_range(..2usize));
                let cat = og0sf_mir_rw::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(og0sf_mir_rw::run(animal, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn bench_og2sf(c: &mut Criterion) {
    let cat: &og2sf::Cat = &og2sf::Cat {
        age: 1,
        num_siblings: 13,
    };
    let mut group = c.benchmark_group("og2sf");

    group.bench_function("og2sf_best", |b| {
        b.iter(|| std::hint::black_box(og2sf::run_best(cat)))
    });
    group.bench_function("og2sf_not_rw", |b| {
        b.iter_batched(
            || og2sf::get_animal(rand::rng().random_range(..2usize)),
            move |animal| std::hint::black_box(og2sf::run_not_rw(animal)),
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
                std::hint::black_box(og2sf::run_src_rw_into_raw(animal, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    /*group.bench_function("og2sf_src_rw_transmutes", |b| {
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
                std::hint::black_box(og2sf::run_src_rw_transmutes(animal, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });*/
    group.bench_function("og2sf_mir_rw", |b| {
        b.iter_batched(
            || {
                let animal = og2sf_mir_rw::get_animal(rand::rng().random_range(..2usize));
                let cat = og2sf_mir_rw::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(og2sf_mir_rw::run(animal, animal_vtable, cat_vtable))
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
        b.iter(|| std::hint::black_box(og5sf::run_best(cat)))
    });
    group.bench_function("og5sf_not_rw", |b| {
        b.iter_batched(
            || og5sf::get_animal(rand::rng().random_range(..2usize)),
            move |animal| std::hint::black_box(og5sf::run_not_rw(animal)),
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
                std::hint::black_box(og5sf::run_src_rw_into_raw(animal, animal_vtable, cat_vtable))
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
                std::hint::black_box(og5sf::run_src_rw_transmutes(animal, animal_vtable, cat_vtable))
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
            b.iter(|| std::hint::black_box(vec0sf::run_best(&vec_hc, cat)))
        });
        group.bench_function(BenchmarkId::new("vec0sf_not_rw", n_elems), |b| {
            b.iter(|| std::hint::black_box(vec0sf::run_not_rw(&vec)))
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
        //                std::hint::black_box(vec0sf::run_src_rw_into_raw(&vec, cat_vtable))
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
                        std::hint::black_box(vec0sf::run_src_rw_transmutes(&vec, cat_vtable))
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
            b.iter(|| std::hint::black_box(vec2sf::run_best(&vec_hc, cat)))
        });
        group.bench_function(BenchmarkId::new("vec2sf_not_rw", n_elems), |b| {
            b.iter(|| std::hint::black_box(vec2sf::run_not_rw(&vec)))
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
        //                std::hint::black_box(vec2sf::run_src_rw_into_raw(&vec, cat_vtable))
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
                        std::hint::black_box(vec2sf::run_src_rw_transmutes(&vec, cat_vtable))
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
    let dog: &visitor0sf_2::Dog = &visitor0sf_2::Dog {};
    let sbd0_2: &visitor0sf_2::Visitor1 = &visitor0sf_2::Visitor1 { };
    let cat0_2: &visitor0sf_2::Cat = &visitor0sf_2::Cat {};
    let mut group = c.benchmark_group("visitor0sf");

    group.bench_function("visitor0sf_best", |b| b.iter(|| std::hint::black_box(visitor0sf::run_best(cat, sbd))));
    group.bench_function("visitor0sf_best_cat", |b| b.iter(|| std::hint::black_box(visitor0sf::run_best_cat(cat, sbd))));

    group.bench_function("visitor0sf_not_rw", |b| {
        b.iter_batched(
            // || visitor0sf::get_animal(rand::rng().random_range(..2usize)),
            || {
                let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf::get_animal(x);
                animal
            },
            move |animal| {
                std::hint::black_box(visitor0sf::run_not_rw(animal, sbd))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_not_rw_alwaysdog", |b| {
        b.iter_batched(
            // || visitor0sf::get_animal(rand::rng().random_range(..2usize)),
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/zero.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf::get_animal(x.into());
                animal
            },
            move |animal| {
                std::hint::black_box(visitor0sf::run_not_rw(animal, sbd))
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("visitor0sf_not_rw_alwayscat", |b| {
        b.iter_batched(
            // || visitor0sf::get_animal(rand::rng().random_range(..2usize)),
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf::get_animal(x.into());
                animal
            },
            move |animal| {
                std::hint::black_box(visitor0sf::run_not_rw(animal, sbd))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_full_not_rw", |b| {
        b.iter_batched(
            || {
                let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf::get_animal(x);
                animal
            },
            move |animal| {
                std::hint::black_box(visitor0sf::run_full_not_rw(animal, sbd))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_src_rw_into_raw", |b| {
        b.iter_batched(
            || {
                let animal = visitor0sf::get_animal(rand::rng().random_range(..2usize));
                // let the_cat = visitor0sf::get_cat();
                let cat = visitor0sf::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf::run_src_rw_into_raw(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("visitor0sf_src_rw_into_raw_alwaysdog", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/zero.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf::get_animal(x.into());
                // let animal = visitor0sf::get_animal(rand::rng().random_range(..2usize));
                // let the_cat = visitor0sf::get_cat();
                let cat = visitor0sf::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf::run_src_rw_into_raw(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("visitor0sf_src_rw_into_raw_alwayscat", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf::get_animal(x.into());
                // let animal = visitor0sf::get_animal(rand::rng().random_range(..2usize));
                // let the_cat = visitor0sf::get_cat();
                let cat = visitor0sf::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf::run_src_rw_into_raw(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal = visitor0sf::get_animal(rand::rng().random_range(..2usize));
                // let cat = visitor0sf::get_cat();
                let cat = visitor0sf::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf::run_src_rw_transmutes(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("visitor0sf_src_rw_transmutes_alwaysdog", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf::get_animal(x.into());
                // let animal = visitor0sf::get_animal(rand::rng().random_range(..2usize));
                // let cat = visitor0sf::get_cat();
                let cat = visitor0sf::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf::run_src_rw_transmutes(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("visitor0sf_src_rw_transmutes_alwayscat", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf::get_animal(x.into());
                // let animal = visitor0sf::get_animal(rand::rng().random_range(..2usize));
                // let cat = visitor0sf::get_cat();
                let cat = visitor0sf::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf::run_src_rw_transmutes(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_2_best", |b| b.iter(|| std::hint::black_box(visitor0sf_2::run_best(dog, sbd0_2))));
    group.bench_function("visitor0sf_2_best_cat", |b| b.iter(|| std::hint::black_box(visitor0sf_2::run_best_cat(cat0_2, sbd0_2))));
    group.bench_function("visitor0sf_2_not_rw", |b| {
        b.iter_batched(
            // || visitor0sf_2::get_animal(rand::rng().random_range(..2usize)),
            || {
                let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf_2::get_animal(x);
                animal
            },
            move |animal| {
                std::hint::black_box(visitor0sf_2::run_not_rw(animal, sbd0_2))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_2_not_rw_alwaysdog", |b| {
        b.iter_batched(
            // || visitor0sf_2::get_animal(rand::rng().random_range(..2usize)),
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/zero.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf_2::get_animal(x.into());
                animal
            },
            move |animal| {
                std::hint::black_box(visitor0sf_2::run_not_rw(animal, sbd0_2))
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("visitor0sf_2_not_rw_alwayscat", |b| {
        b.iter_batched(
            // || visitor0sf_2::get_animal(rand::rng().random_range(..2usize)),
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf_2::get_animal(x.into());
                animal
            },
            move |animal| {
                std::hint::black_box(visitor0sf_2::run_not_rw(animal, sbd0_2))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_2_full_not_rw", |b| {
        b.iter_batched(
            || {
                let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf_2::get_animal(x);
                animal
            },
            move |animal| {
                std::hint::black_box(visitor0sf_2::run_full_not_rw(animal, sbd0_2))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_2_src_rw_into_raw", |b| {
        b.iter_batched(
            || {
                let animal = visitor0sf_2::get_animal(rand::rng().random_range(..2usize));
                // let cat = visitor0sf_2::get_cat();
                let cat = visitor0sf_2::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf_2::run_src_rw_into_raw(animal, sbd0_2, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_2_src_rw_into_raw_alwaysdog", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/zero.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf_2::get_animal(x.into());
                // let animal = visitor0sf_2::get_animal(rand::rng().random_range(..2usize));
                // let the_cat = visitor0sf_2::get_cat();
                let cat = visitor0sf_2::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf_2::run_src_rw_into_raw(animal, sbd0_2, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_2_src_rw_into_raw_alwayscat", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf_2::get_animal(x.into());
                // let animal = visitor0sf_2::get_animal(rand::rng().random_range(..2usize));
                // let the_cat = visitor0sf_2::get_cat();
                let cat = visitor0sf_2::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf_2::run_src_rw_into_raw(animal, sbd0_2, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_2_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal = visitor0sf_2::get_animal(rand::rng().random_range(..2usize));
                // let cat = visitor0sf_2::get_cat();
                let cat = visitor0sf_2::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf_2::run_src_rw_transmutes(animal, sbd0_2, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_2_src_rw_transmutes_alwaysdog", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf_2::get_animal(x.into());
                // let animal = visitor0sf_2::get_animal(rand::rng().random_range(..2usize));
                // let cat = visitor0sf_2::get_cat();
                let cat = visitor0sf_2::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf_2::run_src_rw_transmutes(animal, sbd0_2, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_2_src_rw_transmutes_alwayscat", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf_2::get_animal(x.into());
                // let animal = visitor0sf_2::get_animal(rand::rng().random_range(..2usize));
                // let cat = visitor0sf_2::get_cat();
                let cat = visitor0sf_2::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf_2::run_src_rw_transmutes(animal, sbd0_2, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn read_exact_bytes(path: &str, n: usize) -> Vec<usize> {
    let mut f = File::open(path).unwrap();
    let mut buf = vec![0u8; n];
    f.read_exact(&mut buf).unwrap();
    buf.into_iter().map(|b| (b & 1) as usize).collect()
}

fn bench_visitor_ref(c: &mut Criterion) {
    let mut group = c.benchmark_group("double_visitor0sf_ref");
    group.bench_function("frandom", |b| {
        const INNER: usize = 500_000;
        // Construct concrete values once
        let cat = double_visitor0sf_ref::Cat {};
        let dog = double_visitor0sf_ref::Dog {};

        // Construct visitors once (adjust these constructors to your real types)
        let v1 = double_visitor0sf_ref::Visitor1 {};
        let v2 = double_visitor0sf_ref::Visitor2 {};

        // Store as trait object references (fat pointers)
        let animals: [&dyn double_visitor0sf_ref::Animal; 2] = [&cat, &dog];
        let visitors: [&dyn double_visitor0sf_ref::AnimalVisitor; 2] = [&v1, &v2];
        b.iter_batched(
            || {
                let a = read_exact_bytes("/home/akalaba/verifopt/rewrites/benches/random_a.bin", INNER);
                let v = read_exact_bytes("/home/akalaba/verifopt/rewrites/benches/random_v.bin", INNER);
                (a, v)
            },
            |(a_idx, v_idx)| {
                for i in 0..INNER {
                    // Make indices opaque so compiler can't constant-fold
                    let ai = std::hint::black_box(a_idx[i] & 1);
                    let vi = std::hint::black_box(v_idx[i] & 1);

                    let animal = animals[ai];
                    let visitor = visitors[vi];

                    // Call through dyn trait (this should remain an indirect call)
                    std::hint::black_box(double_visitor0sf_ref::run_full_not_rw(animal, visitor));
                }
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("fknown", |b| {
        const INNER: usize = 500_000;
        // Construct concrete values once
        let cat = double_visitor0sf_ref::Cat {};
        let dog = double_visitor0sf_ref::Dog {};

        // Construct visitors once (adjust these constructors to your real types)
        let v1 = double_visitor0sf_ref::Visitor1 {};
        let v2 = double_visitor0sf_ref::Visitor2 {};

        // Store as trait object references (fat pointers)
        let animals: [&dyn double_visitor0sf_ref::Animal; 2] = [&cat, &dog];
        let visitors: [&dyn double_visitor0sf_ref::AnimalVisitor; 2] = [&v1, &v2];
        b.iter_batched(
            || {
                let a = read_exact_bytes("/home/akalaba/verifopt/rewrites/benches/known_a.bin", INNER);
                let v = read_exact_bytes("/home/akalaba/verifopt/rewrites/benches/known_v.bin", INNER);
                (a, v)
            },
            |(a_idx, v_idx)| {
                for i in 0..INNER {
                    // Make indices opaque so compiler can't constant-fold
                    let ai = std::hint::black_box(a_idx[i] & 1);
                    let vi = std::hint::black_box(v_idx[i] & 1);

                    let animal = animals[ai];
                    let visitor = visitors[vi];

                    // Call through dyn trait (this should remain an indirect call)
                    std::hint::black_box(double_visitor0sf_ref::run_full_not_rw(animal, visitor));
                }
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}


fn bench_visitor_alternating(c: &mut Criterion) {
    let cat: &visitor0sf_2::Cat = &visitor0sf_2::Cat {};
    let dog: &visitor0sf_2::Dog = &visitor0sf_2::Dog {};
    let sbd: &visitor0sf_2::Visitor1 = &visitor0sf_2::Visitor1 { };
    let v1: &double_visitor0sf::Visitor1 = &double_visitor0sf::Visitor1 {};
    let ddog: &double_visitor0sf::Dog = &double_visitor0sf::Dog {};
    let mut group = c.benchmark_group("visitor_alternating");

    group.bench_function("visitor0sf_2_not_rw_alternating", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/alternating.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let mut x = b[0] & 1;
                x = (x + 1) % 2; // alternate
                // write this back to file
                let mut f = File::create("/home/akalaba/verifopt/rewrites/benches/alternating.bin").unwrap();
                f.write_all(&[x]).unwrap();
                let animal = visitor0sf_2::get_animal(x.into());
                animal
            },
            move |animal| {
                std::hint::black_box(visitor0sf_2::run_not_rw(animal, sbd))
            },
            BatchSize::SmallInput,
        )
    });
    
    group.bench_function("double_visitor0sf_run_animal_dispatch_alternating", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/alternating2.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let mut x = b[0] & 1;
                x = (x + 1) % 2; // alternate
                // write this back to file
                let mut f = File::create("/home/akalaba/verifopt/rewrites/benches/alternating2.bin").unwrap();
                f.write_all(&[x]).unwrap();
                let animal = double_visitor0sf::get_animal(x.into());
                let visitor = Box::new(double_visitor0sf::Visitor1 {});
                (animal, visitor)
            },
            move |(animal, visitor)| {
                std::hint::black_box(double_visitor0sf::run_animal_dispatch(animal, visitor))
            },
            BatchSize::SmallInput,
        )
    });
    
    group.bench_function("double_visitor0sf_run_visitor_dispatch_alternating", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/alternating3.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let mut x = b[0] & 1;
                x = (x + 1) % 2; // alternate
                // write this back to file
                let mut f = File::create("/home/akalaba/verifopt/rewrites/benches/alternating3.bin").unwrap();
                f.write_all(&[x]).unwrap();
                let animal = &double_visitor0sf::Dog {};
                let visitor = double_visitor0sf::get_visitor(x.into());
                (animal, visitor)
            },
            move |(animal, visitor)| {
                std::hint::black_box(double_visitor0sf::run_visitor_dispatch(animal, visitor))
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

/*
fn bench_visitor0sf_import(c: &mut Criterion) {
    let cat: &visitor_decl::Cat = &visitor_decl::Cat {};
    let sbd: &visitor_use::SpeakBetterDogs = &visitor_use::SpeakBetterDogs {};
    let mut group = c.benchmark_group("visitor0sf_import");

    group.bench_function("visitor0sf_import_best", |b| b.iter(|| visitor0sf_import::run_best(cat, sbd)));
    group.bench_function("visitor0sf_import_not_rw", |b| {
        b.iter_batched(
            || visitor_decl::get_animal(rand::rng().random_range(..2usize)),
            move |animal| visitor0sf_import::run_not_rw(animal, sbd),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_import_src_rw_into_raw", |b| {
        b.iter_batched(
            || {
                let animal = visitor_decl::get_animal(rand::rng().random_range(..2usize));
                let cat = visitor_decl::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                visitor0sf_import::run_src_rw_into_raw(animal, sbd, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_import_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal = visitor_decl::get_animal(rand::rng().random_range(..2usize));
                let cat = visitor_decl::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                visitor0sf_import::run_src_rw_transmutes(animal, sbd, animal_vtable, cat_vtable)
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}
*/

fn bench_visitor2sf(c: &mut Criterion) {
    let cat: &visitor2sf::Cat = &visitor2sf::Cat {
        age: 1,
        num_siblings: 13,
    };
    let sbd: &visitor2sf::SpeakBetterDogs = &visitor2sf::SpeakBetterDogs { };
    let mut group = c.benchmark_group("visitor2sf");

    group.bench_function("visitor2sf_best", |b| b.iter(|| std::hint::black_box(visitor2sf::run_best(cat, sbd))));
    group.bench_function("visitor2sf_best_cat", |b| b.iter(|| std::hint::black_box(visitor2sf::run_best_cat(cat, sbd))));
    group.bench_function("visitor2sf_not_rw", |b| {
        b.iter_batched(
            || {
                let x = rand::rng().random_range(..2usize);
                let animal = visitor2sf::get_animal(x);
                animal
            },
            move |animal| {
                std::hint::black_box(visitor2sf::run_not_rw(animal, sbd))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor2sf_not_rw_alwaysdog", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/zero.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor2sf::get_animal(x.into());
                animal
            },
            move |animal| {
                std::hint::black_box(visitor2sf::run_not_rw(animal, sbd))
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("visitor2sf_not_rw_alwayscat", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor2sf::get_animal(x.into());
                animal
            },
            move |animal| {
                std::hint::black_box(visitor2sf::run_not_rw(animal, sbd))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor2sf_full_not_rw", |b| {
        b.iter_batched(
            || {
                let x = rand::rng().random_range(..2usize);
                let animal = visitor2sf::get_animal(x);
                animal
            },
            move |animal| {
                std::hint::black_box(visitor2sf::run_full_not_rw(animal, sbd))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor2sf_src_rw_into_raw", |b| {
        b.iter_batched(
            || {
                let animal = visitor2sf::get_animal(rand::rng().random_range(..2usize));
                // let cat = visitor2sf::get_cat();
                let cat = visitor2sf::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor2sf::run_src_rw_into_raw(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor2sf_src_rw_into_raw_alwaysdog", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/zero.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor2sf::get_animal(x.into());
                // let animal = visitor2sf::get_animal(rand::rng().random_range(..2usize));
                // let the_cat = visitor2sf::get_cat();
                let cat = visitor2sf::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor2sf::run_src_rw_into_raw(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor2sf_src_rw_into_raw_alwayscat", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor2sf::get_animal(x.into());
                // let animal = visitor2sf::get_animal(rand::rng().random_range(..2usize));
                // let the_cat = visitor2sf::get_cat();
                let cat = visitor2sf::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor2sf::run_src_rw_into_raw(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor2sf_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal = visitor2sf::get_animal(rand::rng().random_range(..2usize));
                // let cat = visitor2sf::get_cat();
                let cat = visitor2sf::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor2sf::run_src_rw_transmutes(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor2sf_src_rw_transmutes_alwaysdog", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor2sf::get_animal(x.into());
                // let animal = visitor2sf::get_animal(rand::rng().random_range(..2usize));
                // let cat = visitor2sf::get_cat();
                let cat = visitor2sf::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor2sf::run_src_rw_transmutes(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor2sf_src_rw_transmutes_alwayscat", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor2sf::get_animal(x.into());
                // let animal = visitor2sf::get_animal(rand::rng().random_range(..2usize));
                // let cat = visitor2sf::get_cat();
                let cat = visitor2sf::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor2sf::run_src_rw_transmutes(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn bench_visitor0sf_2(c: &mut Criterion) {
    let cat: &visitor0sf_2::Cat = &visitor0sf_2::Cat {};
    let dog: &visitor0sf_2::Dog = &visitor0sf_2::Dog {};
    let sbd: &visitor0sf_2::Visitor1 = &visitor0sf_2::Visitor1 { };
    let v1: &double_visitor0sf::Visitor1 = &double_visitor0sf::Visitor1 {};
    let ddog: &double_visitor0sf::Dog = &double_visitor0sf::Dog {};
    let mut group = c.benchmark_group("visitor0sf_2");

    group.bench_function("visitor0sf_2_best", |b| b.iter(|| std::hint::black_box(visitor0sf_2::run_best(dog, sbd))));
    group.bench_function("visitor0sf_2_best_cat", |b| b.iter(|| std::hint::black_box(visitor0sf_2::run_best_cat(cat, sbd))));
    group.bench_function("visitor0sf_2_not_rw", |b| {
        b.iter_batched(
            || {
                let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf_2::get_animal(x);
                animal
            },
            move |animal| {
                std::hint::black_box(visitor0sf_2::run_not_rw(animal, sbd))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_2_not_rw_alwaysdog", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/zero.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf_2::get_animal(x.into());
                animal
            },
            move |animal| {
                std::hint::black_box(visitor0sf_2::run_not_rw(animal, sbd))
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("visitor0sf_2_not_rw_alwayscat", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf_2::get_animal(x.into());
                animal
            },
            move |animal| {
                std::hint::black_box(visitor0sf_2::run_not_rw(animal, sbd))
            },
            BatchSize::SmallInput,
        )
    });
    
    group.bench_function("visitor0sf_2_full_not_rw", |b| {
        b.iter_batched(
            || {
                let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf_2::get_animal(x);
                animal
            },
            move |animal| {
                std::hint::black_box(visitor0sf_2::run_full_not_rw(animal, sbd))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_2_src_rw_into_raw", |b| {
        b.iter_batched(
            || {
                let animal = visitor0sf_2::get_animal(rand::rng().random_range(..2usize));
                // let cat = visitor0sf_2::get_cat();
                let cat = visitor0sf_2::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                
                // print out the vtable addresses
                // let other_cat = visitor0sf_2::get_cat();
                // let other_vtable = core::ptr::metadata(&*other_cat);
                // let animal_vptr: *const () = unsafe { std::mem::transmute(animal_vtable) };
                // let cat_vptr: *const () = unsafe { std::mem::transmute(cat_vtable) };
                // let other_vptr: *const () = unsafe { std::mem::transmute(other_vtable) };
                // let less_mem_cat_vtable = core::ptr::metadata(&*less_mem_cat);
                // let less_mem_cat_vptr: *const () = unsafe { std::mem::transmute(less_mem_cat_vtable) };
                // println!("animal vtable: {:p}", animal_vptr);
                // println!("cat vtable: {:p}", cat_vptr);
                // println!("other cat vtable: {:p}", other_vptr);
                // println!("less mem cat vtable: {:p}", less_mem_cat_vptr);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf_2::run_src_rw_into_raw(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_2_src_rw_into_raw_alwaysdog", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/zero.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf_2::get_animal(x.into());
                // let animal = visitor0sf_2::get_animal(rand::rng().random_range(..2usize));
                // let the_cat = visitor0sf_2::get_cat();
                let cat = visitor0sf_2::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf_2::run_src_rw_into_raw(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_2_src_rw_into_raw_alwayscat", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf_2::get_animal(x.into());
                // let animal = visitor0sf_2::get_animal(rand::rng().random_range(..2usize));
                // let the_cat = visitor0sf_2::get_cat();
                let cat = visitor0sf_2::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf_2::run_src_rw_into_raw(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_2_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal = visitor0sf_2::get_animal(rand::rng().random_range(..2usize));
                // let cat = visitor0sf_2::get_cat();
                let cat = visitor0sf_2::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf_2::run_src_rw_transmutes(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_2_src_rw_transmutes_alwaysdog", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf_2::get_animal(x.into());
                // let animal = visitor0sf_2::get_animal(rand::rng().random_range(..2usize));
                // let cat = visitor0sf_2::get_cat();
                let cat = visitor0sf_2::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf_2::run_src_rw_transmutes(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("visitor0sf_2_src_rw_transmutes_alwayscat", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                // let x = rand::rng().random_range(..2usize);
                let animal = visitor0sf_2::get_animal(x.into());
                // let animal = visitor0sf_2::get_animal(rand::rng().random_range(..2usize));
                // let cat = visitor0sf_2::get_cat();
                let cat = visitor0sf_2::get_animal(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(visitor0sf_2::run_src_rw_transmutes(animal, sbd, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("double_visitor0sf_run_no_dispatch", |b| b.iter(|| std::hint::black_box(double_visitor0sf::run_no_dispatch(ddog, v1))));
    group.bench_function("double_visitor0sf_run_animal_dispatch", |b| {
        b.iter_batched(
            || {
                let x = rand::rng().random_range(..2usize);
                let animal = double_visitor0sf::get_animal(x);
                let visitor = Box::new(double_visitor0sf::Visitor1 {});
                (animal, visitor)
            },
            move |(animal, visitor)| {
                std::hint::black_box(double_visitor0sf::run_animal_dispatch(animal, visitor))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("double_visitor0sf_run_animal_dispatch_alwayscat", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                let animal = double_visitor0sf::get_animal(x.into());
                let visitor = Box::new(double_visitor0sf::Visitor1 {});
                (animal, visitor)
            },
            move |(animal, visitor)| {
                std::hint::black_box(double_visitor0sf::run_animal_dispatch(animal, visitor))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("double_visitor0sf_run_visitor_dispatch", |b| {
        b.iter_batched(
            || {
                let x = rand::rng().random_range(..2usize);
                let animal = &double_visitor0sf::Dog {};
                let visitor = double_visitor0sf::get_visitor(rand::rng().random_range(..2usize));
                (animal, visitor)
            },
            move |(animal, visitor)| {
                std::hint::black_box(double_visitor0sf::run_visitor_dispatch(animal, visitor))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("double_visitor0sf_run_visitor_dispatch_always_v1", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                let animal = &double_visitor0sf::Dog {};
                let visitor = double_visitor0sf::get_visitor(x.into());
                (animal, visitor)
            },
            move |(animal, visitor)| {
                std::hint::black_box(double_visitor0sf::run_visitor_dispatch(animal, visitor))
            },
            BatchSize::SmallInput,
        )
    });
    
    group.bench_function("double_visitor0sf_full_not_rw", |b| {
        b.iter_batched(
            || {
                let x = rand::rng().random_range(..2usize);
                let animal = double_visitor0sf::get_animal(x);
                let visitor = double_visitor0sf::get_visitor(rand::rng().random_range(..2usize));
                (animal, visitor)
            },
            move |(animal, visitor)| {
                std::hint::black_box(double_visitor0sf::run_full_not_rw(animal, visitor))
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("double_visitor0sf_full_not_rw_alwayscat", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                let animal = double_visitor0sf::get_animal(x.into());
                let visitor = double_visitor0sf::get_visitor(rand::rng().random_range(..2usize));
                (animal, visitor)
            },
            move |(animal, visitor)| {
                std::hint::black_box(double_visitor0sf::run_full_not_rw(animal, visitor))
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("double_visitor0sf_full_not_rw_alwaysv1", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                let animal = double_visitor0sf::get_animal(rand::rng().random_range(..2usize));
                let visitor = double_visitor0sf::get_visitor(x.into());
                (animal, visitor)
            },
            move |(animal, visitor)| {
                std::hint::black_box(double_visitor0sf::run_full_not_rw(animal, visitor))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("double_visitor0sf_full_not_rw_alwayscat_alwaysv1", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("/home/akalaba/verifopt/rewrites/benches/one.bin").unwrap();
                let mut b = [0u8; 1];
                f.read_exact(&mut b).unwrap();
                let x = b[0] & 1;
                let y = x + 1;
                let animal = double_visitor0sf::get_animal(y.into());
                let visitor = double_visitor0sf::get_visitor(x.into());
                (animal, visitor)
            },
            move |(animal, visitor)| {
                std::hint::black_box(double_visitor0sf::run_full_not_rw(animal, visitor))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("double_visitor0sf_src_rw_into_raw", |b| {
        b.iter_batched(
            || {
                let animal = double_visitor0sf::get_animal(rand::rng().random_range(..2usize));
                // let cat = double_visitor0sf::get_cat();
                let visitor = double_visitor0sf::get_visitor(rand::rng().random_range(..2usize));
                let cat = double_visitor0sf::get_animal(0);
                let visitor1 = double_visitor0sf::get_visitor(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                let visitor_vtable = core::ptr::metadata(&*visitor);
                let visitor1_vtable = core::ptr::metadata(&*visitor1);
                (animal, animal_vtable, cat_vtable, visitor_vtable, visitor1_vtable, visitor)
            },
            move |(animal, animal_vtable, cat_vtable, visitor_vtable, visitor1_vtable, visitor)| {
                std::hint::black_box(double_visitor0sf::run_src_rw_into_raw(animal, visitor, animal_vtable, visitor_vtable, cat_vtable, visitor1_vtable))
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("double_visitor0sf_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal = double_visitor0sf::get_animal(rand::rng().random_range(..2usize));
                // let cat = double_visitor0sf::get_cat();
                let visitor = double_visitor0sf::get_visitor(rand::rng().random_range(..2usize));
                let cat = double_visitor0sf::get_animal(0);
                let visitor1 = double_visitor0sf::get_visitor(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                let visitor_vtable = core::ptr::metadata(&*visitor);
                let visitor1_vtable = core::ptr::metadata(&*visitor1);
                (animal, animal_vtable, cat_vtable, visitor_vtable, visitor1_vtable, visitor)
            },
            move |(animal, animal_vtable, cat_vtable, visitor_vtable, visitor1_vtable, visitor)| {
                std::hint::black_box(double_visitor0sf::run_src_rw_transmutes(animal, visitor, animal_vtable, visitor_vtable, cat_vtable, visitor1_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn bench_double_visitor0sf(c: &mut Criterion) {
    let v1: &double_visitor0sf::Visitor1 = &double_visitor0sf::Visitor1 {};
    let dog: &double_visitor0sf::Dog = &double_visitor0sf::Dog {};
    let mut group = c.benchmark_group("double_visitor0sf");
    group.bench_function("double_visitor0sf_run_no_dispatch", |b| b.iter(|| std::hint::black_box(double_visitor0sf::run_no_dispatch(dog, v1))));
    group.bench_function("double_visitor0sf_run_animal_dispatch", |b| {
        b.iter_batched(
            || {
                let x = rand::rng().random_range(..2usize);
                let animal = double_visitor0sf::get_animal(x);
                let visitor = Box::new(double_visitor0sf::Visitor1 {});
                (animal, visitor)
            },
            move |(animal, visitor)| {
                std::hint::black_box(double_visitor0sf::run_animal_dispatch(animal, visitor))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("double_visitor0sf_run_visitor_dispatch", |b| {
        b.iter_batched(
            || {
                let x = rand::rng().random_range(..2usize);
                let animal = &double_visitor0sf::Dog {};
                let visitor = double_visitor0sf::get_visitor(rand::rng().random_range(..2usize));
                (animal, visitor)
            },
            move |(animal, visitor)| {
                std::hint::black_box(double_visitor0sf::run_visitor_dispatch(animal, visitor))
            },
            BatchSize::SmallInput,
        )
    });
    
    group.bench_function("double_visitor0sf_full_not_rw", |b| {
        b.iter_batched(
            // || double_visitor0sf::get_animal(rand::rng().random_range(..2usize)),
            || {
                let x = rand::rng().random_range(..2usize);
                let animal = double_visitor0sf::get_animal(x);
                let visitor = double_visitor0sf::get_visitor(rand::rng().random_range(..2usize));
                (animal, visitor)
            },
            move |(animal, visitor)| {
                std::hint::black_box(double_visitor0sf::run_full_not_rw(animal, visitor))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("double_visitor0sf_src_rw_into_raw", |b| {
        b.iter_batched(
            || {
                let animal = double_visitor0sf::get_animal(rand::rng().random_range(..2usize));
                // let cat = double_visitor0sf::get_cat();
                let visitor = double_visitor0sf::get_visitor(rand::rng().random_range(..2usize));
                let cat = double_visitor0sf::get_animal(0);
                let visitor1 = double_visitor0sf::get_visitor(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                let visitor_vtable = core::ptr::metadata(&*visitor);
                let visitor1_vtable = core::ptr::metadata(&*visitor1);
                (animal, animal_vtable, cat_vtable, visitor_vtable, visitor1_vtable, visitor)
            },
            move |(animal, animal_vtable, cat_vtable, visitor_vtable, visitor1_vtable, visitor)| {
                std::hint::black_box(double_visitor0sf::run_src_rw_into_raw(animal, visitor, animal_vtable, visitor_vtable, cat_vtable, visitor1_vtable))
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("double_visitor0sf_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal = double_visitor0sf::get_animal(rand::rng().random_range(..2usize));
                // let cat = double_visitor0sf::get_cat();
                let visitor = double_visitor0sf::get_visitor(rand::rng().random_range(..2usize));
                let cat = double_visitor0sf::get_animal(0);
                let visitor1 = double_visitor0sf::get_visitor(0);
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                let visitor_vtable = core::ptr::metadata(&*visitor);
                let visitor1_vtable = core::ptr::metadata(&*visitor1);
                (animal, animal_vtable, cat_vtable, visitor_vtable, visitor1_vtable, visitor)
            },
            move |(animal, animal_vtable, cat_vtable, visitor_vtable, visitor1_vtable, visitor)| {
                std::hint::black_box(double_visitor0sf::run_src_rw_transmutes(animal, visitor, animal_vtable, visitor_vtable, cat_vtable, visitor1_vtable))
            },
            BatchSize::SmallInput,
        )
    });

    

    group.finish();
}

fn bench_prime2sf(c: &mut Criterion) {
    let cat: &prime2sf::Cat = &prime2sf::Cat {
        tmp1: 24,
        tmp2: 36,
    };
    let mut group = c.benchmark_group("prime2sf");

    group.bench_function("prime2sf_best", |b| b.iter(|| prime2sf::run_best(cat)));
    group.bench_function("prime2sf_not_rw", |b| {
        b.iter_batched(
            || prime2sf::get_animal(rand::rng().random_range(..2usize)),
            move |animal| std::hint::black_box(prime2sf::run_not_rw(animal)),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("prime2sf_naive_cha", |b| {
        b.iter_batched(
            || {
                let animal = prime2sf::get_animal(rand::rng().random_range(..2usize));
                let a = prime2sf::get_alligator();
                let b = prime2sf::get_bird();
                let c = prime2sf::get_cat();
                let d = prime2sf::get_dog();
                let e = prime2sf::get_elephant();
                let f = prime2sf::get_frog();
                let g = prime2sf::get_giraffe();
                let h = prime2sf::get_hippo();
                let i = prime2sf::get_iguana();
                let j = prime2sf::get_jaguar();
                let k = prime2sf::get_kangaroo();
                let l = prime2sf::get_lion();
                let animal_vtable = core::ptr::metadata(&*animal);
                let a_vtable = core::ptr::metadata(&*a);
                let b_vtable = core::ptr::metadata(&*b);
                let c_vtable = core::ptr::metadata(&*c);
                let d_vtable = core::ptr::metadata(&*d);
                let e_vtable = core::ptr::metadata(&*e);
                let f_vtable = core::ptr::metadata(&*f);
                let g_vtable = core::ptr::metadata(&*g);
                let h_vtable = core::ptr::metadata(&*h);
                let i_vtable = core::ptr::metadata(&*i);
                let j_vtable = core::ptr::metadata(&*j);
                let k_vtable = core::ptr::metadata(&*k);
                let l_vtable = core::ptr::metadata(&*l);
                (
                    animal, 
                    animal_vtable, 
                    a_vtable,
                    b_vtable,
                    c_vtable,
                    d_vtable,
                    e_vtable,
                    f_vtable,
                    g_vtable,
                    h_vtable,
                    i_vtable,
                    j_vtable,
                    k_vtable,
                    l_vtable
                )
            },
            move |(
                    animal, 
                    animal_vtable, 
                    a_vtable,
                    b_vtable,
                    c_vtable,
                    d_vtable,
                    e_vtable,
                    f_vtable,
                    g_vtable,
                    h_vtable,
                    i_vtable,
                    j_vtable,
                    k_vtable,
                    l_vtable
            )| {
                std::hint::black_box(prime2sf::run_naive_cha(
                    animal, 
                    animal_vtable, 
                    a_vtable,
                    b_vtable,
                    c_vtable,
                    d_vtable,
                    e_vtable,
                    f_vtable,
                    g_vtable,
                    h_vtable,
                    i_vtable,
                    j_vtable,
                    k_vtable,
                    l_vtable
                ))
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("prime2sf_src_rw_into_raw", |b| {
        b.iter_batched(
            || {
                let animal = prime2sf::get_animal(rand::rng().random_range(..2usize));
                let cat = prime2sf::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(prime2sf::run_src_rw_into_raw(animal, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });
    /*group.bench_function("prime2sf_src_rw_transmutes", |b| {
        b.iter_batched(
            || {
                let animal = prime2sf::get_animal(rand::rng().random_range(..2usize));
                let cat = prime2sf::get_cat();
                let animal_vtable = core::ptr::metadata(&*animal);
                let cat_vtable = core::ptr::metadata(&*cat);
                (animal, animal_vtable, cat_vtable)
            },
            move |(animal, animal_vtable, cat_vtable)| {
                std::hint::black_box(prime2sf::run_src_rw_transmutes(animal, animal_vtable, cat_vtable))
            },
            BatchSize::SmallInput,
        )
    });*/
    group.finish();
}

// TODO
// more than 2 trait implementations
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

/*
criterion_group! {
    name = visitor0sf_import_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_visitor0sf_import
}
*/

criterion_group! {
    name = visitor2sf_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_visitor2sf
}

criterion_group! {
    name = prime2sf_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_prime2sf
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
        bench_prime2sf,
}

criterion_group! {
    name = og_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets =
        bench_og0sf,
        bench_og2sf,
        bench_og5sf,
}

criterion_group! {
    name = vec_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets =
        bench_vec0sf,
        bench_vec2sf,
}

criterion_group! {
    name = visitor_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets =
        //bench_visitor0sf_2,
        //bench_visitor_alternating,
        bench_visitor_ref,
        // bench_double_visitor0sf,
        // bench_visitor0sf,
        // bench_visitor2sf,
}

criterion_main!(visitor_benches);
