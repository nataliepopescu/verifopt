// Reproducer for the `refs` soundness gap: the same `union()`-on-collision
// pattern as `wtos` (see merge_cstores_timed in interp.rs), but for
// reference/alias tracking instead of basic-block traversal state. This is
// the same shape as the confirmed "genuinely different candidate instances
// colliding on the same alias key" case found earlier (4,176 events in one
// ripgrep run, traced to PrefilterI candidates).
//
// Mechanism this is built to trigger:
//   - `add_ref` (constraints.rs) is called whenever a function is invoked
//     with a reference-typed argument (see the call site in interp.rs
//     around argument resolution). It records:
//       (callee's parameter place, callee_scope) -> (caller's place, caller_scope)
//   - `Holder::touch` is a *default* trait method, inherited verbatim by
//     both `A` and `B`. Like `Worker::run` in the wtos reproducer,
//     `touch::<A>` and `touch::<B>` are different monomorphized VOIDs
//     (Self is part of Instance's GenericArgs even though the body never
//     touches `self`) - but the default body's call to `read_ref(&x)`
//     goes to a plain, non-generic free function with no relationship to
//     Self at all. Its VOID (and its parameter's place) is identical no
//     matter which impl's `touch` calls it.
//   - `main`'s single `h.touch(..)` call site is dynamically dispatched:
//     verifopt explores both `A` and `B` as candidates. Each candidate's
//     simulation calls `read_ref(&x)`, where `x` is *that candidate's
//     own* local (touch::<A>'s local vs touch::<B>'s local - different
//     places, in different caller scopes, since touch::<A>_VOID !=
//     touch::<B>_VOID).
//   - The resulting `add_ref` calls collide: the *key*
//     (read_ref's param0 place, read_ref_VOID) is identical across both
//     candidates, but the *value* (caller place, caller scope) legitimately
//     differs. When the two candidates' contexts are merged back together,
//     `merged.refs = merged.refs.union(store.refs)` keeps whichever
//     candidate's alias fact came first and silently drops the other -
//     meaning the analysis ends up believing `read_ref`'s argument only
//     ever aliases *one* of the two real possibilities, not both.
//
// After running under verifopt, check the `MERGE_OVERLAP_STATS kind=refs`
// debug log lines (or wherever `find_conflicting_keys` reports on `refs`)
// to confirm this collision was actually detected.

trait Holder {
    fn touch(&self, x: i32) -> i32 {
        read_ref(&x)
    }
}

struct A;
struct B;

impl Holder for A {}
impl Holder for B {}

// Self-independent, non-generic, takes a reference: same VOID and same
// parameter place regardless of which impl's `touch` calls it - only the
// caller-side alias target (which candidate's local `x` it points to)
// differs.
fn read_ref(r: &i32) -> i32 {
    *r
}

fn main() {
    let holders: Vec<Box<dyn Holder>> = vec![Box::new(A), Box::new(B)];
    for (i, h) in holders.iter().enumerate() {
        // Single call site, dynamically dispatched - verifopt explores
        // both A and B as candidates here, then merges the resulting
        // contexts (including their `refs` maps) back together.
        let result = h.touch(i as i32);
        println!("{result}");
    }
}
