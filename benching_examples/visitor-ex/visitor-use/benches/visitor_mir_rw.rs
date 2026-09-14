// visitor_mir_rw.rs - deliberately near-identical to visitor_not_rw.rs
// (same plain dyn dispatch throughout, no source-level simulation of a
// rewrite at all) - the only thing that's supposed to differ is how
// this gets compiled. Built via
// `cargo verifopt --bench visitor_mir_rw --skip-analysis` (no
// --skip-rewrite this time), so the real, already-discovered rewrites
// in verifopt_store.json actually get applied by the modified
// compiler's own codegen_mir hook during this build.
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
    group.bench_function("visitor_mir_rw", |b| {
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
