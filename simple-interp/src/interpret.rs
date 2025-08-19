use crate::constraints::{BConstraints, ConstraintMap, Constraints, Difference, VarType};
use crate::error::Error;
use crate::funcs::Funcs;
use crate::statement::{
    AssignmentRVal, BStatement, FuncVal, Merge, RVal, Statement, TraitStructOpt, Type,
};
use crate::traits::Traits;
use std::collections::HashSet;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interp(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        traits: &mut Traits,
        scope: Option<&'static str>,
        stmt: &Statement,
    ) -> Result<Option<Constraints>, Error> {
        match stmt {
            Statement::TraitDecl(trait_name, _) => {
                self.interp_traitdecl(funcs, cmap, traits, scope, trait_name)
            }
            Statement::TraitImpl(trait_name, struct_name, _) => {
                self.interp_traitimpl(funcs, cmap, traits, scope, trait_name, struct_name)
            }
            Statement::Sequence(stmt_vec) => {
                self.interp_seq(funcs, cmap, traits, scope, stmt_vec)
            }
            Statement::Assignment(var, value) => {
                self.interp_assignment(funcs, cmap, traits, scope, var, value)
            }
            Statement::Print(var) => self.interp_print(var),
            Statement::Conditional(condition, true_branch, false_branch) => self
                .interp_conditional(
                    funcs,
                    cmap,
                    traits,
                    scope,
                    condition,
                    true_branch,
                    false_branch,
                ),
            Statement::Switch(val, vec) => {
                self.interp_switch(funcs, cmap, traits, scope, val, vec)
            }
            Statement::Return(rval) => {
                self.interp_return(funcs, cmap, traits, scope, rval)
            }
            Statement::InvokeFunc(name, args) => {
                self.interp_invoke(funcs, cmap, traits, scope, name, args)
            }
            Statement::FuncDecl(..) | Statement::FuncDef(..) | Statement::Struct(..) => {
                Ok(None)
            }
        }
    }

    fn interp_seq(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        traits: &mut Traits,
        scope: Option<&'static str>,
        stmt_vec: &Vec<Box<Statement>>,
    ) -> Result<Option<Constraints>, Error> {
        let mut last_ret = None;
        for stmt in stmt_vec.iter() {
            let res = self.interp(funcs, cmap, traits, scope, stmt)?;
            last_ret = res;
        }
        Ok(last_ret)
    }

    fn assign_funcvec(
        &self,
        cmap: &mut ConstraintMap,
        scope: Option<&'static str>,
        var: &'static str,
        varname: &'static str,
        funcvec: &Vec<(TraitStructOpt, FuncVal)>,
    ) -> Result<(), Error> {
        for (_, funcval) in funcvec.iter() {
            let (_, paramtypes): (Vec<&'static str>, Vec<Type>) =
                funcval.params.clone().into_iter().unzip();
            let functype = Type::Func(paramtypes, funcval.rettype.clone());
            cmap.scoped_set(
                scope,
                var,
                Box::new(VarType::Values(
                    Box::new(functype),
                    (HashSet::from([RVal::Var(varname)]), HashSet::new()),
                )),
            )?
        }
        Ok(())
    }

    fn assign_invoke(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        traits: &mut Traits,
        scope: Option<&'static str>,
        var: &'static str,
        name: &'static str,
        args: &Vec<&'static str>,
    ) -> Result<(), Error> {
        match self.interp_invoke(funcs, cmap, traits, scope, name, args) {
            Ok(Some(constraints)) => {
                let functype;
                match cmap.cmap.get(name) {
                    Some(val) => match *val.clone() {
                        VarType::Values(stored_functype, _) => {
                            functype = stored_functype;
                        }
                        VarType::Scope(_, inst_vec) => {
                            if inst_vec.len() != 1 {
                                todo!("not impl yet (scope vec)");
                            }
                            functype = inst_vec[0].0.clone();
                        }
                    },
                    None => {
                        return Err(Error::UndefinedSymbol(name));
                    }
                }

                match *functype {
                    Type::Func(_, rettype) => cmap.scoped_set(
                        scope,
                        var,
                        Box::new(VarType::Values(rettype.unwrap(), constraints)),
                    )?,
                    _ => return Err(Error::NotAFunction(name)),
                }
            }
            Ok(None) => return Err(Error::CannotAssignNoneRetval()),
            Err(err) => return Err(err),
        }
        Ok(())
    }

    fn interp_assignment(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        traits: &mut Traits,
        scope: Option<&'static str>,
        var: &'static str,
        value: &AssignmentRVal,
    ) -> Result<Option<Constraints>, Error> {
        match value {
            AssignmentRVal::RVal(RVal::Num(num)) => cmap.scoped_set(
                scope,
                var,
                Box::new(VarType::Values(
                    Box::new(Type::Int()),
                    (HashSet::from([RVal::Num(*num)]), HashSet::new()),
                )),
            )?,
            AssignmentRVal::RVal(RVal::IdkNum()) => cmap.scoped_set(
                scope,
                var,
                Box::new(VarType::Values(
                    Box::new(Type::Int()),
                    (HashSet::from([RVal::IdkNum()]), HashSet::new()),
                )),
            )?,
            AssignmentRVal::RVal(RVal::Struct(
                struct_name,
                field_values,
                field_names,
            )) => cmap.scoped_set(
                scope,
                var,
                Box::new(VarType::Values(
                    Box::new(Type::Struct(struct_name)),
                    (
                        HashSet::from([RVal::Struct(
                            struct_name,
                            field_values.to_vec(),
                            field_names.to_vec(),
                        )]),
                        HashSet::new(),
                    ),
                )),
            )?,
            AssignmentRVal::RVal(RVal::IdkStruct(struct_name)) => cmap.scoped_set(
                scope,
                var,
                Box::new(VarType::Values(
                    Box::new(Type::Struct(struct_name)),
                    (
                        HashSet::from([RVal::IdkStruct(struct_name)]),
                        HashSet::new(),
                    ),
                )),
            )?,
            AssignmentRVal::RVal(RVal::Var(varname)) => {
                match cmap.scoped_get(scope, varname, false) {
                    Ok(Some(val)) => match val {
                        set @ VarType::Values(..) => {
                            cmap.scoped_set(scope, var, Box::new(set))?
                        }
                        _ => todo!("not impl yet"),
                    },
                    Ok(None) => match funcs.funcs.get(varname) {
                        Some(funcvec) => {
                            self.assign_funcvec(cmap, scope, var, varname, funcvec)?
                        }
                        None => {
                            return Err(Error::UndefinedSymbol(varname));
                        }
                    },
                    Err(err) => return Err(err),
                };
            }
            AssignmentRVal::RVal(RVal::IdkVar()) => cmap.scoped_set(
                scope,
                var,
                Box::new(VarType::Values(
                    Box::new(Type::Idk()),
                    (HashSet::from([RVal::IdkVar()]), HashSet::new()),
                )),
            )?,
            AssignmentRVal::Statement(stmt) => match *stmt.clone() {
                Statement::InvokeFunc(name, args) => {
                    // assume func has retval (given typechecking) but return
                    // err if none
                    self.assign_invoke(funcs, cmap, traits, scope, var, name, &args)?
                }
                _ => return Err(Error::InvalidAssignmentRVal()),
            },
        }

        Ok(None)
    }

    fn interp_print(&self, _var: &'static str) -> Result<Option<Constraints>, Error> {
        Ok(None)
    }

    fn interp_bool(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        scope: Option<&'static str>,
        b_stmt: BStatement,
    ) -> Result<(BStatement, BConstraints), Error> {
        match b_stmt {
            BStatement::True() | BStatement::False() | BStatement::TrueOrFalse() => {
                Ok((b_stmt, BConstraints::empty()))
            }
            BStatement::Not(inner_b_stmt) => {
                self.interp_not(funcs, cmap, scope, *inner_b_stmt)
            }
            BStatement::Equals(lhs, rhs) => {
                self.interp_equals(funcs, cmap, scope, lhs, rhs)
            }
        }
    }

    fn interp_not(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        scope: Option<&'static str>,
        b_stmt: BStatement,
    ) -> Result<(BStatement, BConstraints), Error> {
        match self.interp_bool(funcs, cmap, scope, b_stmt) {
            Ok(b_res) => Ok((!b_res.0, BConstraints::flip_constraints(b_res.1))),
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
            RVal::Num(_)
            | RVal::IdkNum()
            | RVal::Struct(..)
            | RVal::IdkStruct(_)
            | RVal::IdkVar() => Ok((HashSet::from([rval]), HashSet::new())),
            RVal::Var(var) => match cmap.scoped_get(scope, var, false) {
                Ok(Some(val)) => match val {
                    VarType::Values(_, constraints) => Ok(constraints.clone()),
                    _ => todo!("should be a func"),
                },
                Ok(None) => match funcs.funcs.get(&var) {
                    Some(_funcvec) => Ok((HashSet::from([rval]), HashSet::new())),
                    None => Err(Error::UndefinedSymbol(var)),
                },
                Err(err) => Err(err),
            },
        }
    }

    fn constraints_to_vecs(&self, constraints: &Constraints) -> (Vec<RVal>, Vec<RVal>) {
        let pos_vec: Vec<_> = constraints.0.clone().into_iter().collect();
        let neg_vec: Vec<_> = constraints.1.clone().into_iter().collect();
        (pos_vec, neg_vec)
    }

    fn single_constraint_compare(
        &self,
        lhs_vecs: &(Vec<RVal>, Vec<RVal>),
        rhs_vecs: &(Vec<RVal>, Vec<RVal>),
    ) -> Result<(BStatement, BConstraints), Error> {
        match (lhs_vecs.0[0].clone(), rhs_vecs.0[0].clone()) {
            (RVal::Num(lnum), RVal::Num(rnum)) => {
                if lnum == rnum {
                    Ok((BStatement::True(), BConstraints::empty()))
                } else {
                    Ok((BStatement::False(), BConstraints::empty()))
                }
            }
            (RVal::Var(lfp), RVal::Var(rfp)) => {
                if lfp == rfp {
                    Ok((BStatement::True(), BConstraints::empty()))
                } else {
                    Ok((BStatement::False(), BConstraints::empty()))
                }
            }
            (_, _) => Err(Error::IncomparableTypes(
                lhs_vecs.0[0].clone(),
                rhs_vecs.0[0].clone(),
            )),
        }
    }

    fn many_constraints_compare(
        &self,
        cmap: &ConstraintMap,
        lhs: RVal,
        rhs: RVal,
    ) -> Result<(BStatement, BConstraints), Error> {
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
                            (VarType::Values(type_a, _), VarType::Values(type_b, _)) => {
                                if type_a != type_b {
                                    return Err(Error::TypesDiffer());
                                }
                                vartype = type_a;
                            }
                            (
                                VarType::Values(type_a, _),
                                VarType::Scope(_, inst_vec_b), /* type_b, _), */
                            ) => {
                                if inst_vec_b.len() != 1 {
                                    todo!("not impl yet (scope vec)");
                                }

                                if type_a != inst_vec_b[0].0 {
                                    return Err(Error::TypesDiffer());
                                }
                                vartype = type_a;
                            }
                            (
                                VarType::Scope(_, inst_vec_a), /* type_a, _), */
                                VarType::Values(type_b, _),
                            ) => {
                                if inst_vec_a.len() != 1 {
                                    todo!("not impl yet (scope vec)");
                                }

                                if type_b != inst_vec_a[0].0 {
                                    return Err(Error::TypesDiffer());
                                }
                                vartype = type_b;
                            }
                            (
                                VarType::Scope(_, inst_vec_a), /* type_a, _), */
                                VarType::Scope(_, inst_vec_b), /* type_b, _), */
                            ) => {
                                if inst_vec_a.len() != 1 || inst_vec_b.len() != 1 {
                                    todo!("not impl yet (scope vec)");
                                }

                                if inst_vec_a[0].0 != inst_vec_b[0].0 {
                                    return Err(Error::TypesDiffer());
                                }
                                vartype = inst_vec_a[0].0.clone();
                            }
                        }
                    }
                    (None, _) => {
                        return Err(Error::UndefinedSymbol(a));
                    }
                    (_, None) => {
                        return Err(Error::UndefinedSymbol(b));
                    }
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

                Ok((
                    BStatement::TrueOrFalse(),
                    BConstraints::new(true_branch, false_branch),
                ))
            }
            _ => todo!("not impl yet"),
        }
    }

    fn interp_equals(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        scope: Option<&'static str>,
        lhs: RVal,
        rhs: RVal,
    ) -> Result<(BStatement, BConstraints), Error> {
        let lhs_vecs = self.constraints_to_vecs(&self.interp_rval(
            funcs,
            cmap,
            scope,
            lhs.clone(),
        )?);
        let rhs_vecs = self.constraints_to_vecs(&self.interp_rval(
            funcs,
            cmap,
            scope,
            rhs.clone(),
        )?);

        // eval directly if only a single positive constraint
        if lhs_vecs.0.len() == 1
            && lhs_vecs.1.is_empty()
            && rhs_vecs.0.len() == 1
            && rhs_vecs.1.is_empty()
        {
            self.single_constraint_compare(&lhs_vecs, &rhs_vecs)
        } else {
            self.many_constraints_compare(cmap, lhs, rhs)
        }
    }

    fn interp_conditional(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        traits: &mut Traits,
        scope: Option<&'static str>,
        condition: &BStatement,
        true_branch: &Statement,
        false_branch: &Statement,
    ) -> Result<Option<Constraints>, Error> {
        let mut res_cmap: Vec<ConstraintMap> = vec![];
        let mut res_constraints: Vec<Constraints> = vec![];

        // FIXME mod store if have effects
        match self.interp_bool(funcs, cmap, scope, condition.clone()) {
            Ok((bool_res, bconstraints)) => {
                let true_cmap = cmap.clone();
                let false_cmap = cmap.clone();
                if self.possible(&bool_res) {
                    match vec![true_cmap.clone(), bconstraints.true_branch.clone()]
                        .merge()
                    {
                        Ok(Some(mut new_cmap)) => match self.interp(
                            funcs,
                            &mut new_cmap,
                            traits,
                            scope,
                            true_branch,
                        ) {
                            Ok(true_constraints_opt) => {
                                // remove true branch constraints from
                                // resulting cmap (safe b/c of SSA guarantee)
                                match new_cmap.diff(&bconstraints.true_branch) {
                                    Ok(()) => res_cmap.push(new_cmap),
                                    Err(err) => return Err(err),
                                }
                                if let Some(c) = true_constraints_opt {
                                    res_constraints.push(c);
                                }
                            }
                            err @ Err(_) => return err,
                        },
                        Ok(None) => todo!("got none"),
                        Err(err) => return Err(err),
                    }
                }
                if self.possible(!&bool_res) {
                    match vec![false_cmap.clone(), bconstraints.false_branch.clone()]
                        .merge()
                    {
                        Ok(Some(mut new_cmap)) => match self.interp(
                            funcs,
                            &mut new_cmap,
                            traits,
                            scope,
                            false_branch,
                        ) {
                            Ok(false_constraints_opt) => {
                                // remove false branch constraints from
                                // resulting cmap (safe b/c of SSA guarantee)
                                match new_cmap.diff(&bconstraints.false_branch) {
                                    Ok(()) => res_cmap.push(new_cmap),
                                    Err(err) => return Err(err),
                                }
                                if let Some(c) = false_constraints_opt {
                                    res_constraints.push(c);
                                }
                            }
                            err @ Err(_) => return err,
                        },
                        Ok(None) => todo!("got none"),
                        Err(err) => return Err(err),
                    }
                }
            }
            Err(c_err) => return Err(c_err),
        }

        match res_cmap.merge() {
            Ok(Some(new_cmap)) => {
                *cmap = new_cmap;
            }
            Ok(None) => {}
            Err(err) => return Err(err),
        }

        match res_constraints.merge() {
            res @ Ok(_) => res,
            Err(err) => Err(err),
        }
    }

    fn possible(&self, possible_b: &BStatement) -> bool {
        match possible_b {
            BStatement::True() => true,
            BStatement::False() => false,
            BStatement::TrueOrFalse() => true,
            _ => panic!("boolean statement not fully evaluated"),
        }
    }

    fn interp_switch(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        traits: &mut Traits,
        scope: Option<&'static str>,
        val: &RVal,
        vec: &Vec<(RVal, Box<Statement>)>,
    ) -> Result<Option<Constraints>, Error> {
        // FIXME mod store if have effects
        let resolved_constraints = match val {
            RVal::Num(_)
            | RVal::IdkNum()
            | RVal::Struct(..)
            | RVal::IdkStruct(_)
            | RVal::IdkVar() => (HashSet::from([val.clone()]), HashSet::new()),
            RVal::Var(varname) => match cmap.scoped_get(scope, varname, false) {
                Ok(Some(vartype)) => match vartype {
                    VarType::Values(_, constraints) => constraints.clone(),
                    _ => return Err(Error::NoSwitchOnFuncPtr()),
                },
                Ok(None) => {
                    return Err(Error::UndefinedSymbol(varname));
                }
                Err(err) => return Err(err),
            },
        };

        // filter out switch cases not possible given positive constraints
        let mut filtered: Vec<_> = vec
            .clone()
            .into_iter()
            .filter(|(case_value, _)| resolved_constraints.0.contains(case_value))
            .collect();

        // filter out switch cases not possible given negative constraints
        filtered = filtered
            .clone()
            .into_iter()
            .filter(|(case_value, _)| !resolved_constraints.1.contains(case_value))
            .collect();

        // loop to interp all such elems
        let mut res_cmap: Vec<ConstraintMap> = Vec::new();
        for (_, vec_stmt) in filtered.iter() {
            let mut scoped_cmap = cmap.clone();
            match self.interp(funcs, &mut scoped_cmap, traits, scope, vec_stmt) {
                Ok(_) => res_cmap.push(scoped_cmap),
                err @ Err(_) => return err,
            }
        }

        match res_cmap.merge() {
            Ok(Some(new_cmap)) => {
                *cmap = new_cmap;
                Ok(None)
            }
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn interp_return(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        _traits: &mut Traits,
        scope: Option<&'static str>,
        rval: &RVal,
    ) -> Result<Option<Constraints>, Error> {
        match rval {
            RVal::Num(_)
            | RVal::IdkNum()
            | RVal::Struct(..)
            | RVal::IdkStruct(_)
            | RVal::IdkVar() => Ok(Some((HashSet::from([rval.clone()]), HashSet::new()))),
            RVal::Var(varname) => match cmap.scoped_get(scope, varname, false) {
                Ok(Some(vartype)) => match vartype {
                    VarType::Values(_, constraints) => Ok(Some(constraints)),
                    VarType::Scope(..) => match funcs.funcs.get(varname) {
                        Some(_funcvec) => Ok(Some((
                            HashSet::from([RVal::Var(varname)]),
                            HashSet::new(),
                        ))),
                        None => Err(Error::UndefinedSymbol(varname)),
                    },
                },
                Ok(None) => match funcs.funcs.get(varname) {
                    Some(_) => panic!("IP BUG: func should have been a scope in cmap"),
                    None => Err(Error::UndefinedSymbol(varname)),
                },
                Err(err) => Err(err),
            },
        }
    }

    // if arg_type == trait and param_type == struct that impls that trait,
    // prune arg_constraints
    fn prune(
        &self,
        traits: &mut Traits,
        param_type: &Type,
        arg_type: &Type,
        arg_constraints: &mut Constraints,
    ) {
        match arg_type {
            Type::DynTrait(trait_name) => match traits.traits.get(trait_name) {
                Some(structs) => match param_type {
                    Type::Struct(struct_name) => {
                        if structs.contains(struct_name) {
                            for pos_constraint in arg_constraints.0.clone().iter() {
                                match pos_constraint {
                                    RVal::Struct(c_struct_name, ..)
                                    | RVal::IdkStruct(c_struct_name) => {
                                        // remove if any struct type other than
                                        // that of param_type
                                        if c_struct_name != struct_name {
                                            arg_constraints.0.remove(pos_constraint);
                                        }
                                    }
                                    _ => continue,
                                }
                            }
                        } else {
                            panic!(
                                "struct {} does not impl trait {}",
                                &struct_name, &trait_name
                            );
                        }
                    }
                    _ => {}
                },
                None => panic!("trait {} not implemented", &trait_name),
            },
            _ => {}
        }
    }

    fn resolve_args(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        traits: &mut Traits,
        scope: Option<&'static str>,
        name: &'static str,
        funcval: &FuncVal,
        args: &Vec<&'static str>,
    ) -> Result<(), Error> {
        let mut func_cmap = ConstraintMap::new();
        for ((param_name, param_type), arg) in
            std::iter::zip(funcval.params.clone(), args)
        {
            match cmap.scoped_get(scope, arg, false) {
                Ok(Some(vartype)) => {
                    match vartype {
                        // add args to scope
                        VarType::Values(valstype, mut constraints) => {
                            // prune constraints that are either redundant
                            // or invalid, given a trait param_type
                            //
                            // if the "bad" constraint is pos, then remove it
                            // - i.e. if param_type is a Struct(Cat), but have a
                            // value constraint of ({Struct(Dog)}, {}), remove
                            // pos-Struct(Dog)
                            //
                            // TODO neg example?
                            self.prune(traits, &param_type, &valstype, &mut constraints);

                            func_cmap.cmap.insert(
                                param_name,
                                Box::new(VarType::Values(
                                    Box::new(param_type),
                                    constraints.clone(),
                                )),
                            )
                        }
                        _ => todo!("not impl yet (funcs as args)"),
                    }
                }
                Ok(None) => match funcs.funcs.get(arg) {
                    Some(_funcvec) => {
                        todo!("not impl yet (funcs as args)");
                    }
                    None => {
                        return Err(Error::UndefinedSymbol(arg));
                    }
                },
                Err(err) => return Err(err),
            };
        }

        let (_, paramtypes): (Vec<&'static str>, Vec<Type>) =
            funcval.params.clone().into_iter().unzip();
        match cmap.cmap.get(name) {
            Some(vartype) => match *vartype.clone() {
                VarType::Scope(existing_scope, existing_inst_vecs) => {
                    if existing_scope != scope {
                        panic!("scopes differ, include in vec");
                    }

                    let mut new_inst_vecs = existing_inst_vecs.clone();
                    new_inst_vecs.push((
                        Box::new(Type::Func(paramtypes, funcval.rettype.clone())),
                        func_cmap,
                    ));
                    cmap.cmap
                        .insert(name, Box::new(VarType::Scope(scope, new_inst_vecs)));
                }
                _ => panic!("got values when expected scope"),
            },
            None => {
                cmap.cmap.insert(
                    name,
                    Box::new(VarType::Scope(
                        scope,
                        vec![(
                            Box::new(Type::Func(paramtypes, funcval.rettype.clone())),
                            func_cmap,
                        )],
                    )),
                );
            }
        }

        Ok(())
    }

    fn interp_indirect_invoke_helper(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        traits: &mut Traits,
        scope: Option<&'static str>,
        name: &'static str,
        val: &RVal,
        args: &Vec<&'static str>,
    ) -> Result<Option<Constraints>, Error> {
        match val {
            RVal::Var(varname) => match funcs.funcs.get(varname) {
                Some(funcvec) => {
                    let mut constraints_vec = vec![];

                    for (_tso, funcval) in funcvec.iter() {
                        self.resolve_args(
                            funcs, cmap, traits, scope, varname, funcval, args,
                        )?;

                        match self.interp(
                            funcs,
                            cmap,
                            traits,
                            Some(varname),
                            &funcval.body,
                        ) {
                            Ok(constraints) => {
                                if let Some(c) = constraints {
                                    constraints_vec.push(c);
                                }
                            }
                            err @ Err(_) => return err,
                        }
                    }

                    match constraints_vec.merge() {
                        Ok(constraints) => Ok(constraints),
                        Err(err) => Err(err),
                    }
                }
                None => Err(Error::UndefinedSymbol(varname)),
            },
            _ => Err(Error::NotAFunction(name)),
        }
    }

    fn interp_indirect_invoke(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        traits: &mut Traits,
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
                traits,
                scope,
                name,
                val,
                args,
            ) {
                Ok(constraints) => {
                    res_cmap.push(cmap_clone);
                    if let Some(c) = constraints {
                        constraints_vec.push(c);
                    }
                }
                err @ Err(_) => return err,
            }
        }

        let res = res_cmap.merge()?;
        if res.is_some() {
            *cmap = res.unwrap();
        }

        if !constraints_vec.is_empty() {
            return constraints_vec.merge();
        }
        Ok(None)
    }

    fn prune_funcvec(
        &self,
        cmap: &mut ConstraintMap,
        scope: Option<&'static str>,
        funcvec: &Vec<(TraitStructOpt, FuncVal)>,
        args: &Vec<&'static str>,
    ) -> Result<Vec<(TraitStructOpt, FuncVal)>, Error> {
        if !funcvec.is_empty() && funcvec[0].1.is_method && !args.is_empty() {
            match cmap.scoped_get(scope, args[0], false) {
                Ok(Some(vartype)) => match vartype {
                    VarType::Values(_, constraints) => {
                        let mut new_funcvec = vec![];

                        for (tso, funcval) in funcvec.iter() {
                            let self_type = &funcval.params[0].1;
                            match self_type {
                                Type::Struct(sname) => {
                                    // if no match in positive constraints, remove
                                    let mut mtch = false;
                                    for c in &constraints.0 {
                                        match c {
                                            RVal::IdkStruct(other_sname)
                                            | RVal::Struct(other_sname, ..) => {
                                                if *sname == *other_sname {
                                                    mtch = true;
                                                }
                                            }
                                            _ => todo!("c: {:?}", &c),
                                        }
                                    }
                                    if mtch {
                                        new_funcvec.push((*tso, funcval.clone()));
                                    }
                                }
                                _ => todo!(),
                            }
                        }

                        return Ok(new_funcvec);
                    }
                    _ => return Err(Error::UnexpectedScope()),
                },
                Ok(None) => {}
                Err(err) => return Err(err),
            }
        }

        Ok(funcvec.to_vec())
    }

    fn interp_direct_invoke(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        traits: &mut Traits,
        scope: Option<&'static str>,
        name: &'static str,
        args: &Vec<&'static str>,
    ) -> Result<Option<Constraints>, Error> {
        match funcs.funcs.get(&name) {
            Some(funcvec) => {
                let mut results_vec = vec![];
                let mut cmap_vec = vec![];

                // before launching into interpreting all possible funcvals, figure out
                // which funcvals can be pruned.
                // if first arg == self, use constraints to prune funcvals.
                let new_funcvec = self.prune_funcvec(cmap, scope, funcvec, args)?;

                // interpret remaining funcval options
                for (_, funcval) in new_funcvec.iter() {
                    let mut cmap_clone = cmap.clone();
                    self.resolve_args(
                        funcs,
                        &mut cmap_clone,
                        traits,
                        scope,
                        name,
                        funcval,
                        args,
                    )?;

                    match self.interp(
                        funcs,
                        &mut cmap_clone,
                        traits,
                        Some(name),
                        &*funcval.body,
                    ) {
                        Ok(constraints) => {
                            if let Some(c) = constraints {
                                results_vec.push(c);
                            }
                            cmap_vec.push(cmap_clone);
                        }
                        err @ Err(_) => return err,
                    }
                }

                match cmap_vec.merge() {
                    Ok(Some(new_cmap)) => {
                        *cmap = new_cmap;
                    }
                    Ok(None) => {}
                    Err(err) => return Err(err),
                }

                match results_vec.merge() {
                    Ok(constraints) => Ok(constraints),
                    Err(err) => Err(err),
                }
            }
            None => Err(Error::UndefinedSymbol(name)),
        }
    }

    fn interp_invoke(
        &self,
        funcs: &Funcs,
        cmap: &mut ConstraintMap,
        traits: &mut Traits,
        scope: Option<&'static str>,
        name: &'static str,
        args: &Vec<&'static str>,
    ) -> Result<Option<Constraints>, Error> {
        match cmap.scoped_get(scope, name, false) {
            Ok(Some(vartype)) => match vartype {
                VarType::Values(_, constraints) => self.interp_indirect_invoke(
                    funcs,
                    cmap,
                    traits,
                    scope,
                    name,
                    &constraints,
                    args,
                ),
                _ => Err(Error::UnexpectedScope()),
            },
            Ok(None) => self.interp_direct_invoke(funcs, cmap, traits, scope, name, args),
            Err(err) => Err(err),
        }
    }

    fn interp_traitdecl(
        &self,
        _funcs: &Funcs,
        _cmap: &mut ConstraintMap,
        traits: &mut Traits,
        _scope: Option<&'static str>,
        trait_name: &'static str,
    ) -> Result<Option<Constraints>, Error> {
        match traits.traits.get(trait_name) {
            Some(_) => {
                panic!("trait with name {} already declared", &trait_name)
            }
            None => {
                traits.traits.insert(trait_name, vec![]);
                Ok(None)
            }
        }
    }

    fn interp_traitimpl(
        &self,
        _funcs: &Funcs,
        _cmap: &mut ConstraintMap,
        traits: &mut Traits,
        _scope: Option<&'static str>,
        trait_name: &'static str,
        struct_name: &'static str,
    ) -> Result<Option<Constraints>, Error> {
        match traits.traits.get(trait_name) {
            Some(trait_vec) => {
                let mut updated_trait_vec = trait_vec.clone();
                updated_trait_vec.push(struct_name);
                traits.traits.insert(trait_name, updated_trait_vec);
                Ok(None)
            }
            None => panic!("trait {} does not exist", &trait_name),
        }
    }
}

#[cfg(test)]
mod interpret_tests;
