use crate::{
    AssignmentRVal, BooleanStatement, Error, FuncVal, Funcs, Merge, RVal,
    Statement, Type,
};
use std::collections::{HashMap, HashSet};

pub type Constraints = (HashSet<RVal>, HashSet<RVal>);

impl Merge<Constraints> for Vec<Constraints> {
    fn merge(&self) -> Result<Constraints, Error> {
        if self.len() == 0 {
            return Err(Error::VecSize());
        }
        if self.len() == 1 {
            return Ok(self[0].clone());
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

        Ok(merged)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanConstraints {
    true_branch: ConstraintMap,
    false_branch: ConstraintMap,
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    // scope w backptr to enclosing scope identifier
    // (None == top-level global scope)
    Scope(Box<Type>, Option<&'static str>, ConstraintMap),
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
                VarType::Scope(_, backptr, inner_cmap) => {
                    // is var in inner_cmap? if not:
                    // - nested funcs: return None
                    // - closures: recursively follow backptr to enclosing
                    //   scopes
                    // FIXME closures should not recurse past enclosing function
                    match inner_cmap.cmap.get(var) {
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
                VarType::Scope(functype, backptr, mut inner_cmap) => {
                    if inner_cmap.cmap.get(var).is_some() {
                        return Err(Error::SymbolAlreadyExists(var));
                    }
                    // modify scope w new var
                    inner_cmap.cmap.insert(var, value);
                    self.cmap.insert(
                        scope.unwrap(),
                        Box::new(VarType::Scope(functype, backptr, inner_cmap)),
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
    fn merge(&self) -> Result<ConstraintMap, Error> {
        if self.len() == 0 {
            return Err(Error::VecSize());
        }
        if self.len() == 1 {
            return Ok(self[0].clone());
        }

        let mut merged = self[0].clone();
        for i in 1..self.len() {
            for (key, val) in self[i].clone().cmap.iter() {
                match merged.cmap.get_mut(key) {
                    // mismatched types should be caught by typechecker
                    Some(mval) => match (*mval.clone(), *val.clone()) {
                        (
                            VarType::Scope(functype_a, backptr_a, cmap_a),
                            VarType::Scope(functype_b, backptr_b, cmap_b),
                        ) => {
                            if functype_a != functype_b {
                                return Err(Error::TypesDiffer());
                            }
                            if backptr_a != backptr_b {
                                return Err(Error::BackpointersDiffer());
                            }
                            if cmap_a != cmap_b {
                                let inner_cmap = vec![cmap_a, cmap_b];
                                match inner_cmap.merge() {
                                    Ok(merged_inner) => {
                                        *mval = Box::new(VarType::Scope(
                                            functype_a,
                                            backptr_a,
                                            merged_inner,
                                        ));
                                    }
                                    err @ Err(_) => return err,
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

        Ok(merged)
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

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interp(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        scope: Option<&'static str>,
        stmt: &Statement,
    ) -> Result<Option<Constraints>, Error> {
        match stmt {
            Statement::FuncDef(..)
            | Statement::Struct(..)
            | Statement::TraitDef(..)
            | Statement::TraitImpl(..) => Ok(None),
            Statement::Sequence(stmt_vec) => {
                self.interp_seq(funcs, cmap, scope, stmt_vec)
            }
            Statement::Assignment(var, value) => {
                self.interp_assignment(funcs, cmap, scope, var, value)
            }
            Statement::Print(var) => {
                self.interp_print(var);
                Ok(None)
            }
            Statement::Conditional(condition, true_branch, false_branch) => {
                self.interp_conditional(
                    funcs,
                    cmap,
                    scope,
                    &*condition,
                    &*true_branch,
                    &*false_branch,
                )
            }
            Statement::Switch(val, vec) => {
                self.interp_switch(funcs, cmap, scope, val, vec)
            }
            Statement::Return(rval) => {
                self.interp_return(funcs, cmap, scope, rval)
            }
            Statement::InvokeFunc(name, args) => {
                self.interp_invoke(funcs, cmap, scope, name, args)
            }
        }
    }

    fn interp_seq(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        scope: Option<&'static str>,
        stmt_vec: &Vec<Box<Statement>>,
    ) -> Result<Option<Constraints>, Error> {
        let mut last_ret = None;
        for stmt in stmt_vec.iter() {
            let res = self.interp(&funcs, cmap, scope, &*stmt);
            if res.is_err() {
                return res;
            }
            last_ret = res.unwrap();
        }
        Ok(last_ret)
    }

    fn interp_assignment(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        scope: Option<&'static str>,
        var: &'static str,
        value: &AssignmentRVal,
    ) -> Result<Option<Constraints>, Error> {
        match value {
            AssignmentRVal::RVal(RVal::Num(num)) => {
                let res = cmap.scoped_set(
                    scope,
                    var,
                    Box::new(VarType::Values(
                        Box::new(Type::Int()),
                        (HashSet::from([RVal::Num(*num)]), HashSet::new()),
                    )),
                );

                if res.is_err() {
                    return Err(res.err().unwrap());
                }
            }
            AssignmentRVal::RVal(RVal::Struct(struct_name, field_values)) => {
                let res = cmap.scoped_set(
                    scope,
                    var,
                    Box::new(VarType::Values(
                        Box::new(Type::Struct()), // FIXME
                        (
                            HashSet::from([RVal::Struct(
                                *struct_name,
                                field_values.to_vec(),
                            )]),
                            HashSet::new(),
                        ),
                    )),
                );

                if res.is_err() {
                    return Err(res.err().unwrap());
                }
            }
            AssignmentRVal::RVal(RVal::Var(varname)) => {
                match cmap.scoped_get(scope, varname, false) {
                    Ok(Some(val)) => match val {
                        set @ VarType::Values(..) => {
                            let res =
                                cmap.scoped_set(scope, var, Box::new(set));

                            if res.is_err() {
                                return Err(res.err().unwrap());
                            }
                        }
                        _ => todo!("not impl yet"),
                    },
                    Ok(None) => match funcs.funcs.get(&(varname, None)) {
                        Some(funcval) => {
                            let functype = Type::Func(
                                funcval.paramtypes.clone(),
                                funcval.rettype.clone(),
                            );
                            let res = cmap.scoped_set(
                                scope,
                                var,
                                Box::new(VarType::Values(
                                    Box::new(functype),
                                    (
                                        HashSet::from([RVal::Var(varname)]),
                                        HashSet::new(),
                                    ),
                                )),
                            );

                            if res.is_err() {
                                return Err(res.err().unwrap());
                            }
                        }
                        None => return Err(Error::UndefinedSymbol(varname)),
                    },
                    Err(err) => return Err(err),
                };
            }
            AssignmentRVal::Statement(stmt) => match *stmt.clone() {
                Statement::InvokeFunc(name, args) => {
                    // assume func has retval (given typechecking) but return
                    // err if none
                    match self.interp_invoke(funcs, cmap, scope, name, &args) {
                        Ok(Some(constraints)) => {
                            let functype;
                            match cmap.cmap.get(name) {
                                Some(val) => match *val.clone() {
                                    VarType::Values(stored_functype, _)
                                    | VarType::Scope(stored_functype, ..) => {
                                        functype = stored_functype;
                                    }
                                },
                                None => {
                                    return Err(Error::UndefinedSymbol(name));
                                }
                            }

                            match *functype {
                                Type::Func(_, rettype) => {
                                    let res = cmap.scoped_set(
                                        scope,
                                        var,
                                        Box::new(VarType::Values(
                                            rettype.unwrap(),
                                            constraints,
                                        )),
                                    );

                                    if res.is_err() {
                                        return Err(res.err().unwrap());
                                    }
                                }
                                _ => return Err(Error::NotAFunction(name)),
                            }
                        }
                        Ok(None) => return Err(Error::CannotAssignNoneRetval()),
                        err @ Err(_) => return err,
                    }
                }
                _ => return Err(Error::InvalidAssignmentRVal()),
            },
        }

        Ok(None)
    }

    fn interp_print(&self, var: &'static str) {
        println!("{:#?}", var);
    }

    fn interp_bool(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        scope: Option<&'static str>,
        b_stmt: BooleanStatement,
    ) -> Result<(BooleanStatement, BooleanConstraints), Error> {
        match b_stmt {
            BooleanStatement::True()
            | BooleanStatement::False()
            | BooleanStatement::TrueOrFalse() => {
                Ok((b_stmt, BooleanConstraints::empty()))
            }
            BooleanStatement::Not(inner_b_stmt) => {
                self.interp_not(funcs, cmap, scope, *inner_b_stmt)
            }
            BooleanStatement::Equals(lhs, rhs) => {
                self.interp_equals(funcs, cmap, scope, lhs, rhs)
            }
        }
    }

    fn flip_constraints(
        &self,
        bconstraints: BooleanConstraints,
    ) -> BooleanConstraints {
        BooleanConstraints::new(
            bconstraints.false_branch,
            bconstraints.true_branch,
        )
    }

    fn interp_not(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        scope: Option<&'static str>,
        b_stmt: BooleanStatement,
    ) -> Result<(BooleanStatement, BooleanConstraints), Error> {
        match self.interp_bool(funcs, cmap, scope, b_stmt) {
            Ok(b_res) => Ok((!b_res.0, self.flip_constraints(b_res.1))),
            err @ Err(_) => err,
        }
    }

    fn interp_rval(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        scope: Option<&'static str>,
        rval: RVal,
    ) -> Result<Constraints, Error> {
        match rval.clone() {
            RVal::Num(_) => Ok((HashSet::from([rval]), HashSet::new())),
            RVal::Var(var) => match cmap.scoped_get(scope, var, false) {
                Ok(Some(val)) => match val {
                    VarType::Values(_, constraints) => {
                        return Ok(constraints.clone());
                    }
                    _ => todo!("should be a func"),
                },
                Ok(None) => match funcs.funcs.get(&(var, None)) {
                    Some(_) => Ok((HashSet::from([rval]), HashSet::new())),
                    None => Err(Error::UndefinedSymbol(var)),
                },
                Err(err) => return Err(err),
            },
            RVal::Struct(..) => Ok((HashSet::from([rval]), HashSet::new())),
        }
    }

    fn constraints_to_vecs(
        &self,
        constraints: &Constraints,
    ) -> (Vec<RVal>, Vec<RVal>) {
        let pos_vec: Vec<_> = constraints.0.clone().into_iter().collect();
        let neg_vec: Vec<_> = constraints.1.clone().into_iter().collect();
        (pos_vec, neg_vec)
    }

    fn interp_equals(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        scope: Option<&'static str>,
        lhs: RVal,
        rhs: RVal,
    ) -> Result<(BooleanStatement, BooleanConstraints), Error> {
        let lhs_res = self.interp_rval(&funcs, &cmap, scope, lhs.clone());
        if lhs_res.is_err() {
            return Err(lhs_res.err().unwrap());
        }
        let rhs_res = self.interp_rval(&funcs, &cmap, scope, rhs.clone());
        if rhs_res.is_err() {
            return Err(rhs_res.err().unwrap());
        }

        let lhs_vecs = self.constraints_to_vecs(&lhs_res.unwrap());
        let rhs_vecs = self.constraints_to_vecs(&rhs_res.unwrap());

        // eval if if only a single positive constraint
        if lhs_vecs.0.len() == 1
            && lhs_vecs.1.len() == 0
            && rhs_vecs.0.len() == 1
            && rhs_vecs.1.len() == 0
        {
            match (lhs_vecs.0[0].clone(), rhs_vecs.0[0].clone()) {
                (RVal::Num(lnum), RVal::Num(rnum)) => {
                    if lnum == rnum {
                        return Ok((
                            BooleanStatement::True(),
                            BooleanConstraints::empty(),
                        ));
                    } else {
                        return Ok((
                            BooleanStatement::False(),
                            BooleanConstraints::empty(),
                        ));
                    }
                }
                (RVal::Var(lfp), RVal::Var(rfp)) => {
                    if lfp == rfp {
                        return Ok((
                            BooleanStatement::True(),
                            BooleanConstraints::empty(),
                        ));
                    } else {
                        return Ok((
                            BooleanStatement::False(),
                            BooleanConstraints::empty(),
                        ));
                    }
                }
                (_, _) => {
                    return Err(Error::IncomparableTypes(
                        lhs_vecs.0[0].clone(),
                        rhs_vecs.0[0].clone(),
                    ));
                }
            }
        } else {
            match (lhs.clone(), rhs.clone()) {
                // FIXME impl for nums
                (RVal::Var(a), RVal::Var(b)) => {
                    // right now we know this function is only ever called by
                    // the equality comparison, but in the future we'll need a
                    // check which operator we're evaluating TODO

                    let vartype;
                    // get types of each var + make sure they're the same
                    match (cmap.cmap.get(a), cmap.cmap.get(b)) {
                        (Some(val_a), Some(val_b)) => {
                            match (*val_a.clone(), *val_b.clone()) {
                                (
                                    VarType::Values(type_a, _),
                                    VarType::Values(type_b, _),
                                )
                                | (
                                    VarType::Values(type_a, _),
                                    VarType::Scope(type_b, ..),
                                )
                                | (
                                    VarType::Scope(type_a, ..),
                                    VarType::Values(type_b, _),
                                )
                                | (
                                    VarType::Scope(type_a, ..),
                                    VarType::Scope(type_b, ..),
                                ) => {
                                    if type_a != type_b {
                                        return Err(Error::TypesDiffer());
                                    }
                                    vartype = type_a;
                                }
                            }
                        }
                        (None, _) => return Err(Error::UndefinedSymbol(a)),
                        (_, None) => return Err(Error::UndefinedSymbol(b)),
                    }

                    // in the true branch, we know that lhs == rhs, thus
                    // new constraints: lhs {(rhs), ()}, rhs {(lhs), ()}
                    let mut true_branch = ConstraintMap::new();
                    true_branch.cmap.insert(
                        a,
                        Box::new(VarType::Values(
                            vartype.clone(),
                            (HashSet::from([rhs.clone()]), HashSet::new()),
                        )),
                    );
                    true_branch.cmap.insert(
                        b,
                        Box::new(VarType::Values(
                            vartype.clone(),
                            (HashSet::from([lhs.clone()]), HashSet::new()),
                        )),
                    );

                    // in the false branch, we know that lhs != rhs, thus
                    // new constraints: lhs {(), (rhs)}, rhs {(), (rhs)}
                    let mut false_branch = ConstraintMap::new();
                    false_branch.cmap.insert(
                        a,
                        Box::new(VarType::Values(
                            vartype.clone(),
                            (HashSet::new(), HashSet::from([rhs])),
                        )),
                    );
                    false_branch.cmap.insert(
                        b,
                        Box::new(VarType::Values(
                            vartype,
                            (HashSet::new(), HashSet::from([lhs])),
                        )),
                    );

                    return Ok((
                        BooleanStatement::TrueOrFalse(),
                        BooleanConstraints::new(true_branch, false_branch),
                    ));
                }
                _ => todo!("not impl yet"),
            }
        }
    }

    fn interp_conditional(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        scope: Option<&'static str>,
        condition: &BooleanStatement,
        true_branch: &Statement,
        false_branch: &Statement,
    ) -> Result<Option<Constraints>, Error> {
        let mut res_cmap: Vec<ConstraintMap> = vec![];

        // FIXME mod store if have effects
        match self.interp_bool(funcs, cmap, scope, condition.clone()) {
            Ok((bool_res, bconstraints)) => {
                let true_cmap = cmap.clone();
                let false_cmap = cmap.clone();
                if self.possible(&bool_res) {
                    match vec![
                        true_cmap.clone(),
                        bconstraints.true_branch.clone(),
                    ]
                    .merge()
                    {
                        Ok(mut new_cmap) => match self.interp(
                            funcs,
                            &mut new_cmap,
                            scope,
                            true_branch,
                        ) {
                            Ok(_) => {
                                // remove true branch constraints from
                                // resulting cmap (safe b/c of SSA guarantee)
                                match new_cmap.diff(&bconstraints.true_branch) {
                                    Ok(()) => res_cmap.push(new_cmap),
                                    Err(err) => return Err(err),
                                }
                            }
                            err @ Err(_) => return err,
                        },
                        Err(err) => return Err(err),
                    }
                }
                if self.possible(!&bool_res) {
                    match vec![
                        false_cmap.clone(),
                        bconstraints.false_branch.clone(),
                    ]
                    .merge()
                    {
                        Ok(mut new_cmap) => match self.interp(
                            funcs,
                            &mut new_cmap,
                            scope,
                            false_branch,
                        ) {
                            Ok(_) => {
                                // remove false branch constraints from
                                // resulting cmap (safe b/c of SSA guarantee)
                                match new_cmap.diff(&bconstraints.false_branch)
                                {
                                    Ok(()) => res_cmap.push(new_cmap),
                                    Err(err) => return Err(err),
                                }
                            }
                            err @ Err(_) => return err,
                        },
                        Err(err) => return Err(err),
                    }
                }
            }
            Err(c_err) => return Err(c_err),
        }

        match res_cmap.merge() {
            Ok(new_cmap) => {
                *cmap = new_cmap;
                Ok(None)
            }
            Err(err) => Err(err),
        }
    }

    fn possible(&self, possible_b: &BooleanStatement) -> bool {
        match possible_b {
            BooleanStatement::True() => true,
            BooleanStatement::False() => false,
            BooleanStatement::TrueOrFalse() => true,
            _ => panic!("boolean statement not fully evaluated"),
        }
    }

    fn interp_switch(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        scope: Option<&'static str>,
        val: &RVal,
        vec: &Vec<(RVal, Box<Statement>)>,
    ) -> Result<Option<Constraints>, Error> {
        // FIXME mod store if have effects
        let resolved_constraints = match val {
            num @ RVal::Num(_) => {
                (HashSet::from([num.clone()]), HashSet::new())
            }
            RVal::Var(varname) => {
                match cmap.scoped_get(scope, varname, false) {
                    Ok(Some(vartype)) => match vartype {
                        VarType::Values(_, constraints) => constraints.clone(),
                        _ => return Err(Error::NoSwitchOnFuncPtr()),
                    },
                    Ok(None) => return Err(Error::UndefinedSymbol(varname)),
                    Err(err) => return Err(err),
                }
            }
            struct_inst @ RVal::Struct(..) => {
                (HashSet::from([struct_inst.clone()]), HashSet::new())
            }
        };

        // filter out switch cases not possible given positive constraints
        let mut filtered: Vec<_> = vec
            .clone()
            .into_iter()
            .filter(|(case_value, _)| {
                resolved_constraints.0.contains(case_value)
            })
            .collect();

        // filter out switch cases not possible given negative constraints
        filtered = filtered
            .clone()
            .into_iter()
            .filter(|(case_value, _)| {
                !resolved_constraints.1.contains(case_value)
            })
            .collect();

        // loop to interp all such elems
        let mut res_cmap: Vec<ConstraintMap> = Vec::new();
        for (_, vec_stmt) in filtered.iter() {
            let mut scoped_cmap = cmap.clone();
            match self.interp(funcs, &mut scoped_cmap, scope, &*vec_stmt) {
                Ok(_) => res_cmap.push(scoped_cmap),
                err @ Err(_) => return err,
            }
        }

        match res_cmap.merge() {
            Ok(new_cmap) => {
                *cmap = new_cmap;
                Ok(None)
            }
            Err(err) => Err(err),
        }
    }

    fn interp_return(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        scope: Option<&'static str>,
        rval: &RVal,
    ) -> Result<Option<Constraints>, Error> {
        match rval {
            num @ RVal::Num(_) => {
                return Ok(Some((
                    HashSet::from([num.clone()]),
                    HashSet::new(),
                )));
            }
            RVal::Var(varname) => {
                match cmap.scoped_get(scope, varname, false) {
                    Ok(Some(vartype)) => match vartype {
                        VarType::Values(_, constraints) => {
                            Ok(Some(constraints))
                        }
                        VarType::Scope(..) => match funcs
                            .funcs
                            .get(&(varname, None))
                        {
                            Some(_) => Ok(Some((
                                HashSet::from([RVal::Var(*varname)]),
                                HashSet::new(),
                            ))),
                            None => {
                                return Err(Error::UndefinedSymbol(varname));
                            }
                        },
                    },
                    Ok(None) => match funcs.funcs.get(&(varname, None)) {
                        Some(_) => panic!(
                            "IP BUG: func should have been a scope in cmap"
                        ),
                        None => return Err(Error::UndefinedSymbol(varname)),
                    },
                    Err(err) => return Err(err),
                }
            }
            struct_inst @ RVal::Struct(..) => {
                return Ok(Some((
                    HashSet::from([struct_inst.clone()]),
                    HashSet::new(),
                )));
            }
        }
    }

    fn resolve_args(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        scope: Option<&'static str>,
        name: &'static str,
        funcval: &FuncVal,
        args: &Vec<&'static str>,
    ) -> Result<(), Error> {
        let mut func_cmap = ConstraintMap::new();
        for (param, arg) in std::iter::zip(funcval.params.clone(), args) {
            match cmap.scoped_get(scope, arg, false) {
                Ok(Some(vartype)) => match vartype {
                    // add args to scope
                    VarType::Values(valtype, constraints) => {
                        func_cmap.cmap.insert(
                            param,
                            Box::new(VarType::Values(
                                valtype,
                                constraints.clone(),
                            )),
                        )
                    }
                    _ => todo!("not impl yet (funcs as args)"),
                },
                Ok(None) => match funcs.funcs.get(&(arg, None)) {
                    Some(_funcval) => {
                        todo!("not impl yet (funcs as args)");
                    }
                    None => return Err(Error::UndefinedSymbol(arg)),
                },
                Err(err) => return Err(err),
            };
        }

        cmap.cmap.insert(
            name,
            Box::new(VarType::Scope(
                Box::new(Type::Func(
                    funcval.paramtypes.clone(),
                    funcval.rettype.clone(),
                )),
                scope,
                func_cmap,
            )),
        );

        Ok(())
    }

    fn interp_indirect_invoke_helper(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        scope: Option<&'static str>,
        name: &'static str,
        val: &RVal,
        args: &Vec<&'static str>,
    ) -> Result<Option<Constraints>, Error> {
        match val {
            RVal::Var(varname) => match funcs.funcs.get(&(varname, None)) {
                Some(funcval) => {
                    let res = self.resolve_args(
                        funcs, cmap, scope, varname, &funcval, args,
                    );
                    if res.is_err() {
                        return Err(res.err().unwrap());
                    }

                    match self.interp(
                        funcs,
                        cmap,
                        Some(varname),
                        &*funcval.body,
                    ) {
                        Ok(constraints) => return Ok(constraints),
                        err @ Err(_) => return err,
                    }
                }
                None => return Err(Error::UndefinedSymbol(varname)),
            },
            _ => return Err(Error::NotAFunction(name)),
        }
    }

    fn interp_indirect_invoke(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        scope: Option<&'static str>,
        name: &'static str,
        constraints: &Constraints,
        args: &Vec<&'static str>,
    ) -> Result<Option<Constraints>, Error> {
        let mut res_cmap: Vec<ConstraintMap> = vec![];
        let mut constraints_vec: Vec<Constraints> = vec![];
        // TODO what to do with constraints.1 (neg)
        for val in constraints.0.iter() {
            let mut cmap_clone = cmap.clone();
            match self.interp_indirect_invoke_helper(
                funcs,
                &mut cmap_clone,
                scope,
                name,
                val,
                args,
            ) {
                Ok(constraints) => {
                    res_cmap.push(cmap_clone);
                    if constraints.is_some() {
                        constraints_vec.push(constraints.unwrap());
                    }
                }
                err @ Err(_) => return err,
            }
        }

        let res = res_cmap.merge();
        if res.is_err() {
            return Err(res.err().unwrap());
        }
        *cmap = res.unwrap();

        if constraints_vec.len() > 0 {
            return match constraints_vec.merge() {
                Ok(constraints) => Ok(Some(constraints)),
                Err(err) => Err(err),
            };
        }
        Ok(None)
    }

    fn interp_direct_invoke(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        scope: Option<&'static str>,
        name: &'static str,
        args: &Vec<&'static str>,
    ) -> Result<Option<Constraints>, Error> {
        match funcs.funcs.get(&(name, None)) {
            Some(funcval) => {
                let res =
                    self.resolve_args(funcs, cmap, scope, name, &funcval, args);
                if res.is_err() {
                    return Err(res.err().unwrap());
                }

                match self.interp(funcs, cmap, Some(name), &*funcval.body) {
                    res @ Ok(_) => res,
                    err @ Err(_) => err,
                }
            }
            None => return Err(Error::UndefinedSymbol(name)),
        }
    }

    fn interp_invoke(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        scope: Option<&'static str>,
        name: &'static str,
        args: &Vec<&'static str>,
    ) -> Result<Option<Constraints>, Error> {
        match cmap.scoped_get(scope, name, false) {
            Ok(Some(vartype)) => match vartype {
                VarType::Values(_, constraints) => self.interp_indirect_invoke(
                    funcs,
                    cmap,
                    scope,
                    name,
                    &constraints,
                    args,
                ),
                _ => panic!("should not get scope here"),
            },
            Ok(None) => {
                self.interp_direct_invoke(funcs, cmap, scope, name, args)
            }
            Err(err) => return Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Statement::{
        Assignment, Conditional, FuncDef, InvokeFunc, Print, Return, Sequence,
        Struct, Switch, TraitDef, TraitImpl,
    };
    use crate::func_collect::Funcs;
    use crate::{DefFuncVal, FuncVal, Type};

    #[test]
    fn test_merge_none() {
        let vec: Vec<ConstraintMap> = Vec::new();
        assert_eq!(vec.merge(), Err(Error::VecSize()));
    }

    #[test]
    fn test_merge_one() {
        let mut cmap = ConstraintMap::new();
        cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        let vec: Vec<ConstraintMap> = vec![cmap];
        assert_eq!(vec[0].clone(), vec.merge().unwrap());
    }

    #[test]
    fn test_merge_two() {
        let mut cmap1 = ConstraintMap::new();
        cmap1.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        let mut cmap2 = ConstraintMap::new();
        cmap2.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(6)]), HashSet::new()),
            )),
        );
        let vec: Vec<ConstraintMap> = vec![cmap1, cmap2];

        let mut end_cmap = ConstraintMap::new();
        end_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5), RVal::Num(6)]), HashSet::new()),
            )),
        );
        assert_eq!(end_cmap, vec.merge().unwrap());
    }

    #[test]
    fn test_print() {
        let interp = Interpreter::new();
        let stmt = Print("hello");
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, ConstraintMap::new());
    }

    #[test]
    fn test_assign_num() {
        let interp = Interpreter::new();
        let stmt =
            Assignment("x", Box::new(AssignmentRVal::RVal(RVal::Num(5))));
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_seq() {
        let interp = Interpreter::new();
        let stmt_vec = vec![Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))];
        let stmt = Sequence(stmt_vec);
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_seq_assign() {
        let interp = Interpreter::new();
        let stmt_vec = vec![
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Assignment(
                "y",
                Box::new(AssignmentRVal::RVal(RVal::Num(6))),
            )),
        ];
        let stmt = Sequence(stmt_vec);
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(6)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_nested_seq() {
        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(Sequence(vec![Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            ))])),
            Box::new(Sequence(vec![Box::new(Assignment(
                "y",
                Box::new(AssignmentRVal::RVal(RVal::Num(6))),
            ))])),
        ]);
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(6)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_var_undef() {
        let interp = Interpreter::new();
        let stmt = Sequence(vec![Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Var("y"))),
        ))]);
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        assert_eq!(res.err(), Some(Error::UndefinedSymbol("y")));
    }

    #[test]
    fn test_assign_var() {
        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Assignment(
                "y",
                Box::new(AssignmentRVal::RVal(RVal::Var("x"))),
            )),
        ]);
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_conditional_true() {
        let interp = Interpreter::new();
        let stmt = Conditional(
            Box::new(BooleanStatement::True()),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(6))),
            )),
        );
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_conditional_false() {
        let interp = Interpreter::new();
        let stmt = Conditional(
            Box::new(BooleanStatement::False()),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(6))),
            )),
        );
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(6)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_conditional_uncertain() {
        let interp = Interpreter::new();
        let stmt = Conditional(
            Box::new(BooleanStatement::TrueOrFalse()),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(6))),
            )),
        );
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5), RVal::Num(6)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_conditional_not() {
        let interp = Interpreter::new();
        let stmt = Conditional(
            Box::new(BooleanStatement::Not(Box::new(BooleanStatement::True()))),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(6))),
            )),
        );
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(6)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_conditional_equals_num() {
        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(3))),
            )),
            Box::new(Assignment(
                "y",
                Box::new(AssignmentRVal::RVal(RVal::Num(3))),
            )),
            Box::new(Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("x"),
                    RVal::Var("y"),
                )),
                Box::new(Assignment(
                    "z",
                    Box::new(AssignmentRVal::RVal(RVal::Num(1))),
                )),
                Box::new(Assignment(
                    "z",
                    Box::new(AssignmentRVal::RVal(RVal::Num(2))),
                )),
            )),
        ]);
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "z",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(1)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_conditional_equals_func() {
        let mut funcs = Funcs::new();
        let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(vec![], vec![], None, foo_body.clone()),
        );
        funcs.funcs.insert(
            ("bar", None),
            FuncVal::new(vec![], vec![], None, foo_body.clone()),
        );

        let mut cmap = ConstraintMap::new();

        // note: `equals` is _shallow_, which is why it evals to false here
        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, foo_body.clone())),
            Box::new(FuncDef("bar", vec![], vec![], None, foo_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("foo"),
                    RVal::Var("bar"),
                )),
                Box::new(Assignment(
                    "z",
                    Box::new(AssignmentRVal::RVal(RVal::Num(1))),
                )),
                Box::new(Assignment(
                    "z",
                    Box::new(AssignmentRVal::RVal(RVal::Num(2))),
                )),
            )),
        ]);
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "z",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(2)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_conditional_equals_func_ref() {
        let mut funcs = Funcs::new();
        let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(vec![], vec![], None, foo_body.clone()),
        );

        let mut cmap = ConstraintMap::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, foo_body.clone())),
            Box::new(Assignment(
                "bar",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("foo"),
                    RVal::Var("bar"),
                )),
                Box::new(Assignment(
                    "z",
                    Box::new(AssignmentRVal::RVal(RVal::Num(1))),
                )),
                Box::new(Assignment(
                    "z",
                    Box::new(AssignmentRVal::RVal(RVal::Num(2))),
                )),
            )),
        ]);
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let foo_type = Type::Func(vec![], None);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "bar",
            Box::new(VarType::Values(
                Box::new(foo_type),
                (HashSet::from([RVal::Var("foo")]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "z",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(1)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_conditional_equals_uncertain() {
        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(3))),
            )),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "y",
                    Box::new(AssignmentRVal::RVal(RVal::Num(3))),
                )),
                Box::new(Assignment(
                    "y",
                    Box::new(AssignmentRVal::RVal(RVal::Num(4))),
                )),
            )),
            Box::new(Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("x"),
                    RVal::Var("y"),
                )),
                Box::new(Assignment(
                    "z",
                    Box::new(AssignmentRVal::RVal(RVal::Num(1))),
                )),
                Box::new(Assignment(
                    "z",
                    Box::new(AssignmentRVal::RVal(RVal::Num(2))),
                )),
            )),
        ]);
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3), RVal::Num(4)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "z",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(1), RVal::Num(2)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_conditional_equals_err() {
        let mut funcs = Funcs::new();
        let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(vec![], vec![], None, foo_body.clone()),
        );

        let mut cmap = ConstraintMap::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, foo_body)),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("foo"),
                    RVal::Var("x"),
                )),
                Box::new(Assignment(
                    "z",
                    Box::new(AssignmentRVal::RVal(RVal::Num(1))),
                )),
                Box::new(Assignment(
                    "z",
                    Box::new(AssignmentRVal::RVal(RVal::Num(2))),
                )),
            )),
        ]);
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        assert_eq!(
            res,
            Err(Error::IncomparableTypes(RVal::Var("foo"), RVal::Num(5)))
        );
    }

    #[test]
    fn test_nested_conditional() {
        let interp = Interpreter::new();
        let stmt = Conditional(
            Box::new(BooleanStatement::True()),
            Box::new(Conditional(
                Box::new(BooleanStatement::True()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Num(5))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Num(6))),
                )),
            )),
            Box::new(Conditional(
                Box::new(BooleanStatement::True()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Num(7))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Num(8))),
                )),
            )),
        );
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_conditional_scope() {
        let interp = Interpreter::new();
        let stmt = Sequence(vec![Box::new(Conditional(
            Box::new(BooleanStatement::True()),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(6))),
            )),
        ))]);
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_funcdef() {
        let mut funcs = Funcs::new();
        let body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(vec![], vec![], None, body.clone()),
        );

        let mut cmap = ConstraintMap::new();

        let interp = Interpreter::new();
        let stmt = FuncDef("foo", vec![], vec![], None, body);
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, ConstraintMap::new());
    }

    #[test]
    fn test_direct_invoke() {
        let mut funcs = Funcs::new();
        let body = Box::new(Sequence(vec![Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(vec![], vec![], None, body.clone()),
        );

        let mut cmap = ConstraintMap::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, body)),
            Box::new(InvokeFunc("foo", vec![])),
        ]);
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        let mut foo_cmap = ConstraintMap::new();
        foo_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                None,
                foo_cmap,
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_funcdef_args_direct() {
        let mut funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let interp = Interpreter::new();

        let body = Box::new(Sequence(vec![
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("x"),
                    RVal::Var("y"),
                )),
                Box::new(Assignment(
                    "z",
                    Box::new(AssignmentRVal::RVal(RVal::Num(0))),
                )),
                Box::new(Assignment(
                    "z",
                    Box::new(AssignmentRVal::RVal(RVal::Num(1))),
                )),
            )),
        ]));

        let stmt = Sequence(vec![
            Box::new(FuncDef(
                "foo",
                vec![Type::Int()],
                vec!["y"],
                None,
                body.clone(),
            )),
            Box::new(Assignment(
                "arg",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(InvokeFunc("foo", vec!["arg"])),
        ]);

        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(vec![Type::Int()], vec!["y"], None, body.clone()),
        );
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        let mut foo_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "arg",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        foo_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        foo_cmap.cmap.insert(
            "z",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(0)]), HashSet::new()),
            )),
        );
        foo_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![Type::Int()], None)),
                None,
                foo_cmap,
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_indirect_invoke_simple() {
        let mut funcs = Funcs::new();
        let body = Box::new(Sequence(vec![Box::new(Assignment(
            "z",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(vec![], vec![], None, body.clone()),
        );

        let mut cmap = ConstraintMap::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, body)),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]);
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let foo_type = Type::Func(vec![], None);
        let mut check_cmap = ConstraintMap::new();
        let mut foo_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(foo_type),
                (HashSet::from([RVal::Var("foo")]), HashSet::new()),
            )),
        );
        foo_cmap.cmap.insert(
            "z",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                None,
                foo_cmap,
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_funcdef_args_indirect() {
        let mut funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let interp = Interpreter::new();

        let body = Box::new(Sequence(vec![
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("x"),
                    RVal::Var("y"),
                )),
                Box::new(Assignment(
                    "z",
                    Box::new(AssignmentRVal::RVal(RVal::Num(0))),
                )),
                Box::new(Assignment(
                    "z",
                    Box::new(AssignmentRVal::RVal(RVal::Num(1))),
                )),
            )),
        ]));

        let stmt = Sequence(vec![
            Box::new(FuncDef(
                "foo",
                vec![Type::Int()],
                vec!["y"],
                None,
                body.clone(),
            )),
            Box::new(Assignment(
                "w",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(Assignment(
                "arg",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(InvokeFunc("w", vec!["arg"])),
        ]);

        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(vec![Type::Int()], vec!["y"], None, body.clone()),
        );
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let foo_type = Type::Func(vec![Type::Int()], None);
        let mut check_cmap = ConstraintMap::new();
        let mut foo_cmap = ConstraintMap::new();
        foo_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        foo_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        foo_cmap.cmap.insert(
            "z",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(0)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "arg",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "w",
            Box::new(VarType::Values(
                Box::new(foo_type),
                (HashSet::from([RVal::Var("foo")]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![Type::Int()], None)),
                None,
                foo_cmap,
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_nested_direct_calls_no_args() {
        let bar_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        let foo_body = Box::new(Sequence(vec![
            Box::new(FuncDef("bar", vec![], vec![], None, bar_body.clone())),
            Box::new(InvokeFunc("bar", vec![])),
        ]));
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, foo_body.clone())),
            Box::new(InvokeFunc("foo", vec![])),
        ]);

        let mut funcs = Funcs::new();
        funcs.funcs.insert(
            ("bar", None),
            FuncVal::new(vec![], vec![], None, bar_body),
        );
        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(vec![], vec![], None, foo_body),
        );

        let mut cmap = ConstraintMap::new();

        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        let foo_cmap = ConstraintMap::new();
        let mut bar_cmap = ConstraintMap::new();
        bar_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "bar",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                Some("foo"),
                bar_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                None,
                foo_cmap,
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_nested_indirect_calls_no_args() {
        let baz_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(1))),
        ));
        let qux_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(2))),
        ));
        let baz2_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(3))),
        ));
        let qux2_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(4))),
        ));
        let foo_body = Box::new(Sequence(vec![
            Box::new(FuncDef("baz", vec![], vec![], None, baz_body.clone())),
            Box::new(FuncDef("qux", vec![], vec![], None, qux_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("baz"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("qux"))),
                )),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]));
        let bar_body = Box::new(Sequence(vec![
            Box::new(FuncDef("baz2", vec![], vec![], None, baz2_body.clone())),
            Box::new(FuncDef("qux2", vec![], vec![], None, qux2_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("baz2"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("qux2"))),
                )),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]));

        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, foo_body.clone())),
            Box::new(FuncDef("bar", vec![], vec![], None, bar_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("bar"))),
                )),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]);

        let mut funcs = Funcs::new();
        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(vec![], vec![], None, foo_body.clone()),
        );
        funcs.funcs.insert(
            ("bar", None),
            FuncVal::new(vec![], vec![], None, bar_body.clone()),
        );
        funcs.funcs.insert(
            ("baz", None),
            FuncVal::new(vec![], vec![], None, baz_body.clone()),
        );
        funcs.funcs.insert(
            ("qux", None),
            FuncVal::new(vec![], vec![], None, qux_body.clone()),
        );
        funcs.funcs.insert(
            ("baz2", None),
            FuncVal::new(vec![], vec![], None, baz2_body.clone()),
        );
        funcs.funcs.insert(
            ("qux2", None),
            FuncVal::new(vec![], vec![], None, qux2_body.clone()),
        );

        let mut cmap = ConstraintMap::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        let mut foo_cmap = ConstraintMap::new();
        let mut bar_cmap = ConstraintMap::new();
        let mut baz_cmap = ConstraintMap::new();
        let mut qux_cmap = ConstraintMap::new();
        let mut baz2_cmap = ConstraintMap::new();
        let mut qux2_cmap = ConstraintMap::new();

        baz_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(1)]), HashSet::new()),
            )),
        );
        qux_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(2)]), HashSet::new()),
            )),
        );
        baz2_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        qux2_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(4)]), HashSet::new()),
            )),
        );

        foo_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Func(vec![], None)),
                (
                    HashSet::from([RVal::Var("baz"), RVal::Var("qux")]),
                    HashSet::new(),
                ),
            )),
        );
        bar_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Func(vec![], None)),
                (
                    HashSet::from([RVal::Var("baz2"), RVal::Var("qux2")]),
                    HashSet::new(),
                ),
            )),
        );

        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Func(vec![], None)),
                (
                    HashSet::from([RVal::Var("foo"), RVal::Var("bar")]),
                    HashSet::new(),
                ),
            )),
        );
        check_cmap.cmap.insert(
            "baz2",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                Some("bar"),
                baz2_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                None,
                foo_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "qux",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                Some("foo"),
                qux_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "baz",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                Some("foo"),
                baz_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "bar",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                None,
                bar_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "qux2",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                Some("bar"),
                qux2_cmap,
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_nested_direct_calls_with_args() {
        let bar_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Var("y"))),
        ));
        let foo_body = Box::new(Sequence(vec![
            Box::new(FuncDef(
                "bar",
                vec![Type::Int()],
                vec!["y"],
                None,
                bar_body.clone(),
            )),
            Box::new(InvokeFunc("bar", vec!["y"])),
        ]));
        let stmt = Sequence(vec![
            Box::new(FuncDef(
                "foo",
                vec![Type::Int()],
                vec!["y"],
                None,
                foo_body.clone(),
            )),
            Box::new(Assignment(
                "arg",
                Box::new(AssignmentRVal::RVal(RVal::Num(3))),
            )),
            Box::new(InvokeFunc("foo", vec!["arg"])),
        ]);

        let mut funcs = Funcs::new();
        funcs.funcs.insert(
            ("bar", None),
            FuncVal::new(vec![Type::Int()], vec!["y"], None, bar_body),
        );
        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(vec![Type::Int()], vec!["y"], None, foo_body),
        );

        let mut cmap = ConstraintMap::new();

        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        let mut foo_cmap = ConstraintMap::new();
        let mut bar_cmap = ConstraintMap::new();
        bar_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        bar_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        foo_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "arg",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "bar",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![Type::Int()], None)),
                Some("foo"),
                bar_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![Type::Int()], None)),
                None,
                foo_cmap,
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_nested_indirect_calls_with_args() {
        let baz_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Var("y"))),
        ));
        let qux_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Var("y"))),
        ));
        let baz2_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Var("y"))),
        ));
        let qux2_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Var("y"))),
        ));
        let foo_body = Box::new(Sequence(vec![
            Box::new(FuncDef(
                "baz",
                vec![Type::Int()],
                vec!["y"],
                None,
                baz_body.clone(),
            )),
            Box::new(FuncDef(
                "qux",
                vec![Type::Int()],
                vec!["y"],
                None,
                qux_body.clone(),
            )),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("baz"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("qux"))),
                )),
            )),
            Box::new(InvokeFunc("x", vec!["y"])),
        ]));
        let bar_body = Box::new(Sequence(vec![
            Box::new(FuncDef(
                "baz2",
                vec![Type::Int()],
                vec!["y"],
                None,
                baz2_body.clone(),
            )),
            Box::new(FuncDef(
                "qux2",
                vec![Type::Int()],
                vec!["y"],
                None,
                qux2_body.clone(),
            )),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("baz2"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("qux2"))),
                )),
            )),
            Box::new(InvokeFunc("x", vec!["y"])),
        ]));

        let stmt = Sequence(vec![
            Box::new(FuncDef(
                "foo",
                vec![Type::Int()],
                vec!["y"],
                None,
                foo_body.clone(),
            )),
            Box::new(FuncDef(
                "bar",
                vec![Type::Int()],
                vec!["y"],
                None,
                bar_body.clone(),
            )),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("bar"))),
                )),
            )),
            Box::new(Assignment(
                "y",
                Box::new(AssignmentRVal::RVal(RVal::Num(3))),
            )),
            Box::new(InvokeFunc("x", vec!["y"])),
        ]);

        let mut funcs = Funcs::new();
        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(vec![Type::Int()], vec!["y"], None, foo_body.clone()),
        );
        funcs.funcs.insert(
            ("bar", None),
            FuncVal::new(vec![Type::Int()], vec!["y"], None, bar_body.clone()),
        );
        funcs.funcs.insert(
            ("baz", None),
            FuncVal::new(vec![Type::Int()], vec!["y"], None, baz_body.clone()),
        );
        funcs.funcs.insert(
            ("qux", None),
            FuncVal::new(vec![Type::Int()], vec!["y"], None, qux_body.clone()),
        );
        funcs.funcs.insert(
            ("baz2", None),
            FuncVal::new(vec![Type::Int()], vec!["y"], None, baz2_body.clone()),
        );
        funcs.funcs.insert(
            ("qux2", None),
            FuncVal::new(vec![Type::Int()], vec!["y"], None, qux2_body.clone()),
        );

        let mut cmap = ConstraintMap::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        let mut foo_cmap = ConstraintMap::new();
        let mut bar_cmap = ConstraintMap::new();
        let mut baz_cmap = ConstraintMap::new();
        let mut qux_cmap = ConstraintMap::new();
        let mut baz2_cmap = ConstraintMap::new();
        let mut qux2_cmap = ConstraintMap::new();

        baz_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        baz_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        qux_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        qux_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        baz2_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        baz2_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        qux2_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        qux2_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );

        foo_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Func(vec![Type::Int()], None)),
                (
                    HashSet::from([RVal::Var("baz"), RVal::Var("qux")]),
                    HashSet::new(),
                ),
            )),
        );
        foo_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        bar_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Func(vec![Type::Int()], None)),
                (
                    HashSet::from([RVal::Var("baz2"), RVal::Var("qux2")]),
                    HashSet::new(),
                ),
            )),
        );
        bar_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );

        check_cmap.cmap.insert(
            "baz",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![Type::Int()], None)),
                Some("foo"),
                baz_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "bar",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![Type::Int()], None)),
                None,
                bar_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![Type::Int()], None)),
                None,
                foo_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Func(vec![Type::Int()], None)),
                (
                    HashSet::from([RVal::Var("foo"), RVal::Var("bar")]),
                    HashSet::new(),
                ),
            )),
        );
        check_cmap.cmap.insert(
            "qux",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![Type::Int()], None)),
                Some("foo"),
                qux_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "baz2",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![Type::Int()], None)),
                Some("bar"),
                baz2_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "qux2",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![Type::Int()], None)),
                Some("bar"),
                qux2_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_direct_invoke_uncertain() {
        let mut funcs = Funcs::new();
        let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        let bar_body = Box::new(Sequence(vec![Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(6))),
        ))]));
        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(vec![], vec![], None, foo_body.clone()),
        );
        funcs.funcs.insert(
            ("bar", None),
            FuncVal::new(vec![], vec![], None, bar_body.clone()),
        );

        let mut cmap = ConstraintMap::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, foo_body)),
            Box::new(FuncDef("bar", vec![], vec![], None, bar_body)),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(InvokeFunc("foo", vec![])),
                Box::new(InvokeFunc("bar", vec![])),
            )),
        ]);
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        let mut foo_cmap = ConstraintMap::new();
        let mut bar_cmap = ConstraintMap::new();
        foo_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        bar_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(6)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                None,
                foo_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "bar",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                None,
                bar_cmap,
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_indirect_invoke_uncertain() {
        let mut funcs = Funcs::new();
        let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
            "y",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        let bar_body = Box::new(Sequence(vec![Box::new(Assignment(
            "z",
            Box::new(AssignmentRVal::RVal(RVal::Num(6))),
        ))]));
        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(vec![], vec![], None, foo_body.clone()),
        );
        funcs.funcs.insert(
            ("bar", None),
            FuncVal::new(vec![], vec![], None, bar_body.clone()),
        );

        let mut cmap = ConstraintMap::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, foo_body)),
            Box::new(FuncDef("bar", vec![], vec![], None, bar_body)),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("bar"))),
                )),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]);
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        let mut foo_cmap = ConstraintMap::new();
        let mut bar_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Func(vec![], None)),
                (
                    HashSet::from([RVal::Var("foo"), RVal::Var("bar")]),
                    HashSet::new(),
                ),
            )),
        );
        foo_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        bar_cmap.cmap.insert(
            "z",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(6)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                None,
                foo_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "bar",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                None,
                bar_cmap,
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_indirect_invoke_err() {
        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(Assignment(
                "foo",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(InvokeFunc("foo", vec![])),
        ]);
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        assert_eq!(res, Err(Error::NotAFunction("foo")));
    }

    #[test]
    fn test_switch_err() {
        let interp = Interpreter::new();
        let switch_vec: Vec<(RVal, Box<Statement>)> = vec![];
        let stmt = Switch(RVal::Var("x"), switch_vec);
        let res = interp.interp(
            &Funcs::new(),
            &mut ConstraintMap::new(),
            None,
            &stmt,
        );

        assert_eq!(res, Err(Error::UndefinedSymbol("x")));
    }

    #[test]
    fn test_switch() {
        let interp = Interpreter::new();
        let switch_vec: Vec<(RVal, Box<Statement>)> = vec![
            (
                RVal::Num(4),
                Box::new(Assignment(
                    "y",
                    Box::new(AssignmentRVal::RVal(RVal::Num(0))),
                )),
            ),
            (
                RVal::Num(5),
                Box::new(Assignment(
                    "y",
                    Box::new(AssignmentRVal::RVal(RVal::Num(1))),
                )),
            ),
        ];
        let stmt = Sequence(vec![
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Switch(RVal::Var("x"), switch_vec)),
        ]);
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(1)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_switch_uncertain() {
        let interp = Interpreter::new();
        let switch_vec: Vec<(RVal, Box<Statement>)> = vec![
            (
                RVal::Num(4),
                Box::new(Assignment(
                    "y",
                    Box::new(AssignmentRVal::RVal(RVal::Num(0))),
                )),
            ),
            (
                RVal::Num(5),
                Box::new(Assignment(
                    "y",
                    Box::new(AssignmentRVal::RVal(RVal::Num(1))),
                )),
            ),
        ];
        let stmt = Sequence(vec![
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Num(5))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Num(4))),
                )),
            )),
            Box::new(Switch(RVal::Var("x"), switch_vec)),
        ]);
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "y",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(0), RVal::Num(1)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5), RVal::Num(4)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_direct_func_return_num() {
        let mut funcs = Funcs::new();
        let body = Box::new(Return(RVal::Num(5)));
        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(
                vec![],
                vec![],
                Some(Box::new(Type::Int())),
                body.clone(),
            ),
        );

        let mut cmap = ConstraintMap::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef(
                "foo",
                vec![],
                vec![],
                Some(Box::new(Type::Int())),
                body,
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::Statement(Box::new(InvokeFunc(
                    "foo",
                    vec![],
                )))),
            )),
        ]);
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], Some(Box::new(Type::Int())))),
                None,
                ConstraintMap::new(),
            )),
        );
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_direct_func_return_set() {
        let mut funcs = Funcs::new();
        let body = Box::new(Sequence(vec![
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Num(5))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Num(6))),
                )),
            )),
            Box::new(Return(RVal::Var("x"))),
        ]));
        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(
                vec![],
                vec![],
                Some(Box::new(Type::Int())),
                body.clone(),
            ),
        );

        let mut cmap = ConstraintMap::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef(
                "foo",
                vec![],
                vec![],
                Some(Box::new(Type::Int())),
                body,
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::Statement(Box::new(InvokeFunc(
                    "foo",
                    vec![],
                )))),
            )),
        ]);
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        let mut foo_cmap = ConstraintMap::new();
        foo_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5), RVal::Num(6)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], Some(Box::new(Type::Int())))),
                None,
                foo_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5), RVal::Num(6)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_direct_func_return_func() {
        let bar_body = Box::new(Return(RVal::Num(0)));
        let baz_body = Box::new(Return(RVal::Num(1)));
        let foo_body = Box::new(Sequence(vec![
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("bar"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("baz"))),
                )),
            )),
            Box::new(Return(RVal::Var("x"))),
        ]));

        let rettype = Box::new(Type::Int());
        let mut funcs = Funcs::new();
        funcs.funcs.insert(
            ("foo", None),
            FuncVal::new(
                vec![],
                vec![],
                Some(Box::new(Type::Func(vec![], Some(rettype.clone())))),
                foo_body.clone(),
            ),
        );
        funcs.funcs.insert(
            ("bar", None),
            FuncVal::new(
                vec![],
                vec![],
                Some(Box::new(Type::Int())),
                bar_body.clone(),
            ),
        );
        funcs.funcs.insert(
            ("baz", None),
            FuncVal::new(
                vec![],
                vec![],
                Some(Box::new(Type::Int())),
                baz_body.clone(),
            ),
        );

        let mut cmap = ConstraintMap::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef(
                "foo",
                vec![],
                vec![],
                Some(Box::new(Type::Func(vec![], Some(Box::new(Type::Int()))))),
                foo_body,
            )),
            Box::new(FuncDef(
                "bar",
                vec![],
                vec![],
                Some(Box::new(Type::Int())),
                bar_body,
            )),
            Box::new(FuncDef(
                "baz",
                vec![],
                vec![],
                Some(Box::new(Type::Int())),
                baz_body,
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::Statement(Box::new(InvokeFunc(
                    "foo",
                    vec![],
                )))),
            )),
            Box::new(Assignment(
                "res",
                Box::new(AssignmentRVal::Statement(Box::new(InvokeFunc(
                    "x",
                    vec![],
                )))),
            )),
        ]);
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        let mut foo_cmap = ConstraintMap::new();
        foo_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Func(vec![], Some(Box::new(Type::Int())))),
                (
                    HashSet::from([RVal::Var("bar"), RVal::Var("baz")]),
                    HashSet::new(),
                ),
            )),
        );
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                Box::new(Type::Func(
                    vec![],
                    Some(Box::new(Type::Func(
                        vec![],
                        Some(Box::new(Type::Int())),
                    ))),
                )),
                None,
                foo_cmap,
            )),
        );
        check_cmap.cmap.insert(
            "bar",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], Some(Box::new(Type::Int())))),
                None,
                ConstraintMap::new(),
            )),
        );
        check_cmap.cmap.insert(
            "baz",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], Some(Box::new(Type::Int())))),
                None,
                ConstraintMap::new(),
            )),
        );
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Func(vec![], Some(Box::new(Type::Int())))),
                (
                    HashSet::from([RVal::Var("bar"), RVal::Var("baz")]),
                    HashSet::new(),
                ),
            )),
        );
        check_cmap.cmap.insert(
            "res",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(1), RVal::Num(0)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_negative_constraints_eq() {
        let stmt = Sequence(vec![
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Num(5))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Num(3))),
                )),
            )),
            Box::new(Assignment(
                "f",
                Box::new(AssignmentRVal::RVal(RVal::Num(3))),
            )),
            Box::new(Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("x"),
                    RVal::Var("f"),
                )),
                Box::new(Assignment(
                    "g",
                    Box::new(AssignmentRVal::RVal(RVal::Var("x"))),
                )),
                Box::new(Assignment(
                    "h",
                    Box::new(AssignmentRVal::RVal(RVal::Num(0))),
                )),
            )),
        ]);

        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5), RVal::Num(3)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "f",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "g",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (
                    HashSet::from([RVal::Num(5), RVal::Num(3), RVal::Var("f")]),
                    HashSet::new(),
                ),
            )),
        );
        check_cmap.cmap.insert(
            "h",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(0)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_negative_constraints_neq() {
        let stmt = Sequence(vec![
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Num(5))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Num(3))),
                )),
            )),
            Box::new(Assignment(
                "f",
                Box::new(AssignmentRVal::RVal(RVal::Num(3))),
            )),
            Box::new(Conditional(
                Box::new(BooleanStatement::Not(Box::new(
                    BooleanStatement::Equals(RVal::Var("x"), RVal::Var("f")),
                ))),
                Box::new(Assignment(
                    "g",
                    Box::new(AssignmentRVal::RVal(RVal::Var("x"))),
                )),
                Box::new(Assignment(
                    "h",
                    Box::new(AssignmentRVal::RVal(RVal::Num(0))),
                )),
            )),
        ]);

        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5), RVal::Num(3)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "f",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(3)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "g",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (
                    HashSet::from([RVal::Num(5), RVal::Num(3)]),
                    HashSet::from([RVal::Var("f")]),
                ),
            )),
        );
        check_cmap.cmap.insert(
            "h",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(0)]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_struct() {
        let stmt = Sequence(vec![
            Box::new(Struct("Cat", vec![Type::Int()], vec!["age"])),
            Box::new(Assignment(
                "edgar",
                Box::new(AssignmentRVal::RVal(RVal::Struct(
                    "Cat",
                    vec![RVal::Var("9")],
                ))),
            )),
        ]);

        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "edgar",
            Box::new(VarType::Values(
                Box::new(Type::Struct()),
                (
                    HashSet::from([RVal::Struct("Cat", vec![RVal::Var("9")])]),
                    HashSet::new(),
                ),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    // TODO test structs as args/retvals

    #[test]
    fn test_trait_impl() {
        let cat_speak_body = Box::new(Sequence(vec![Box::new(Print("meow"))]));

        let funcdef = DefFuncVal::new(vec![], vec![], None);
        let cat_funcimpl =
            FuncVal::new(vec![], vec![], None, cat_speak_body.clone());

        let stmt = Sequence(vec![
            Box::new(TraitDef("Animal", vec!["speak"], vec![funcdef.clone()])),
            Box::new(Struct("Cat", vec![], vec![])),
            Box::new(TraitImpl(
                "Animal",
                "Cat",
                vec!["speak"],
                vec![cat_funcimpl.clone()],
            )),
            Box::new(Assignment(
                "edgar",
                Box::new(AssignmentRVal::RVal(RVal::Struct("Cat", vec![]))),
            )),
        ]);

        let mut funcs = Funcs::new();
        funcs
            .funcs
            .insert(("speak", Some(("Animal", "Cat"))), cat_funcimpl.clone());

        let mut cmap = ConstraintMap::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "edgar",
            Box::new(VarType::Values(
                Box::new(Type::Struct()),
                (HashSet::from([RVal::Struct("Cat", vec![])]), HashSet::new()),
            )),
        );

        // TODO call func - either `self` or flexible arg type?

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }
}
