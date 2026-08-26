// Reproducer for the `wtos` soundness gap: `merge_contexts_timed`'s
// `m_wtos = m_wtos.union(wtos)` (in interp.rs) keeps the *first* candidate's
// BBDeps for a shared scope on key collision and silently drops the other's
// - unlike `cmap`, which properly reconciles instead of overwriting.
//
// Mechanism this is built to trigger:
//   - `Worker::run` is a *default* trait method, inherited verbatim by both
//     `Fast` and `Slow`. Each concrete impl gets its own monomorphized
//     Instance (so `Fast::run` and `Slow::run` are different VOIDs), but
//     the default body's own call to `step_a()`/`step_b()` goes to plain,
//     non-generic free functions with no relationship to `Self` at all -
//     their VOID (Instance, no GenericArgs) is identical no matter which
//     impl's `run` is being simulated.
//   - `main`'s single `w.run()` call site is dynamically dispatched:
//     verifopt's CHA/FSA candidate exploration simulates *both* `Fast` and
//     `Slow` as candidates for it, and each candidate's simulation
//     independently visits `step_a`'s (and `step_b`'s) shared scope,
//     building its own BBDeps (preds/ordering/visited) for it.
//   - `step_a` has enough internal branching (an early return plus a
//     loop) that its BBDeps *should* end up differing in some way across
//     the two candidates' visits, rather than being trivially identical -
//     the whole point being to give `merge_contexts_timed`'s wtos union a
//     real collision to reconcile (or silently discard) rather than a
//     harmless no-op union of two identical entries.
//   - When the two candidates' contexts get merged back together, the
//     shared `step_a`/`step_b` scope's wtos entry collides. Whichever
//     candidate's BBDeps `union()` keeps, the other's traversal state is
//     gone - if that discarded side had reached basic blocks the kept
//     side hadn't, a later re-entry into that scope could wrongly treat
//     it as fully processed (empty `ordering`) when it isn't.
//
// After running under verifopt, check whether this scope shows up in the
// `wtos_merge_conflicts` set (or in the `MERGE_OVERLAP_STATS kind=wtos`
// debug log lines) to confirm the collision was actually detected, not
// just structurally present.

trait Worker {
    fn run(&self, n: i32) -> i32 {
        step_a(n) + step_b()
    }
}

struct Fast;
struct Slow;

impl Worker for Fast {}
impl Worker for Slow {}

// Self-independent, non-generic: same VOID (Instance, no GenericArgs)
// regardless of which impl's `run` calls it.
fn step_a(n: i32) -> i32 {
    if n < 0 {
        // early-return branch - only reachable depending on the argument,
        // giving the two candidates' traversals a chance to differ.
        return -1;
    }

    let mut acc = 0;
    let mut i = 0;
    while i < 4 {
        if i % 2 == 0 {
            acc += i;
        } else {
            acc -= i;
        }
        i += 1;
    }
    acc
}

fn step_b() -> i32 {
    42
}

fn main() {
    let workers: Vec<Box<dyn Worker>> = vec![Box::new(Fast), Box::new(Slow)];
    for (i, w) in workers.iter().enumerate() {
        // Single call site, dynamically dispatched - verifopt explores
        // *both* Fast and Slow as candidates here, then merges the
        // resulting contexts (including their `wtos` maps) back together.
        let n = if i == 0 { 3 } else { -1 };
        let result = w.run(n);
        println!("{result}");
    }
}
