use crate::constraints::unique_append;
use crate::constraints::{
    Constraints, ConstraintsAndFields, EnclosingScopes, InterpStore, MapValue,
};
use crate::error::Error;

//use log::debug;

pub fn merge_stores(
    cur_store: &InterpStore,
    cur_es: &EnclosingScopes,
    new_store: &InterpStore,
    new_es: &EnclosingScopes,
) -> (InterpStore, EnclosingScopes) {
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

    let merged_store = if *cur_store != *new_store {
        merge_stores_helper(cur_store, new_store)
    } else {
        cur_store.clone()
    };

    (merged_store, merged_es)
}

fn merge_stores_helper(cur_store: &InterpStore, new_store: &InterpStore) -> InterpStore {
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
    if merged != *new_constraints {
        unique_append(&mut merged, new_constraints.to_vec());
    }
    merged
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

impl Merge<InterpStore> for Vec<InterpStore> {
    fn merge(&self) -> Result<Option<InterpStore>, Error> {
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
            for (key, val) in store.clone().cmap.iter() {
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

impl Merge<ConstraintsAndFields> for Vec<ConstraintsAndFields> {
    fn merge(&self) -> Result<Option<ConstraintsAndFields>, Error> {
        todo!();
    }
}
