#![allow(dead_code)]

extern crate rustc_data_structures;
extern crate rustc_middle;

use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
//use rustc_middle::ty::ParamTy;
//use rustc_span::Ident;
use rustc_span::symbol::Symbol;

use rustc_data_structures::fx::{FxHashMap as HashMap, FxHashSet as HashSet};

use crate::core::{Merge, Type, VerifoptRval};
use crate::error::Error;
use crate::wto::BBDeps;

// FIXME no negative constraints, resolve negative constraints immediately by removing from positive
// constraint set
pub(crate) type Constraints<'tcx> = HashSet<VerifoptRval<'tcx>>;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum VarType<'tcx> {
    // scope w backptr to enclosing scope identifier
    // (None == top-level global scope)
    // FIXME change meaning of backptr: only closures have one (not regular functions)
    SubScope(Option<DefId>, Vec<(Box<Type>, ConstraintMap<'tcx>)>),
    // set of positive constraints
    // FIXME ignoring types for now, can add back in if need, but type-checking has already
    // happened so maybe we can just trust that
    //Values(Box<Ty<'tcx>>, Constraints<'tcx>),
    // FIXME add type back in?
    Values(Constraints<'tcx>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum MapKey<'tcx> {
    //Arg(Option<Ident>), //Place<'tcx>),
    Place(Place<'tcx>),
    Generic(Symbol),
    // FIXME Option str? if so, remove from VarType::Scope
    ScopeId(DefId),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConstraintMap<'tcx> {
    pub cmap: HashMap<MapKey<'tcx>, Box<VarType<'tcx>>>,
    pub wtos: HashMap<DefId, BBDeps>,
}

impl<'tcx> ConstraintMap<'tcx> {
    pub(crate) fn new() -> Self {
        Self {
            cmap: HashMap::<MapKey<'tcx>, Box<VarType<'tcx>>>::default(),
            wtos: HashMap::<DefId, BBDeps>::default(),
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
                VarType::SubScope(backptr, subscope_cmaps_vec) => {
                    if subscope_cmaps_vec.len() != 1 {
                        todo!("not impl yet (scope vec)");
                    }
                    //println!("subscope_cmaps_vec: {:?}", subscope_cmaps_vec);
                    //println!("subscope_cmaps_vec[0].1.cmap: {:?}", subscope_cmaps_vec[0].1.cmap);
                    //println!("var: {:?}", var);

                    // is var in inner_cmap? if not:
                    // - nested funcs: return None
                    // - closures: recursively follow backptr to enclosing scopes
                    // FIXME closures should not recurse past enclosing function
                    match subscope_cmaps_vec[0].1.cmap.get(var) {
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

    pub(crate) fn scoped_add(
        &mut self,
        scope: Option<DefId>,
        var: MapKey<'tcx>,
        value: Box<VarType<'tcx>>,
    ) {
        // first get the `cmap` object pertaining to `this` scope
        if scope.is_none() {
            if self.cmap.contains_key(&var) {
                // FIXME MIR is not SSA
                println!("symbol already exists: {:?}", var);
            }

            self.cmap.insert(var, value.clone());
            return ();
        }

        match self.cmap.get(&MapKey::ScopeId(scope.unwrap())) {
            Some(vartype) => match *vartype.clone() {
                VarType::SubScope(backptr, mut subscope_cmaps_vec) => {
                    if subscope_cmaps_vec.len() != 1 {
                        todo!("not impl yet (scope vec)");
                    }
                    let subscope_cmap = &mut subscope_cmaps_vec[0].1.cmap;
                    let old_vartype_opt = subscope_cmap.get(&var);
                    match old_vartype_opt {
                        Some(old_vartype) => {
                            let mut cvec = vec![];
                            cvec.push(*value);
                            cvec.push(*old_vartype.clone());
                            //println!("cvec: {:?}", cvec);
                            match cvec.merge() {
                                Ok(Some(new_vartype)) => {
                                    subscope_cmap.insert(var, Box::new(new_vartype));
                                    self.cmap.insert(
                                        MapKey::ScopeId(scope.unwrap()),
                                        Box::new(VarType::SubScope(backptr, subscope_cmaps_vec)),
                                    );
                                }
                                _ => todo!("not impl yet"),
                            }
                        }
                        None => {
                            // modify scope w new var
                            subscope_cmap.insert(var, value);
                            self.cmap.insert(
                                MapKey::ScopeId(scope.unwrap()),
                                Box::new(VarType::SubScope(backptr, subscope_cmaps_vec)),
                            );
                        }
                    }
                }
                VarType::Values(..) => {
                    panic!("not a scope: {:?}", scope.unwrap());
                }
            },
            None => panic!("undefined scope: {:?}", scope.unwrap()),
        }
    }
}

// Implement Merge trait

impl<'tcx> Merge<VarType<'tcx>> for Vec<VarType<'tcx>> {
    fn merge(&self) -> Result<Option<VarType<'tcx>>, Error> {
        if self.is_empty() {
            return Ok(None);
        }

        if self.len() == 1 {
            return Ok(Some(self[0].clone()));
        }

        let mut merged_vartype = self[0].clone();
        for new_vartype in self.iter() {
            match (merged_vartype.clone(), new_vartype.clone()) {
                (VarType::Values(constraints_a), VarType::Values(constraints_b)) => {
                    if constraints_a != constraints_b {
                        let union: HashSet<_> =
                            constraints_a.union(&constraints_b).cloned().collect();
                        merged_vartype = VarType::Values(union);
                    }
                }
                _ => todo!("merge scopes"),
            }
        }

        Ok(Some(merged_vartype))
    }
}

/*
impl<'tcx> Merge<Constraints<'tcx>> for Vec<Constraints<'tcx>> {
    fn merge(&self) -> Result<Option<Constraints<'tcx>>, Error> {
        if self.is_empty() {
            return Ok(None);
        }

        if self.len() == 1 {
            return Ok(Some(self[0].clone()));
        }

        let mut merged_constraints = self[0].clone();
        for new_constraint_set in self.iter() {
            if merged_constraints != *new_constraint_set {
                let union: HashSet<_> = merged_constraints.union(new_constraint_set).cloned().collect();
                merged_constraints = union;
            }
        }

        Ok(Some(merged_constraints))
    }
}
*/

impl<'tcx> Merge<ConstraintMap<'tcx>> for Vec<ConstraintMap<'tcx>> {
    fn merge(&self) -> Result<Option<ConstraintMap<'tcx>>, Error> {
        if self.is_empty() {
            return Ok(None);
        }

        if self.len() == 1 {
            return Ok(Some(self[0].clone()));
        }

        let mut merged = self[0].clone();
        let mut first = true;
        for cmap in self.iter() {
            if first {
                first = false;
                continue;
            }
            for (key, val) in cmap.clone().cmap.iter() {
                match merged.cmap.get_mut(key) {
                    // mismatched types should be caught by typechecker
                    Some(mval) => match (*mval.clone(), *val.clone()) {
                        (VarType::SubScope(_, _), VarType::SubScope(_, _)) => {
                            todo!("merging two scopes");
                        }
                        (VarType::Values(constraints_a), VarType::Values(constraints_b)) => {
                            println!("constraints_a: {:?}", constraints_a);
                            println!("constraints_b: {:?}", constraints_b);
                            let mut constraints = constraints_a.clone();
                            if constraints_a != constraints_b {
                                let union: HashSet<_> = constraints_a.union(&constraints_b).cloned().collect();
                                println!("union constraints: {:?}", union);
                                constraints = union;
                            }
                            merged.cmap.insert(
                                key.clone(),
                                Box::new(VarType::Values(constraints))
                            );
                        }
                        _ => panic!("incomparable VarTypes"),
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
