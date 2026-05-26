use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_public::mir::Local;
use rustc_public::mir::mono::Instance;
use rustc_public::ty::{AdtDef, ClosureDef, FnDef, GenericArgs, Ty};

use crate::common::log_scope;
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

/// Using `Instance` as unique ID (internal objects are interned so this is apparently cheap)
pub type VOID = Instance;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapKey {
    Local(Local),
    ScopeId(VOID),
}

pub type EnclosingScope = Option<VOID>;

#[derive(Debug, Clone, PartialEq)]
pub enum MapValue {
    Store(InterpStore, EnclosingScope),
    Constraints(Constraints),
}

// Set of positive constraints; negative constraints are resolved immediately by removing them from the set
pub type Constraints = Vec<VORval>;

// Alias around VORval to make it semantically easier to tell when we are processing generic arguments
pub type VOGenargs = Vec<VOGenarg>;
pub type VOGenarg = VORval;

// Preserving these int types for closure kind encoding
//#[derive(Debug, Clone, PartialEq, Eq, Hash)]
//pub enum IntTy {
//    I8,
//    I16,
//    I32,
//    Other
//}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VORval {
    Idk,
    IdkType(Ty),
    Adt(AdtDef, Option<VOGenargs>),
    AddressOf(Box<VORval>),
    RawPtr(Box<VORval>),
    Ref(Box<VORval>),
    Tuple(Vec<VORval>),
    Scalar(u128),
    Bool,
    Int, //(IntTy),
    Uint,
    Slice(Ty),
    Array(Ty),
    //Closure(ClosureDef, Option<VOGenargs>), //, ClosureKind),
    Closure(ClosureDef, GenericArgs), //, ClosureKind),
    FnDef(FnDef, Option<VOGenargs>),
    FnPtr(Vec<VORval>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterpStore {
    pub cmap: HashMap<MapKey, Box<MapValue>>,
    pub wtos: HashMap<VOID, BBDeps>,
}

impl InterpStore {
    pub fn new() -> InterpStore {
        Self {
            cmap: HashMap::default(),
            wtos: HashMap::default(),
        }
    }

    pub fn scoped_get(&self, scope: VOID, key: &MapKey, is_closure: bool) -> Option<MapValue> {
        debug!("IN SCOPED_GET");
        log_scope(scope);
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
                MapValue::Store(store, enclosing_scope) => {
                    // Is key in inner_cmap? if not:
                    // - Is nested func: return None
                    // - Is closure: follow backptr to enclosing scope
                    match store.cmap.get(key) {
                        Some(boxed) => Some(*boxed.clone()),
                        None => {
                            debug!("is_closure?: {:?}", is_closure);
                            debug!("enclosing_scope: {:?}", enclosing_scope);
                            if is_closure && enclosing_scope.is_some() {
                                // Check enclosing scopes for missing key(s)
                                self.scoped_get(enclosing_scope.unwrap(), key, false)
                            } else {
                                None
                            }
                        }
                    }
                }
                _ => panic!("not a scope: {:?}", scope),
            },
            None => None,
        }
    }

    pub fn scoped_update(&mut self, scope: VOID, key: MapKey, value: Box<MapValue>) {
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
                MapValue::Store(mut store, enclosing_scope) => {
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
                    self.cmap.insert(
                        MapKey::ScopeId(scope),
                        Box::new(MapValue::Store(store, enclosing_scope)),
                    );
                }
                MapValue::Constraints(..) => {
                    panic!("defid is not a scope: {:?}", scope);
                }
            },
            None => panic!("undefined scope: {:?}", scope),
        }
    }
}

pub fn merge_stores(
    cur_store: &InterpStore,
    cur_store_es: &EnclosingScope,
    new_store: &InterpStore,
    new_store_es: &EnclosingScope,
) -> (InterpStore, EnclosingScope) {
    let merged_es = cur_store_es.clone();
    if cur_store_es != new_store_es {
        debug!("existing es: {:?}", cur_store_es);
        debug!("new es: {:?}", new_store_es);
        todo!("enclosing scopes differ");
    }
    let merged_store = cur_store.clone();
    if merged_store != *new_store {
        todo!("merge different stores");
    }
    (merged_store, merged_es)
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
        (MapValue::Store(cur_store, cur_store_es), MapValue::Store(new_store, new_store_es)) => {
            let (store, es) = merge_stores(&cur_store, &cur_store_es, &new_store, &new_store_es);
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
