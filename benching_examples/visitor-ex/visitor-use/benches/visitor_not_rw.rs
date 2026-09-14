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

struct SpeakBetterDogs;

impl AnimalVisitor for SpeakBetterDogs {
    fn receive_dog(&self, _a: &dyn Animal) -> usize {
        44444
    }
    fn receive_cat(&self, a: &dyn Animal) -> usize {
        a.speak()
    }
}

fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        Box::new(Cat {})
    } else {
        Box::new(Dog {})
    }
}

pub fn run(a: Box<dyn Animal>, dc: &SpeakBetterDogs) -> usize {
    a.visit(dc)
}

fn bench_visitor(c: &mut Criterion) {
    let dc = SpeakBetterDogs;
    let mut group = c.benchmark_group("visitor");
    group.bench_function("visitor_not_rw", |b| {
        b.iter_batched(
            || get_animal(rand::rng().random_range(..2usize)),
            |animal| std::hint::black_box(run(animal, &dc)),
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

criterion_group!(benches, bench_visitor);
criterion_main!(benches);
