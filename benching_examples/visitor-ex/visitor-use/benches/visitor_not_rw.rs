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
use visitor_decl::Animal;
use visitor_use::{SpeakBetterDogs, get_animal, wrap_dyn_call};
use rand::Rng;


pub fn run(a: Box<dyn Animal>, dc: &SpeakBetterDogs) -> usize {
    wrap_dyn_call(a, dc)
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
