use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_data_structures::fx::FxHashSet as HashSet;
use rustc_public::DefId;
use rustc_public::mir::Place;
use rustc_public::mir::mono::InstanceDef;
use rustc_public::ty::Ty;

use crate::wto::BBDeps;

use log::debug;

pub type ScopeId = (DefId, InstanceDef);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapKey {
    // FIXME maybe make this Local(Local<'tcx>) instead of Place, so don't index the map with random projections
    Place(Place),
    ScopeId(ScopeId),
}

// Set of positive constraints; negative constraints are resolved immediately by removing them from the set
pub type Constraints = HashSet<VerifoptRval>;

#[derive(Debug, Clone, PartialEq)]
pub enum MapValue {
    // TODO add scope backptr in for closures
    // (Option<DefId>, where None == top-level global scope)
    Store(Vec<InterpStore>),
    Constraints(Constraints),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VerifoptRval {
    Scalar(u128),
    Ptr(Box<VerifoptRval>),
    Ref(Box<VerifoptRval>),
    ConstSlice(),
    IndirectConst(Ty),
    IdkStruct(DefId, Option<Vec<Vec<VerifoptRval>>>),
    //IdkGeneric(Symbol),
    IdkStr(), //Const<'tcx>),
    // FIXME don't want types
    IdkType(Ty),
    IdkDefId(DefId),
    Idk(),
    Undef(),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterpStore {
    pub cmap: HashMap<MapKey, Box<MapValue>>,
    pub wtos: HashMap<ScopeId, BBDeps>,
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
        scope: ScopeId,
        key: &MapKey,
        //traverse_backptr: bool,
    ) -> Option<MapValue> {
        debug!("IN SCOPED_GET");
        debug!("scope: {:?}", scope);
        debug!("key: {:?}", key);
        debug!("cmap: {:#?}", self.cmap);

        //if scope.is_none() {
        //    match self.cmap.get(key) {
        //        Some(boxed) => return Some(*boxed.clone()),
        //        None => return None,
        //    }
        //}

        match self.cmap.get(&MapKey::ScopeId(scope)) {
            Some(vartype) => match *vartype.clone() {
                MapValue::Store(substores) => {
                    if substores.len() != 1 {
                        todo!(
                            "not impl yet (vec of substores w len = {:?})",
                            substores.len()
                        );
                    }

                    // Is key in inner_cmap? if not:
                    // - Is nested func: return None
                    // - Is closure: follow backptr to enclosing scope
                    match substores[0].cmap.get(key) {
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

    pub fn scoped_update(&mut self, scope: ScopeId, key: MapKey, value: Box<MapValue>) {
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
                MapValue::Store(mut substores) => {
                    if substores.len() != 1 {
                        todo!(
                            "not impl yet (vec of substores w len = {:?})",
                            substores.len()
                        );
                    }
                    let mut new_val = value.clone();
                    let substore = &mut substores[0].cmap;
                    let old_val = substore.get(&key);
                    match old_val {
                        Some(old_val_) => {
                            new_val = Box::new(self.merge_vals(*old_val_.clone(), *value));
                        }
                        None => {}
                    }

                    // modify scope w new key/val
                    substore.insert(key, new_val);
                    self.cmap
                        .insert(MapKey::ScopeId(scope), Box::new(MapValue::Store(substores)));
                }
                MapValue::Constraints(..) => {
                    panic!("defid is not a scope: {:?}", scope);
                }
            },
            None => panic!("undefined scope: {:?}", scope),
        }
    }

    fn merge_vals(&mut self, old_val: MapValue, new_val: MapValue) -> MapValue {
        debug!("old val: {:?}", old_val);
        debug!("new val: {:?}", new_val);

        //let mut cvec = vec![];
        //cvec.push(*value);
        //cvec.push(*old_val.clone());
        //debug!("cvec: {:?}", cvec);

        let merged = old_val.clone();
        match (merged.clone(), new_val.clone()) {
            (MapValue::Constraints(_c_merged), MapValue::Constraints(_c_new)) => {
                // FIXME
                //if c_merged !=
            }
            _ => todo!("merge scopes"),
        }

        merged

        // FIXME don't need merge if only ever comparing two values
        /*
        match cvec.merge() {
            Ok(Some(new_val)) => {
                subscope_cmap.insert(key, Box::new(new_val));
                self.cmap.insert(
                    MapKey::ScopeId(scope.unwrap()),
                    Box::new(MapValue::Store(substores)),
                );
            }
            _ => todo!("not impl yet"),
        }
        */
    }
}
