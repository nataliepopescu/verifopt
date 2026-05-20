use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_public::mir::Local;
use rustc_public::mir::mono::Instance;
use rustc_public::ty::{AdtDef, Ty};

use crate::error::Error;
use crate::wto::BBDeps;

use log::debug;

pub fn unique_push(vec: &mut Constraints, elem: VORval) -> Option<VORval> {
    if vec.contains(&elem) {
        Some(elem)
    } else {
        vec.push(elem);
        None
    }
}

pub fn unique_append(vec: &mut Constraints, to_append: Constraints) {
    for elem in to_append {
        unique_push(vec, elem);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapKey {
    Local(Local),
    ScopeId(Instance),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MapValue {
    // TODO add scope backptr in for closures
    // (Option<DefId>, where None == top-level global scope)
    Store(InterpStore),
    Constraints(Constraints),
}

// Set of positive constraints; negative constraints are resolved immediately by removing them from the set
pub type Constraints = Vec<VORval>;

// Alias around VORval to make it semantically easier to tell when we are processing generic arguments
pub type VOGenargs = Vec<VOGenarg>;
pub type VOGenarg = VORval;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VORval {
    IdkAdt(AdtDef, Option<VOGenargs>),
    IdkType(Ty),
    Idk,
    AddressOf(Box<VORval>),
    RawPtr(Box<VORval>),
    Ref(Box<VORval>),
    Tuple(Vec<VORval>),
    Scalar(u128),
    Bool,
    Uint,
    Slice(Ty),
    Array(Ty),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterpStore {
    pub cmap: HashMap<MapKey, Box<MapValue>>,
    pub wtos: HashMap<Instance, BBDeps>,
}

impl InterpStore {
    pub fn new() -> InterpStore {
        Self {
            cmap: HashMap::default(),
            wtos: HashMap::default(),
        }
    }

    pub fn scoped_get(
        &self,
        scope: Instance,
        key: &MapKey,
        //traverse_backptr: bool,
    ) -> Option<MapValue> {
        debug!("IN SCOPED_GET");
        debug!("scope: {:?}", scope);
        debug!("key: {:?}", key);
        //debug!("cmap: {:#?}", self.cmap);

        //if scope.is_none() {
        //    match self.cmap.get(key) {
        //        Some(boxed) => return Some(*boxed.clone()),
        //        None => return None,
        //    }
        //}

        match self.cmap.get(&MapKey::ScopeId(scope)) {
            Some(vartype) => match *vartype.clone() {
                MapValue::Store(store) => {
                    //if substores.len() != 1 {
                    //    todo!(
                    //        "not impl yet (vec of substores w len = {:?})",
                    //        substores.len()
                    //    );
                    //}

                    // Is key in inner_cmap? if not:
                    // - Is nested func: return None
                    // - Is closure: follow backptr to enclosing scope
                    match store.cmap.get(key) {
                        Some(boxed) => Some(*boxed.clone()),
                        None => None,
                        // FIXME closures should not recurse past enclosing function
                        //if traverse_backptr {
                        //    self.scoped_get(backptr, var, traverse_backptr)
                        //} else {
                        //    None
                        //}
                    }
                }
                _ => panic!("not a scope: {:?}", scope),
            },
            None => None,
        }
    }

    pub fn scoped_update(&mut self, scope: Instance, key: MapKey, value: Box<MapValue>) {
        //if scope.is_none() {
        //    if self.cmap.contains_key(&key) {
        //        // FIXME MIR is not SSA
        //        error!("symbol already exists: {:?}", key);
        //    }

        //    self.cmap.insert(key, value.clone());
        //    return;
        //}

        match self.cmap.get(&MapKey::ScopeId(scope)) {
            Some(vartype) => match *vartype.clone() {
                MapValue::Store(mut store) => {
                    //if substores.len() != 1 {
                    //    todo!(
                    //        "not impl yet (vec of substores w len = {:?})",
                    //        substores.len()
                    //    );
                    //}
                    let mut new_val = value.clone();
                    let old_val = store.cmap.get(&key);
                    match old_val {
                        Some(old_val_) => {
                            new_val = Box::new(merge_mapvals(old_val_, &value));
                        }
                        None => {}
                    }

                    // modify scope w new key/val
                    store.cmap.insert(key, new_val);
                    self.cmap
                        .insert(MapKey::ScopeId(scope), Box::new(MapValue::Store(store)));
                }
                MapValue::Constraints(..) => {
                    panic!("defid is not a scope: {:?}", scope);
                }
            },
            None => panic!("undefined scope: {:?}", scope),
        }
    }
}

fn merge_stores(cur_store: &InterpStore, new_store: &InterpStore) -> InterpStore {
    let merged = cur_store.clone();
    if merged != *new_store {
        todo!("merge different stores");
    }
    merged
}

fn merge_constraints(cur_constraints: &Constraints, new_constraints: &Constraints) -> Constraints {
    let mut merged = cur_constraints.clone();
    if merged != *new_constraints {
        unique_append(&mut merged, new_constraints.to_vec());
    }
    merged
}

fn merge_mapvals(cur_val: &MapValue, new_val: &MapValue) -> MapValue {
    match (cur_val.clone(), new_val.clone()) {
        (MapValue::Constraints(cur_constraints), MapValue::Constraints(new_constraints)) => {
            MapValue::Constraints(merge_constraints(&cur_constraints, &new_constraints))
        }
        (MapValue::Store(cur_store), MapValue::Store(new_store)) => {
            MapValue::Store(merge_stores(&cur_store, &new_store))
        }
        _ => panic!("incomparable MapValue types"),
    }
}

pub trait Merge<T> {
    fn merge(&self) -> Result<Option<T>, Error>;
}

impl Merge<InterpStore> for Vec<InterpStore> {
    fn merge(&self) -> Result<Option<InterpStore>, Error> {
        debug!("interp stores to merge: {:?}", self);

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
