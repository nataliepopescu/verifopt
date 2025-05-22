use crate::{BooleanStatement, Error, Funcs, RVal, Statement};
use std::collections::HashMap;

/// Define interpreter state

#[derive(Debug, Clone, PartialEq)]
pub struct Vars {
    pub vars: HashMap<&'static str, Vec<RVal>>,
}

impl Vars {
    pub fn new() -> Self {
        Self {
            vars: HashMap::<&'static str, Vec<RVal>>::new(),
        }
    }
}

// FIXME remove Statement from retval

pub trait Merge {
    fn merge(&self) -> Result<Vars, Error>;
}

impl Merge for Vec<Vars> {
    fn merge(&self) -> Result<Vars, Error> {
        if self.len() == 0 {
            return Err(Error::VecSize());
        }
        if self.len() == 1 {
            return Ok(self[0].clone());
        }

        let mut merged = self[0].clone();
        for i in 1..self.len() {
            for (key, val) in self[i].clone().vars.iter_mut() {
                match merged.vars.get_mut(key) {
                    Some(mut mval) => {
                        if val != mval {
                            val.append(&mut mval);
                            merged.vars.insert(key, val.to_vec());
                        }
                    }
                    None => {
                        merged.vars.insert(key, val.to_vec());
                    }
                }
            }
        }
        Ok(merged)
    }
}

pub struct Interpreter {}

/// Implement interpreter

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interp(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        stmt: Statement,
    ) -> Result<Statement, Error> {
        match stmt {
            Statement::Sequence(stmt_vec) => {
                self.interp_seq(funcs, vars, stmt_vec)
            }
            Statement::Assignment(var, value) => {
                self.interp_assignment(funcs, vars, var, value)
            }
            Statement::Print(var) => {
                self.interp_print(var);
                Ok(stmt)
            }
            Statement::Conditional(condition, true_branch, false_branch) => {
                self.interp_conditional(
                    funcs,
                    vars,
                    *condition,
                    *true_branch,
                    *false_branch,
                )
            }
            Statement::Switch(val, vec) => {
                self.interp_switch(funcs, vars, val, vec)
            }
            Statement::FuncDef(_, _) => Ok(stmt),
            Statement::InvokeFunc(name) => {
                self.interp_invoke(funcs, vars, name)
            }
        }
    }

    pub fn interp_seq(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        stmt_vec: Vec<Box<Statement>>,
    ) -> Result<Statement, Error> {
        for stmt in stmt_vec.iter() {
            let res = self.interp(&funcs, vars, *stmt.clone());
            if res.is_err() {
                return res;
            }
        }
        Ok(Statement::Sequence(stmt_vec))
    }

    pub fn interp_assignment(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        var: &'static str,
        value: RVal,
    ) -> Result<Statement, Error> {
        if vars.vars.get(var).is_some() {
            return Err(Error::VarAlreadyExists(var));
        }

        match value {
            RVal::Num(_) => {
                vars.vars.insert(var, vec![value.clone().into()]);
            }
            ref r @ RVal::Var(varname) => {
                match vars.vars.get(varname) {
                    Some(val) => vars.vars.insert(var, val.clone()),
                    None => match funcs.funcs.get(varname) {
                        Some(_) => vars.vars.insert(var, vec![r.clone()]),
                        None => return Err(Error::UndefinedSymbol(varname)),
                    },
                };
            }
        }
        Ok(Statement::Assignment(var, value))
    }

    pub fn interp_print(&self, var: &'static str) {
        println!("{:#?}", var);
    }

    pub fn interp_bool(
        &self,
        funcs: &Funcs,
        vars: &Vars,
        b_stmt: BooleanStatement,
    ) -> Result<BooleanStatement, Error> {
        match b_stmt {
            BooleanStatement::True()
            | BooleanStatement::False()
            | BooleanStatement::TrueOrFalse() => Ok(b_stmt),
            BooleanStatement::Not(inner_b_stmt) => {
                self.interp_not(funcs, vars, *inner_b_stmt)
            }
            BooleanStatement::Equals(lhs, rhs) => {
                self.interp_equals(funcs, vars, lhs, rhs)
            }
        }
    }

    pub fn interp_not(
        &self,
        funcs: &Funcs,
        vars: &Vars,
        b_stmt: BooleanStatement,
    ) -> Result<BooleanStatement, Error> {
        match self.interp_bool(funcs, vars, b_stmt) {
            Ok(b_res) => Ok(!b_res),
            err @ Err(_) => err,
        }
    }

    pub fn interp_rval(
        &self,
        funcs: &Funcs,
        vars: &Vars,
        rval: RVal,
    ) -> Result<Vec<RVal>, Error> {
        match rval.clone() {
            RVal::Num(_) => Ok(vec![rval]),
            RVal::Var(var) => match vars.vars.get(var) {
                Some(val) => Ok(val.clone()),
                None => match funcs.funcs.get(var) {
                    Some(_) => Ok(vec![rval]),
                    None => Err(Error::UndefinedSymbol(var)),
                },
            },
        }
    }

    pub fn interp_equals(
        &self,
        funcs: &Funcs,
        vars: &Vars,
        lhs: RVal,
        rhs: RVal,
    ) -> Result<BooleanStatement, Error> {
        let lhs_res = self.interp_rval(&funcs, &vars, lhs);
        if lhs_res.is_err() {
            return Err(lhs_res.err().unwrap());
        }
        let rhs_res = self.interp_rval(&funcs, &vars, rhs);
        if rhs_res.is_err() {
            return Err(rhs_res.err().unwrap());
        }

        let lhs_vec = lhs_res.as_ref().unwrap();
        let rhs_vec = rhs_res.as_ref().unwrap();
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
            return Ok(BooleanStatement::TrueOrFalse());
        }
    }

    pub fn interp_conditional(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        condition: BooleanStatement,
        true_branch: Statement,
        false_branch: Statement,
    ) -> Result<Statement, Error> {
        let mut res_vars: Vec<Vars> = vec![];

        // FIXME mod store if have effects
        match self.interp_bool(funcs, vars, condition.clone()) {
            Ok(bool_res) => {
                let mut vars_clone = vars.clone();
                if self.possible(&bool_res) {
                    match self.interp(funcs, vars, true_branch.clone()) {
                        Ok(_) => res_vars.push(vars.clone()),
                        err @ Err(_) => return err,
                    }
                }
                if self.possible(!&bool_res) {
                    match self.interp(
                        funcs,
                        &mut vars_clone,
                        false_branch.clone(),
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
                Ok(Statement::Conditional(
                    Box::new(condition),
                    Box::new(true_branch),
                    Box::new(false_branch),
                ))
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
        val: RVal,
        vec: Vec<(RVal, Box<Statement>)>,
    ) -> Result<Statement, Error> {
        // FIXME mod store if have effects
        let resolved_vals = match val {
            ref num @ RVal::Num(_) => vec![num.clone()],
            RVal::Var(varname) => match vars.vars.get(varname) {
                Some(possible_vals) => possible_vals.to_vec(),
                None => return Err(Error::UndefinedSymbol(varname)),
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
            match self.interp(funcs, &mut scoped_vars, *vec_stmt.clone()) {
                Ok(_) => res_vars.push(scoped_vars.clone()),
                err @ Err(_) => return err,
            }
        }

        match res_vars.merge() {
            Ok(new_vars) => {
                *vars = new_vars;
                Ok(Statement::Switch(val, vec))
            }
            Err(err) => Err(err),
        }
    }

    fn interp_indirect_invoke_helper(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        name: &'static str,
        val: &RVal,
    ) -> Result<Statement, Error> {
        match val {
            RVal::Var(name) => match funcs.funcs.get(name) {
                Some(func_val) => {
                    match self.interp(funcs, vars, *func_val.body.clone()) {
                        Ok(stmt) => return Ok(stmt),
                        err @ Err(_) => return err,
                    }
                }
                None => return Err(Error::UndefinedSymbol(name)),
            },
            _ => return Err(Error::NotAFunction(name)),
        }
    }

    fn interp_indirect_invoke(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        name: &'static str,
        vec: &Vec<RVal>,
    ) -> Result<Statement, Error> {
        let mut res_vars: Vec<Vars> = vec![];
        for val in vec.iter() {
            match self.interp_indirect_invoke_helper(funcs, vars, name, val) {
                Ok(_) => res_vars.push(vars.clone()),
                err @ Err(_) => return err,
            }
        }
        match res_vars.merge() {
            Ok(new_vars) => {
                *vars = new_vars;
                Ok(Statement::InvokeFunc(name))
            }
            Err(err) => Err(err),
        }
    }

    fn interp_direct_invoke(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        name: &'static str,
    ) -> Result<Statement, Error> {
        match funcs.funcs.get(name) {
            Some(func) => match self.interp(funcs, vars, *func.body.clone()) {
                Ok(_) => {
                    return Ok(Statement::InvokeFunc(name));
                }
                err @ Err(_) => return err,
            },
            None => return Err(Error::UndefinedSymbol(name)),
        }
    }

    pub fn interp_invoke(
        &self,
        funcs: &Funcs,
        vars: &mut Vars,
        name: &'static str,
    ) -> Result<Statement, Error> {
        match vars.clone().vars.get(name) {
            Some(vec) => self.interp_indirect_invoke(funcs, vars, name, vec),
            None => self.interp_direct_invoke(funcs, vars, name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FuncVal;
    use crate::func_collect::Funcs;

    #[test]
    fn test_merge_none() {
        let vec: Vec<Vars> = Vec::new();
        assert_eq!(vec.merge(), Err(Error::VecSize()));
    }

    #[test]
    fn test_merge_one() {
        let mut vars = Vars::new();
        vars.vars.insert("x", vec![RVal::Num(5)]);
        let vec: Vec<Vars> = vec![vars];
        assert_eq!(vec[0].clone(), vec.merge().unwrap());
    }

    #[test]
    fn test_merge_two() {
        let mut vars1 = Vars::new();
        vars1.vars.insert("x", vec![RVal::Num(5)]);
        let mut vars2 = Vars::new();
        vars2.vars.insert("x", vec![RVal::Num(6)]);
        let vec: Vec<Vars> = vec![vars1, vars2];

        let mut end_vars = Vars::new();
        end_vars.vars.insert("x", vec![RVal::Num(6), RVal::Num(5)]);
        assert_eq!(end_vars, vec.merge().unwrap());
    }

    #[test]
    fn test_print() {
        let interp = Interpreter::new();
        let stmt = Statement::Print("hello");
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, Vars::new());
    }

    #[test]
    fn test_assign_num() {
        let interp = Interpreter::new();
        let stmt = Statement::Assignment("x", RVal::Num(5));
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_seq() {
        let interp = Interpreter::new();
        let stmt_vec = vec![Box::new(Statement::Assignment("x", RVal::Num(5)))];
        let stmt = Statement::Sequence(stmt_vec);
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_seq_assign() {
        let interp = Interpreter::new();
        let stmt_vec = vec![
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("y", RVal::Num(6))),
        ];
        let stmt = Statement::Sequence(stmt_vec);
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Num(5)]);
        check_vars.vars.insert("y", vec![RVal::Num(6)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_nested_seq() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Sequence(vec![Box::new(
                Statement::Assignment("x", RVal::Num(5)),
            )])),
            Box::new(Statement::Sequence(vec![Box::new(
                Statement::Assignment("y", RVal::Num(6)),
            )])),
        ]);
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Num(5)]);
        check_vars.vars.insert("y", vec![RVal::Num(6)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_var_undef() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![Box::new(Statement::Assignment(
            "x",
            RVal::Var("y"),
        ))]);
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt);

        assert_eq!(res.err(), Some(Error::UndefinedSymbol("y")));
    }

    #[test]
    fn test_assign_var() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("y", RVal::Var("x"))),
        ]);
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Num(5)]);
        check_vars.vars.insert("y", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_conditional_true() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::True()),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_conditional_false() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::False()),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Num(6)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_conditional_uncertain() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::TrueOrFalse()),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars
            .vars
            .insert("x", vec![RVal::Num(6), RVal::Num(5)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_conditional_not() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::Not(Box::new(BooleanStatement::True()))),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Num(6)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_conditional_equals_num() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", RVal::Num(3))),
            Box::new(Statement::Assignment("y", RVal::Num(3))),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("x"),
                    RVal::Var("y"),
                )),
                Box::new(Statement::Assignment("z", RVal::Num(1))),
                Box::new(Statement::Assignment("z", RVal::Num(2))),
            )),
        ]);
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Num(3)]);
        check_vars.vars.insert("y", vec![RVal::Num(3)]);
        check_vars.vars.insert("z", vec![RVal::Num(1)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_conditional_equals_func() {
        let mut funcs = Funcs::new();
        let foo_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
        funcs.funcs.insert("foo", FuncVal::new(foo_body.clone()));
        funcs.funcs.insert("bar", FuncVal::new(foo_body.clone()));

        let mut vars = Vars::new();

        // note: `equals` is _shallow_, which is why it evals to false here
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", foo_body.clone())),
            Box::new(Statement::FuncDef("bar", foo_body.clone())),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("foo"),
                    RVal::Var("bar"),
                )),
                Box::new(Statement::Assignment("z", RVal::Num(1))),
                Box::new(Statement::Assignment("z", RVal::Num(2))),
            )),
        ]);
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("z", vec![RVal::Num(2)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_conditional_equals_func_ref() {
        let mut funcs = Funcs::new();
        let foo_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
        funcs.funcs.insert("foo", FuncVal::new(foo_body.clone()));

        let mut vars = Vars::new();

        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", foo_body.clone())),
            Box::new(Statement::Assignment("bar", RVal::Var("foo"))),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("foo"),
                    RVal::Var("bar"),
                )),
                Box::new(Statement::Assignment("z", RVal::Num(1))),
                Box::new(Statement::Assignment("z", RVal::Num(2))),
            )),
        ]);
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("bar", vec![RVal::Var("foo")]);
        check_vars.vars.insert("z", vec![RVal::Num(1)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_conditional_equals_uncertain() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", RVal::Num(3))),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Statement::Assignment("y", RVal::Num(3))),
                Box::new(Statement::Assignment("y", RVal::Num(4))),
            )),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("x"),
                    RVal::Var("y"),
                )),
                Box::new(Statement::Assignment("z", RVal::Num(1))),
                Box::new(Statement::Assignment("z", RVal::Num(2))),
            )),
        ]);
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Num(3)]);
        check_vars
            .vars
            .insert("y", vec![RVal::Num(4), RVal::Num(3)]);
        check_vars
            .vars
            .insert("z", vec![RVal::Num(2), RVal::Num(1)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_conditional_equals_err() {
        let mut funcs = Funcs::new();
        let foo_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
        funcs.funcs.insert("foo", FuncVal::new(foo_body.clone()));

        let mut vars = Vars::new();

        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", foo_body)),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("foo"),
                    RVal::Var("x"),
                )),
                Box::new(Statement::Assignment("z", RVal::Num(1))),
                Box::new(Statement::Assignment("z", RVal::Num(2))),
            )),
        ]);
        let res = interp.interp(&funcs, &mut vars, stmt);

        assert_eq!(
            res,
            Err(Error::IncomparableTypes(RVal::Var("foo"), RVal::Num(5)))
        );
    }

    #[test]
    fn test_nested_conditional() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::True()),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::True()),
                Box::new(Statement::Assignment("x", RVal::Num(5))),
                Box::new(Statement::Assignment("x", RVal::Num(6))),
            )),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::True()),
                Box::new(Statement::Assignment("x", RVal::Num(7))),
                Box::new(Statement::Assignment("x", RVal::Num(8))),
            )),
        );
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_conditional_scope() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![Box::new(Statement::Conditional(
            Box::new(BooleanStatement::True()),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        ))]);
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_funcdef() {
        let mut funcs = Funcs::new();
        let body = Box::new(Statement::Assignment("x", RVal::Num(5)));
        funcs.funcs.insert("foo", FuncVal::new(body.clone()));

        let mut vars = Vars::new();

        let interp = Interpreter::new();
        let stmt = Statement::FuncDef("foo", body);
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, Vars::new());
    }

    #[test]
    fn test_direct_invoke() {
        let mut funcs = Funcs::new();
        let body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
        funcs.funcs.insert("foo", FuncVal::new(body.clone()));

        let mut vars = Vars::new();

        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body)),
            Box::new(Statement::InvokeFunc("foo")),
        ]);
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_indirect_invoke() {
        let mut funcs = Funcs::new();
        let body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("z", RVal::Num(5)),
        )]));
        funcs.funcs.insert("foo", FuncVal::new(body.clone()));

        let mut vars = Vars::new();

        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body)),
            Box::new(Statement::Assignment("x", RVal::Var("foo"))),
            Box::new(Statement::InvokeFunc("x")),
        ]);
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Var("foo")]);
        check_vars.vars.insert("z", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_direct_invoke_uncertain() {
        let mut funcs = Funcs::new();
        let foo_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
        let bar_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(6)),
        )]));
        funcs.funcs.insert("foo", FuncVal::new(foo_body.clone()));
        funcs.funcs.insert("bar", FuncVal::new(bar_body.clone()));

        let mut vars = Vars::new();

        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", foo_body)),
            Box::new(Statement::FuncDef("bar", bar_body)),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Statement::InvokeFunc("foo")),
                Box::new(Statement::InvokeFunc("bar")),
            )),
        ]);
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars
            .vars
            .insert("x", vec![RVal::Num(6), RVal::Num(5)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_indirect_invoke_uncertain() {
        let mut funcs = Funcs::new();
        let foo_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("y", RVal::Num(5)),
        )]));
        let bar_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("z", RVal::Num(6)),
        )]));
        funcs.funcs.insert("foo", FuncVal::new(foo_body.clone()));
        funcs.funcs.insert("bar", FuncVal::new(bar_body.clone()));

        let mut vars = Vars::new();

        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", foo_body)),
            Box::new(Statement::FuncDef("bar", bar_body)),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Statement::Assignment("x", RVal::Var("foo"))),
                Box::new(Statement::Assignment("x", RVal::Var("bar"))),
            )),
            Box::new(Statement::InvokeFunc("x")),
        ]);
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars
            .vars
            .insert("x", vec![RVal::Var("bar"), RVal::Var("foo")]);
        check_vars.vars.insert("y", vec![RVal::Num(5)]);
        check_vars.vars.insert("z", vec![RVal::Num(6)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_indirect_invoke_err() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("foo", RVal::Num(5))),
            Box::new(Statement::InvokeFunc("foo")),
        ]);
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt);

        assert_eq!(res, Err(Error::NotAFunction("foo")));
    }

    #[test]
    fn test_switch_err() {
        let interp = Interpreter::new();
        let switch_vec: Vec<(RVal, Box<Statement>)> = vec![];
        let stmt = Statement::Switch(RVal::Var("x"), switch_vec);
        let res = interp.interp(&Funcs::new(), &mut Vars::new(), stmt);

        assert_eq!(res, Err(Error::UndefinedSymbol("x")));
    }

    #[test]
    fn test_switch() {
        let interp = Interpreter::new();
        let switch_vec: Vec<(RVal, Box<Statement>)> = vec![
            (
                RVal::Num(4),
                Box::new(Statement::Assignment("y", RVal::Num(0))),
            ),
            (
                RVal::Num(5),
                Box::new(Statement::Assignment("y", RVal::Num(1))),
            ),
        ];
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Switch(RVal::Var("x"), switch_vec)),
        ]);
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Num(5)]);
        check_vars.vars.insert("y", vec![RVal::Num(1)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }

    #[test]
    fn test_switch_uncertain() {
        let interp = Interpreter::new();
        let switch_vec: Vec<(RVal, Box<Statement>)> = vec![
            (
                RVal::Num(4),
                Box::new(Statement::Assignment("y", RVal::Num(0))),
            ),
            (
                RVal::Num(5),
                Box::new(Statement::Assignment("y", RVal::Num(1))),
            ),
        ];
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Statement::Assignment("x", RVal::Num(5))),
                Box::new(Statement::Assignment("x", RVal::Num(4))),
            )),
            Box::new(Statement::Switch(RVal::Var("x"), switch_vec)),
        ]);
        let funcs = Funcs::new();
        let mut vars = Vars::new();
        let res = interp.interp(&funcs, &mut vars, stmt.clone());

        let mut check_vars = Vars::new();
        check_vars
            .vars
            .insert("x", vec![RVal::Num(4), RVal::Num(5)]);
        check_vars
            .vars
            .insert("y", vec![RVal::Num(1), RVal::Num(0)]);
        assert_eq!(res.unwrap(), stmt);
        assert_eq!(vars, check_vars);
    }
}
