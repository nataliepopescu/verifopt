#![allow(dead_code)]

extern crate rustc_data_structures;
extern crate rustc_middle;

use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::ty::*;

use rustc_data_structures::fx::{FxHashMap as HashMap, FxHashSet as HashSet};

use crate::core::{Type, VerifoptRval};

// FIXME no negative constraints, resolve negative constraints immediately by removing from positive
// constraint set
pub(crate) type Constraints<'tcx> = HashSet<VerifoptRval<'tcx>>;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum VarType<'tcx> {
    // scope w backptr to enclosing scope identifier
    // (None == top-level global scope)
    Scope(Option<DefId>, Vec<(Box<Type>, ConstraintMap<'tcx>)>),
    // set of positive constraints
    // FIXME ignoring types for now, can add back in if need, but type-checking has already
    // happened so maybe we can just trust that
    //Values(Box<Ty<'tcx>>, Constraints<'tcx>),
    // FIXME add type back in?
    Values(Constraints<'tcx>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum MapKey<'tcx> {
    Place(Place<'tcx>),
    // FIXME Option str? if so, remove from VarType::Scope
    ScopeId(DefId),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConstraintMap<'tcx> {
    pub cmap: HashMap<MapKey<'tcx>, Box<VarType<'tcx>>>,
}

impl<'tcx> ConstraintMap<'tcx> {
    pub(crate) fn new() -> Self {
        Self {
            cmap: HashMap::<MapKey<'tcx>, Box<VarType<'tcx>>>::default(),
        }
    }

    pub(crate) fn scoped_get(
        &self,
        scope: Option<DefId>,
        var: &MapKey<'tcx>,
        traverse_backptr: bool,
    ) -> Option<VarType<'tcx>> {
        // first get the `cmap` object pertaining to `this` scope
        if scope.is_none() {
            match self.cmap.get(var) {
                Some(boxed) => return Some(*boxed.clone()),
                None => return None,
            }
        }

        match self.cmap.get(&MapKey::ScopeId(scope.unwrap())) {
            Some(vartype) => match *vartype.clone() {
                VarType::Scope(backptr, instance_vec) => {
                    if instance_vec.len() != 1 {
                        todo!("not impl yet (scope vec)");
                    }
                    // is var in inner_cmap? if not:
                    // - nested funcs: return None
                    // - closures: recursively follow backptr to enclosing scopes
                    // FIXME closures should not recurse past enclosing function
                    match instance_vec[0].1.cmap.get(var) {
                        Some(boxed) => Some(*boxed.clone()),
                        None => {
                            if traverse_backptr {
                                self.scoped_get(backptr, var, traverse_backptr)
                            } else {
                                None
                            }
                        }
                    }
                }
                _ => panic!("not a scope: {:?}", scope.unwrap()),
            },
            None => panic!("undefined scope: {:?}", scope.unwrap()),
        }
    }

    pub(crate) fn scoped_set(
        &mut self,
        scope: Option<DefId>,
        var: MapKey<'tcx>,
        value: Box<VarType<'tcx>>,
    ) {
        // first get the `cmap` object pertaining to `this` scope
        if scope.is_none() {
            // FIXME why are places getting re-assigned, i thought MIR was ssa?
            // commenting this panic out, i guess _during_ compiler compilation this
            // assumption may be violated, but when compiling source code the assumption is true...?
            if self.cmap.contains_key(&var) {
                //panic!("symbol already exists: {:?}", var);
                println!("symbol already exists: {:?}", var);
            }

            self.cmap.insert(var, value.clone());
            return ();
        }

        match self.cmap.get(&MapKey::ScopeId(scope.unwrap())) {
            Some(vartype) => match *vartype.clone() {
                VarType::Scope(backptr, mut instance_vec) => {
                    if instance_vec.len() != 1 {
                        todo!("not impl yet (scope vec)");
                    }
                    if instance_vec[0].1.cmap.contains_key(&var) {
                        panic!("symbol already exists: {:?}", var);
                    }
                    // modify scope w new var
                    instance_vec[0].1.cmap.insert(var, value);
                    self.cmap.insert(
                        MapKey::ScopeId(scope.unwrap()),
                        Box::new(VarType::Scope(backptr, instance_vec)),
                    );
                }
                VarType::Values(..) => {
                    panic!("not a scope: {:?}", scope.unwrap());
                }
            },
            None => panic!("undefined scope: {:?}", scope.unwrap()),
        }
    }
}

