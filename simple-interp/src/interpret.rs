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
            Statement::FuncDef(..) | Statement::Struct(..) => Ok(None),
            Statement::TraitDecl(trait_name, _, _) => {
                self.interp_traitdecl(funcs, cmap, traits, scope, trait_name)
            }
            Statement::TraitImpl(trait_name, struct_name, _, _) => {
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
                    &*condition,
                    &*true_branch,
                    &*false_branch,
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
            Statement::InvokeTraitFunc(..) => Ok(None),
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
            let res = self.interp(&funcs, cmap, traits, scope, &*stmt)?;
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
        match self.interp_invoke(funcs, cmap, traits, scope, name, &args) {
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
            AssignmentRVal::RVal(RVal::Struct(struct_name, field_values)) => cmap
                .scoped_set(
                    scope,
                    var,
                    Box::new(VarType::Values(
                        Box::new(Type::Struct(*struct_name)),
                        (
                            HashSet::from([RVal::Struct(
                                *struct_name,
                                field_values.to_vec(),
                            )]),
                            HashSet::new(),
                        ),
                    )),
                )?,
            AssignmentRVal::RVal(RVal::IdkStruct(struct_name)) => cmap.scoped_set(
                scope,
                var,
                Box::new(VarType::Values(
                    Box::new(Type::Struct(*struct_name)),
                    (
                        HashSet::from([RVal::IdkStruct(*struct_name)]),
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
                    VarType::Values(_, constraints) => {
                        return Ok(constraints.clone());
                    }
                    _ => todo!("should be a func"),
                },
                Ok(None) => match funcs.funcs.get(&var) {
                    Some(_funcvec) => Ok((HashSet::from([rval]), HashSet::new())),
                    None => {
                        return Err(Error::UndefinedSymbol(var));
                    }
                },
                Err(err) => return Err(err),
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
                    return Ok((BStatement::True(), BConstraints::empty()));
                } else {
                    return Ok((BStatement::False(), BConstraints::empty()));
                }
            }
            (RVal::Var(lfp), RVal::Var(rfp)) => {
                if lfp == rfp {
                    return Ok((BStatement::True(), BConstraints::empty()));
                } else {
                    return Ok((BStatement::False(), BConstraints::empty()));
                }
            }
            (_, _) => {
                return Err(Error::IncomparableTypes(
                    lhs_vecs.0[0].clone(),
                    rhs_vecs.0[0].clone(),
                ));
            }
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

                return Ok((
                    BStatement::TrueOrFalse(),
                    BConstraints::new(true_branch, false_branch),
                ));
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
            &funcs,
            &cmap,
            scope,
            lhs.clone(),
        )?);
        let rhs_vecs = self.constraints_to_vecs(&self.interp_rval(
            &funcs,
            &cmap,
            scope,
            rhs.clone(),
        )?);

        // eval directly if only a single positive constraint
        if lhs_vecs.0.len() == 1
            && lhs_vecs.1.len() == 0
            && rhs_vecs.0.len() == 1
            && rhs_vecs.1.len() == 0
        {
            return self.single_constraint_compare(&lhs_vecs, &rhs_vecs);
        } else {
            return self.many_constraints_compare(cmap, lhs, rhs);
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
                                if true_constraints_opt.is_some() {
                                    res_constraints.push(true_constraints_opt.unwrap());
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
                                if false_constraints_opt.is_some() {
                                    res_constraints.push(false_constraints_opt.unwrap());
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
            match self.interp(funcs, &mut scoped_cmap, traits, scope, &*vec_stmt) {
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
            | RVal::IdkVar() => {
                return Ok(Some((HashSet::from([rval.clone()]), HashSet::new())));
            }
            RVal::Var(varname) => match cmap.scoped_get(scope, varname, false) {
                Ok(Some(vartype)) => match vartype {
                    VarType::Values(_, constraints) => Ok(Some(constraints)),
                    VarType::Scope(..) => match funcs.funcs.get(varname) {
                        Some(_funcvec) => Ok(Some((
                            HashSet::from([RVal::Var(*varname)]),
                            HashSet::new(),
                        ))),
                        None => {
                            return Err(Error::UndefinedSymbol(varname));
                        }
                    },
                },
                Ok(None) => match funcs.funcs.get(varname) {
                    Some(_) => panic!("IP BUG: func should have been a scope in cmap"),
                    None => {
                        return Err(Error::UndefinedSymbol(varname));
                    }
                },
                Err(err) => return Err(err),
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
    ) -> () {
        match arg_type {
            Type::DynTrait(trait_name) => match traits.traits.get(trait_name) {
                Some(structs) => match param_type {
                    Type::Struct(struct_name) => {
                        if structs.contains(&struct_name) {
                            for pos_constraint in arg_constraints.0.clone().iter() {
                                match pos_constraint {
                                    RVal::Struct(c_struct_name, _)
                                    | RVal::IdkStruct(c_struct_name) => {
                                        // remove if any struct type other than
                                        // that of param_type
                                        if c_struct_name != struct_name {
                                            arg_constraints.0.remove(&pos_constraint);
                                        }
                                    }
                                    _ => continue,
                                }
                            }
                            ()
                        } else {
                            panic!(
                                "struct {} does not impl trait {}",
                                &struct_name, &trait_name
                            );
                        }
                    }
                    _ => (),
                },
                None => panic!("trait {} not implemented", &trait_name),
            },
            _ => (),
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
                            funcs, cmap, traits, scope, varname, &funcval, args,
                        )?;

                        match self.interp(
                            funcs,
                            cmap,
                            traits,
                            Some(varname),
                            &*funcval.body,
                        ) {
                            Ok(constraints) => {
                                if constraints.is_some() {
                                    constraints_vec.push(constraints.unwrap());
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
                None => {
                    return Err(Error::UndefinedSymbol(varname));
                }
            },
            _ => return Err(Error::NotAFunction(name)),
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
                    if constraints.is_some() {
                        constraints_vec.push(constraints.unwrap());
                    }
                }
                err @ Err(_) => return err,
            }
        }

        let res = res_cmap.merge()?;
        if res.is_some() {
            *cmap = res.unwrap();
        }

        if constraints_vec.len() > 0 {
            return Ok(constraints_vec.merge()?);
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
        if funcvec.len() > 0 && funcvec[0].1.is_method && args.len() > 0 {
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
                                            RVal::IdkStruct(idk_sname) => {
                                                if *sname == *idk_sname {
                                                    mtch = true;
                                                }
                                            }
                                            _ => todo!(),
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
                let new_funcvec = self.prune_funcvec(cmap, scope, &funcvec, args)?;

                // interpret remaining funcval options
                for (_, funcval) in new_funcvec.iter() {
                    let mut cmap_clone = cmap.clone();
                    self.resolve_args(
                        funcs,
                        &mut cmap_clone,
                        traits,
                        scope,
                        name,
                        &funcval,
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
                            if constraints.is_some() {
                                results_vec.push(constraints.unwrap());
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
            None => {
                return Err(Error::UndefinedSymbol(name));
            }
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
                _ => return Err(Error::UnexpectedScope()),
            },
            Ok(None) => self.interp_direct_invoke(funcs, cmap, traits, scope, name, args),
            Err(err) => return Err(err),
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
mod tests {
    use super::*;
    use crate::funcs::Funcs;
    use crate::statement::Statement::{
        Assignment, Conditional, FuncDef, InvokeFunc, Print, Return, Sequence, Struct,
        Switch, TraitDecl, TraitImpl,
    };
    use crate::statement::{FuncDecl, FuncVal, Type};

    #[test]
    fn test_merge_none() {
        let vec: Vec<ConstraintMap> = Vec::new();
        assert_eq!(vec.merge(), Ok(None));
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
        assert_eq!(vec[0].clone(), vec.merge().unwrap().unwrap());
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
        assert_eq!(end_cmap, vec.merge().unwrap().unwrap());
    }

    #[test]
    fn test_print() {
        let interp = Interpreter::new();
        let stmt = Print("hello");
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, ConstraintMap::new());
    }

    #[test]
    fn test_assign_num() {
        let interp = Interpreter::new();
        let stmt = Assignment("x", Box::new(AssignmentRVal::RVal(RVal::Num(5))));
        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
            Box::new(BStatement::True()),
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
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
            Box::new(BStatement::False()),
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
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
            Box::new(BStatement::TrueOrFalse()),
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
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
            Box::new(BStatement::Not(Box::new(BStatement::True()))),
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
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
                Box::new(BStatement::Equals(RVal::Var("x"), RVal::Var("y"))),
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
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, foo_body.clone()))],
        );
        funcs.funcs.insert(
            "bar",
            vec![(None, FuncVal::new(false, vec![], None, foo_body.clone()))],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();

        // note: `equals` is _shallow_, which is why it evals to false here
        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, foo_body.clone())),
            Box::new(FuncDef("bar", false, vec![], None, foo_body.clone())),
            Box::new(Conditional(
                Box::new(BStatement::Equals(RVal::Var("foo"), RVal::Var("bar"))),
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
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, foo_body.clone()))],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, foo_body.clone())),
            Box::new(Assignment(
                "bar",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(Conditional(
                Box::new(BStatement::Equals(RVal::Var("foo"), RVal::Var("bar"))),
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
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
                Box::new(BStatement::TrueOrFalse()),
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
                Box::new(BStatement::Equals(RVal::Var("x"), RVal::Var("y"))),
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
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, foo_body.clone()))],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, foo_body)),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Conditional(
                Box::new(BStatement::Equals(RVal::Var("foo"), RVal::Var("x"))),
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
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

        assert_eq!(
            res,
            Err(Error::IncomparableTypes(RVal::Var("foo"), RVal::Num(5)))
        );
    }

    #[test]
    fn test_nested_conditional() {
        let interp = Interpreter::new();
        let stmt = Conditional(
            Box::new(BStatement::True()),
            Box::new(Conditional(
                Box::new(BStatement::True()),
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
                Box::new(BStatement::True()),
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
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
            Box::new(BStatement::True()),
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
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, body.clone()))],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();

        let interp = Interpreter::new();
        let stmt = FuncDef("foo", false, vec![], None, body);
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, body.clone()))],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, body)),
            Box::new(InvokeFunc("foo", vec![])),
        ]);
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
                None,
                vec![(Box::new(Type::Func(vec![], None)), foo_cmap)],
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_funcdef_args_direct() {
        let mut funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();
        let interp = Interpreter::new();

        let body = Box::new(Sequence(vec![
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Conditional(
                Box::new(BStatement::Equals(RVal::Var("x"), RVal::Var("y"))),
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
                false,
                vec![("y", Type::Int())],
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
            "foo",
            vec![(
                None,
                FuncVal::new(false, vec![("y", Type::Int())], None, body.clone()),
            )],
        );
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
                None,
                vec![(Box::new(Type::Func(vec![Type::Int()], None)), foo_cmap)],
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
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, body.clone()))],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, body)),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]);
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
                None,
                vec![(Box::new(Type::Func(vec![], None)), foo_cmap)],
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_funcdef_args_indirect() {
        let mut funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();
        let interp = Interpreter::new();

        let body = Box::new(Sequence(vec![
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Conditional(
                Box::new(BStatement::Equals(RVal::Var("x"), RVal::Var("y"))),
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
                false,
                vec![("y", Type::Int())],
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
            "foo",
            vec![(
                None,
                FuncVal::new(false, vec![("y", Type::Int())], None, body.clone()),
            )],
        );
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
                None,
                vec![(Box::new(Type::Func(vec![Type::Int()], None)), foo_cmap)],
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
            Box::new(FuncDef("bar", false, vec![], None, bar_body.clone())),
            Box::new(InvokeFunc("bar", vec![])),
        ]));
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, foo_body.clone())),
            Box::new(InvokeFunc("foo", vec![])),
        ]);

        let mut funcs = Funcs::new();
        funcs.funcs.insert(
            "bar",
            vec![(None, FuncVal::new(false, vec![], None, bar_body))],
        );
        funcs.funcs.insert(
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, foo_body))],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();

        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
                Some("foo"),
                vec![(Box::new(Type::Func(vec![], None)), bar_cmap)],
            )),
        );
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                None,
                vec![(Box::new(Type::Func(vec![], None)), foo_cmap)],
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
            Box::new(FuncDef("baz", false, vec![], None, baz_body.clone())),
            Box::new(FuncDef("qux", false, vec![], None, qux_body.clone())),
            Box::new(Conditional(
                Box::new(BStatement::TrueOrFalse()),
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
            Box::new(FuncDef("baz2", false, vec![], None, baz2_body.clone())),
            Box::new(FuncDef("qux2", false, vec![], None, qux2_body.clone())),
            Box::new(Conditional(
                Box::new(BStatement::TrueOrFalse()),
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
            Box::new(FuncDef("foo", false, vec![], None, foo_body.clone())),
            Box::new(FuncDef("bar", false, vec![], None, bar_body.clone())),
            Box::new(Conditional(
                Box::new(BStatement::TrueOrFalse()),
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
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, foo_body.clone()))],
        );
        funcs.funcs.insert(
            "bar",
            vec![(None, FuncVal::new(false, vec![], None, bar_body.clone()))],
        );
        funcs.funcs.insert(
            "baz",
            vec![(None, FuncVal::new(false, vec![], None, baz_body.clone()))],
        );
        funcs.funcs.insert(
            "qux",
            vec![(None, FuncVal::new(false, vec![], None, qux_body.clone()))],
        );
        funcs.funcs.insert(
            "baz2",
            vec![(None, FuncVal::new(false, vec![], None, baz2_body.clone()))],
        );
        funcs.funcs.insert(
            "qux2",
            vec![(None, FuncVal::new(false, vec![], None, qux2_body.clone()))],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
                Some("bar"),
                vec![(Box::new(Type::Func(vec![], None)), baz2_cmap)],
            )),
        );
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                None,
                vec![(Box::new(Type::Func(vec![], None)), foo_cmap)],
            )),
        );
        check_cmap.cmap.insert(
            "qux",
            Box::new(VarType::Scope(
                Some("foo"),
                vec![(Box::new(Type::Func(vec![], None)), qux_cmap)],
            )),
        );
        check_cmap.cmap.insert(
            "baz",
            Box::new(VarType::Scope(
                Some("foo"),
                vec![(Box::new(Type::Func(vec![], None)), baz_cmap)],
            )),
        );
        check_cmap.cmap.insert(
            "bar",
            Box::new(VarType::Scope(
                None,
                vec![(Box::new(Type::Func(vec![], None)), bar_cmap)],
            )),
        );
        check_cmap.cmap.insert(
            "qux2",
            Box::new(VarType::Scope(
                Some("bar"),
                vec![(Box::new(Type::Func(vec![], None)), qux2_cmap)],
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
                false,
                vec![("y", Type::Int())],
                None,
                bar_body.clone(),
            )),
            Box::new(InvokeFunc("bar", vec!["y"])),
        ]));
        let stmt = Sequence(vec![
            Box::new(FuncDef(
                "foo",
                false,
                vec![("y", Type::Int())],
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
            "bar",
            vec![(
                None,
                FuncVal::new(false, vec![("y", Type::Int())], None, bar_body),
            )],
        );
        funcs.funcs.insert(
            "foo",
            vec![(
                None,
                FuncVal::new(false, vec![("y", Type::Int())], None, foo_body),
            )],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();

        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
                Some("foo"),
                vec![(Box::new(Type::Func(vec![Type::Int()], None)), bar_cmap)],
            )),
        );
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                None,
                vec![(Box::new(Type::Func(vec![Type::Int()], None)), foo_cmap)],
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
                false,
                vec![("y", Type::Int())],
                None,
                baz_body.clone(),
            )),
            Box::new(FuncDef(
                "qux",
                false,
                vec![("y", Type::Int())],
                None,
                qux_body.clone(),
            )),
            Box::new(Conditional(
                Box::new(BStatement::TrueOrFalse()),
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
                false,
                vec![("y", Type::Int())],
                None,
                baz2_body.clone(),
            )),
            Box::new(FuncDef(
                "qux2",
                false,
                vec![("y", Type::Int())],
                None,
                qux2_body.clone(),
            )),
            Box::new(Conditional(
                Box::new(BStatement::TrueOrFalse()),
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
                false,
                vec![("y", Type::Int())],
                None,
                foo_body.clone(),
            )),
            Box::new(FuncDef(
                "bar",
                false,
                vec![("y", Type::Int())],
                None,
                bar_body.clone(),
            )),
            Box::new(Conditional(
                Box::new(BStatement::TrueOrFalse()),
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
            "foo",
            vec![(
                None,
                FuncVal::new(false, vec![("y", Type::Int())], None, foo_body.clone()),
            )],
        );
        funcs.funcs.insert(
            "bar",
            vec![(
                None,
                FuncVal::new(false, vec![("y", Type::Int())], None, bar_body.clone()),
            )],
        );
        funcs.funcs.insert(
            "baz",
            vec![(
                None,
                FuncVal::new(false, vec![("y", Type::Int())], None, baz_body.clone()),
            )],
        );
        funcs.funcs.insert(
            "qux",
            vec![(
                None,
                FuncVal::new(false, vec![("y", Type::Int())], None, qux_body.clone()),
            )],
        );
        funcs.funcs.insert(
            "baz2",
            vec![(
                None,
                FuncVal::new(false, vec![("y", Type::Int())], None, baz2_body.clone()),
            )],
        );
        funcs.funcs.insert(
            "qux2",
            vec![(
                None,
                FuncVal::new(false, vec![("y", Type::Int())], None, qux2_body.clone()),
            )],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
                Some("foo"),
                vec![(Box::new(Type::Func(vec![Type::Int()], None)), baz_cmap)],
            )),
        );
        check_cmap.cmap.insert(
            "bar",
            Box::new(VarType::Scope(
                None,
                vec![(Box::new(Type::Func(vec![Type::Int()], None)), bar_cmap)],
            )),
        );
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                None,
                vec![(Box::new(Type::Func(vec![Type::Int()], None)), foo_cmap)],
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
                Some("foo"),
                vec![(Box::new(Type::Func(vec![Type::Int()], None)), qux_cmap)],
            )),
        );
        check_cmap.cmap.insert(
            "baz2",
            Box::new(VarType::Scope(
                Some("bar"),
                vec![(Box::new(Type::Func(vec![Type::Int()], None)), baz2_cmap)],
            )),
        );
        check_cmap.cmap.insert(
            "qux2",
            Box::new(VarType::Scope(
                Some("bar"),
                vec![(Box::new(Type::Func(vec![Type::Int()], None)), qux2_cmap)],
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
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, foo_body.clone()))],
        );
        funcs.funcs.insert(
            "bar",
            vec![(None, FuncVal::new(false, vec![], None, bar_body.clone()))],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, foo_body)),
            Box::new(FuncDef("bar", false, vec![], None, bar_body)),
            Box::new(Conditional(
                Box::new(BStatement::TrueOrFalse()),
                Box::new(InvokeFunc("foo", vec![])),
                Box::new(InvokeFunc("bar", vec![])),
            )),
        ]);
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
                None,
                vec![(Box::new(Type::Func(vec![], None)), foo_cmap)],
            )),
        );
        check_cmap.cmap.insert(
            "bar",
            Box::new(VarType::Scope(
                None,
                vec![(Box::new(Type::Func(vec![], None)), bar_cmap)],
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
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, foo_body.clone()))],
        );
        funcs.funcs.insert(
            "bar",
            vec![(None, FuncVal::new(false, vec![], None, bar_body.clone()))],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, foo_body)),
            Box::new(FuncDef("bar", false, vec![], None, bar_body)),
            Box::new(Conditional(
                Box::new(BStatement::TrueOrFalse()),
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
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
                None,
                vec![(Box::new(Type::Func(vec![], None)), foo_cmap)],
            )),
        );
        check_cmap.cmap.insert(
            "bar",
            Box::new(VarType::Scope(
                None,
                vec![(Box::new(Type::Func(vec![], None)), bar_cmap)],
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
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
            &mut Traits::new(),
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
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
                Box::new(BStatement::TrueOrFalse()),
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
        let mut traits = Traits::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
            "foo",
            vec![(
                None,
                FuncVal::new(false, vec![], Some(Box::new(Type::Int())), body.clone()),
            )],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef(
                "foo",
                false,
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
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                None,
                vec![(
                    Box::new(Type::Func(vec![], Some(Box::new(Type::Int())))),
                    ConstraintMap::new(),
                )],
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
                Box::new(BStatement::TrueOrFalse()),
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
            "foo",
            vec![(
                None,
                FuncVal::new(false, vec![], Some(Box::new(Type::Int())), body.clone()),
            )],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef(
                "foo",
                false,
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
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
                None,
                vec![(
                    Box::new(Type::Func(vec![], Some(Box::new(Type::Int())))),
                    foo_cmap,
                )],
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
                Box::new(BStatement::TrueOrFalse()),
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
            "foo",
            vec![(
                None,
                FuncVal::new(
                    false,
                    vec![],
                    Some(Box::new(Type::Func(vec![], Some(rettype.clone())))),
                    foo_body.clone(),
                ),
            )],
        );
        funcs.funcs.insert(
            "bar",
            vec![(
                None,
                FuncVal::new(
                    false,
                    vec![],
                    Some(Box::new(Type::Int())),
                    bar_body.clone(),
                ),
            )],
        );
        funcs.funcs.insert(
            "baz",
            vec![(
                None,
                FuncVal::new(
                    false,
                    vec![],
                    Some(Box::new(Type::Int())),
                    baz_body.clone(),
                ),
            )],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef(
                "foo",
                false,
                vec![],
                Some(Box::new(Type::Func(vec![], Some(Box::new(Type::Int()))))),
                foo_body,
            )),
            Box::new(FuncDef(
                "bar",
                false,
                vec![],
                Some(Box::new(Type::Int())),
                bar_body,
            )),
            Box::new(FuncDef(
                "baz",
                false,
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
                Box::new(AssignmentRVal::Statement(Box::new(InvokeFunc("x", vec![])))),
            )),
        ]);
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
                None,
                vec![(
                    Box::new(Type::Func(
                        vec![],
                        Some(Box::new(Type::Func(vec![], Some(Box::new(Type::Int()))))),
                    )),
                    foo_cmap,
                )],
            )),
        );
        check_cmap.cmap.insert(
            "bar",
            Box::new(VarType::Scope(
                None,
                vec![(
                    Box::new(Type::Func(vec![], Some(Box::new(Type::Int())))),
                    ConstraintMap::new(),
                )],
            )),
        );
        check_cmap.cmap.insert(
            "baz",
            Box::new(VarType::Scope(
                None,
                vec![(
                    Box::new(Type::Func(vec![], Some(Box::new(Type::Int())))),
                    ConstraintMap::new(),
                )],
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
                Box::new(BStatement::TrueOrFalse()),
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
                Box::new(BStatement::Equals(RVal::Var("x"), RVal::Var("f"))),
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
        let mut traits = Traits::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
                Box::new(BStatement::TrueOrFalse()),
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
                Box::new(BStatement::Not(Box::new(BStatement::Equals(
                    RVal::Var("x"),
                    RVal::Var("f"),
                )))),
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
        let mut traits = Traits::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

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
        let mut traits = Traits::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "edgar",
            Box::new(VarType::Values(
                Box::new(Type::Struct("Cat")),
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

        let funcdef = FuncDecl::new(true, vec![("self", Type::DynTrait("Animal"))], None);
        let cat_funcimpl = FuncVal::new(
            true,
            vec![("self", Type::Struct("Cat"))],
            None,
            cat_speak_body.clone(),
        );

        let stmt = Sequence(vec![
            Box::new(TraitDecl("Animal", vec!["speak"], vec![funcdef.clone()])),
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
        funcs.funcs.insert(
            "speak",
            vec![(Some(("Animal", "Cat")), cat_funcimpl.clone())],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

        let mut check_cmap = ConstraintMap::new();
        check_cmap.cmap.insert(
            "edgar",
            Box::new(VarType::Values(
                Box::new(Type::Struct("Cat")),
                (HashSet::from([RVal::Struct("Cat", vec![])]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
    }

    #[test]
    fn test_dyn_traits_single_impl() {
        let funcdef = FuncDecl::new(true, vec![("self", Type::DynTrait("Animal"))], None);

        let cat_speak_body = Box::new(Assignment(
            "spoken",
            Box::new(AssignmentRVal::RVal(RVal::Num(1))),
        ));
        let cat_funcimpl = FuncVal::new(
            true,
            vec![("self", Type::Struct("Cat"))],
            None,
            cat_speak_body.clone(),
        );

        let gmaa = Box::new(Sequence(vec![Box::new(Return(RVal::IdkStruct("Cat")))]));

        let stmt = Sequence(vec![
            Box::new(TraitDecl("Animal", vec!["speak"], vec![funcdef.clone()])),
            Box::new(FuncDef(
                "giveMeAnAnimal",
                false,
                vec![],
                Some(Box::new(Type::DynTrait("Animal"))),
                gmaa.clone(),
            )),
            Box::new(Struct("Cat", vec![Type::Int()], vec!["age"])),
            Box::new(TraitImpl(
                "Animal",
                "Cat",
                vec!["speak"],
                vec![cat_funcimpl.clone()],
            )),
            Box::new(Assignment(
                "specific_animal",
                Box::new(AssignmentRVal::Statement(Box::new(Statement::InvokeFunc(
                    "giveMeAnAnimal",
                    vec![],
                )))),
            )),
            Box::new(InvokeFunc("speak", vec!["specific_animal"])),
        ]);

        let mut funcs = Funcs::new();
        funcs.funcs.insert(
            "speak",
            vec![(Some(("Animal", "Cat")), cat_funcimpl.clone())],
        );
        funcs.funcs.insert(
            "giveMeAnAnimal",
            vec![(
                None,
                FuncVal::new(
                    false,
                    vec![],
                    Some(Box::new(Type::DynTrait("Animal"))),
                    gmaa.clone(),
                ),
            )],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

        let mut check_traits = Traits::new();
        check_traits.traits.insert("Animal", vec!["Cat"]);

        let mut check_cmap = ConstraintMap::new();
        let mut speak_cmap = ConstraintMap::new();
        speak_cmap.cmap.insert(
            "self",
            Box::new(VarType::Values(
                Box::new(Type::Struct("Cat")),
                (HashSet::from([RVal::IdkStruct("Cat")]), HashSet::new()),
            )),
        );
        speak_cmap.cmap.insert(
            "spoken",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(1)]), HashSet::new()),
            )),
        );
        check_cmap.cmap.insert(
            "speak",
            Box::new(VarType::Scope(
                None,
                vec![(
                    Box::new(Type::Func(vec![Type::Struct("Cat")], None)),
                    speak_cmap,
                )],
            )),
        );
        check_cmap.cmap.insert(
            "giveMeAnAnimal",
            Box::new(VarType::Scope(
                None,
                vec![(
                    Box::new(Type::Func(
                        vec![],
                        Some(Box::new(Type::DynTrait("Animal"))),
                    )),
                    ConstraintMap::new(),
                )],
            )),
        );
        check_cmap.cmap.insert(
            "specific_animal",
            Box::new(VarType::Values(
                Box::new(Type::DynTrait("Animal")),
                (HashSet::from([RVal::IdkStruct("Cat")]), HashSet::new()),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
        assert_eq!(traits, check_traits);
    }

    #[test]
    fn test_dyn_traits_two_impl() {
        let funcdecl =
            FuncDecl::new(true, vec![("self", Type::DynTrait("Animal"))], None);

        let cat_speak_body = Box::new(Assignment(
            "spoken",
            Box::new(AssignmentRVal::RVal(RVal::Num(1))),
        ));
        let cat_speak = FuncVal::new(
            true,
            vec![("self", Type::Struct("Cat"))],
            None,
            cat_speak_body.clone(),
        );

        let dog_speak_body = Box::new(Assignment(
            "spoken",
            Box::new(AssignmentRVal::RVal(RVal::Num(2))),
        ));
        let dog_speak = FuncVal::new(
            true,
            vec![("self", Type::Struct("Dog"))],
            None,
            dog_speak_body.clone(),
        );

        let gmaa = Box::new(Sequence(vec![Box::new(Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(Sequence(vec![Box::new(Return(RVal::IdkStruct("Cat")))])),
            Box::new(Sequence(vec![Box::new(Return(RVal::IdkStruct("Dog")))])),
        ))]));

        let stmt = Sequence(vec![
            Box::new(TraitDecl("Animal", vec!["speak"], vec![funcdecl.clone()])),
            Box::new(FuncDef(
                "giveMeAnAnimal",
                false,
                vec![],
                Some(Box::new(Type::DynTrait("Animal"))),
                gmaa.clone(),
            )),
            Box::new(Struct("Cat", vec![Type::Int()], vec!["age"])),
            Box::new(Struct("Dog", vec![Type::Int()], vec!["age"])),
            Box::new(TraitImpl(
                "Animal",
                "Cat",
                vec!["speak"],
                vec![cat_speak.clone()],
            )),
            Box::new(TraitImpl(
                "Animal",
                "Dog",
                vec!["speak"],
                vec![dog_speak.clone()],
            )),
            Box::new(Assignment(
                "specific_animal",
                Box::new(AssignmentRVal::Statement(Box::new(Statement::InvokeFunc(
                    "giveMeAnAnimal",
                    vec![],
                )))),
            )),
            Box::new(InvokeFunc("speak", vec!["specific_animal"])),
        ]);

        let mut funcs = Funcs::new();
        funcs.funcs.insert(
            "speak",
            vec![
                (Some(("Animal", "Dog")), dog_speak.clone()),
                (Some(("Animal", "Cat")), cat_speak.clone()),
            ],
        );
        funcs.funcs.insert(
            "giveMeAnAnimal",
            vec![(
                None,
                FuncVal::new(
                    false,
                    vec![],
                    Some(Box::new(Type::DynTrait("Animal"))),
                    gmaa.clone(),
                ),
            )],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

        let mut check_traits = Traits::new();
        check_traits.traits.insert("Animal", vec!["Cat", "Dog"]);

        let mut check_cmap = ConstraintMap::new();
        let mut cat_speak_cmap = ConstraintMap::new();
        let mut dog_speak_cmap = ConstraintMap::new();

        dog_speak_cmap.cmap.insert(
            "self",
            Box::new(VarType::Values(
                Box::new(Type::Struct("Dog")),
                (HashSet::from([RVal::IdkStruct("Dog")]), HashSet::new()),
            )),
        );
        dog_speak_cmap.cmap.insert(
            "spoken",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(2)]), HashSet::new()),
            )),
        );

        cat_speak_cmap.cmap.insert(
            "self",
            Box::new(VarType::Values(
                Box::new(Type::Struct("Cat")),
                (HashSet::from([RVal::IdkStruct("Cat")]), HashSet::new()),
            )),
        );
        cat_speak_cmap.cmap.insert(
            "spoken",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(1)]), HashSet::new()),
            )),
        );

        check_cmap.cmap.insert(
            "speak",
            Box::new(VarType::Scope(
                None,
                vec![
                    (
                        Box::new(Type::Func(vec![Type::Struct("Dog")], None)),
                        dog_speak_cmap,
                    ),
                    (
                        Box::new(Type::Func(vec![Type::Struct("Cat")], None)),
                        cat_speak_cmap,
                    ),
                ],
            )),
        );
        check_cmap.cmap.insert(
            "giveMeAnAnimal",
            Box::new(VarType::Scope(
                None,
                vec![(
                    Box::new(Type::Func(
                        vec![],
                        Some(Box::new(Type::DynTrait("Animal"))),
                    )),
                    ConstraintMap::new(),
                )],
            )),
        );
        check_cmap.cmap.insert(
            "specific_animal",
            Box::new(VarType::Values(
                Box::new(Type::DynTrait("Animal")),
                (
                    HashSet::from([RVal::IdkStruct("Cat"), RVal::IdkStruct("Dog")]),
                    HashSet::new(),
                ),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
        assert_eq!(traits, check_traits);
    }

    #[test]
    fn test_dyn_traits_three_impl_two_used() {
        let funcdecl =
            FuncDecl::new(true, vec![("self", Type::DynTrait("Animal"))], None);

        let bird_speak_body = Box::new(Assignment(
            "spoken",
            Box::new(AssignmentRVal::RVal(RVal::Num(0))),
        ));
        let bird_speak = FuncVal::new(
            true,
            vec![("self", Type::Struct("Bird"))],
            None,
            bird_speak_body.clone(),
        );

        let cat_speak_body = Box::new(Assignment(
            "spoken",
            Box::new(AssignmentRVal::RVal(RVal::Num(1))),
        ));
        let cat_speak = FuncVal::new(
            true,
            vec![("self", Type::Struct("Cat"))],
            None,
            cat_speak_body.clone(),
        );

        let dog_speak_body = Box::new(Assignment(
            "spoken",
            Box::new(AssignmentRVal::RVal(RVal::Num(2))),
        ));
        let dog_speak = FuncVal::new(
            true,
            vec![("self", Type::Struct("Dog"))],
            None,
            dog_speak_body.clone(),
        );

        let gmaa = Box::new(Sequence(vec![Box::new(Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(Sequence(vec![Box::new(Return(RVal::IdkStruct("Cat")))])),
            Box::new(Sequence(vec![Box::new(Return(RVal::IdkStruct("Dog")))])),
        ))]));

        let stmt = Sequence(vec![
            Box::new(TraitDecl("Animal", vec!["speak"], vec![funcdecl.clone()])),
            Box::new(FuncDef(
                "giveMeAnAnimal",
                false,
                vec![],
                Some(Box::new(Type::DynTrait("Animal"))),
                gmaa.clone(),
            )),
            Box::new(Struct("Bird", vec![Type::Int()], vec!["age"])),
            Box::new(Struct("Cat", vec![Type::Int()], vec!["age"])),
            Box::new(Struct("Dog", vec![Type::Int()], vec!["age"])),
            Box::new(TraitImpl(
                "Animal",
                "Bird",
                vec!["speak"],
                vec![bird_speak.clone()],
            )),
            Box::new(TraitImpl(
                "Animal",
                "Cat",
                vec!["speak"],
                vec![cat_speak.clone()],
            )),
            Box::new(TraitImpl(
                "Animal",
                "Dog",
                vec!["speak"],
                vec![dog_speak.clone()],
            )),
            Box::new(Assignment(
                "specific_animal",
                Box::new(AssignmentRVal::Statement(Box::new(Statement::InvokeFunc(
                    "giveMeAnAnimal",
                    vec![],
                )))),
            )),
            Box::new(InvokeFunc("speak", vec!["specific_animal"])),
        ]);

        let mut funcs = Funcs::new();
        funcs.funcs.insert(
            "speak",
            vec![
                (Some(("Animal", "Dog")), dog_speak.clone()),
                (Some(("Animal", "Cat")), cat_speak.clone()),
                (Some(("Animal", "Bird")), bird_speak.clone()),
            ],
        );
        funcs.funcs.insert(
            "giveMeAnAnimal",
            vec![(
                None,
                FuncVal::new(
                    false,
                    vec![],
                    Some(Box::new(Type::DynTrait("Animal"))),
                    gmaa.clone(),
                ),
            )],
        );

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

        let mut check_traits = Traits::new();
        check_traits
            .traits
            .insert("Animal", vec!["Bird", "Cat", "Dog"]);

        let mut check_cmap = ConstraintMap::new();
        let mut cat_speak_cmap = ConstraintMap::new();
        let mut dog_speak_cmap = ConstraintMap::new();

        dog_speak_cmap.cmap.insert(
            "self",
            Box::new(VarType::Values(
                Box::new(Type::Struct("Dog")),
                (HashSet::from([RVal::IdkStruct("Dog")]), HashSet::new()),
            )),
        );
        dog_speak_cmap.cmap.insert(
            "spoken",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(2)]), HashSet::new()),
            )),
        );

        cat_speak_cmap.cmap.insert(
            "self",
            Box::new(VarType::Values(
                Box::new(Type::Struct("Cat")),
                (HashSet::from([RVal::IdkStruct("Cat")]), HashSet::new()),
            )),
        );
        cat_speak_cmap.cmap.insert(
            "spoken",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(1)]), HashSet::new()),
            )),
        );

        check_cmap.cmap.insert(
            "speak",
            Box::new(VarType::Scope(
                None,
                vec![
                    (
                        Box::new(Type::Func(vec![Type::Struct("Dog")], None)),
                        dog_speak_cmap,
                    ),
                    (
                        Box::new(Type::Func(vec![Type::Struct("Cat")], None)),
                        cat_speak_cmap,
                    ),
                ],
            )),
        );
        check_cmap.cmap.insert(
            "giveMeAnAnimal",
            Box::new(VarType::Scope(
                None,
                vec![(
                    Box::new(Type::Func(
                        vec![],
                        Some(Box::new(Type::DynTrait("Animal"))),
                    )),
                    ConstraintMap::new(),
                )],
            )),
        );
        check_cmap.cmap.insert(
            "specific_animal",
            Box::new(VarType::Values(
                Box::new(Type::DynTrait("Animal")),
                (
                    HashSet::from([RVal::IdkStruct("Cat"), RVal::IdkStruct("Dog")]),
                    HashSet::new(),
                ),
            )),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(cmap, check_cmap);
        assert_eq!(traits, check_traits);
    }
}
