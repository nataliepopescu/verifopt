use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use std::time::Duration;

use rand::Rng;
use rewrites::{simple, struct_fields, vec};
use visitor_decl;
use visitor_use;

fn get_input_num_vec() -> Vec<usize> {
    let mut nums_vec: Vec<usize> = vec![];
    for _ in 0..1000 {
        nums_vec.push(rand::rng().random_range(..2));
    }
    nums_vec
}

//fn get_struct_fields_input_animal_vec() -> Vec<Box<dyn struct_fields::Animal>> {
//    let mut animal_vec: Vec<Box<dyn struct_fields::Animal>> = vec![];
//    for _ in 0..1000 {
//        if rand::rng().random_range(..2usize) == 0 {
//            animal_vec.push(Box::new(struct_fields::Cat{}));
//        } else {
//            animal_vec.push(Box::new(struct_fields::Dog{}));
//        }
//    }
//    animal_vec
//}

//fn get_visitor_input_animal_vec() -> Vec<Box<dyn visitor_decl::Animal>> {
//    let mut animal_vec: Vec<Box<dyn visitor_decl::Animal>> = vec![];
//    for _ in 0..1000 {
//        if rand::rng().random_range(..2usize) == 0 {
//            animal_vec.push(Box::new(visitor_decl::Cat{}));
//        } else {
//            animal_vec.push(Box::new(visitor_decl::Dog{}));
//        }
//    }
//    animal_vec
//}

/* simple pattern */

fn bench_simple_best(c: &mut Criterion) {
    let cat: &simple::Cat = &simple::Cat {};

    c.bench_function(
        "simple_best",
        |b| b.iter(|| {
            simple::run_best(cat)
        })
    );
}

fn bench_simple_best_normalized(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;
    let cat: &simple::Cat = &simple::Cat {};

    c.bench_function(
        "simple_best_normalized",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_best_normalized(num, cat)
        })
    );
}

fn bench_simple_not_rw(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;

    c.bench_function(
        "simple_not_rw",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_not_rw(num)
        })
    );
}

fn bench_simple_src_rw(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;

    c.bench_function(
        "simple_src_rw",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_src_rw(num)
        })
    );
}

fn bench_simple_best_normalized_fallback(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;
    let cat: &simple::Cat = &simple::Cat {};

    c.bench_function(
        "simple_best_normalized_fallback",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_best_normalized_fallback(num, cat)
        })
    );
}

fn bench_simple_not_rw_fallback(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;

    c.bench_function(
        "simple_not_rw_fallback",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_not_rw_fallback(num)
        })
    );
}

fn bench_simple_src_rw_fallback(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;

    c.bench_function(
        "simple_src_rw_fallback",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_src_rw_fallback(num)
        })
    );
}

/*
fn bench_simple_mir_rw(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;

    c.bench_function(
        "simple_mir_rw",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            mir_rw::run(num)
        })
    );
}
*/

/* struct_fields pattern */

fn bench_struct_fields_best(c: &mut Criterion) {
    let cat: &struct_fields::Cat = &struct_fields::Cat {
        name: "sally",
        age: 9,
        fav_toy: "curtains",
    };

    c.bench_function(
        "struct_fields_best",
        |b| b.iter(|| {
            struct_fields::run_best(cat)
        })
    );
}

fn bench_struct_fields_best_normalized(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;
    let cat: &struct_fields::Cat = &struct_fields::Cat {
        name: "sally",
        age: 9,
        fav_toy: "curtains",
    };

    c.bench_function(
        "struct_fields_best_normalized",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_best_normalized(num, cat)
        })
    );
}

fn bench_struct_fields_not_rw(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;

    c.bench_function(
        "struct_fields_not_rw",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_not_rw(num)
        })
    );
}

fn bench_struct_fields_src_rw(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;

    c.bench_function(
        "struct_fields_src_rw",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_src_rw(num)
        })
    );
}

fn bench_struct_fields_best_normalized_fallback(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;
    let cat: &struct_fields::Cat = &struct_fields::Cat {
        name: "sally",
        age: 9,
        fav_toy: "curtains",
    };

    c.bench_function(
        "struct_fields_best_normalized_fallback",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_best_normalized_fallback(num, cat)
        })
    );
}

fn bench_struct_fields_not_rw_fallback(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;

    c.bench_function(
        "struct_fields_not_rw_fallback",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_not_rw_fallback(num)
        })
    );
}

fn bench_struct_fields_src_rw_fallback(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;

    c.bench_function(
        "struct_fields_src_rw_fallback",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            struct_fields::run_src_rw_fallback(num)
        })
    );
}

// TODO
// multiple trait methods
// different paths

/* vec pattern */

fn bench_vec_best(c: &mut Criterion) {
	let vec = vec::mk_vec();
    vec.lock().unwrap().insert(0, Box::new(vec::Cat));
    vec.lock().unwrap().insert(0, Box::new(vec::Cat));
    let cat: &vec::Cat = &vec::Cat {};

	c.bench_function(
		"vec_best",
		|b| b.iter(|| {
			vec::run_best(&vec.lock().unwrap(), cat)
		})
    );
}

fn bench_vec_best_normalized(c: &mut Criterion) {
	let vec = vec::mk_vec();
    vec.lock().unwrap().insert(0, Box::new(vec::Cat));
    vec.lock().unwrap().insert(0, Box::new(vec::Cat));
    let cat: &vec::Cat = &vec::Cat {};

	c.bench_function(
		"vec_best_normalized",
		|b| b.iter(|| {
			vec::run_best_normalized(&vec.lock().unwrap(), cat)
		})
    );
}

fn bench_vec_not_rw(c: &mut Criterion) {
	let vec = vec::mk_vec();
    for _ in 0..2 {
        if rand::rng().random_range(..2usize) == 0 {
            vec.lock().unwrap().insert(0, Box::new(vec::Cat));
		} else {
            vec.lock().unwrap().insert(0, Box::new(vec::Dog));
		}
	}

	c.bench_function(
		"vec_not_rw",
		|b| b.iter(|| {
			vec::run_not_rw(&vec.lock().unwrap())
		})
	);
}

fn bench_vec_src_rw(c: &mut Criterion) {
	let vec = vec::mk_vec();
    for _ in 0..2 {
        if rand::rng().random_range(..2usize) == 0 {
            vec.lock().unwrap().insert(0, Box::new(vec::Cat));
		} else {
            vec.lock().unwrap().insert(0, Box::new(vec::Dog));
		}
	}

	c.bench_function(
		"vec_src_rw",
		|b| b.iter(|| {
			vec::run_src_rw(&vec.lock().unwrap())
		})
	);
}

/* visitor pattern */

// TODO
//fn bench_visitor_best(c: &mut Criterion) {}

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

// TODO
//fn bench_visitor_src_rw(c: &mut Criterion) {}

criterion_group!{
    name = benches;
    config = Criterion::default()
        .sample_size(200)
        .warm_up_time(Duration::new(5, 0))
        .measurement_time(Duration::new(10, 0))
            ;
    targets = 
        /* simple pattern */

        //bench_simple_best, 
        //bench_simple_best_normalized, 
        //bench_simple_not_rw, 
        //bench_simple_src_rw, 
        //bench_simple_best_normalized_fallback, 
        //bench_simple_not_rw_fallback, 
        //bench_simple_src_rw_fallback, 
        ////bench_simple_mir_rw,

        /* struct fields pattern */

        //bench_struct_fields_best, 
        //bench_struct_fields_best_normalized, 
        //bench_struct_fields_not_rw, 
        //bench_struct_fields_src_rw, 
        //bench_struct_fields_best_normalized_fallback, 
        //bench_struct_fields_not_rw_fallback, 
        //bench_struct_fields_src_rw_fallback, 

        /* vec pattern */

        bench_vec_best,
        bench_vec_best_normalized,
        bench_vec_not_rw, 
        bench_vec_src_rw,
        //bench_vec_best_normalized_fallback,
        //bench_vec_not_rw_fallback, 
        //bench_vec_src_rw_fallback, 

        /* visitor pattern */

        //bench_visitor_best,
        //bench_visitor_best_normalized,
        //bench_visitor_not_rw,
        //bench_visitor_src_rw,
        //bench_visitor_best_normalized_fallback,
        //bench_visitor_not_rw_fallback,
        //bench_visitor_src_rw_fallback,
}
criterion_main!(benches);

