use crate::{AssignmentRVal, BooleanStatement, Error, Funcs, RVal, Statement};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    // scope w backptr to enclosing scope identifier
    // (None == top-level global scope)
    Scope(Option<&'static str>, Vars),
    Values(HashSet<RVal>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vars {
    pub vars: HashMap<&'static str, Box<VarType>>,
}

impl Vars {
    pub fn new() -> Self {
        Self {
            vars: HashMap::<&'static str, Box<VarType>>::new(),
        }
    }

    pub fn scoped_get(
        &self,
        scope: Option<&'static str>,
        var: &'static str,
    ) -> Result<Option<VarType>, Error> {
        // first get the `vars` object pertaining to `this` scope
        if scope.is_none() {
            match self.vars.get(var) {
                Some(boxed) => return Ok(Some(*boxed.clone())),
                None => return Ok(None),
            }
        }

        match self.vars.get(scope.unwrap()) {
            Some(vartype) => match *vartype.clone() {
                VarType::Scope(backptr, inner_vars) => {
                    // is var in inner_vars? if not, recursively follow backptr
                    // to enclosing scopes
                    match inner_vars.vars.get(var) {
                        Some(boxed) => return Ok(Some(*boxed.clone())),
                        None => {
                            return self.scoped_get(backptr, var);
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
        // first get the `vars` object pertaining to `this` scope
        if scope.is_none() {
            if self.vars.get(var).is_some() {
                return Err(Error::SymbolAlreadyExists(var));
            }

            self.vars.insert(var, value.clone());
            return Ok(());
        }

        match self.vars.get(scope.unwrap()) {
            Some(vartype) => match *vartype.clone() {
                VarType::Scope(backptr, mut inner_vars) => {
                    if inner_vars.vars.get(var).is_some() {
                        return Err(Error::SymbolAlreadyExists(var));
                    }
                    // modify scope w new var
                    inner_vars.vars.insert(var, value);
                    self.vars.insert(
                        scope.unwrap(),
                        Box::new(VarType::Scope(backptr, inner_vars)),
                    );
                }
                VarType::Values(_) => {
                    return Err(Error::NotAScope(scope.unwrap()));
                }
            },
            None => return Err(Error::UndefinedScope(scope.unwrap())),
        }
        Ok(())
    }
}

pub trait Merge<T> {
    fn merge(&self) -> Result<T, Error>;
}

impl Merge<Vars> for Vec<Vars> {
    fn merge(&self) -> Result<Vars, Error> {
        if self.len() == 0 {
            return Err(Error::VecSize());
        }
        if self.len() == 1 {
            return Ok(self[0].clone());
        }

        let mut merged = self[0].clone();
        for i in 1..self.len() {
            for (key, val) in self[i].clone().vars.iter() {
                match merged.vars.get_mut(key) {
                    Some(mval) => match (*mval.clone(), *val.clone()) {
                        (
                            VarType::Scope(backptr_a, vars_a),
                            VarType::Scope(backptr_b, vars_b),
                        ) => {
                            if vars_a != vars_b {
                                let inner_vars = vec![vars_a, vars_b];
                                match inner_vars.merge() {
                                    Ok(merged_inner) => {
                                        if backptr_a != backptr_b {
                                            todo!(
                                                "when UNEQUAL scope backptrs?"
                                            );
                                        }
                                        *mval = Box::new(VarType::Scope(
                                            backptr_a,
                                            merged_inner,
                                        ));
                                    }
                                    err @ Err(_) => return err,
                                }
                            }
                        }
                        (VarType::Values(a), VarType::Values(b)) => {
                            if a != b {
                                let union: HashSet<_> =
                                    a.union(&b).map(|x| x.clone()).collect();
                                merged.vars.insert(
                                    key,
                                    Box::new(VarType::Values(union)),
                                );
                            }
                        }
                        _ => return Err(Error::IncomparableVarTypes()),
                    },
                    None => {
                        merged.vars.insert(key, val.clone());
                    }
                }
            }
        }

        Ok(merged)
    }
}

type Value = HashSet<RVal>;

impl Merge<Value> for Vec<Value> {
    fn merge(&self) -> Result<Value, Error> {
        if self.len() == 0 {
            return Err(Error::VecSize());
        }
        if self.len() == 1 {
            return Ok(self[0].clone());
        }

        // TODO check here if all ints vs vars?
        let mut merged = self[0].clone();
        for i in 1..self.len() {
            let union: HashSet<_> =
                merged.union(&self[i]).map(|x| x.clone()).collect();
            merged = union;
        }

        Ok(merged)
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
        vars: &mut Vars,
        scope: Option<&'static str>,
        stmt: &Statement,
    ) -> Result<Option<Value>, Error> {
        match stmt {
            Statement::Sequence(stmt_vec) => {
                self.interp_seq(funcs, vars, scope, stmt_vec)
            }
            Statement::Assignment(var, value) => {
                self.interp_assignment(funcs, vars, scope, var, value)
            }
            Statement::Print(var) => {
                self.interp_print(var);
                Ok(None)
            }
            Statement::Conditional(condition, true_branch, false_branch) => {
                self.interp_conditional(
                    funcs,
                    vars,
                    scope,
                    &*condition,
                    &*true_branch,
                    &*false_branch,
                )
            }
            Statement::Switch(val, vec) => {
                self.interp_switch(funcs, vars, scope, val, vec)
            }
            Statement::Return(rval) => {
                self.interp_return(funcs, vars, scope, rval)
            }
            Statement::FuncDef(..) => Ok(None),
            Statement::InvokeFunc(name, args) => {
                self.interp_invoke(funcs, vars, scope, name, args)
            }
        }
    }

    pub fn interp_seq(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        scope: Option<&'static str>,
        stmt_vec: &Vec<Box<Statement>>,
    ) -> Result<Option<Value>, Error> {
        let mut last_ret = None;
        for stmt in stmt_vec.iter() {
            let res = self.interp(&funcs, vars, scope, &*stmt);
            if res.is_err() {
                return res;
            }
            last_ret = res.unwrap();
        }
        Ok(last_ret)
    }

    pub fn interp_assignment(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        scope: Option<&'static str>,
        var: &'static str,
        value: &AssignmentRVal,
    ) -> Result<Option<Value>, Error> {
        match value {
            AssignmentRVal::RVal(RVal::Num(num)) => {
                let res = vars.scoped_set(
                    scope,
                    var,
                    Box::new(VarType::Values(HashSet::from([RVal::Num(*num)]))),
                );

                if res.is_err() {
                    return Err(res.err().unwrap());
                }
            }
            AssignmentRVal::RVal(RVal::Var(varname)) => {
                match vars.scoped_get(scope, varname) {
                    Ok(Some(val)) => match val {
                        set @ VarType::Values(_) => {
                            let res =
                                vars.scoped_set(scope, var, Box::new(set));

                            if res.is_err() {
                                return Err(res.err().unwrap());
                            }
                        }
                        _ => todo!("not impl yet"),
                    },
                    Ok(None) => match funcs.funcs.get(varname) {
                        Some(_) => {
                            let res = vars.scoped_set(
                                scope,
                                var,
                                Box::new(VarType::Values(HashSet::from([
                                    RVal::Var(varname),
                                ]))),
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
                    match self.interp_invoke(funcs, vars, scope, name, &args) {
                        Ok(Some(retval)) => {
                            let res = vars.scoped_set(
                                scope,
                                var,
                                Box::new(VarType::Values(retval)),
                            );

                            if res.is_err() {
                                return Err(res.err().unwrap());
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

    pub fn interp_print(&self, var: &'static str) {
        println!("{:#?}", var);
    }

    pub fn interp_bool(
        &self,
        funcs: &Funcs,
        vars: &Vars,
        scope: Option<&'static str>,
        b_stmt: BooleanStatement,
    ) -> Result<BooleanStatement, Error> {
        match b_stmt {
            BooleanStatement::True()
            | BooleanStatement::False()
            | BooleanStatement::TrueOrFalse() => Ok(b_stmt),
            BooleanStatement::Not(inner_b_stmt) => {
                self.interp_not(funcs, vars, scope, *inner_b_stmt)
            }
            BooleanStatement::Equals(lhs, rhs) => {
                self.interp_equals(funcs, vars, scope, lhs, rhs)
            }
        }
    }

    pub fn interp_not(
        &self,
        funcs: &Funcs,
        vars: &Vars,
        scope: Option<&'static str>,
        b_stmt: BooleanStatement,
    ) -> Result<BooleanStatement, Error> {
        match self.interp_bool(funcs, vars, scope, b_stmt) {
            Ok(b_res) => Ok(!b_res),
            err @ Err(_) => err,
        }
    }

    pub fn interp_rval(
        &self,
        funcs: &Funcs,
        vars: &Vars,
        scope: Option<&'static str>,
        rval: RVal,
    ) -> Result<HashSet<RVal>, Error> {
        match rval.clone() {
            RVal::Num(_) => Ok(HashSet::from([rval])),
            RVal::Var(var) => match vars.scoped_get(scope, var) {
                Ok(Some(val)) => match val {
                    VarType::Values(set) => return Ok(set.clone()),
                    _ => todo!("should be a func"),
                },
                Ok(None) => match funcs.funcs.get(var) {
                    Some(_) => Ok(HashSet::from([rval])),
                    None => Err(Error::UndefinedSymbol(var)),
                },
                Err(err) => return Err(err),
            },
        }
    }

    pub fn interp_equals(
        &self,
        funcs: &Funcs,
        vars: &Vars,
        scope: Option<&'static str>,
        lhs: RVal,
        rhs: RVal,
    ) -> Result<BooleanStatement, Error> {
        let lhs_res = self.interp_rval(&funcs, &vars, scope, lhs);
        if lhs_res.is_err() {
            return Err(lhs_res.err().unwrap());
        }
        let rhs_res = self.interp_rval(&funcs, &vars, scope, rhs);
        if rhs_res.is_err() {
            return Err(rhs_res.err().unwrap());
        }

        let lhs_vec: Vec<_> = lhs_res.as_ref().unwrap().iter().collect();
        let rhs_vec: Vec<_> = rhs_res.as_ref().unwrap().iter().collect();
        if lhs_vec.len() == 1 && rhs_vec.len() == 1 {
            match (lhs_vec[0].clone(), rhs_vec[0].clone()) {
                (RVal::Num(lnum), RVal::Num(rnum)) => {
                    if lnum == rnum {
                        return Ok(BooleanStatement::True());
                    } else {
                        return Ok(BooleanStatement::False());
                    }
                }
                (RVal::Var(lfp), RVal::Var(rfp)) => {
                    if lfp == rfp {
                        return Ok(BooleanStatement::True());
                    } else {
                        return Ok(BooleanStatement::False());
                    }
                }
                (_, _) => {
                    return Err(Error::IncomparableTypes(
                        lhs_vec[0].clone(),
                        rhs_vec[0].clone(),
                    ));
                }
            }
        } else {
            // TODO could potentially do more analysis here
            return Ok(BooleanStatement::TrueOrFalse());
        }
    }

    pub fn interp_conditional(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        scope: Option<&'static str>,
        condition: &BooleanStatement,
        true_branch: &Statement,
        false_branch: &Statement,
    ) -> Result<Option<Value>, Error> {
        let mut res_vars: Vec<Vars> = vec![];

        // FIXME mod store if have effects
        match self.interp_bool(funcs, vars, scope, condition.clone()) {
            Ok(bool_res) => {
                let mut vars_clone = vars.clone();
                if self.possible(&bool_res) {
                    match self.interp(funcs, vars, scope, true_branch) {
                        Ok(_) => res_vars.push(vars.clone()),
                        err @ Err(_) => return err,
                    }
                }
                if self.possible(!&bool_res) {
                    match self.interp(
                        funcs,
                        &mut vars_clone,
                        scope,
                        false_branch,
                    ) {
                        Ok(_) => res_vars.push(vars_clone),
                        err @ Err(_) => return err,
                    }
                }
            }
            Err(c_err) => return Err(c_err),
        }

        match res_vars.merge() {
            Ok(new_vars) => {
                *vars = new_vars;
                Ok(None)
            }
            Err(err) => Err(err),
        }
    }

    pub fn possible(&self, possible_b: &BooleanStatement) -> bool {
        match possible_b {
            BooleanStatement::True() => true,
            BooleanStatement::False() => false,
            BooleanStatement::TrueOrFalse() => true,
            _ => panic!("boolean statement not fully evaluated"),
        }
    }

    pub fn interp_switch(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        scope: Option<&'static str>,
        val: &RVal,
        vec: &Vec<(RVal, Box<Statement>)>,
    ) -> Result<Option<Value>, Error> {
        // FIXME mod store if have effects
        let resolved_vals = match val {
            num @ RVal::Num(_) => HashSet::from([num.clone()]),
            RVal::Var(varname) => match vars.scoped_get(scope, varname) {
                Ok(Some(vartype)) => match vartype {
                    VarType::Values(possible_vals) => possible_vals.clone(),
                    _ => panic!("should not get scope here"),
                },
                Ok(None) => return Err(Error::UndefinedSymbol(varname)),
                Err(err) => return Err(err),
            },
        };

        // grab all vec elems where vec_val is in vars_vals
        let matching_vals: Vec<_> = vec
            .clone()
            .into_iter()
            .filter(|(vec_val, _)| resolved_vals.contains(vec_val))
            .collect();

        // loop to interp all such elems
        let mut res_vars: Vec<Vars> = Vec::new();
        for (_, vec_stmt) in matching_vals.iter() {
            let mut scoped_vars = vars.clone();
            match self.interp(funcs, &mut scoped_vars, scope, &*vec_stmt) {
                Ok(_) => res_vars.push(scoped_vars),
                err @ Err(_) => return err,
            }
        }

        match res_vars.merge() {
            Ok(new_vars) => {
                *vars = new_vars;
                Ok(None)
            }
            Err(err) => Err(err),
        }
    }

    fn interp_return(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        scope: Option<&'static str>,
        rval: &RVal,
    ) -> Result<Option<Value>, Error> {
        match rval {
            num @ RVal::Num(_) => {
                return Ok(Some(HashSet::from([num.clone()])));
            }
            RVal::Var(varname) => match vars.scoped_get(scope, varname) {
                Ok(Some(vartype)) => match vartype {
                    VarType::Values(value_set) => Ok(Some(value_set)),
                    VarType::Scope(..) => match funcs.funcs.get(varname) {
                        Some(_) => {
                            Ok(Some(HashSet::from([RVal::Var(*varname)])))
                        }
                        None => return Err(Error::UndefinedSymbol(varname)),
                    },
                },
                Ok(None) => match funcs.funcs.get(varname) {
                    Some(_) => panic!("func should have been a scope in vars"),
                    None => return Err(Error::UndefinedSymbol(varname)),
                },
                Err(err) => return Err(err),
            },
        }
    }

    fn resolve_args(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        scope: Option<&'static str>,
        name: &'static str,
        params: &Vec<&'static str>,
        args: &Vec<&'static str>,
    ) -> Result<(), Error> {
        let mut func_vars = Vars::new();
        for (param, arg) in std::iter::zip(params, args) {
            match vars.scoped_get(scope, arg) {
                Ok(Some(vartype)) => match vartype {
                    // add args to scope
                    VarType::Values(value_set) => func_vars.vars.insert(
                        param,
                        Box::new(VarType::Values(value_set.clone())),
                    ),
                    _ => panic!("should not get scope here"),
                },
                Ok(None) => match funcs.funcs.get(arg) {
                    Some(_func_data) => {
                        todo!("not impl yet");
                    }
                    None => return Err(Error::UndefinedSymbol(arg)),
                },
                Err(err) => return Err(err),
            };
        }

        vars.vars
            .insert(name, Box::new(VarType::Scope(scope, func_vars)));

        Ok(())
    }

    fn interp_indirect_invoke_helper(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        scope: Option<&'static str>,
        name: &'static str,
        val: &RVal,
        args: &Vec<&'static str>,
    ) -> Result<Option<Value>, Error> {
        match val {
            RVal::Var(varname) => match funcs.funcs.get(varname) {
                Some(func_data) => {
                    let res = self.resolve_args(
                        funcs,
                        vars,
                        scope,
                        varname,
                        &func_data.params,
                        args,
                    );
                    if res.is_err() {
                        return Err(res.err().unwrap());
                    }

                    match self.interp(
                        funcs,
                        vars,
                        Some(varname),
                        &*func_data.body,
                    ) {
                        Ok(retval) => return Ok(retval),
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
        vars: &mut Vars,
        scope: Option<&'static str>,
        name: &'static str,
        set: &HashSet<RVal>,
        args: &Vec<&'static str>,
    ) -> Result<Option<Value>, Error> {
        let mut res_vars: Vec<Vars> = vec![];
        let mut retvals: Vec<Value> = vec![];
        for val in set.iter() {
            let mut vars_clone = vars.clone();
            match self.interp_indirect_invoke_helper(
                funcs,
                &mut vars_clone,
                scope,
                name,
                val,
                args,
            ) {
                Ok(retval) => {
                    res_vars.push(vars_clone);
                    if retval.is_some() {
                        retvals.push(retval.unwrap());
                    }
                }
                err @ Err(_) => return err,
            }
        }

        let res = res_vars.merge();
        if res.is_err() {
            return Err(res.err().unwrap());
        }
        *vars = res.unwrap();

        if retvals.len() > 0 {
            return match retvals.merge() {
                Ok(retval) => Ok(Some(retval)),
                Err(err) => Err(err),
            };
        }
        Ok(None)
    }

    fn interp_direct_invoke(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        scope: Option<&'static str>,
        name: &'static str,
        args: &Vec<&'static str>,
    ) -> Result<Option<Value>, Error> {
        match funcs.funcs.get(name) {
            Some(func_data) => {
                let res = self.resolve_args(
                    funcs,
                    vars,
                    scope,
                    name,
                    &func_data.params,
                    args,
                );
                if res.is_err() {
                    return Err(res.err().unwrap());
                }

                match self.interp(funcs, vars, Some(name), &*func_data.body) {
                    res @ Ok(_) => res,
                    err @ Err(_) => err,
                }
            }
            None => return Err(Error::UndefinedSymbol(name)),
        }
    }

    pub fn interp_invoke(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        scope: Option<&'static str>,
        name: &'static str,
        args: &Vec<&'static str>,
    ) -> Result<Option<Value>, Error> {
        match vars.scoped_get(scope, name) {
            Ok(Some(vartype)) => match vartype {
                VarType::Values(set) => self.interp_indirect_invoke(
                    funcs, vars, scope, name, &set, args,
                ),
                _ => panic!("should not get scope here"),
            },
            Ok(None) => {
                self.interp_direct_invoke(funcs, vars, scope, name, args)
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
        Switch,
    };
    use crate::func_collect::Funcs;
    use crate::{FuncVal, Type};

    #[test]
    fn test_merge_none() {
        let vec: Vec<Vars> = Vec::new();
        assert_eq!(vec.merge(), Err(Error::VecSize()));
    }

    #[test]
    fn test_merge_one() {
        let mut vars = Vars::new();
        vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        let vec: Vec<Vars> = vec![vars];
        assert_eq!(vec[0].clone(), vec.merge().unwrap());
    }

    #[test]
    fn test_merge_two() {
        let mut vars1 = Vars::new();
        vars1.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        let mut vars2 = Vars::new();
        vars2.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(6)]))),
        );
        let vec: Vec<Vars> = vec![vars1, vars2];

        let mut end_vars = Vars::new();
        end_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([
                RVal::Num(5),
                RVal::Num(6),
            ]))),
        );
        assert_eq!(end_vars, vec.merge().unwrap());
    }

    #[test]
    fn test_print() {
        let interp = Interpreter::new();
        let stmt = Print("hello");
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, Vars::new());
    }

    #[test]
    fn test_assign_num() {
        let interp = Interpreter::new();
        let stmt =
            Assignment("x", Box::new(AssignmentRVal::RVal(RVal::Num(5))));
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        check_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(6)]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        check_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(6)]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_var_undef() {
        let interp = Interpreter::new();
        let stmt = Sequence(vec![Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Var("y"))),
        ))]);
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

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
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        check_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(6)]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([
                RVal::Num(5),
                RVal::Num(6),
            ]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(6)]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );
        check_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );
        check_vars.vars.insert(
            "z",
            Box::new(VarType::Values(HashSet::from([RVal::Num(1)]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
            FuncVal::new(vec![], vec![], None, foo_body.clone()),
        );
        funcs.funcs.insert(
            "bar",
            FuncVal::new(vec![], vec![], None, foo_body.clone()),
        );

        let mut vars = Vars::new();

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
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "z",
            Box::new(VarType::Values(HashSet::from([RVal::Num(2)]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
            FuncVal::new(vec![], vec![], None, foo_body.clone()),
        );

        let mut vars = Vars::new();

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
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "bar",
            Box::new(VarType::Values(HashSet::from([RVal::Var("foo")]))),
        );
        check_vars.vars.insert(
            "z",
            Box::new(VarType::Values(HashSet::from([RVal::Num(1)]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );
        check_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([
                RVal::Num(3),
                RVal::Num(4),
            ]))),
        );
        check_vars.vars.insert(
            "z",
            Box::new(VarType::Values(HashSet::from([
                RVal::Num(1),
                RVal::Num(2),
            ]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
            FuncVal::new(vec![], vec![], None, foo_body.clone()),
        );

        let mut vars = Vars::new();

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
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

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
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_funcdef() {
        let mut funcs = Funcs::new();
        let body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        funcs
            .funcs
            .insert("foo", FuncVal::new(vec![], vec![], None, body.clone()));

        let mut vars = Vars::new();

        let interp = Interpreter::new();
        let stmt = FuncDef("foo", vec![], vec![], None, body);
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, Vars::new());
    }

    #[test]
    fn test_direct_invoke() {
        let mut funcs = Funcs::new();
        let body = Box::new(Sequence(vec![Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        funcs
            .funcs
            .insert("foo", FuncVal::new(vec![], vec![], None, body.clone()));

        let mut vars = Vars::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, body)),
            Box::new(InvokeFunc("foo", vec![])),
        ]);
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        let mut foo_vars = Vars::new();
        foo_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_funcdef_args_direct() {
        let mut funcs = Funcs::new();
        let mut vars = Vars::new();
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
            "foo",
            FuncVal::new(vec![Type::Int()], vec!["y"], None, body.clone()),
        );
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        let mut foo_vars = Vars::new();
        check_vars.vars.insert(
            "arg",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        foo_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        foo_vars.vars.insert(
            "z",
            Box::new(VarType::Values(HashSet::from([RVal::Num(0)]))),
        );
        foo_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_indirect_invoke_simple() {
        let mut funcs = Funcs::new();
        let body = Box::new(Sequence(vec![Box::new(Assignment(
            "z",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        funcs
            .funcs
            .insert("foo", FuncVal::new(vec![], vec![], None, body.clone()));

        let mut vars = Vars::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, body)),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]);
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        let mut foo_vars = Vars::new();
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Var("foo")]))),
        );
        foo_vars.vars.insert(
            "z",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_funcdef_args_indirect() {
        let mut funcs = Funcs::new();
        let mut vars = Vars::new();
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
            "foo",
            FuncVal::new(vec![Type::Int()], vec!["y"], None, body.clone()),
        );
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        let mut foo_vars = Vars::new();
        foo_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        foo_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        foo_vars.vars.insert(
            "z",
            Box::new(VarType::Values(HashSet::from([RVal::Num(0)]))),
        );
        check_vars.vars.insert(
            "arg",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        check_vars.vars.insert(
            "w",
            Box::new(VarType::Values(HashSet::from([RVal::Var("foo")]))),
        );
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
        funcs
            .funcs
            .insert("bar", FuncVal::new(vec![], vec![], None, bar_body));
        funcs
            .funcs
            .insert("foo", FuncVal::new(vec![], vec![], None, foo_body));

        let mut vars = Vars::new();

        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        let foo_vars = Vars::new();
        let mut bar_vars = Vars::new();
        bar_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        check_vars
            .vars
            .insert("bar", Box::new(VarType::Scope(Some("foo"), bar_vars)));
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
            "foo",
            FuncVal::new(vec![], vec![], None, foo_body.clone()),
        );
        funcs.funcs.insert(
            "bar",
            FuncVal::new(vec![], vec![], None, bar_body.clone()),
        );
        funcs.funcs.insert(
            "baz",
            FuncVal::new(vec![], vec![], None, baz_body.clone()),
        );
        funcs.funcs.insert(
            "qux",
            FuncVal::new(vec![], vec![], None, qux_body.clone()),
        );
        funcs.funcs.insert(
            "baz2",
            FuncVal::new(vec![], vec![], None, baz2_body.clone()),
        );
        funcs.funcs.insert(
            "qux2",
            FuncVal::new(vec![], vec![], None, qux2_body.clone()),
        );

        let mut vars = Vars::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        let mut foo_vars = Vars::new();
        let mut bar_vars = Vars::new();
        let mut baz_vars = Vars::new();
        let mut qux_vars = Vars::new();
        let mut baz2_vars = Vars::new();
        let mut qux2_vars = Vars::new();

        baz_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(1)]))),
        );
        qux_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(2)]))),
        );
        baz2_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );
        qux2_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(4)]))),
        );

        foo_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([
                RVal::Var("baz"),
                RVal::Var("qux"),
            ]))),
        );
        bar_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([
                RVal::Var("baz2"),
                RVal::Var("qux2"),
            ]))),
        );

        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([
                RVal::Var("foo"),
                RVal::Var("bar"),
            ]))),
        );
        check_vars
            .vars
            .insert("baz2", Box::new(VarType::Scope(Some("bar"), baz2_vars)));
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));
        check_vars
            .vars
            .insert("qux", Box::new(VarType::Scope(Some("foo"), qux_vars)));
        check_vars
            .vars
            .insert("baz", Box::new(VarType::Scope(Some("foo"), baz_vars)));
        check_vars
            .vars
            .insert("bar", Box::new(VarType::Scope(None, bar_vars)));
        check_vars
            .vars
            .insert("qux2", Box::new(VarType::Scope(Some("bar"), qux2_vars)));

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
            "bar",
            FuncVal::new(vec![Type::Int()], vec!["y"], None, bar_body),
        );
        funcs.funcs.insert(
            "foo",
            FuncVal::new(vec![Type::Int()], vec!["y"], None, foo_body),
        );

        let mut vars = Vars::new();

        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        let mut foo_vars = Vars::new();
        let mut bar_vars = Vars::new();
        bar_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );
        bar_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );
        foo_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );
        check_vars.vars.insert(
            "arg",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );
        check_vars
            .vars
            .insert("bar", Box::new(VarType::Scope(Some("foo"), bar_vars)));
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
            "foo",
            FuncVal::new(vec![Type::Int()], vec!["y"], None, foo_body.clone()),
        );
        funcs.funcs.insert(
            "bar",
            FuncVal::new(vec![Type::Int()], vec!["y"], None, bar_body.clone()),
        );
        funcs.funcs.insert(
            "baz",
            FuncVal::new(vec![Type::Int()], vec!["y"], None, baz_body.clone()),
        );
        funcs.funcs.insert(
            "qux",
            FuncVal::new(vec![Type::Int()], vec!["y"], None, qux_body.clone()),
        );
        funcs.funcs.insert(
            "baz2",
            FuncVal::new(vec![Type::Int()], vec!["y"], None, baz2_body.clone()),
        );
        funcs.funcs.insert(
            "qux2",
            FuncVal::new(vec![Type::Int()], vec!["y"], None, qux2_body.clone()),
        );

        let mut vars = Vars::new();
        let interp = Interpreter::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        let mut foo_vars = Vars::new();
        let mut bar_vars = Vars::new();
        let mut baz_vars = Vars::new();
        let mut qux_vars = Vars::new();
        let mut baz2_vars = Vars::new();
        let mut qux2_vars = Vars::new();

        baz_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );
        baz_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );
        qux_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );
        qux_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );
        baz2_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );
        baz2_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );
        qux2_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );
        qux2_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );

        foo_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([
                RVal::Var("baz"),
                RVal::Var("qux"),
            ]))),
        );
        foo_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );
        bar_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([
                RVal::Var("baz2"),
                RVal::Var("qux2"),
            ]))),
        );
        bar_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );

        check_vars
            .vars
            .insert("baz", Box::new(VarType::Scope(Some("foo"), baz_vars)));
        check_vars
            .vars
            .insert("bar", Box::new(VarType::Scope(None, bar_vars)));
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([
                RVal::Var("foo"),
                RVal::Var("bar"),
            ]))),
        );
        check_vars
            .vars
            .insert("qux", Box::new(VarType::Scope(Some("foo"), qux_vars)));
        check_vars
            .vars
            .insert("baz2", Box::new(VarType::Scope(Some("bar"), baz2_vars)));
        check_vars
            .vars
            .insert("qux2", Box::new(VarType::Scope(Some("bar"), qux2_vars)));
        check_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(3)]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
            FuncVal::new(vec![], vec![], None, foo_body.clone()),
        );
        funcs.funcs.insert(
            "bar",
            FuncVal::new(vec![], vec![], None, bar_body.clone()),
        );

        let mut vars = Vars::new();

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
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        let mut foo_vars = Vars::new();
        let mut bar_vars = Vars::new();
        foo_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        bar_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(6)]))),
        );
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));
        check_vars
            .vars
            .insert("bar", Box::new(VarType::Scope(None, bar_vars)));

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
            FuncVal::new(vec![], vec![], None, foo_body.clone()),
        );
        funcs.funcs.insert(
            "bar",
            FuncVal::new(vec![], vec![], None, bar_body.clone()),
        );

        let mut vars = Vars::new();

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
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        let mut foo_vars = Vars::new();
        let mut bar_vars = Vars::new();
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([
                RVal::Var("foo"),
                RVal::Var("bar"),
            ]))),
        );
        foo_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        bar_vars.vars.insert(
            "z",
            Box::new(VarType::Values(HashSet::from([RVal::Num(6)]))),
        );
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));
        check_vars
            .vars
            .insert("bar", Box::new(VarType::Scope(None, bar_vars)));

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        assert_eq!(res, Err(Error::NotAFunction("foo")));
    }

    #[test]
    fn test_switch_err() {
        let interp = Interpreter::new();
        let switch_vec: Vec<(RVal, Box<Statement>)> = vec![];
        let stmt = Switch(RVal::Var("x"), switch_vec);
        let res = interp.interp(&Funcs::new(), &mut Vars::new(), None, &stmt);

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
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );
        check_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([RVal::Num(1)]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars.vars.insert(
            "y",
            Box::new(VarType::Values(HashSet::from([
                RVal::Num(0),
                RVal::Num(1),
            ]))),
        );
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([
                RVal::Num(5),
                RVal::Num(4),
            ]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_direct_func_return_num() {
        let mut funcs = Funcs::new();
        let body = Box::new(Return(RVal::Num(5)));
        funcs
            .funcs
            .insert("foo", FuncVal::new(vec![], vec![], None, body.clone()));

        let mut vars = Vars::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, body)),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::Statement(Box::new(InvokeFunc(
                    "foo",
                    vec![],
                )))),
            )),
        ]);
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, Vars::new())));
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([RVal::Num(5)]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
            "foo",
            FuncVal::new(
                vec![],
                vec![],
                Some(Box::new(Type::Int())),
                body.clone(),
            ),
        );

        let mut vars = Vars::new();

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
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        let mut foo_vars = Vars::new();
        foo_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([
                RVal::Num(5),
                RVal::Num(6),
            ]))),
        );
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([
                RVal::Num(5),
                RVal::Num(6),
            ]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
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
            "foo",
            FuncVal::new(
                vec![],
                vec![],
                Some(Box::new(Type::Func(vec![], rettype.clone()))),
                foo_body.clone(),
            ),
        );
        funcs.funcs.insert(
            "bar",
            FuncVal::new(
                vec![],
                vec![],
                Some(Box::new(Type::Func(vec![], rettype.clone()))),
                bar_body.clone(),
            ),
        );
        funcs.funcs.insert(
            "baz",
            FuncVal::new(
                vec![],
                vec![],
                Some(Box::new(Type::Func(vec![], rettype.clone()))),
                baz_body.clone(),
            ),
        );

        let mut vars = Vars::new();

        let interp = Interpreter::new();
        let stmt = Sequence(vec![
            Box::new(FuncDef(
                "foo",
                vec![],
                vec![],
                Some(Box::new(Type::Int())),
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
        let res = interp.interp(&funcs, &mut vars, None, &stmt);

        let mut check_vars = Vars::new();
        let mut foo_vars = Vars::new();
        foo_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([
                RVal::Var("bar"),
                RVal::Var("baz"),
            ]))),
        );
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));
        check_vars
            .vars
            .insert("bar", Box::new(VarType::Scope(None, Vars::new())));
        check_vars
            .vars
            .insert("baz", Box::new(VarType::Scope(None, Vars::new())));
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(HashSet::from([
                RVal::Var("bar"),
                RVal::Var("baz"),
            ]))),
        );
        check_vars.vars.insert(
            "res",
            Box::new(VarType::Values(HashSet::from([
                RVal::Num(1),
                RVal::Num(0),
            ]))),
        );

        assert_eq!(res.unwrap(), None);
        assert_eq!(vars, check_vars);
    }
}
