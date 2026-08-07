use crate::constraints::{ConstraintStore, Constraints, Context, EnclosingScopes, MapValue};
use crate::constraints::unique_append;
use crate::error::Error;
use rustc_public::mir::Place;

//use log::debug;

pub fn merge_stores(
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

    // merge_stores_helper (via Vec<ConstraintStore>::merge) now carries its
    // own O(1) ptr_eq fast path internally, so pre-checking equality here
    // would just be a second full walk of the same map before the one
    // merge_stores_helper already does cheaply when nothing's changed.
    let merged_store = merge_stores_helper(cur_store, new_store);

    (merged_store, merged_es)
}

/*
fn merge_field_stores(cur_store: &FieldStore, new_store: &FieldStore) -> FieldStore {
    let merged_store = if *cur_store != *new_store {
        merge_stores_helper(cur_store, new_store)
    } else {
        cur_store.clone()
    };

    merged_store
}
*/

fn merge_stores_helper<T>(cur_store: &T, new_store: &T) -> T
where
    T: Clone + std::fmt::Debug,
    Vec<T>: Merge<T>,
{
    let vec = vec![cur_store.clone(), new_store.clone()];
    match vec.merge() {
        Ok(Some(merged)) => merged,
        Ok(None) => panic!("no stores to merge?"),
        e @ _ => panic!("error merging stores: {:?}", e),
    }
}

// FIXME merge span of RunningConstraints into a single vec if the RunningConstraintsInner are equal
fn merge_constraints(cur_constraints: &Constraints, new_constraints: &Constraints) -> Constraints {
    let mut merged = cur_constraints.clone();
    // No != pre-check: IndexSet::extend's per-element insert already no-ops
    // in O(1) for anything already present, so comparing first would just
    // be a second full walk of the set before the append that already
    // handles "nothing new" cheaply on its own.
    merged.append(new_constraints.clone());
    merged
}

/*
fn merge_fields(cur_fields: &Vec<Place>, new_fields: &Vec<Place>) -> Vec<Place> {
    let mut merged = cur_fields.clone();
    if merged != *new_fields {
        unique_append(&mut merged, new_fields.to_vec());
    }
    merged
}
*/

pub fn merge_mapvals(cur_val: &MapValue, new_val: &MapValue) -> MapValue {
    match (cur_val.clone(), new_val.clone()) {
        (MapValue::Constraints(cur_constraints), MapValue::Constraints(new_constraints)) => {
            MapValue::Constraints(merge_constraints(&cur_constraints, &new_constraints))
        }
        (MapValue::Store(cur_store, cur_es), MapValue::Store(new_store, new_es)) => {
            let (store, es) = merge_stores(&cur_store, &cur_es, &new_store, &new_es);
            MapValue::Store(store, es)
        }
        _ => panic!("incomparable MapValue types"),
    }
}

/*
pub fn merge_mapfieldvals(cur_val: &MapFieldValue, new_val: &MapFieldValue) -> MapFieldValue {
    match (cur_val.clone(), new_val.clone()) {
        (MapFieldValue::Fields(cur_fields), MapFieldValue::Fields(new_fields)) => {
            MapFieldValue::Fields(merge_fields(&cur_fields, &new_fields))
        }
        (MapFieldValue::Store(cur_store), MapFieldValue::Store(new_store)) => {
            let store = merge_field_stores(&cur_store, &new_store);
            MapFieldValue::Store(store)
        }
        _ => panic!("incomparable MapFieldValue types"),
    }
}
*/

pub trait Merge<T> {
    fn merge(&self) -> Result<Option<T>, Error>;
}

/*
impl Merge<FieldStore> for Vec<FieldStore> {
    fn merge(&self) -> Result<Option<FieldStore>, Error> {
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
            for (key, val) in store.clone().field_map.iter() {
                match merged.field_map.get_mut(key) {
                    Some(merged_val) => {
                        let new_merged_val = merge_mapfieldvals(merged_val, val);
                        merged
                            .field_map
                            .insert(key.clone(), Box::new(new_merged_val));
                    }
                    None => {
                        merged.field_map.insert(key.clone(), val.clone());
                    }
                }
            }
        }

        Ok(Some(merged))
    }
}
*/

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

            // O(1) fast path: im::HashMap::ptr_eq is true whenever the two
            // maps still share their underlying tree - guaranteed right
            // after a plain .clone() with no mutation since, which is
            // exactly the common case at a dynamic-dispatch site where
            // several candidates end up producing identical state. Skips
            // the O(n) per-key walk below entirely instead of paying a
            // full structural comparison just to *decide* there's nothing
            // to do (that comparison would cost as much as the merge).
            if merged.cmap.ptr_eq(&store.cmap) && merged.refs == store.refs {
                continue;
            }

            for (key, val) in store.cmap.iter() {
                match merged.cmap.get_mut(key) {
                    Some(merged_val) => {
                        let new_merged_val = merge_mapvals(merged_val, val);
                        merged.cmap.insert(key.clone(), Box::new(new_merged_val));
                    }
                    None => {
                        merged.cmap.insert(key.clone(), val.clone());
                    }
                }
            }
        }

        //debug!("merged stores: {:?}", merged);

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
            merged_constraints = merge_constraints(&merged_constraints, &constraints);
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

/*
impl Merge<CAFs> for Vec<CAFs> {
    fn merge(&self) -> Result<Option<CAFs>, Error> {
        if self.is_empty() {
            return Ok(None);
        }

        if self.len() == 1 {
            return Ok(Some(self[0].clone()));
        }

        let mut constraints = Vec::new();
        let mut fields = Vec::new();
        for caf in self.iter() {
            unique_push(&mut constraints, caf.constraints.clone());
            unique_push(&mut fields, caf.fields.clone());
        }
        let m_constraints = match constraints.merge() {
            Ok(Some(merged)) => merged,
            Ok(None) => todo!(),
            _ => panic!(),
        };
        let m_fields = match fields.merge() {
            Ok(Some(merged)) => merged,
            Ok(None) => todo!(),
            _ => panic!(),
        };
        Ok(Some(CAFs::new(
            m_constraints,
            m_fields,
            self[0].scope.clone(),
        )))
    }
}
*/

impl Merge<Context> for Vec<Context> {
    fn merge(&self) -> Result<Option<Context>, Error> {
        if self.is_empty() {
            return Ok(None);
        }

        if self.len() == 1 {
            return Ok(Some(self[0].clone()));
        }

        let mut cstores = Vec::new();
        //let mut fstores = Vec::new();
        for ctxt in self.iter() {
            // No unique_push here: it deduped via a full ConstraintStore
            // PartialEq comparison (walking the whole cmap) for every
            // existing entry - exactly the cost we're trying to avoid.
            // Vec<ConstraintStore>::merge() now has its own O(1) ptr_eq
            // fast path per pair, so duplicates get absorbed there just as
            // cheaply without an extra full-content scan up front.
            cstores.push(ctxt.cstore.clone());
            //unique_push(&mut fstores, ctxt.fstore.clone());
        }
        let m_cstores = match cstores.merge() {
            Ok(Some(merged)) => merged,
            Ok(None) => todo!(),
            _ => panic!(),
        };
        //let m_fstores = match fstores.merge() {
        //    Ok(Some(merged)) => merged,
        //    Ok(None) => todo!(),
        //    _ => panic!(),
        //};

        Ok(Some(Context::new(
            m_cstores,
            //m_fstores,
            self[0].wtos.clone(),
        )))
    }
}
