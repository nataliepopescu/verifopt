use crate::constraints::VOID;
use crate::constraints::unique_append;
use crate::constraints::{ConstraintStore, Constraints, EnclosingScopes, MapValue};
use crate::error::Error;
use crate::interp::{InterpPass, TimingCat};
use rustc_public::mir::Place;

use log::debug;

const MERGE_WIDEN_THRESHOLD: usize = 50;

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
        let widened = crate::constraints::widen_constraints(&merged);
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
            // `scoped_update` (in constraints.rs) is a method on
            // ConstraintStore itself, not InterpPass - it has no `self`
            // to call `merge_stores_timed` through, and threading
            // `&InterpPass` down to it would mean touching every caller
            // of `scoped_update` throughout the codebase, well beyond
            // this fix's scope. So this is the one remaining place that
            // still needs an un-instrumented fallback - every other
            // caller of `merge_mapvals` passes `Some`, and goes through
            // the fully-instrumented `merge_stores_timed` instead.
            let (store, es) = match timing {
                Some((pass, scope)) => {
                    pass.merge_stores_timed(scope, &cur_store, &cur_es, &new_store, &new_es)
                }
                None => merge_stores_fallback(&cur_store, &cur_es, &new_store, &new_es),
            };
            MapValue::Store(store, es)
        }
        _ => panic!("incomparable MapValue types"),
    }
}

pub trait Merge<T> {
    fn merge(&self) -> Result<Option<T>, Error>;
}

/// Used only when `merge_mapvals` is called with `timing: None` (i.e. from
/// `scoped_update`, a `ConstraintStore` method with no `&InterpPass` to
/// call `merge_stores_timed` through) - an intentionally un-instrumented
/// fallback, not a competing implementation of the same logic. Every
/// other caller goes through `InterpPass::merge_stores_timed` instead,
/// which is the one that gets `REFS MERGE CONFLICT` detection and the
/// rest of the timing/diagnostic machinery.
fn merge_stores_fallback(
    cur_store: &ConstraintStore,
    cur_es: &EnclosingScopes,
    new_store: &ConstraintStore,
    new_es: &EnclosingScopes,
) -> (ConstraintStore, EnclosingScopes) {
    let merged_es = match (cur_es, new_es) {
        (Some(cur_es_vec), Some(new_es_vec)) => {
            let mut merged_es_vec = cur_es_vec.clone();
            unique_append(&mut merged_es_vec, new_es_vec.to_vec());
            Some(merged_es_vec)
        }
        (Some(cur_es_vec), None) => Some(cur_es_vec.to_vec()),
        (None, Some(new_es_vec)) => Some(new_es_vec.to_vec()),
        (None, None) => None,
    };

    let vec = vec![cur_store.clone(), new_store.clone()];
    let merged_store = match vec.merge() {
        Ok(Some(merged)) => merged,
        Ok(None) => panic!("no stores to merge?"),
        e @ _ => panic!("error merging stores: {:?}", e),
    };

    (merged_store, merged_es)
}

impl Merge<ConstraintStore> for Vec<ConstraintStore> {
    fn merge(&self) -> Result<Option<ConstraintStore>, Error> {
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

            for (key, val) in store.cmap.iter() {
                match merged.cmap.get_mut(key) {
                    Some(merged_val) => {
                        let new_merged_val = merge_mapvals(merged_val, val, None);
                        merged.cmap.insert(key.clone(), Box::new(new_merged_val));
                    }
                    None => {
                        merged.cmap.insert(key.clone(), val.clone());
                    }
                }
            }
            merged.refs = merged.refs.union(store.refs.clone());
        }

        Ok(Some(merged))
    }
}

impl Merge<Constraints> for Vec<Constraints> {
    fn merge(&self) -> Result<Option<Constraints>, Error> {
        if self.is_empty() {
            return Ok(None);
        }

        if self.len() == 1 {
            return Ok(Some(self[0].clone()));
        }

        let mut merged_constraints = self[0].clone();
        for constraints in self.iter() {
            merged_constraints = merge_constraints(&merged_constraints, &constraints, None);
        }

        Ok(Some(merged_constraints))
    }
}

impl Merge<Vec<Place>> for Vec<Vec<Place>> {
    fn merge(&self) -> Result<Option<Vec<Place>>, Error> {
        if self.is_empty() {
            return Ok(None);
        }

        if self.len() == 1 {
            return Ok(Some(self[0].clone()));
        }

        todo!();
    }
}
