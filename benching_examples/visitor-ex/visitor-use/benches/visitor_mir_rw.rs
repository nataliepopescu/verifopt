// visitor_mir_rw.rs - deliberately near-identical to visitor_not_rw.rs
// (same plain dyn dispatch throughout, no source-level simulation of a
// rewrite at all) - the only thing that's supposed to differ is how
// this gets compiled. Built via
// `cargo verifopt --bench visitor_mir_rw --skip-analysis` (no
// --skip-rewrite this time), so the real, already-discovered rewrites
// in verifopt_store.json actually get applied by the modified
// compiler's own codegen_mir hook during this build.
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use visitor_decl::Animal;
use visitor_use::{SpeakBetterDogs, get_animal, wrap_dyn_call};
use rand::Rng;

pub fn run(a: Box<dyn Animal>, dc: &SpeakBetterDogs) -> usize {
    wrap_dyn_call(a, dc)
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
