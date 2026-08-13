use crate::constraints::unique_append;
use crate::constraints::{ConstraintStore, Constraints, Context, EnclosingScopes, MapValue};
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

    let merged_store = merge_stores_helper(cur_store, new_store);

    (merged_store, merged_es)
}

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

const MERGE_WIDEN_THRESHOLD: usize = 50;

fn merge_constraints(cur_constraints: &Constraints, new_constraints: &Constraints) -> Constraints {
    let mut merged = cur_constraints.clone();
    merged.append(new_constraints.clone());
    if merged.inner.len() > MERGE_WIDEN_THRESHOLD {
        crate::constraints::widen_constraints(&merged)
    } else {
        merged
    }
}

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

pub trait Merge<T> {
    fn merge(&self) -> Result<Option<T>, Error>;
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
                        let new_merged_val = merge_mapvals(merged_val, val);
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
            cstores.push(ctxt.cstore.clone());
        }
        let m_cstores = match cstores.merge() {
            Ok(Some(merged)) => merged,
            Ok(None) => todo!(),
            _ => panic!(),
        };
        let mut m_wtos = self[0].wtos.clone();
        for ctxt in self.iter().skip(1) {
            m_wtos = m_wtos.union(ctxt.wtos.clone());
        }

        Ok(Some(Context::new(m_cstores, m_wtos)))
    }
}
