use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use std::time::Duration;

use rand::Rng;
use rewrites::{simple, pub_trait, vec};
use visitor_decl;
use visitor_use;

fn get_input_num_vec() -> Vec<usize> {
    let mut nums_vec: Vec<usize> = vec![];
    for _ in 0..1000 {
        nums_vec.push(rand::rng().random_range(..2));
    }
    nums_vec
}

//fn get_pub_trait_input_animal_vec() -> Vec<Box<dyn pub_trait::Animal>> {
//    let mut animal_vec: Vec<Box<dyn pub_trait::Animal>> = vec![];
//    for _ in 0..1000 {
//        if rand::rng().random_range(..2usize) == 0 {
//            animal_vec.push(Box::new(pub_trait::Cat{}));
//        } else {
//            animal_vec.push(Box::new(pub_trait::Dog{}));
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

//////

fn bench_simple_best(c: &mut Criterion) {
    let cat: &simple::Cat = &simple::Cat {};

    c.bench_function(
        "simple_best",
        |b| b.iter(|| {
            simple::run_best(cat)
        })
    );
}

fn bench_simple_best_norm(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;
    let cat: &simple::Cat = &simple::Cat {};

    c.bench_function(
        "simple_best_norm",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_best_norm(num, cat)
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

fn bench_simple_best_norm_fallback(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;
    let cat: &simple::Cat = &simple::Cat {};

    c.bench_function(
        "simple_best_norm_fallback",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            simple::run_best_norm_fallback(num, cat)
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

fn bench_pub_trait_best(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    let mut idx = 0;
    let cat: &pub_trait::Cat = &pub_trait::Cat {};

    c.bench_function(
        "pub_trait_best",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            idx += 1;
            pub_trait::run_best(num, cat)
        })
    );
}

fn bench_pub_trait_not_rw(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    //let animal_vec = get_pub_trait_input_animal_vec();
    //let animals: &[Box<dyn pub_trait::Animal>] = &animal_vec[..];
    let mut idx = 0;

    c.bench_function(
        "pub_trait_not_rw",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            //let animal = black_box(&*animals[idx % 1000]);
            idx += 1;
            pub_trait::run_not_rw(num) //, animal)
        })
    );
}

fn bench_pub_trait_src_rw(c: &mut Criterion) {
    let nums_vec = get_input_num_vec();
    let nums: &[usize] = &nums_vec[..];
    //let animal_vec = get_pub_trait_input_animal_vec();
    //let animals: &[Box<dyn pub_trait::Animal>] = &animal_vec[..];
    let mut idx = 0;

    c.bench_function(
        "pub_trait_src_rw",
        |b| b.iter(|| {
            let num = black_box(nums[idx % 1000]);
            //let animal = black_box(&*animals[idx % 1000]);
            idx += 1;
            pub_trait::run_src_rw(num) //, animal)
        })
    );
}

// complicated structs

// multiple trait methods

// different paths

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
        bench_simple_best, 
        bench_simple_best_norm, 
        bench_simple_not_rw, 
        bench_simple_src_rw, 
        bench_simple_best_norm_fallback, 
        bench_simple_not_rw_fallback, 
        bench_simple_src_rw_fallback, 
        //bench_mir_rw,

        /* pub trait pattern */
        //bench_pub_trait_best,
        //bench_pub_trait_not_rw, 
        //bench_pub_trait_src_rw, 

        /* vec pattern */
        //bench_vec_best,
        //bench_vec_not_rw, 
        //bench_vec_src_rw, 

        /* visitor pattern */
        //bench_visitor_best,
        //bench_visitor_not_rw,
        //bench_visitor_src_rw,
}
criterion_main!(benches);

