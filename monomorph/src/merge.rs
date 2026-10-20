use crate::constraints::VOID;
use crate::constraints::{ConstraintStore, Constraints, EnclosingScopes, MapValue};
use crate::error::Error;
use crate::interp::InterpPass;
use crate::timing::TimingCat;
use rustc_public::mir::Place;

use log::debug;

const MERGE_WIDEN_THRESHOLD: usize = 50;

/// Progress / growth signal for long runs: every 4096th merge, measures the
/// merged value (a full walk, hence the sampling) and logs at warn level
/// whenever it's the largest seen so far by at least 2x. Steadily climbing
/// sizes mean values keep growing (e.g. closures capturing earlier
/// iterations' values) rather than the analysis converging.
fn note_merge_size(merged: &Constraints, scope: Option<&VOID>) {
    use std::cell::Cell;
    thread_local! {
        static MERGES: Cell<u64> = const { Cell::new(0) };
        static LARGEST: Cell<usize> = const { Cell::new(0) };
        static START: std::time::Instant = std::time::Instant::now();
    }
    let n = MERGES.with(|m| {
        m.set(m.get() + 1);
        m.get()
    });
    if n % 4096 != 0 {
        return;
    }
    let size = crate::constraints::constraints_size(merged);
    if size >= 2 * LARGEST.with(|l| l.get()).max(512) {
        LARGEST.with(|l| l.set(size));
        let secs = START.with(|s| s.elapsed().as_secs());
        log::warn!(
            "[verifopt progress] merges={n} elapsed={secs}s largest merged value so far: {size} nodes, {} disjuncts, in {}",
            merged.inner.len(),
            scope.map(|s| s.0.name()).unwrap_or_default()
        );
        if size >= 1_000_000 && std::env::var_os("VERIFOPT_SIZE_HISTOGRAM").is_some() {
            log::warn!("[verifopt progress] composition: {}", crate::constraints::size_histogram(merged, 12));
        }
    }
}

fn merge_constraints(
    cur_constraints: &Constraints,
    new_constraints: &Constraints,
    timing: Option<(&InterpPass, &VOID)>,
) -> Constraints {
    let mut merged = cur_constraints.clone();
    let append_guard =
        timing.map(|(pass, scope)| pass.timing_span(TimingCat::TermMergeConstraintsAppend, scope));
    merged.append(new_constraints.clone());
    drop(append_guard);
    note_merge_size(&merged, timing.map(|(_, scope)| scope));
    debug!(
        "merge_constraints: merged_disjuncts={}",
        crate::constraints::constraints_size(&merged)
    );
    debug!(
        "merge_constraints: merged.inner.len()={}",
        merged.inner.len()
    );
    //debug!("MERGED CONSTRAINTS: {:?}", merged);
    if merged.inner.len() > MERGE_WIDEN_THRESHOLD {
        debug!("merge_constraints: WIDENING");
        let widen_guard = timing
            .map(|(pass, scope)| pass.timing_span(TimingCat::TermMergeConstraintsWiden, scope));
        let widened = crate::assumptions::with_widen_cause(
            crate::assumptions::Reason::WidenMerge,
            || crate::constraints::widen_constraints(&merged),
        );
        drop(widen_guard);
        widened
    } else {
        merged
    }
}

pub fn merge_mapvals(
    cur_val: &MapValue,
    new_val: &MapValue,
    timing: Option<(&InterpPass, &VOID)>,
) -> MapValue {
    match (cur_val.clone(), new_val.clone()) {
        (MapValue::Constraints(cur_constraints), MapValue::Constraints(new_constraints)) => {
            MapValue::Constraints(merge_constraints(
                &cur_constraints,
                &new_constraints,
                timing,
            ))
        }
        (MapValue::Store(cur_store, cur_es), MapValue::Store(new_store, new_es)) => {
            // `merge_stores_fallback` used to be permanently un-instrumented
            // here (this was the one call site with no real InterpPass to
            // route through `merge_stores_timed`). `scoped_update` now
            // threads `timing` through, so in practice this `None` arm
            // should rarely if ever fire - kept as a defensive fallback,
            // and now properly instrumented in its own right rather than
            // silently invisible if it ever does.
            let (store, es) = match timing {
                Some((pass, scope)) => {
                    pass.merge_stores_timed(scope, &cur_store, &cur_es, &new_store, &new_es)
                }
                None => merge_stores_fallback(&cur_store, &cur_es, &new_store, &new_es, timing),
            };
            MapValue::Store(store, es)
        }
        _ => panic!("incomparable MapValue types"),
    }
}

/// Whether two map values are the same value by identity (the same shared
/// constraint set, or stores with the same persistent maps), so merging them
/// would give that value back. A cheap check, not equality: false doesn't
/// mean they differ.
pub fn mapvals_identical(a: &MapValue, b: &MapValue) -> bool {
    // Entries the two contexts still share (persistent-map nodes not copied
    // since the clone) are the very same object.
    if std::ptr::eq(a, b) {
        return true;
    }
    match (a, b) {
        (MapValue::Constraints(x), MapValue::Constraints(y)) => std::rc::Rc::ptr_eq(&x.inner, &y.inner),
        (MapValue::Store(sx, ex), MapValue::Store(sy, ey)) => {
            sx.cmap.ptr_eq(&sy.cmap)
                && sx.refs.ptr_eq(&sy.refs)
                && match (ex, ey) {
                    (None, None) => true,
                    (Some(x), Some(y)) => std::rc::Rc::ptr_eq(x, y) || x == y,
                    _ => false,
                }
        }
        _ => false,
    }
}

pub trait Merge<T> {
    fn merge(&self, timing: Option<(&InterpPass, &VOID)>) -> Result<Option<T>, Error>;
}

/// Reached when `merge_mapvals` is called with `timing: None` - in practice
/// this should be rare now that `scoped_update` threads real timing through,
/// but kept as a defensive fallback for any caller that genuinely has no
/// `&InterpPass` available. Gets its own `MergeStoresFallback*` categories,
/// separate from `merge_stores_timed`'s, so the two paths' costs (and how
/// often this one actually fires) stay distinguishable in the timing data.
fn merge_stores_fallback(
    cur_store: &ConstraintStore,
    cur_es: &EnclosingScopes,
    new_store: &ConstraintStore,
    new_es: &EnclosingScopes,
    timing: Option<(&InterpPass, &VOID)>,
) -> (ConstraintStore, EnclosingScopes) {
    let es_guard =
        timing.map(|(pass, scope)| pass.timing_span(TimingCat::MergeStoresFallbackEs, scope));
    let merged_es = crate::constraints::merge_enclosing_scopes(cur_es, new_es);
    drop(es_guard);

    let vec = vec![cur_store.clone(), new_store.clone()];
    let merged_store = match vec.merge(timing) {
        Ok(Some(merged)) => merged,
        Ok(None) => panic!("no stores to merge?"),
        e @ _ => panic!("error merging stores: {:?}", e),
    };

    (merged_store, merged_es)
}

impl Merge<ConstraintStore> for Vec<ConstraintStore> {
    fn merge(
        &self,
        timing: Option<(&InterpPass, &VOID)>,
    ) -> Result<Option<ConstraintStore>, Error> {
        //debug!("interp stores to merge: {:?}", self);

        if self.is_empty() {
            return Ok(None);
        }

        if self.len() == 1 {
            return Ok(Some(self[0].clone()));
        }

        let mut merged = self[0].clone();
        let mut first = true;
        for store in self.iter() {
            if first {
                first = false;
                continue;
            }

            if merged.cmap.ptr_eq(&store.cmap) && merged.refs == store.refs {
                continue;
            }

            let cmap_guard = timing.map(|(pass, scope)| {
                pass.timing_span(TimingCat::MergeStoresFallbackCmapLoop, scope)
            });
            for (key, val) in store.cmap.iter() {
                match merged.cmap.get_mut(key) {
                    Some(merged_val) => {
                        let new_merged_val = merge_mapvals(merged_val, val, timing);
                        merged.cmap.insert(key.clone(), Box::new(new_merged_val));
                    }
                    None => {
                        merged.cmap.insert(key.clone(), val.clone());
                    }
                }
            }
            drop(cmap_guard);

            let refs_guard = timing.map(|(pass, scope)| {
                pass.timing_span(TimingCat::MergeStoresFallbackRefsUnion, scope)
            });
            merged.refs = crate::constraints::union_refs(merged.refs, store.refs.clone());
            drop(refs_guard);
        }

        Ok(Some(merged))
    }
}

impl Merge<Constraints> for Vec<Constraints> {
    fn merge(&self, timing: Option<(&InterpPass, &VOID)>) -> Result<Option<Constraints>, Error> {
        if self.is_empty() {
            return Ok(None);
        }

        if self.len() == 1 {
            return Ok(Some(self[0].clone()));
        }

        let mut merged_constraints = self[0].clone();
        for constraints in self.iter() {
            merged_constraints = merge_constraints(&merged_constraints, &constraints, timing);
        }

        Ok(Some(merged_constraints))
    }
}

impl Merge<Vec<Place>> for Vec<Vec<Place>> {
    fn merge(&self, _timing: Option<(&InterpPass, &VOID)>) -> Result<Option<Vec<Place>>, Error> {
        if self.is_empty() {
            return Ok(None);
        }

        if self.len() == 1 {
            return Ok(Some(self[0].clone()));
        }

        todo!();
    }
}
