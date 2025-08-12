use crate::constraints::{ConstraintMap, Constraints, VarType};
use crate::error::Error;
use crate::funcs::Funcs;
use crate::sigs::{SigVal, Sigs};
use crate::statement::RWStatement::{
    Assignment, Conditional, FuncDecl, FuncDef, InvokeFunc, InvokeTraitFunc, Print,
    Return, Sequence, Struct, Switch, TraitDecl, TraitImpl, TraitImplList, TraitSwitch,
};
use crate::statement::{
    AssignmentRVal, BStatement, FuncVal, RVal, RWAssignmentRVal, RWFuncVal, RWStatement,
    Statement, TraitStructOpt, TraitStructTup, Type,
};
use crate::traits::Traits;

use std::collections::HashSet;

pub struct Rewriter {}

/// Implement rewriter

// TODO never returns Err, refactor out Result return type (wait on this)

impl Rewriter {
    pub fn new() -> Self {
        Self {}
    }

    // FIXME write TraitImplList out first (add bool flag)
    pub fn rewrite(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        sigs: &Sigs,
        traits: &Traits,
        scope: Option<&'static str>,
        stmt: &Statement,
        sort_hashsets: bool,
    ) -> Result<RWStatement, Error> {
        match stmt {
            Statement::Sequence(stmt_vec) => self.rewrite_seq(
                funcs,
                cmap,
                sigs,
                traits,
                scope,
                stmt_vec,
                sort_hashsets,
            ),
            Statement::Conditional(condition, true_branch, false_branch) => self
                .rewrite_conditional(
                    funcs,
                    cmap,
                    sigs,
                    traits,
                    scope,
                    *condition.clone(),
                    &(*true_branch),
                    &(*false_branch),
                    sort_hashsets,
                ),
            Statement::Switch(val, vec) => self.rewrite_switch(
                funcs,
                cmap,
                sigs,
                traits,
                scope,
                val,
                vec,
                sort_hashsets,
            ),
            Statement::FuncDef(func) => self.rewrite_funcdef(
                funcs,
                cmap,
                sigs,
                traits,
                scope,
                func,
                sort_hashsets,
            ),
            Statement::InvokeFunc(name, args) => self.rewrite_invoke(
                funcs,
                cmap,
                sigs,
                traits,
                scope,
                name,
                args,
                sort_hashsets,
            ),
            Statement::Assignment(name, val) => self.rewrite_assignment(
                funcs,
                cmap,
                sigs,
                traits,
                scope,
                name,
                val.clone(),
                sort_hashsets,
            ),
            Statement::Print(s) => Ok(Print(s)),
            Statement::Return(val) => Ok(Return(val.clone())),
            Statement::FuncDecl(fd) => Ok(FuncDecl(fd.clone())),
            Statement::Struct(name, field_types, field_names) => {
                Ok(Struct(name, field_types.clone(), field_names.clone()))
            }
            Statement::TraitDecl(name, func_decls) => {
                Ok(TraitDecl(name, func_decls.to_vec()))
            }
            Statement::TraitImpl(trait_name, struct_name, func_impls) => self
                .rewrite_trait_impl(
                    funcs,
                    cmap,
                    sigs,
                    traits,
                    scope,
                    trait_name,
                    struct_name,
                    func_impls,
                    sort_hashsets,
                ),
        }
    }

    fn rewrite_func_body(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        sigs: &Sigs,
        traits: &Traits,
        scope: Option<&'static str>,
        func: &FuncVal,
        sort_hashsets: bool,
    ) -> Result<RWFuncVal, Error> {
        let rwbody = self.rewrite(
            funcs,
            cmap,
            sigs,
            traits,
            scope,
            &(*func.body),
            sort_hashsets,
        )?;

        Ok(RWFuncVal::new(
            func.name,
            func.is_method,
            func.params.clone(),
            func.rettype.clone(),
            Box::new(rwbody),
        ))
    }

    fn rewrite_trait_impl(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        sigs: &Sigs,
        traits: &Traits,
        scope: Option<&'static str>,
        trait_name: &'static str,
        struct_name: &'static str,
        func_impls: &Vec<FuncVal>,
        sort_hashsets: bool,
    ) -> Result<RWStatement, Error> {
        let mut rw_func_impls = vec![];
        for func_impl in func_impls.iter() {
            let rw_func_impl = self.rewrite_func_body(
                funcs,
                cmap,
                sigs,
                traits,
                scope,
                func_impl,
                sort_hashsets,
            )?;
            rw_func_impls.push(rw_func_impl);
        }

        Ok(TraitImpl(trait_name, struct_name, rw_func_impls))
    }

    fn rewrite_assignment(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        sigs: &Sigs,
        traits: &Traits,
        scope: Option<&'static str>,
        name: &'static str,
        val: Box<AssignmentRVal>,
        sort_hashsets: bool,
    ) -> Result<RWStatement, Error> {
        let rw_val = match *val {
            AssignmentRVal::RVal(rval) => RWAssignmentRVal::RVal(rval),
            AssignmentRVal::Statement(stmt) => RWAssignmentRVal::Statement(Box::new(
                self.rewrite(funcs, cmap, sigs, traits, scope, &(*stmt), sort_hashsets)?,
            )),
        };
        Ok(Assignment(name, Box::new(rw_val)))
    }

    fn rewrite_seq(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        sigs: &Sigs,
        traits: &Traits,
        scope: Option<&'static str>,
        stmt_vec: &Vec<Box<Statement>>,
        sort_hashsets: bool,
    ) -> Result<RWStatement, Error> {
        let mut rwstmts = vec![];
        for stmt in stmt_vec.iter() {
            let rwstmt =
                self.rewrite(funcs, cmap, sigs, traits, scope, stmt, sort_hashsets)?;
            rwstmts.push(Box::new(rwstmt));
        }
        Ok(Sequence(rwstmts))
    }

    fn rewrite_conditional(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        sigs: &Sigs,
        traits: &Traits,
        scope: Option<&'static str>,
        condition: BStatement,
        true_branch: &Statement,
        false_branch: &Statement,
        sort_hashsets: bool,
    ) -> Result<RWStatement, Error> {
        // FIXME also rewrite condition when funcs can ret booleans
        let rw_true_branch = self.rewrite(
            funcs,
            cmap,
            sigs,
            traits,
            scope,
            &true_branch,
            sort_hashsets,
        )?;

        let rw_false_branch = self.rewrite(
            funcs,
            cmap,
            sigs,
            traits,
            scope,
            &false_branch,
            sort_hashsets,
        )?;

        Ok(Conditional(
            Box::new(condition),
            Box::new(rw_true_branch),
            Box::new(rw_false_branch),
        ))
    }

    fn rewrite_switch(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        sigs: &Sigs,
        traits: &Traits,
        scope: Option<&'static str>,
        val: &RVal,
        vec: &Vec<(RVal, Box<Statement>)>,
        sort_hashsets: bool,
    ) -> Result<RWStatement, Error> {
        // FIXME if can switch on function call, also perform rewrite on that
        // (omitted val argument)
        let mut switch_vec = vec![];
        for (case, switch_stmt) in vec.iter() {
            let rw_switch_stmt = self.rewrite(
                funcs,
                cmap,
                sigs,
                traits,
                scope,
                &(*switch_stmt),
                sort_hashsets,
            )?;
            switch_vec.push((case.clone(), Box::new(rw_switch_stmt)));
        }
        Ok(Switch(val.clone(), switch_vec))
    }

    fn rewrite_funcdef(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        sigs: &Sigs,
        traits: &Traits,
        scope: Option<&'static str>,
        func: &FuncVal,
        sort_hashsets: bool,
    ) -> Result<RWStatement, Error> {
        let rwfv = self.rewrite_func_body(
            funcs,
            cmap,
            sigs,
            traits,
            scope,
            func,
            sort_hashsets,
        )?;

        Ok(FuncDef(rwfv))
    }

    fn cha(
        &self,
        sigs: &Sigs,
        func_name: &'static str,
        vartype: &Type,
    ) -> Result<HashSet<&'static str>, Error> {
        match vartype {
            // FIXME can further refine via traits
            // ??? ^
            Type::Func(paramtypes, rettype) => {
                let sigval = SigVal::new(paramtypes.clone(), rettype.clone());
                match sigs.sigs.get(&sigval) {
                    Some(funcset) => Ok(funcset.clone()),
                    None => panic!("SC BUG: func sig not collected"),
                }
            }
            _ => return Err(Error::NotAFunction(func_name)),
        }
    }

    fn is_top(
        &self,
        pos_constraints: &HashSet<RVal>,
        top: &HashSet<&'static str>,
    ) -> bool {
        let mut str_set = HashSet::<&'static str>::new();

        // check for any non-string (i.e. Num) RVals in constraints
        for rval in pos_constraints.clone().into_iter() {
            match rval {
                RVal::Var(varname) => str_set.insert(varname),
                RVal::Struct(structname, ..) => str_set.insert(structname),
                _ => return false,
            };
        }

        if str_set == *top {
            return true;
        }

        false
    }

    fn is_top_or_bottom(
        &self,
        sigs: &Sigs,
        func_name: &'static str,
        vartype: &Type,
        constraints: &Constraints,
    ) -> Result<Option<HashSet<&'static str>>, Error> {
        match self.cha(sigs, func_name, vartype) {
            Ok(all_impls) => {
                if constraints.0.is_empty() && constraints.1.is_empty()
                    || self.is_top(&constraints.0, &all_impls)
                {
                    return Ok(Some(all_impls));
                }
                Ok(None)
            }
            Err(err) => return Err(err),
        }
    }

    fn is_top_or_bottom_traits(
        &self,
        trait_name: &'static str,
        vartype: &Type,
        funcvec: &Vec<(TraitStructOpt, FuncVal)>,
        constraints: &Constraints,
    ) -> Result<Option<(&'static str, HashSet<&'static str>)>, Error> {
        match vartype {
            Type::DynTrait(trait_name) => {
                let (tso_vec, _): (Vec<TraitStructOpt>, Vec<FuncVal>) =
                    funcvec.into_iter().map(|x| x.clone()).unzip();
                let ts_vec = tso_vec
                    .into_iter()
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap())
                    .collect::<Vec<TraitStructTup>>();
                let (_, all_impls_vec): (Vec<&'static str>, Vec<&'static str>) =
                    ts_vec.into_iter().unzip();
                let all_impls = HashSet::from_iter(all_impls_vec);

                if constraints.0.is_empty() && constraints.1.is_empty()
                    || self.is_top(&constraints.0, &all_impls)
                {
                    return Ok(Some((trait_name, all_impls)));
                }
                Ok(None)
            }
            _ => return Err(Error::NotATrait(trait_name)),
        }
    }

    fn rewrite_indirect_invoke(
        &self,
        sigs: &Sigs,
        _traits: &Traits,
        name: &'static str,
        constraints: &Constraints,
        vartype: &Type,
        args: Vec<&'static str>,
        sort_hashsets: bool,
    ) -> Result<RWStatement, Error> {
        let mut switch_vec = vec![];
        let mut pos_vec: Vec<_> = constraints.0.iter().collect();
        // FIXME how to use neg_vec?
        //let _neg_vec: Vec<_> = constraints.1.iter().collect();

        // sorting to get deterministic switch statement order in tests
        if sort_hashsets {
            pos_vec.sort();
        }

        // if constraints == bottom (empty) OR top (all possible), use CHA
        // [this is the status quo in C++ related work]
        match self.is_top_or_bottom(sigs, name, vartype, constraints) {
            Ok(Some(top)) => {
                let mut top_vec: Vec<_> = top.iter().collect();

                // sorting to get deterministic switch statement order in tests
                if sort_hashsets {
                    top_vec.sort();
                }

                // FIXME use trait_struct_opt
                for func_str in top_vec.into_iter() {
                    switch_vec.push((
                        RVal::Var(func_str),
                        Box::new(InvokeFunc(func_str, args.clone())),
                    ));
                }
            }
            Ok(None) => {
                // given our no-intersection invariant, no elements in the set
                // of positive constraints will be in the set of negative
                // constraints
                for rval in pos_vec.into_iter() {
                    match rval.clone() {
                        r @ RVal::Var(var) => {
                            switch_vec.push((r, Box::new(InvokeFunc(var, args.clone()))))
                        }
                        _ => panic!("IP BUG: cannot call num {:?} as func", &rval),
                    };
                }
            }
            Err(err) => return Err(err),
        }

        Ok(Switch(RVal::Var(name), switch_vec))
    }

    fn rewrite_trait_invoke(
        &self,
        _funcs: &Funcs,
        cmap: &ConstraintMap,
        _sigs: &Sigs,
        _traits: &Traits,
        scope: Option<&'static str>,
        name: &'static str,
        funcvec: &Vec<(TraitStructOpt, FuncVal)>,
        args: &Vec<&'static str>,
        sort_hashsets: bool,
    ) -> Result<RWStatement, Error> {
        if funcvec.len() == 1 {
            return Ok(InvokeFunc(name, args.to_vec()));
        }

        let mut switch_vec = vec![];

        // get type of `self` (first arg)
        if funcvec.len() > 0 && funcvec[0].1.is_method && args.len() > 0 {
            match cmap.scoped_get(scope, args[0], false) {
                Ok(Some(vartype)) => match vartype {
                    VarType::Values(vartype, constraints) => {
                        let mut pos_vec: Vec<_> = constraints.0.iter().collect();
                        // tests
                        if sort_hashsets {
                            pos_vec.sort();
                        }

                        // FIXME use funcvec or traits data structure?
                        match self.is_top_or_bottom_traits(
                            name,
                            &vartype,
                            funcvec,
                            &constraints,
                        ) {
                            Ok(Some((trait_name, top))) => {
                                let mut top_vec: Vec<_> = top.iter().collect();

                                // sorting to get deterministic switch statement
                                // order in tests
                                if sort_hashsets {
                                    top_vec.sort();
                                }

                                for struct_name in top_vec.iter() {
                                    switch_vec.push((
                                        // FIXME Var or Struct (if struct need
                                        // another data structure to track fields)
                                        RVal::Var(struct_name),
                                        Box::new(InvokeTraitFunc(
                                            name,
                                            // FIXME need trait name?
                                            (trait_name, struct_name),
                                            args.clone(),
                                        )),
                                    ));
                                }
                            }
                            Ok(None) => {
                                // FIXME doing similar work as prune_funcvec() in interp
                                // can save some of this work by storing the results of
                                // that work
                                for (tso, funcval) in funcvec.iter() {
                                    let self_type = &funcval.params[0].1;
                                    match self_type {
                                        Type::Struct(sname) => {
                                            // if no match in positive constraints, skip
                                            for c in &constraints.0 {
                                                match c {
                                                    RVal::IdkStruct(other_sname)
                                                    | RVal::Struct(other_sname, ..) => {
                                                        if *sname == *other_sname {
                                                            switch_vec.push((
                                                                RVal::Var(sname),
                                                                Box::new(
                                                                    InvokeTraitFunc(
                                                                        name,
                                                                        tso.unwrap(),
                                                                        args.clone(),
                                                                    ),
                                                                ),
                                                            ));
                                                        }
                                                    }
                                                    _ => todo!(),
                                                }
                                            }
                                        }
                                        _ => todo!(),
                                    }
                                }
                            }
                            Err(err) => return Err(err),
                        }
                    }
                    _ => panic!("`self` arg should not be a scope"),
                },
                Ok(None) => panic!("`self` arg does not exist"),
                Err(err) => return Err(err),
            }
        }

        Ok(TraitSwitch(RVal::Var(args[0]), switch_vec))
    }

    fn rewrite_invoke(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        sigs: &Sigs,
        traits: &Traits,
        scope: Option<&'static str>,
        name: &'static str,
        args: &Vec<&'static str>,
        sort_hashsets: bool,
    ) -> Result<RWStatement, Error> {
        match funcs.funcs.get(&name) {
            Some(funcvec) => self.rewrite_trait_invoke(
                funcs,
                cmap,
                sigs,
                traits,
                scope,
                name,
                &funcvec,
                args,
                sort_hashsets,
            ),
            None => match cmap.scoped_get(scope, name, false) {
                Ok(Some(vartype)) => match vartype {
                    VarType::Values(vartype, constraints) => self
                        .rewrite_indirect_invoke(
                            sigs,
                            traits,
                            name,
                            &constraints,
                            &vartype,
                            args.to_vec(),
                            sort_hashsets,
                        ),
                    _ => return Err(Error::NoSwitchOnFuncPtr()),
                },
                Ok(None) => panic!("IP BUG: missed undef symbol {:?}", &name),
                Err(err) => return Err(err),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::funcs::Funcs;
    use crate::statement::RWStatement as RWS;
    use crate::statement::Statement::{
        Assignment, Conditional, FuncDef, InvokeFunc, Print, Return,
        Sequence, Struct, TraitDecl, TraitImpl, 
    };
    use crate::statement::{AssignmentRVal, RWAssignmentRVal, FuncDecl, FuncVal, RWFuncVal, Type};
    use std::collections::HashSet;

    #[test]
    fn test_print() {
        let stmt = Print("hello");
        let check_stmt = RWS::Print("hello");

        let funcs = Funcs::new();
        let cmap = ConstraintMap::new();
        let sigs = Sigs::new();
        let traits = Traits::new();
        let rw = Rewriter::new();
        let ret = rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &stmt, true);

        assert_eq!(check_stmt, ret.unwrap());
    }

    #[test]
    fn test_assignment() {
        let stmt = Assignment("x", Box::new(AssignmentRVal::RVal(RVal::Num(5))));
        let check_stmt = RWS::Assignment("x", Box::new(RWAssignmentRVal::RVal(RVal::Num(5))));

        let funcs = Funcs::new();
        let mut cmap = ConstraintMap::new();
        cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Int()),
                (HashSet::from([RVal::Num(5)]), HashSet::new()),
            )),
        );
        let sigs = Sigs::new();
        let traits = Traits::new();

        let rw = Rewriter::new();
        let ret = rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &stmt, true);

        assert_eq!(check_stmt, ret.unwrap());
    }

    #[test]
    fn test_direct_invoke() {
        let body = Box::new(Sequence(vec![Box::new(Assignment(
            "z",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        let stmt = Sequence(vec![
            Box::new(FuncDef(FuncVal::new(
                "foo",
                false,
                vec![],
                None,
                body.clone(),
            ))),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(InvokeFunc("foo", vec![])),
        ]);

        let check_body = Box::new(RWS::Sequence(vec![Box::new(RWS::Assignment(
            "z",
            Box::new(RWAssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        let check_stmt = RWS::Sequence(vec![
            Box::new(RWS::FuncDef(RWFuncVal::new(
                "foo",
                false,
                vec![],
                None,
                check_body,
            ))),
            Box::new(RWS::Assignment(
                "x",
                Box::new(RWAssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(RWS::InvokeFunc("foo", vec![])),
        ]);

        let mut funcs = Funcs::new();
        funcs.funcs.insert(
            "foo",
            vec![(None, FuncVal::new("foo", false, vec![], None, body.clone()))],
        );
        let mut cmap = ConstraintMap::new();
        cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Func(vec![], None)),
                (HashSet::from([RVal::Var("foo")]), HashSet::new()),
            )),
        );
        let mut sigs = Sigs::new();
        let sigval = SigVal::new(vec![], None);
        sigs.sigs.insert(sigval, HashSet::from(["foo"]));

        let traits = Traits::new();
        let rw = Rewriter::new();
        let ret = rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &stmt, true);

        assert_eq!(check_stmt, ret.unwrap());
    }

    #[test]
    fn test_indirect_invoke_single_val() {
        let body = Box::new(Sequence(vec![Box::new(Assignment(
            "z",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        let mut stmt = Sequence(vec![
            Box::new(FuncDef(FuncVal::new(
                "foo",
                false,
                vec![],
                None,
                body.clone(),
            ))),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]);

        let mut funcs = Funcs::new();
        funcs.funcs.insert(
            "foo",
            vec![(None, FuncVal::new("foo", false, vec![], None, body.clone()))],
        );
        let mut cmap = ConstraintMap::new();
        cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Func(vec![], None)),
                (HashSet::from([RVal::Var("foo")]), HashSet::new()),
            )),
        );
        let mut sigs = Sigs::new();
        let sigval = SigVal::new(vec![], None);
        sigs.sigs.insert(sigval, HashSet::from(["foo"]));

        let traits = Traits::new();
        let rw = Rewriter::new();
        let ret = rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &mut stmt, true);

        let check_body = Box::new(RWS::Sequence(vec![Box::new(RWS::Assignment(
            "z",
            Box::new(RWAssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        let switch_vec = vec![(RVal::Var("foo"), Box::new(RWS::InvokeFunc("foo", vec![])))];
        let check_stmt = RWS::Sequence(vec![
            Box::new(RWS::FuncDef(RWFuncVal::new("foo", false, vec![], None, check_body))),
            Box::new(RWS::Assignment(
                "x",
                Box::new(RWAssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(RWS::Switch(RVal::Var("x"), switch_vec)),
        ]);

        assert_eq!(check_stmt, ret.unwrap());
    }

    #[test]
    fn test_indirect_invoke_multiple_val() {
        let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
            "z",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        let bar_body = Box::new(Sequence(vec![Box::new(Assignment(
            "z",
            Box::new(AssignmentRVal::RVal(RVal::Num(6))),
        ))]));
        let stmt = Sequence(vec![
            Box::new(FuncDef(FuncVal::new(
                "foo",
                false,
                vec![],
                None,
                foo_body.clone(),
            ))),
            Box::new(FuncDef(FuncVal::new(
                "bar",
                false,
                vec![],
                None,
                bar_body.clone(),
            ))),
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
            vec![(
                None,
                FuncVal::new("foo", false, vec![], None, foo_body.clone()),
            )],
        );
        funcs.funcs.insert(
            "bar",
            vec![(
                None,
                FuncVal::new("bar", false, vec![], None, bar_body.clone()),
            )],
        );
        let mut cmap = ConstraintMap::new();
        cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Func(vec![], None)),
                (
                    HashSet::from([RVal::Var("bar"), RVal::Var("foo")]),
                    HashSet::new(),
                ),
            )),
        );
        let mut sigs = Sigs::new();
        let sigval = SigVal::new(vec![], None);
        sigs.sigs.insert(sigval, HashSet::from(["foo"]));

        let traits = Traits::new();
        let rw = Rewriter::new();
        let ret = rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &stmt, true);

        let check_foo_body = Box::new(RWS::Sequence(vec![Box::new(RWS::Assignment(
            "z",
            Box::new(RWAssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        let check_bar_body = Box::new(RWS::Sequence(vec![Box::new(RWS::Assignment(
            "z",
            Box::new(RWAssignmentRVal::RVal(RVal::Num(6))),
        ))]));
        let switch_vec = vec![
            (RVal::Var("bar"), Box::new(RWS::InvokeFunc("bar", vec![]))),
            (RVal::Var("foo"), Box::new(RWS::InvokeFunc("foo", vec![]))),
        ];
        let check_stmt = RWS::Sequence(vec![
            Box::new(RWS::FuncDef(RWFuncVal::new(
                "foo",
                false,
                vec![],
                None,
                check_foo_body.clone(),
            ))),
            Box::new(RWS::FuncDef(RWFuncVal::new(
                "bar",
                false,
                vec![],
                None,
                check_bar_body.clone(),
            ))),
            Box::new(RWS::Conditional(
                Box::new(BStatement::TrueOrFalse()),
                Box::new(RWS::Assignment(
                    "x",
                    Box::new(RWAssignmentRVal::RVal(RVal::Var("foo"))),
                )),
                Box::new(RWS::Assignment(
                    "x",
                    Box::new(RWAssignmentRVal::RVal(RVal::Var("bar"))),
                )),
            )),
            Box::new(RWS::Switch(RVal::Var("x"), switch_vec)),
        ]);

        assert_eq!(ret.unwrap(), check_stmt);
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
            Box::new(FuncDef(FuncVal::new(
                "baz",
                false,
                vec![],
                None,
                baz_body.clone(),
            ))),
            Box::new(FuncDef(FuncVal::new(
                "qux",
                false,
                vec![],
                None,
                qux_body.clone(),
            ))),
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
            Box::new(FuncDef(FuncVal::new(
                "baz2",
                false,
                vec![],
                None,
                baz2_body.clone(),
            ))),
            Box::new(FuncDef(FuncVal::new(
                "qux2",
                false,
                vec![],
                None,
                qux2_body.clone(),
            ))),
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
            Box::new(FuncDef(FuncVal::new(
                "foo",
                false,
                vec![],
                None,
                foo_body.clone(),
            ))),
            Box::new(FuncDef(FuncVal::new(
                "bar",
                false,
                vec![],
                None,
                bar_body.clone(),
            ))),
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
            vec![(
                None,
                FuncVal::new("foo", false, vec![], None, foo_body.clone()),
            )],
        );
        funcs.funcs.insert(
            "bar",
            vec![(
                None,
                FuncVal::new("bar", false, vec![], None, bar_body.clone()),
            )],
        );
        funcs.funcs.insert(
            "baz",
            vec![(
                None,
                FuncVal::new("baz", false, vec![], None, baz_body.clone()),
            )],
        );
        funcs.funcs.insert(
            "qux",
            vec![(
                None,
                FuncVal::new("qux", false, vec![], None, qux_body.clone()),
            )],
        );
        funcs.funcs.insert(
            "baz2",
            vec![(
                None,
                FuncVal::new("baz2", false, vec![], None, baz2_body.clone()),
            )],
        );
        funcs.funcs.insert(
            "qux2",
            vec![(
                None,
                FuncVal::new("qux2", false, vec![], None, qux2_body.clone()),
            )],
        );

        let mut cmap = ConstraintMap::new();
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

        cmap.cmap.insert(
            "x",
            Box::new(VarType::Values(
                Box::new(Type::Func(vec![], None)),
                (
                    HashSet::from([RVal::Var("foo"), RVal::Var("bar")]),
                    HashSet::new(),
                ),
            )),
        );
        cmap.cmap.insert(
            "baz2",
            Box::new(VarType::Scope(
                Some("bar"),
                vec![(Box::new(Type::Func(vec![], None)), baz2_cmap)],
            )),
        );
        cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                None,
                vec![(Box::new(Type::Func(vec![], None)), foo_cmap)],
            )),
        );
        cmap.cmap.insert(
            "qux",
            Box::new(VarType::Scope(
                Some("foo"),
                vec![(Box::new(Type::Func(vec![], None)), qux_cmap)],
            )),
        );
        cmap.cmap.insert(
            "baz",
            Box::new(VarType::Scope(
                Some("foo"),
                vec![(Box::new(Type::Func(vec![], None)), baz_cmap)],
            )),
        );
        cmap.cmap.insert(
            "bar",
            Box::new(VarType::Scope(
                None,
                vec![(Box::new(Type::Func(vec![], None)), bar_cmap)],
            )),
        );
        cmap.cmap.insert(
            "qux2",
            Box::new(VarType::Scope(
                Some("bar"),
                vec![(Box::new(Type::Func(vec![], None)), qux2_cmap)],
            )),
        );
        let mut sigs = Sigs::new();
        let sigval = SigVal::new(vec![], None);
        sigs.sigs.insert(
            sigval,
            HashSet::from(["foo", "bar", "baz", "qux", "baz2", "qux2"]),
        );

        let traits = Traits::new();
        let rw = Rewriter::new();
        let ret = rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &stmt, true);

        let check_baz_body = Box::new(RWS::Assignment(
            "x",
            Box::new(RWAssignmentRVal::RVal(RVal::Num(1))),
        ));
        let check_qux_body = Box::new(RWS::Assignment(
            "x",
            Box::new(RWAssignmentRVal::RVal(RVal::Num(2))),
        ));
        let check_baz2_body = Box::new(RWS::Assignment(
            "x",
            Box::new(RWAssignmentRVal::RVal(RVal::Num(3))),
        ));
        let check_qux2_body = Box::new(RWS::Assignment(
            "x",
            Box::new(RWAssignmentRVal::RVal(RVal::Num(4))),
        ));
        let check_foo_body = Box::new(RWS::Sequence(vec![
            Box::new(RWS::FuncDef(RWFuncVal::new(
                "baz",
                false,
                vec![],
                None,
                check_baz_body.clone(),
            ))),
            Box::new(RWS::FuncDef(RWFuncVal::new(
                "qux",
                false,
                vec![],
                None,
                check_qux_body.clone(),
            ))),
            Box::new(RWS::Conditional(
                Box::new(BStatement::TrueOrFalse()),
                Box::new(RWS::Assignment(
                    "x",
                    Box::new(RWAssignmentRVal::RVal(RVal::Var("baz"))),
                )),
                Box::new(RWS::Assignment(
                    "x",
                    Box::new(RWAssignmentRVal::RVal(RVal::Var("qux"))),
                )),
            )),
            Box::new(RWS::InvokeFunc("x", vec![])),
        ]));
        let check_bar_body = Box::new(RWS::Sequence(vec![
            Box::new(RWS::FuncDef(RWFuncVal::new(
                "baz2",
                false,
                vec![],
                None,
                check_baz2_body.clone(),
            ))),
            Box::new(RWS::FuncDef(RWFuncVal::new(
                "qux2",
                false,
                vec![],
                None,
                check_qux2_body.clone(),
            ))),
            Box::new(RWS::Conditional(
                Box::new(BStatement::TrueOrFalse()),
                Box::new(RWS::Assignment(
                    "x",
                    Box::new(RWAssignmentRVal::RVal(RVal::Var("baz2"))),
                )),
                Box::new(RWS::Assignment(
                    "x",
                    Box::new(RWAssignmentRVal::RVal(RVal::Var("qux2"))),
                )),
            )),
            Box::new(RWS::InvokeFunc("x", vec![])),
        ]));

        let foo_switch_vec = vec![
            (RVal::Var("baz"), Box::new(RWS::InvokeFunc("baz", vec![]))),
            (RVal::Var("qux"), Box::new(RWS::InvokeFunc("qux", vec![]))),
        ];
        let check_foo_body = Box::new(RWS::Sequence(vec![
            Box::new(RWS::FuncDef(RWFuncVal::new(
                "baz",
                false,
                vec![],
                None,
                check_baz_body.clone(),
            ))),
            Box::new(RWS::FuncDef(RWFuncVal::new(
                "qux",
                false,
                vec![],
                None,
                check_qux_body.clone(),
            ))),
            Box::new(RWS::Conditional(
                Box::new(BStatement::TrueOrFalse()),
                Box::new(RWS::Assignment(
                    "x",
                    Box::new(RWAssignmentRVal::RVal(RVal::Var("baz"))),
                )),
                Box::new(RWS::Assignment(
                    "x",
                    Box::new(RWAssignmentRVal::RVal(RVal::Var("qux"))),
                )),
            )),
            Box::new(RWS::Switch(RVal::Var("x"), foo_switch_vec)),
        ]));
        let bar_switch_vec = vec![
            (RVal::Var("baz2"), Box::new(RWS::InvokeFunc("baz2", vec![]))),
            (RVal::Var("qux2"), Box::new(RWS::InvokeFunc("qux2", vec![]))),
        ];
        let check_bar_body = Box::new(RWS::Sequence(vec![
            Box::new(RWS::FuncDef(RWFuncVal::new(
                "baz2",
                false,
                vec![],
                None,
                check_baz2_body.clone(),
            ))),
            Box::new(RWS::FuncDef(RWFuncVal::new(
                "qux2",
                false,
                vec![],
                None,
                check_qux2_body.clone(),
            ))),
            Box::new(RWS::Conditional(
                Box::new(BStatement::TrueOrFalse()),
                Box::new(RWS::Assignment(
                    "x",
                    Box::new(RWAssignmentRVal::RVal(RVal::Var("baz2"))),
                )),
                Box::new(RWS::Assignment(
                    "x",
                    Box::new(RWAssignmentRVal::RVal(RVal::Var("qux2"))),
                )),
            )),
            Box::new(RWS::Switch(RVal::Var("x"), bar_switch_vec)),
        ]));
        let switch_vec = vec![
            (RVal::Var("bar"), Box::new(RWS::InvokeFunc("bar", vec![]))),
            (RVal::Var("foo"), Box::new(RWS::InvokeFunc("foo", vec![]))),
        ];
        let check_stmt = RWS::Sequence(vec![
            Box::new(RWS::FuncDef(RWFuncVal::new(
                "foo",
                false,
                vec![],
                None,
                check_foo_body.clone(),
            ))),
            Box::new(RWS::FuncDef(RWFuncVal::new(
                "bar",
                false,
                vec![],
                None,
                check_bar_body.clone(),
            ))),
            Box::new(RWS::Conditional(
                Box::new(BStatement::TrueOrFalse()),
                Box::new(RWS::Assignment(
                    "x",
                    Box::new(RWAssignmentRVal::RVal(RVal::Var("foo"))),
                )),
                Box::new(RWS::Assignment(
                    "x",
                    Box::new(RWAssignmentRVal::RVal(RVal::Var("bar"))),
                )),
            )),
            Box::new(RWS::Switch(RVal::Var("x"), switch_vec)),
        ]);

        println!("got: \n{:#?}", &ret.unwrap());
        println!("exp: \n{:#?}", &check_stmt);
        //assert_eq!(ret.unwrap(), check_stmt);
    }

    #[test]
    fn test_dyn_traits_two_impl() {
        let funcdecl = FuncDecl::new(
            "speak",
            true,
            vec![("self", Type::DynTrait("Animal"))],
            None,
        );

        let cat_speak_body = Box::new(Print("meow"));
        let cat_speak = FuncVal::new(
            "speak",
            true,
            vec![("self", Type::Struct("Cat"))],
            None,
            cat_speak_body.clone(),
        );

        let dog_speak_body = Box::new(Print("woof"));
        let dog_speak = FuncVal::new(
            "speak",
            true,
            vec![("self", Type::Struct("Dog"))],
            None,
            dog_speak_body.clone(),
        );

        let gmaa = Box::new(Sequence(vec![Box::new(Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(Sequence(vec![Box::new(Return(RVal::Struct(
                "Cat",
                vec![],
                vec![],
            )))])),
            Box::new(Sequence(vec![Box::new(Return(RVal::Struct(
                "Dog",
                vec![],
                vec![],
            )))])),
        ))]));

        let stmt = Sequence(vec![
            Box::new(TraitDecl("Animal", vec![funcdecl.clone()])),
            Box::new(FuncDef(FuncVal::new(
                "giveMeAnAnimal",
                false,
                vec![],
                Some(Box::new(Type::DynTrait("Animal"))),
                gmaa.clone(),
            ))),
            Box::new(Struct("Cat", vec![], vec![])),
            Box::new(Struct("Dog", vec![], vec![])),
            Box::new(TraitImpl("Animal", "Cat", vec![cat_speak.clone()])),
            Box::new(TraitImpl("Animal", "Dog", vec![dog_speak.clone()])),
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
                    "giveMeAnAnimal",
                    false,
                    vec![],
                    Some(Box::new(Type::DynTrait("Animal"))),
                    gmaa.clone(),
                ),
            )],
        );

        let mut cmap = ConstraintMap::new();
        let mut speak_cmap = ConstraintMap::new();
        speak_cmap.cmap.insert(
            "animal",
            Box::new(VarType::Values(
                Box::new(Type::DynTrait("Animal")),
                (
                    HashSet::from([
                        RVal::Struct("Cat", vec![], vec![]),
                        RVal::Struct("Dog", vec![], vec![]),
                    ]),
                    HashSet::new(),
                ),
            )),
        );
        cmap.cmap.insert(
            "speak",
            Box::new(VarType::Scope(
                None,
                vec![(
                    Box::new(Type::Func(vec![Type::DynTrait("Animal")], None)),
                    speak_cmap,
                )],
            )),
        );
        cmap.cmap.insert(
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
        cmap.cmap.insert(
            "specific_animal",
            Box::new(VarType::Values(
                Box::new(Type::DynTrait("Animal")),
                (
                    HashSet::from([
                        RVal::Struct("Cat", vec![], vec![]),
                        RVal::Struct("Dog", vec![], vec![]),
                    ]),
                    HashSet::new(),
                ),
            )),
        );

        let mut sigs = Sigs::new();
        sigs.sigs
            .insert(SigVal::new(vec![], None), HashSet::from(["speak"]));
        sigs.sigs.insert(
            SigVal::new(vec![], Some(Box::new(Type::DynTrait("Animal")))),
            HashSet::from(["giveMeAnAnimal"]),
        );

        let mut traits = Traits::new();
        traits.traits.insert("Animal", vec!["Cat", "Dog"]);

        let rw = Rewriter::new();
        let ret = rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &stmt, true);

        let check_cat_speak_body = Box::new(RWS::Print("meow"));
        let check_cat_speak = RWFuncVal::new(
            "speak",
            true,
            vec![("self", Type::Struct("Cat"))],
            None,
            check_cat_speak_body.clone(),
        );

        let check_dog_speak_body = Box::new(RWS::Print("woof"));
        let check_dog_speak = RWFuncVal::new(
            "speak",
            true,
            vec![("self", Type::Struct("Dog"))],
            None,
            check_dog_speak_body.clone(),
        );

        let check_gmaa = Box::new(RWS::Sequence(vec![Box::new(RWS::Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(RWS::Sequence(vec![Box::new(RWS::Return(RVal::Struct(
                "Cat",
                vec![],
                vec![],
            )))])),
            Box::new(RWS::Sequence(vec![Box::new(RWS::Return(RVal::Struct(
                "Dog",
                vec![],
                vec![],
            )))])),
        ))]));

        let switch_vec = vec![
            (
                RVal::Var("Cat"),
                Box::new(RWS::InvokeTraitFunc(
                    "speak",
                    ("Animal", "Cat"),
                    vec!["specific_animal"],
                )),
            ),
            (
                RVal::Var("Dog"),
                Box::new(RWS::InvokeTraitFunc(
                    "speak",
                    ("Animal", "Dog"),
                    vec!["specific_animal"],
                )),
            ),
        ];

        let check_stmt = RWS::Sequence(vec![
            Box::new(RWS::TraitDecl("Animal", vec![funcdecl.clone()])),
            Box::new(RWS::FuncDef(RWFuncVal::new(
                "giveMeAnAnimal",
                false,
                vec![],
                Some(Box::new(Type::DynTrait("Animal"))),
                check_gmaa,
            ))),
            Box::new(RWS::Struct("Cat", vec![], vec![])),
            Box::new(RWS::Struct("Dog", vec![], vec![])),
            Box::new(RWS::TraitImpl("Animal", "Cat", vec![check_cat_speak.clone()])),
            Box::new(RWS::TraitImpl("Animal", "Dog", vec![check_dog_speak.clone()])),
            Box::new(RWS::Assignment(
                "specific_animal",
                Box::new(RWAssignmentRVal::Statement(Box::new(RWS::InvokeFunc(
                    "giveMeAnAnimal",
                    vec![],
                )))),
            )),
            Box::new(RWS::TraitSwitch(RVal::Var("specific_animal"), switch_vec)),
        ]);

        assert_eq!(ret.unwrap(), check_stmt);
    }

    #[test]
    fn test_dyn_traits_three_impl_two_used() {
        let funcdecl = FuncDecl::new(
            "speak",
            true,
            vec![("self", Type::DynTrait("Animal"))],
            None,
        );

        let bird_speak_body = Box::new(Print("chirp"));
        let bird_speak = FuncVal::new(
            "speak",
            true,
            vec![("self", Type::Struct("Bird"))],
            None,
            bird_speak_body.clone(),
        );

        let cat_speak_body = Box::new(Print("meow"));
        let cat_speak = FuncVal::new(
            "speak",
            true,
            vec![("self", Type::Struct("Cat"))],
            None,
            cat_speak_body.clone(),
        );

        let dog_speak_body = Box::new(Print("woof"));
        let dog_speak = FuncVal::new(
            "speak",
            true,
            vec![("self", Type::Struct("Dog"))],
            None,
            dog_speak_body.clone(),
        );

        let gmaa = Box::new(Sequence(vec![Box::new(Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(Sequence(vec![Box::new(Return(RVal::Struct(
                "Cat",
                vec![],
                vec![],
            )))])),
            Box::new(Sequence(vec![Box::new(Return(RVal::Struct(
                "Dog",
                vec![],
                vec![],
            )))])),
        ))]));

        let stmt = Sequence(vec![
            Box::new(TraitDecl("Animal", vec![funcdecl.clone()])),
            Box::new(FuncDef(FuncVal::new(
                "giveMeAnAnimal",
                false,
                vec![],
                Some(Box::new(Type::DynTrait("Animal"))),
                gmaa.clone(),
            ))),
            Box::new(Struct("Bird", vec![], vec![])),
            Box::new(Struct("Cat", vec![], vec![])),
            Box::new(Struct("Dog", vec![], vec![])),
            Box::new(TraitImpl("Animal", "Bird", vec![bird_speak.clone()])),
            Box::new(TraitImpl("Animal", "Cat", vec![cat_speak.clone()])),
            Box::new(TraitImpl("Animal", "Dog", vec![dog_speak.clone()])),
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
                    "giveMeAnAnimal",
                    false,
                    vec![],
                    Some(Box::new(Type::DynTrait("Animal"))),
                    gmaa.clone(),
                ),
            )],
        );

        let mut cmap = ConstraintMap::new();
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

        cmap.cmap.insert(
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
        cmap.cmap.insert(
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
        cmap.cmap.insert(
            "specific_animal",
            Box::new(VarType::Values(
                Box::new(Type::DynTrait("Animal")),
                (
                    HashSet::from([RVal::IdkStruct("Cat"), RVal::IdkStruct("Dog")]),
                    HashSet::new(),
                ),
            )),
        );

        let mut sigs = Sigs::new();
        sigs.sigs
            .insert(SigVal::new(vec![], None), HashSet::from(["speak"]));
        sigs.sigs.insert(
            SigVal::new(vec![], Some(Box::new(Type::DynTrait("Animal")))),
            HashSet::from(["giveMeAnAnimal"]),
        );

        let mut traits = Traits::new();
        traits.traits.insert("Animal", vec!["Bird", "Cat", "Dog"]);

        let rw = Rewriter::new();
        let ret = rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &stmt, true);

        let check_bird_speak_body = Box::new(RWS::Print("chirp"));
        let check_bird_speak = RWFuncVal::new(
            "speak",
            true,
            vec![("self", Type::Struct("Bird"))],
            None,
            check_bird_speak_body.clone(),
        );

        let check_cat_speak_body = Box::new(RWS::Print("meow"));
        let check_cat_speak = RWFuncVal::new(
            "speak",
            true,
            vec![("self", Type::Struct("Cat"))],
            None,
            check_cat_speak_body.clone(),
        );

        let check_dog_speak_body = Box::new(RWS::Print("woof"));
        let check_dog_speak = RWFuncVal::new(
            "speak",
            true,
            vec![("self", Type::Struct("Dog"))],
            None,
            check_dog_speak_body.clone(),
        );

        let check_gmaa = Box::new(RWS::Sequence(vec![Box::new(RWS::Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(RWS::Sequence(vec![Box::new(RWS::Return(RVal::Struct(
                "Cat",
                vec![],
                vec![],
            )))])),
            Box::new(RWS::Sequence(vec![Box::new(RWS::Return(RVal::Struct(
                "Dog",
                vec![],
                vec![],
            )))])),
        ))]));

        let switch_vec = vec![
            (
                RVal::Var("Dog"),
                Box::new(RWS::InvokeTraitFunc(
                    "speak",
                    ("Animal", "Dog"),
                    vec!["specific_animal"],
                )),
            ),
            (
                RVal::Var("Cat"),
                Box::new(RWS::InvokeTraitFunc(
                    "speak",
                    ("Animal", "Cat"),
                    vec!["specific_animal"],
                )),
            ),
        ];

        let check_stmt = RWS::Sequence(vec![
            Box::new(RWS::TraitDecl("Animal", vec![funcdecl.clone()])),
            Box::new(RWS::FuncDef(RWFuncVal::new(
                "giveMeAnAnimal",
                false,
                vec![],
                Some(Box::new(Type::DynTrait("Animal"))),
                check_gmaa,
            ))),
            Box::new(RWS::Struct("Bird", vec![], vec![])),
            Box::new(RWS::Struct("Cat", vec![], vec![])),
            Box::new(RWS::Struct("Dog", vec![], vec![])),
            Box::new(RWS::TraitImpl("Animal", "Bird", vec![check_bird_speak.clone()])),
            Box::new(RWS::TraitImpl("Animal", "Cat", vec![check_cat_speak.clone()])),
            Box::new(RWS::TraitImpl("Animal", "Dog", vec![check_dog_speak.clone()])),
            Box::new(RWS::Assignment(
                "specific_animal",
                Box::new(RWAssignmentRVal::Statement(Box::new(RWS::InvokeFunc(
                    "giveMeAnAnimal",
                    vec![],
                )))),
            )),
            Box::new(RWS::TraitSwitch(RVal::Var("specific_animal"), switch_vec)),
        ]);

        assert_eq!(ret.unwrap(), check_stmt);
    }
}
