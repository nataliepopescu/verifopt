use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

pub trait Animal {
    fn speak(&self) -> usize;
    fn walk(&self) -> usize;
}

#[inline(never)]
pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        Box::new(Cat {})
    } else {
        Box::new(Dog {})
    }
}

#[inline(always)]
pub fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {});
}

#[inline(always)]
pub fn get_dog() -> Box<dyn Animal> {
    return Box::new(Dog {});
}

pub struct Cat;
pub struct Dog;

impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }
    fn walk(&self) -> usize {
        33333
    }
}

impl Animal for Dog {
    fn speak(&self) -> usize {
        22222
    }
    fn walk(&self) -> usize {
        44444
    }
}

pub fn run(a: Box<dyn Animal>) -> usize {
    a.speak()
}

fn bench_negative(c: &mut Criterion) {
    let mut group = c.benchmark_group("negative");
    let x = 0;
    group.bench_function("negative_mir_rw", |b| {
        b.iter_batched(
            || get_animal(x), //rand::rng().random_range(..2usize)),
            |animal| std::hint::black_box(run(animal)),
            BatchSize::SmallInput,
        )
    });
    group.finish()
}

criterion_group!(benches, bench_negative);
criterion_main!(benches);
