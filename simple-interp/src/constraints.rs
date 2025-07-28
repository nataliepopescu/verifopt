use crate::statement::{Merge, RVal, Type};
use crate::error::Error;

use std::collections::{HashMap, HashSet};

pub type Constraints = (HashSet<RVal>, HashSet<RVal>);

impl Merge<Constraints> for Vec<Constraints> {
    fn merge(&self) -> Result<Option<Constraints>, Error> {
        if self.len() == 0 {
            return Ok(None);
        }
        if self.len() == 1 {
            return Ok(Some(self[0].clone()));
        }

        // typechecker should ensure that all values are of the same type...
        let mut merged = self[0].clone();
        for i in 1..self.len() {
            // merge positive constraints
            if merged.0 != self[i].0 {
                let pos_union: HashSet<_> =
                    merged.0.union(&self[i].0).map(|x| x.clone()).collect();
                merged.0 = pos_union;
            }
            // merge negative constraints
            if merged.1 != self[i].1 {
                let neg_union: HashSet<_> =
                    merged.1.union(&self[i].1).map(|x| x.clone()).collect();
                merged.1 = neg_union;
            }
            let intersection: HashSet<_> = merged
                .0
                .intersection(&merged.1)
                .map(|x| x.clone())
                .collect();
            if !intersection.is_empty() {
                todo!("impl: pos/neg intersection removal");
            }
        }

        Ok(Some(merged))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanConstraints {
    pub true_branch: ConstraintMap,
    pub false_branch: ConstraintMap,
}

impl BooleanConstraints {
    pub fn new(
        true_branch: ConstraintMap,
        false_branch: ConstraintMap,
    ) -> Self {
        Self {
            true_branch,
            false_branch,
        }
    }

    pub fn empty() -> Self {
        Self {
            true_branch: ConstraintMap::new(),
            false_branch: ConstraintMap::new(),
        }
    }

    pub fn flip_constraints(
        bconstraints: BooleanConstraints,
    ) -> BooleanConstraints {
        BooleanConstraints::new(
            bconstraints.false_branch,
            bconstraints.true_branch,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    // scope w backptr to enclosing scope identifier
    // (None == top-level global scope)
    Scope(Option<&'static str>, Vec<(Box<Type>, ConstraintMap)>),
    // sets of positive and negative constraints
    Values(Box<Type>, Constraints),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintMap {
    pub cmap: HashMap<&'static str, Box<VarType>>,
}

impl ConstraintMap {
    pub fn new() -> Self {
        Self {
            cmap: HashMap::<&'static str, Box<VarType>>::new(),
        }
    }

    pub fn scoped_get(
        &self,
        scope: Option<&'static str>,
        var: &'static str,
        traverse_backptr: bool,
    ) -> Result<Option<VarType>, Error> {
        // first get the `cmap` object pertaining to `this` scope
        if scope.is_none() {
            match self.cmap.get(var) {
                Some(boxed) => return Ok(Some(*boxed.clone())),
                None => return Ok(None),
            }
        }

        match self.cmap.get(scope.unwrap()) {
            Some(vartype) => match *vartype.clone() {
                VarType::Scope(backptr, instance_vec) => {
                    if instance_vec.len() != 1 {
                        todo!("not impl yet (scope vec)");
                    }
                    // is var in inner_cmap? if not:
                    // - nested funcs: return None
                    // - closures: recursively follow backptr to enclosing
                    //   scopes
                    // FIXME closures should not recurse past enclosing function
                    match instance_vec[0].1.cmap.get(var) {
                        Some(boxed) => return Ok(Some(*boxed.clone())),
                        None => {
                            if traverse_backptr {
                                return self.scoped_get(
                                    backptr,
                                    var,
                                    traverse_backptr,
                                );
                            } else {
                                return Ok(None);
                            }
                        }
                    }
                }
                _ => return Err(Error::NotAScope(scope.unwrap())),
            },
            None => return Err(Error::UndefinedScope(scope.unwrap())),
        }
    }

    pub fn scoped_set(
        &mut self,
        scope: Option<&'static str>,
        var: &'static str,
        value: Box<VarType>,
    ) -> Result<(), Error> {
        // first get the `cmap` object pertaining to `this` scope
        if scope.is_none() {
            if self.cmap.get(var).is_some() {
                return Err(Error::SymbolAlreadyExists(var));
            }

            self.cmap.insert(var, value.clone());
            return Ok(());
        }

        match self.cmap.get(scope.unwrap()) {
            Some(vartype) => match *vartype.clone() {
                VarType::Scope(backptr, mut instance_vec) => {
                    if instance_vec.len() != 1 {
                        todo!("not impl yet (scope vec)");
                    }
                    if instance_vec[0].1.cmap.get(var).is_some() {
                        return Err(Error::SymbolAlreadyExists(var));
                    }
                    // modify scope w new var
                    instance_vec[0].1.cmap.insert(var, value);
                    self.cmap.insert(
                        scope.unwrap(),
                        Box::new(VarType::Scope(backptr, instance_vec)),
                    );
                }
                VarType::Values(..) => {
                    return Err(Error::NotAScope(scope.unwrap()));
                }
            },
            None => return Err(Error::UndefinedScope(scope.unwrap())),
        }

        Ok(())
    }
}

impl Merge<ConstraintMap> for Vec<ConstraintMap> {
    fn merge(&self) -> Result<Option<ConstraintMap>, Error> {
        if self.len() == 0 {
            return Ok(None);
        }
        if self.len() == 1 {
            return Ok(Some(self[0].clone()));
        }

        let mut merged = self[0].clone();
        for i in 1..self.len() {
            for (key, val) in self[i].clone().cmap.iter() {
                match merged.cmap.get_mut(key) {
                    // mismatched types should be caught by typechecker
                    Some(mval) => match (*mval.clone(), *val.clone()) {
                        (
                            VarType::Scope(backptr_a, mut instance_vec_a),
                            VarType::Scope(backptr_b, mut instance_vec_b),
                        ) => {
                            if instance_vec_a.len() != 1
                                || instance_vec_b.len() != 1
                            {
                                todo!("not impl yet (scope vec)");
                            }

                            let functype_a = instance_vec_a[0].0.clone();
                            let functype_b = instance_vec_b[0].0.clone();
                            let cmap_a = instance_vec_a[0].1.clone();
                            let cmap_b = instance_vec_b[0].1.clone();

                            if backptr_a != backptr_b {
                                return Err(Error::BackpointersDiffer());
                            }
                            if functype_a != functype_b {
                                if cmap_a != cmap_b {
                                    instance_vec_a.append(&mut instance_vec_b);
                                    *mval = Box::new(VarType::Scope(
                                        backptr_a,
                                        instance_vec_a,
                                    ));
                                }
                            } else {
                                if cmap_a != cmap_b {
                                    let inner_cmap = vec![cmap_a, cmap_b];
                                    match inner_cmap.merge() {
                                        Ok(Some(merged_inner)) => {
                                            *mval = Box::new(VarType::Scope(
                                                backptr_a,
                                                vec![(
                                                    functype_a,
                                                    merged_inner,
                                                )],
                                            ));
                                        }
                                        Ok(None) => {
                                            todo!("got none for inner cmap")
                                        }
                                        err @ Err(_) => return err,
                                    }
                                }
                            }
                        }
                        (
                            VarType::Values(valtype_a, (pos_a, neg_a)),
                            VarType::Values(valtype_b, (pos_b, neg_b)),
                        ) => {
                            if valtype_a != valtype_b {
                                return Err(Error::TypesDiffer());
                            }
                            let mut pos = pos_a.clone();
                            let mut neg = neg_a.clone();
                            if pos_a != pos_b {
                                let pos_union: HashSet<_> = pos_a
                                    .union(&pos_b)
                                    .map(|x| x.clone())
                                    .collect();
                                pos = pos_union;
                            }
                            if neg_a != neg_b {
                                let neg_union: HashSet<_> = neg_a
                                    .union(&neg_b)
                                    .map(|x| x.clone())
                                    .collect();
                                neg = neg_union;
                            }
                            merged.cmap.insert(
                                key,
                                Box::new(VarType::Values(
                                    valtype_a,
                                    (pos, neg),
                                )),
                            );
                        }
                        _ => return Err(Error::IncomparableVarTypes()),
                    },
                    None => {
                        merged.cmap.insert(key, val.clone());
                    }
                }
            }
        }

        // remove all negative constraints are also positive constraints
        // (intersections)
        for (_, val) in merged.cmap.iter_mut() {
            match &mut **val {
                &mut VarType::Values(_, (ref pos, ref mut neg)) => {
                    let intersection: HashSet<_> =
                        pos.intersection(&neg).map(|x| x.clone()).collect();
                    let diff: HashSet<_> = neg
                        .difference(&intersection)
                        .map(|x| x.clone())
                        .collect();
                    *neg = diff;
                }
                _ => continue,
            }
        }

        Ok(Some(merged))
    }
}

pub trait Difference<T> {
    fn diff(&mut self, other: &T) -> Result<(), Error>;
}

impl Difference<ConstraintMap> for ConstraintMap {
    fn diff(&mut self, other: &ConstraintMap) -> Result<(), Error> {
        // FIXME multi-layer cmap (i.e. other w scopes)

        for (other_key, other_val) in other.cmap.iter() {
            match self.cmap.get(other_key) {
                Some(self_val) => {
                    // mismatched types should be caught by typechecker
                    match (*self_val.clone(), *other_val.clone()) {
                        (
                            VarType::Values(valtype_self, (self_pos, self_neg)),
                            VarType::Values(
                                valtype_other,
                                (other_pos, other_neg),
                            ),
                        ) => {
                            if valtype_self != valtype_other {
                                return Err(Error::TypesDiffer());
                            }
                            let diff_pos: HashSet<_> = self_pos
                                .difference(&other_pos)
                                .map(|x| x.clone())
                                .collect();
                            let diff_neg: HashSet<_> = self_neg
                                .difference(&other_neg)
                                .map(|x| x.clone())
                                .collect();
                            self.cmap.insert(
                                other_key,
                                Box::new(VarType::Values(
                                    valtype_self,
                                    (diff_pos, diff_neg),
                                )),
                            );
                        }
                        _ => todo!("scopes ever?"),
                    }
                }
                None => continue,
            }
        }

        Ok(())
    }
}

