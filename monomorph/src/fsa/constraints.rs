use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_data_structures::fx::FxHashSet as HashSet;
use rustc_public::DefId;
use rustc_public::mir::Place;
use rustc_public::mir::mono::InstanceDef;
use rustc_public::ty::Ty;

use crate::fsa::wto::BBDeps;

use log::{debug, error};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum MapKey {
    // FIXME maybe make this Local(Local<'tcx>) instead of Place, so don't index the map with random projections
    Place(Place),
    ScopeId(DefId),
}

// Set of positive constraints; negative constraints are resolved immediately by removing them from the set
pub(crate) type Constraints = HashSet<VerifoptRval>;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum VarType {
    // TODO add scope backptr in for closures
    // (Option<DefId>, where None == top-level global scope)
    SubScope(Vec<InterpStore>),
    Values(Constraints),
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
    pub cmap: HashMap<MapKey, Box<VarType>>,
    pub wtos: HashMap<(DefId, InstanceDef), BBDeps>,
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
        scope: Option<DefId>,
        key: &MapKey,
        //traverse_backptr: bool,
    ) -> Option<VarType> {
        if scope.is_none() {
            match self.cmap.get(key) {
                Some(boxed) => return Some(*boxed.clone()),
                None => return None,
            }
        }

        match self.cmap.get(&MapKey::ScopeId(scope.unwrap())) {
            Some(vartype) => match *vartype.clone() {
                VarType::SubScope(substores) => {
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
                _ => panic!("not a scope: {:?}", scope.unwrap()),
            },
            None => panic!("undefined scope: {:?}", scope.unwrap()),
        }
    }

    pub fn scoped_update(&mut self, scope: Option<DefId>, key: MapKey, value: Box<VarType>) {
        if scope.is_none() {
            if self.cmap.contains_key(&key) {
                // FIXME MIR is not SSA
                error!("symbol already exists: {:?}", key);
            }

            self.cmap.insert(key, value.clone());
            return;
        }

        match self.cmap.get(&MapKey::ScopeId(scope.unwrap())) {
            Some(vartype) => match *vartype.clone() {
                VarType::SubScope(mut substores) => {
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
                    self.cmap.insert(
                        MapKey::ScopeId(scope.unwrap()),
                        Box::new(VarType::SubScope(substores)),
                    );
                }
                VarType::Values(..) => {
                    panic!("defid is not a scope: {:?}", scope.unwrap());
                }
            },
            None => panic!("undefined scope: {:?}", scope.unwrap()),
        }
    }

    fn merge_vals(&mut self, old_val: VarType, new_val: VarType) -> VarType {
        debug!("old val: {:?}", old_val);
        debug!("new val: {:?}", new_val);

        //let mut cvec = vec![];
        //cvec.push(*value);
        //cvec.push(*old_val.clone());
        //debug!("cvec: {:?}", cvec);

        let mut merged = old_val.clone();
        match (merged.clone(), new_val.clone()) {
            (VarType::Values(_c_merged), VarType::Values(_c_new)) => {
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
                    Box::new(VarType::SubScope(substores)),
                );
            }
            _ => todo!("not impl yet"),
        }
        */
    }
}
