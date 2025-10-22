use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::hint::black_box;
use std::time::Duration;

use rand::Rng;
use rewrites::{simple, simple_mir_rw, struct_fields, vec_simple};
use visitor_decl;
use visitor_use;

fn get_input_num_vec() -> Vec<usize> {
    let mut nums_vec: Vec<usize> = vec![];
    for _ in 0..1000 {
        nums_vec.push(rand::rng().random_range(..2));
    }
    nums_vec
}

fn bench_simple(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;
    let cat: &simple::Cat = &simple::Cat {};

    let mut group = c.benchmark_group("simple");

    group.bench_function(
        "simple_best",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_best(num, cat)
        })
    );
    group.bench_function(
        "simple_best_normalized",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_best_normalized(num, cat)
        })
    );
    group.bench_function(
        "simple_not_rw",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_not_rw(num)
        })
    );
    group.bench_function(
        "simple_not_rw_normalized",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_not_rw_normalized(num)
        })
    );
    group.bench_function(
        "simple_src_rw_into_raw",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_src_rw_into_raw(num)
        })
    );
    group.bench_function(
        "simple_mir_rw_transmutes",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple_mir_rw::run(num)
        })
    );
    group.bench_function(
        "simple_src_rw_transmutes",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_src_rw_transmutes(num)
        })
    );
    group.bench_function(
        "simple_best_normalized_fallback",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_best_normalized_fallback(num, cat)
        })
    );
    group.bench_function(
        "simple_not_rw_fallback",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_not_rw_fallback(num)
        })
    );
    group.bench_function(
        "simple_not_rw_fallback_normalized",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_not_rw_fallback_normalized(num)
        })
    );
    group.bench_function(
        "simple_src_rw_into_raw_fallback",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_src_rw_into_raw_fallback(num)
        })
    );
    group.bench_function(
        "simple_src_rw_transmutes_fallback",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_src_rw_transmutes_fallback(num)
        })
    );
    group.finish();
}

fn bench_struct_fields(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;
    let cat: &struct_fields::Cat = &struct_fields::Cat {
        name: "sally",
        age: 9,
        fav_toy: "curtains",
    };

    let mut group = c.benchmark_group("struct_fields");

    group.bench_function(
        "struct_fields_best",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_best(num, cat)
        })
    );
    group.bench_function(
        "struct_fields_best_normalized",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_best_normalized(num, cat)
        })
    );
    group.bench_function(
        "struct_fields_not_rw",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_not_rw(num)
        })
    );
    group.bench_function(
        "struct_fields_not_rw_normalized",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_not_rw_normalized(num)
        })
    );
    group.bench_function(
        "struct_fields_src_rw_into_raw",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_src_rw_into_raw(num)
        })
    );
    group.bench_function(
        "struct_fields_src_rw_transmutes",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_src_rw_transmutes(num)
        })
    );
    group.bench_function(
        "struct_fields_best_normalized_fallback",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_best_normalized_fallback(num, cat)
        })
    );
    group.bench_function(
        "struct_fields_not_rw_fallback",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_not_rw_fallback(num)
        })
    );
    group.bench_function(
        "struct_fields_not_rw_fallback_normalized",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_not_rw_fallback_normalized(num)
        })
    );
    group.bench_function(
        "struct_fields_src_rw_into_raw_fallback",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_src_rw_into_raw_fallback(num)
        })
    );
    group.bench_function(
        "struct_fields_src_rw_transmutes_fallback",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_src_rw_transmutes_fallback(num)
        })
    );
    group.finish();
}

fn bench_vec_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_simple");

    for n_elems in [1000, 4000, 8000, 12000].iter() {
	    let vec_hc = vec_simple::mk_vec();
        for _ in 0..*n_elems {
            vec_hc.lock().unwrap().insert(0, Box::new(vec_simple::Cat));
        }
        let cat: &vec_simple::Cat = &vec_simple::Cat {};

	    let vec = vec_simple::mk_vec();
        for _ in 0..*n_elems {
            if rand::rng().random_range(..2usize) == 0 {
                vec.lock().unwrap().insert(0, Box::new(vec_simple::Cat));
	    	} else {
                vec.lock().unwrap().insert(0, Box::new(vec_simple::Dog));
	    	}
	    }

	    group.bench_function(
	    	BenchmarkId::new("vec_simple_best", n_elems),
	    	|b| b.iter(|| {
	    		vec_simple::run_best(&vec_hc.lock().unwrap(), cat)
	    	})
        );
	    group.bench_function(
	    	BenchmarkId::new("vec_simple_best_normalized", n_elems),
	    	|b| b.iter(|| {
	    		vec_simple::run_best_normalized(&vec_hc.lock().unwrap(), cat)
	    	})
        );
	    group.bench_function(
	    	BenchmarkId::new("vec_simple_not_rw", n_elems),
	    	|b| b.iter(|| {
	    		vec_simple::run_not_rw(&vec.lock().unwrap())
	    	})
	    );
	    group.bench_function(
	    	BenchmarkId::new("vec_simple_not_rw_normalized", n_elems),
	    	|b| b.iter(|| {
	    		vec_simple::run_not_rw_normalized(&vec.lock().unwrap())
	    	})
	    );
	    group.bench_function(
	    	BenchmarkId::new("vec_simple_src_rw", n_elems),
	    	|b| b.iter(|| {
	    		vec_simple::run_src_rw(&vec.lock().unwrap())
	    	})
	    );
	    group.bench_function(
	    	BenchmarkId::new("vec_simple_best_normalized_fallback", n_elems),
	    	|b| b.iter(|| {
	    		vec_simple::run_best_normalized_fallback(&vec_hc.lock().unwrap(), cat)
	    	})
        );
	    group.bench_function(
	    	BenchmarkId::new("vec_simple_not_rw_fallback", n_elems),
	    	|b| b.iter(|| {
	    		vec_simple::run_not_rw_fallback(&vec.lock().unwrap())
	    	})
	    );
	    group.bench_function(
	    	BenchmarkId::new("vec_simple_not_rw_fallback_normalized", n_elems),
	    	|b| b.iter(|| {
	    		vec_simple::run_not_rw_fallback_normalized(&vec.lock().unwrap())
	    	})
	    );
	    group.bench_function(
	    	BenchmarkId::new("vec_simple_src_rw_fallback", n_elems),
	    	|b| b.iter(|| {
	    		vec_simple::run_src_rw_fallback(&vec.lock().unwrap())
	    	})
	    );
    }
    group.finish();
}


// TODO
// more than 2 trait implementations
// multiple trait methods
// different paths -> trait method call
// visitor

//fn bench_visitor_not_rw(c: &mut Criterion) {
//    //let nums_vec = get_input_num_vec();
//    //let nums: &[usize] = &nums_vec[..];
//    let animal_vec = get_visitor_input_animal_vec();
//    let animals: &[Box<dyn visitor_decl::Animal>] = &animal_vec[..];
//    let dc = &visitor_use::SpeakBetterDogs {};
//    let mut idx = 0;
//
//    c.bench_function(
//        "visitor_not_rw",
//        |b| b.iter(|| {
//            //let num = black_box(nums[idx % 1000]);
//            let animal = black_box(&*animals[idx % 1000]);
//            idx += 1;
//            visitor_use::run_not_rw(animal, dc)
//        })
//    );
//}

const SAMPLE_SIZE: usize = 200;
const WARMUP_TIME: u64 = 5;
const MEASUREMENT_TIME: u64 = 10;

criterion_group!{
    name = simple_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_simple
}

criterion_group!{
    name = struct_fields_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_struct_fields
}

criterion_group!{
    name = vec_simple_benches;
    config = Criterion::default()
        .sample_size(SAMPLE_SIZE)
        .warm_up_time(Duration::new(WARMUP_TIME, 0))
        .measurement_time(Duration::new(MEASUREMENT_TIME, 0));
    targets = bench_vec_simple
}

criterion_main!(simple_benches);

