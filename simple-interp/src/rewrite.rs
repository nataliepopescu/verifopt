use crate::constraints::{ConstraintMap, Constraints, VarType};
use crate::error::Error;
use crate::funcs::Funcs;
use crate::sigs::{SigVal, Sigs};
use crate::statement::RWStatement::{
    Assignment, Conditional, FuncDecl, FuncDef, InvokeFunc, InvokeTraitFunc, Print,
    Return, Sequence, Struct, Switch, TraitDecl, TraitImpl, TraitSwitch,
};
use crate::statement::{
    AssignmentRVal, BStatement, FuncVal, RVal, RWAssignmentRVal, RWFuncVal, RWStatement,
    Statement, TraitStructOpt, TraitStructTup, Type,
};
use crate::traits::Traits;

use std::collections::HashSet;

pub struct Rewriter {}

// Implement rewriter

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
                    true_branch,
                    false_branch,
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
                *val.clone(),
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
        let rwbody =
            self.rewrite(funcs, cmap, sigs, traits, scope, &*func.body, sort_hashsets)?;

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
        val: AssignmentRVal,
        sort_hashsets: bool,
    ) -> Result<RWStatement, Error> {
        let rw_val = match val {
            AssignmentRVal::RVal(rval) => RWAssignmentRVal::RVal(rval),
            AssignmentRVal::Statement(stmt) => RWAssignmentRVal::Statement(Box::new(
                self.rewrite(funcs, cmap, sigs, traits, scope, &stmt, sort_hashsets)?,
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
        let rw_true_branch =
            self.rewrite(funcs, cmap, sigs, traits, scope, true_branch, sort_hashsets)?;

        let rw_false_branch = self.rewrite(
            funcs,
            cmap,
            sigs,
            traits,
            scope,
            false_branch,
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
                switch_stmt,
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
        _scope: Option<&'static str>,
        func: &FuncVal,
        sort_hashsets: bool,
    ) -> Result<RWStatement, Error> {
        let rwfv = self.rewrite_func_body(
            funcs,
            cmap,
            sigs,
            traits,
            Some(func.name),
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
            _ => Err(Error::NotAFunction(func_name)),
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
            Err(err) => Err(err),
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
                    funcvec.iter().cloned().unzip(); //map(|x| x.clone()).unzip();
                let ts_vec = tso_vec
                    .into_iter()
                    .flatten()
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
            _ => Err(Error::NotATrait(trait_name)),
        }
    }

    fn rewrite_indirect_invoke_helper(
        &self,
        _funcs: &Funcs,
        _cmap: &ConstraintMap,
        sigs: &Sigs,
        _traits: &Traits,
        _scope: Option<&'static str>,
        name: &'static str,
        constraints: &Constraints,
        vartype: &Type,
        args: Vec<&'static str>,
        sort_hashsets: bool,
    ) -> Result<RWStatement, Error> {
        let mut switch_vec = vec![];
        let mut pos_vec: Vec<_> = constraints.0.iter().collect();

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
        if !funcvec.is_empty() && funcvec[0].1.is_method && !args.is_empty() {
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

    fn rewrite_indirect_invoke(
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
        match cmap.scoped_get(scope, name, false) {
            Ok(Some(vartype)) => match vartype {
                VarType::Values(vartype, constraints) => self
                    .rewrite_indirect_invoke_helper(
                        funcs,
                        cmap,
                        sigs,
                        traits,
                        scope,
                        name,
                        &constraints,
                        &vartype,
                        args.to_vec(),
                        sort_hashsets,
                    ),
                _ => Err(Error::NoSwitchOnFuncPtr()),
            },
            Ok(None) => panic!("IP BUG: missed undef symbol {:?}", &name),
            Err(err) => Err(err),
        }
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
                funcvec,
                args,
                sort_hashsets,
            ),
            None => self.rewrite_indirect_invoke(
                funcs,
                cmap,
                sigs,
                traits,
                scope,
                name,
                args,
                sort_hashsets,
            ),
        }
    }
}

#[cfg(test)]
mod rewrite_tests;
