use crate::func_collect::Funcs;
use crate::interpret::{ConstraintMap, Constraints, Traits, VarType};
use crate::{
    BooleanStatement, Error, FuncVal, RVal, SigVal, Sigs, Statement,
    TraitStructOpt, TraitStructTup, Type,
};

use std::collections::HashSet;

pub struct Rewriter {}

/// Implement rewriter

// TODO never returns Err, refactor out Result return type (wait on this)

impl Rewriter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn rewrite(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        sigs: &Sigs,
        traits: &Traits,
        scope: Option<&'static str>,
        stmt: &mut Statement,
        sort_hashsets: bool,
    ) -> Result<(), Error> {
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
            Statement::Conditional(condition, true_branch, false_branch) => {
                self.rewrite_conditional(
                    funcs,
                    cmap,
                    sigs,
                    traits,
                    scope,
                    *condition.clone(),
                    &mut (*true_branch),
                    &mut (*false_branch),
                    sort_hashsets,
                )
            }
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
            Statement::FuncDef(name, _, _, _, body) => self.rewrite_funcdef(
                funcs,
                cmap,
                sigs,
                traits,
                name,
                body,
                sort_hashsets,
            ),
            Statement::InvokeFunc(name, args) => {
                match self.rewrite_invoke(
                    funcs,
                    cmap,
                    sigs,
                    traits,
                    scope,
                    name,
                    args,
                    sort_hashsets,
                ) {
                    Ok(res) => {
                        *stmt = res;
                        Ok(())
                    }
                    Err(err) => Err(err),
                }
            }
            _ => Ok(()),
        }
    }

    fn rewrite_seq(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        sigs: &Sigs,
        traits: &Traits,
        scope: Option<&'static str>,
        stmt_vec: &mut Vec<Box<Statement>>,
        sort_hashsets: bool,
    ) -> Result<(), Error> {
        for stmt in stmt_vec.iter_mut() {
            let res = self.rewrite(
                funcs,
                cmap,
                sigs,
                traits,
                scope,
                stmt,
                sort_hashsets,
            );
            if res.is_err() {
                return res;
            }
        }
        Ok(())
    }

    fn rewrite_conditional(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        sigs: &Sigs,
        traits: &Traits,
        scope: Option<&'static str>,
        _condition: BooleanStatement,
        mut true_branch: &mut Statement,
        mut false_branch: &mut Statement,
        sort_hashsets: bool,
    ) -> Result<(), Error> {
        // FIXME also rewrite condition when funcs can ret booleans
        let res_true = self.rewrite(
            funcs,
            cmap,
            sigs,
            traits,
            scope,
            &mut true_branch,
            sort_hashsets,
        );
        if res_true.is_err() {
            return res_true;
        }

        let res_false = self.rewrite(
            funcs,
            cmap,
            sigs,
            traits,
            scope,
            &mut false_branch,
            sort_hashsets,
        );
        if res_false.is_err() {
            return res_false;
        }

        Ok(())
    }

    fn rewrite_switch(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        sigs: &Sigs,
        traits: &Traits,
        scope: Option<&'static str>,
        _val: &mut RVal,
        vec: &mut Vec<(RVal, Box<Statement>)>,
        sort_hashsets: bool,
    ) -> Result<(), Error> {
        // FIXME if can switch on function call, also perform rewrite on that
        // (omitted val argument)
        for (_, switch_stmt) in vec.iter_mut() {
            let res = self.rewrite(
                funcs,
                cmap,
                sigs,
                traits,
                scope,
                &mut (*switch_stmt),
                sort_hashsets,
            );
            if res.is_err() {
                return res;
            }
        }
        Ok(())
    }

    fn rewrite_funcdef(
        &self,
        funcs: &Funcs,
        cmap: &ConstraintMap,
        sigs: &Sigs,
        traits: &Traits,
        name: &'static str,
        body: &mut Box<Statement>,
        sort_hashsets: bool,
    ) -> Result<(), Error> {
        let res = self.rewrite(
            funcs,
            cmap,
            sigs,
            traits,
            Some(name),
            &mut (*body),
            sort_hashsets,
        );
        if res.is_err() {
            return res;
        }
        Ok(())
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
                RVal::Struct(structname, _) => str_set.insert(structname),
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
    ) -> Result<Statement, Error> {
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
                        Box::new(Statement::InvokeFunc(func_str, args.clone())),
                    ));
                }
            }
            Ok(None) => {
                // given our no-intersection invariant, no elements in the set
                // of positive constraints will be in the set of negative
                // constraints
                for rval in pos_vec.into_iter() {
                    match rval.clone() {
                        r @ RVal::Var(var) => switch_vec.push((
                            r,
                            Box::new(Statement::InvokeFunc(var, args.clone())),
                        )),
                        _ => panic!(
                            "IP BUG: cannot call num {:?} as func",
                            &rval
                        ),
                    };
                }
            }
            Err(err) => return Err(err),
        }

        Ok(Statement::Switch(RVal::Var(name), switch_vec))
    }

    // FIXME merge trait_invoke w indirect_invoke ?
    fn rewrite_trait_invoke(
        &self,
        _funcs: &Funcs,
        cmap: &ConstraintMap,
        _sigs: &Sigs,
        _traits: &Traits,
        _scope: Option<&'static str>,
        name: &'static str,
        funcvec: &Vec<(TraitStructOpt, FuncVal)>,
        args: &Vec<&'static str>,
        sort_hashsets: bool,
    ) -> Result<Statement, Error> {
        if funcvec.len() == 1 {
            return Ok(Statement::InvokeFunc(name, args.to_vec()));
        }

        let mut switch_vec = vec![];

        // get type of `self` (first arg) FIXME add bool for this
        match cmap.cmap.get(args[0]) {
            Some(val) => match *val.clone() {
                VarType::Values(vartype, constraints) => {
                    let mut pos_vec: Vec<_> = constraints.0.iter().collect();
                    // sorting to get deterministic switch statement order in
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
                                    Box::new(Statement::InvokeTraitFunc(
                                        name,
                                        // FIXME need trait name?
                                        (trait_name, struct_name),
                                        args.clone(),
                                    )),
                                ));
                            }
                        }
                        Ok(None) => {
                            // FIXME use constraints
                            panic!("not impl yet");
                            //for (tso, _) in funcvec.iter() {
                            //    switch_vec.push((
                            //        RVal::Var(args[0]),
                            //        Box::new(Statement::InvokeTraitFunc(
                            //            name,
                            //            tso.unwrap(),
                            //            args.clone(),
                            //        )),
                            //    ));
                            //}
                        }
                        Err(err) => return Err(err),
                    }
                }
                _ => panic!("`self` arg should not be a scope"),
            },
            None => panic!("`self` arg does not exist"),
        }

        Ok(Statement::Switch(RVal::Var(args[0]), switch_vec))
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
    ) -> Result<Statement, Error> {
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
    use crate::Statement::{
        Assignment, Conditional, FuncDef, InvokeFunc, InvokeTraitFunc, Print,
        Return, Sequence, Struct, Switch, TraitDecl, TraitImpl,
    };
    use crate::func_collect::Funcs;
    use crate::{AssignmentRVal, FuncDecl, FuncVal, Type};
    use std::collections::HashSet;

    #[test]
    fn test_print() {
        let mut stmt = Print("hello");
        let check_stmt = stmt.clone();

        let funcs = Funcs::new();
        let cmap = ConstraintMap::new();
        let sigs = Sigs::new();
        let traits = Traits::new();
        let rw = Rewriter::new();
        let _ =
            rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &mut stmt, true);

        assert_eq!(check_stmt, stmt);
    }

    #[test]
    fn test_assignment() {
        let mut stmt =
            Assignment("x", Box::new(AssignmentRVal::RVal(RVal::Num(5))));
        let check_stmt = stmt.clone();

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
        let _ =
            rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &mut stmt, true);

        assert_eq!(check_stmt, stmt);
    }

    #[test]
    fn test_direct_invoke() {
        let body = Box::new(Sequence(vec![Box::new(Assignment(
            "z",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        let mut stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, body.clone())),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(InvokeFunc("foo", vec![])),
        ]);
        let check_stmt = stmt.clone();

        let mut funcs = Funcs::new();
        funcs.funcs.insert(
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, body.clone()))],
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
        let _ =
            rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &mut stmt, true);

        assert_eq!(check_stmt, stmt);
    }

    #[test]
    fn test_indirect_invoke_single_val() {
        let body = Box::new(Sequence(vec![Box::new(Assignment(
            "z",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        let mut stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, body.clone())),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]);

        let mut funcs = Funcs::new();
        funcs.funcs.insert(
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, body.clone()))],
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
        let _ =
            rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &mut stmt, true);

        let switch_vec =
            vec![(RVal::Var("foo"), Box::new(InvokeFunc("foo", vec![])))];
        let check_stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, body)),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(Switch(RVal::Var("x"), switch_vec)),
        ]);

        assert_eq!(stmt, check_stmt);
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
        let mut stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, foo_body.clone())),
            Box::new(FuncDef("bar", false, vec![], None, bar_body.clone())),
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
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, foo_body.clone()))],
        );
        funcs.funcs.insert(
            "bar",
            vec![(None, FuncVal::new(false, vec![], None, bar_body.clone()))],
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
        let _ =
            rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &mut stmt, true);

        let switch_vec = vec![
            (RVal::Var("bar"), Box::new(InvokeFunc("bar", vec![]))),
            (RVal::Var("foo"), Box::new(InvokeFunc("foo", vec![]))),
        ];
        let check_stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, foo_body.clone())),
            Box::new(FuncDef("bar", false, vec![], None, bar_body.clone())),
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
            Box::new(Switch(RVal::Var("x"), switch_vec)),
        ]);

        assert_eq!(stmt, check_stmt);
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
            Box::new(FuncDef("baz2", false, vec![], None, baz2_body.clone())),
            Box::new(FuncDef("qux2", false, vec![], None, qux2_body.clone())),
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

        let mut stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, foo_body.clone())),
            Box::new(FuncDef("bar", false, vec![], None, bar_body.clone())),
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
                Box::new(Type::Func(vec![], None)),
                Some("bar"),
                baz2_cmap,
            )),
        );
        cmap.cmap.insert(
            "foo",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                None,
                foo_cmap,
            )),
        );
        cmap.cmap.insert(
            "qux",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                Some("foo"),
                qux_cmap,
            )),
        );
        cmap.cmap.insert(
            "baz",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                Some("foo"),
                baz_cmap,
            )),
        );
        cmap.cmap.insert(
            "bar",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                None,
                bar_cmap,
            )),
        );
        cmap.cmap.insert(
            "qux2",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![], None)),
                Some("bar"),
                qux2_cmap,
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
        let _ =
            rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &mut stmt, true);

        let foo_switch_vec = vec![
            (RVal::Var("baz"), Box::new(InvokeFunc("baz", vec![]))),
            (RVal::Var("qux"), Box::new(InvokeFunc("qux", vec![]))),
        ];
        let check_foo_body = Box::new(Sequence(vec![
            Box::new(FuncDef("baz", false, vec![], None, baz_body.clone())),
            Box::new(FuncDef("qux", false, vec![], None, qux_body.clone())),
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
            Box::new(Switch(RVal::Var("x"), foo_switch_vec)),
        ]));
        let bar_switch_vec = vec![
            (RVal::Var("baz2"), Box::new(InvokeFunc("baz2", vec![]))),
            (RVal::Var("qux2"), Box::new(InvokeFunc("qux2", vec![]))),
        ];
        let check_bar_body = Box::new(Sequence(vec![
            Box::new(FuncDef("baz2", false, vec![], None, baz2_body.clone())),
            Box::new(FuncDef("qux2", false, vec![], None, qux2_body.clone())),
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
            Box::new(Switch(RVal::Var("x"), bar_switch_vec)),
        ]));
        let switch_vec = vec![
            (RVal::Var("bar"), Box::new(InvokeFunc("bar", vec![]))),
            (RVal::Var("foo"), Box::new(InvokeFunc("foo", vec![]))),
        ];
        let check_stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, check_foo_body.clone())),
            Box::new(FuncDef("bar", false, vec![], None, check_bar_body.clone())),
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
            Box::new(Switch(RVal::Var("x"), switch_vec)),
        ]);

        assert_eq!(stmt, check_stmt);
    }

    #[test]
    fn test_trait_impl() {
        let cat_speak_body = Box::new(Sequence(vec![Box::new(Print("meow"))]));

        let funcdef = FuncDecl::new(true, vec![], None);
        let cat_funcimpl = FuncVal::new(true, vec![("self", Type::Struct("Cat"))], None, cat_speak_body.clone());

        let mut stmt = Sequence(vec![
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
        cmap.cmap.insert(
            "edgar",
            Box::new(VarType::Values(
                Box::new(Type::Struct("Cat")),
                (HashSet::from([RVal::Struct("Cat", vec![])]), HashSet::new()),
            )),
        );

        let mut sigs = Sigs::new();
        sigs.sigs
            .insert(SigVal::new(vec![], None), HashSet::from(["speak"]));

        let traits = Traits::new();
        let rw = Rewriter::new();
        let _ =
            rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &mut stmt, true);

        // TODO call func - either `self` or flexible arg type?

        //assert_eq!(stmt, check_stmt);
    }

    #[test]
    fn test_dyn_traits_two_impl() {
        let funcdecl =
            FuncDecl::new(true, vec![("self", Type::DynTrait("Animal"))], None);

        let cat_speak_body = Box::new(Print("meow"));
        let cat_speak = FuncVal::new(
            true,
            vec![("self", Type::Struct("Cat"))],
            None,
            cat_speak_body.clone(),
        );

        let dog_speak_body = Box::new(Print("woof"));
        let dog_speak = FuncVal::new(
            true,
            vec![("self", Type::Struct("Dog"))],
            None,
            dog_speak_body.clone(),
        );

        let gmaa = Box::new(Sequence(vec![Box::new(Conditional(
            Box::new(BooleanStatement::TrueOrFalse()),
            Box::new(Sequence(vec![Box::new(Return(RVal::Struct(
                "Cat",
                vec![],
            )))])),
            Box::new(Sequence(vec![Box::new(Return(RVal::Struct(
                "Dog",
                vec![],
            )))])),
        ))]));

        let mut stmt = Sequence(vec![
            Box::new(TraitDecl(
                "Animal",
                vec!["speak"],
                vec![funcdecl.clone()],
            )),
            Box::new(FuncDef(
                "giveMeAnAnimal",
                false, 
                vec![],
                Some(Box::new(Type::DynTrait("Animal"))),
                gmaa.clone(),
            )),
            Box::new(Struct("Cat", vec![], vec![])),
            Box::new(Struct("Dog", vec![], vec![])),
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
                Box::new(AssignmentRVal::Statement(Box::new(
                    Statement::InvokeFunc("giveMeAnAnimal", vec![]),
                ))),
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
        let mut speak_cmap = ConstraintMap::new();
        // FIXME
        speak_cmap.cmap.insert(
            "animal",
            Box::new(VarType::Values(
                Box::new(Type::DynTrait("Animal")),
                (
                    HashSet::from([
                        RVal::Struct("Cat", vec![]),
                        RVal::Struct("Dog", vec![]),
                    ]),
                    HashSet::new(),
                ),
            )),
        );
        cmap.cmap.insert(
            "speak",
            Box::new(VarType::Scope(
                Box::new(Type::Func(vec![Type::DynTrait("Animal")], None)),
                None,
                speak_cmap,
            )),
        );
        cmap.cmap.insert(
            "giveMeAnAnimal",
            Box::new(VarType::Scope(
                Box::new(Type::Func(
                    vec![],
                    Some(Box::new(Type::DynTrait("Animal"))),
                )),
                None,
                ConstraintMap::new(),
            )),
        );
        cmap.cmap.insert(
            "specific_animal",
            Box::new(VarType::Values(
                Box::new(Type::DynTrait("Animal")),
                (
                    HashSet::from([
                        RVal::Struct("Cat", vec![]),
                        RVal::Struct("Dog", vec![]),
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
        let _ =
            rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &mut stmt, true);

        let switch_vec = vec![
            (
                RVal::Var("Cat"),
                Box::new(InvokeTraitFunc(
                    "speak",
                    ("Animal", "Cat"),
                    vec!["specific_animal"],
                )),
            ),
            (
                RVal::Var("Dog"),
                Box::new(InvokeTraitFunc(
                    "speak",
                    ("Animal", "Dog"),
                    vec!["specific_animal"],
                )),
            ),
        ];

        let check_stmt = Sequence(vec![
            Box::new(TraitDecl(
                "Animal",
                vec!["speak"],
                vec![funcdecl.clone()],
            )),
            Box::new(FuncDef(
                "giveMeAnAnimal",
                false, 
                vec![],
                Some(Box::new(Type::DynTrait("Animal"))),
                gmaa.clone(),
            )),
            Box::new(Struct("Cat", vec![], vec![])),
            Box::new(Struct("Dog", vec![], vec![])),
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
                Box::new(AssignmentRVal::Statement(Box::new(
                    Statement::InvokeFunc("giveMeAnAnimal", vec![]),
                ))),
            )),
            Box::new(Switch(RVal::Var("specific_animal"), switch_vec)),
        ]);

        assert_eq!(stmt, check_stmt);
    }
}
