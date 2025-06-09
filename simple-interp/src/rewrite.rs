use crate::func_collect::Funcs;
use crate::interpret::Vars;
use crate::{BooleanStatement, Error, RVal, Statement};

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
        vars: &Vars,
        stmt: &mut Statement,
    ) -> Result<(), Error> {
        match stmt {
            // FIXME impl when funcs have retvals + can print result
            Statement::Print(_) => Ok(()),
            // FIXME impl when funcs have retvals + can be assigned
            Statement::Assignment(_, _) => Ok(()),
            Statement::Sequence(stmt_vec) => {
                self.rewrite_seq(funcs, vars, stmt_vec)
            }
            Statement::Conditional(condition, true_branch, false_branch) => {
                self.rewrite_conditional(
                    funcs,
                    vars,
                    *condition.clone(),
                    &mut (*true_branch),
                    &mut (*false_branch),
                )
            }
            Statement::Switch(val, vec) => {
                self.rewrite_switch(funcs, vars, val.clone(), vec)
            }
            Statement::FuncDef(name, body) => {
                self.rewrite_funcdef(funcs, vars, name, body)
            }
            Statement::InvokeFunc(name) => {
                match self.rewrite_invoke(funcs, vars, name) {
                    Ok(res) => {
                        *stmt = res;
                        Ok(())
                    }
                    Err(err) => Err(err),
                }
            }
        }
    }

    pub fn rewrite_seq(
        &self,
        funcs: &Funcs,
        vars: &Vars,
        stmt_vec: &mut Vec<Box<Statement>>,
    ) -> Result<(), Error> {
        for stmt in stmt_vec.iter_mut() {
            let res = self.rewrite(funcs, vars, stmt);
            if res.is_err() {
                return res;
            }
        }
        Ok(())
    }

    pub fn rewrite_conditional(
        &self,
        funcs: &Funcs,
        vars: &Vars,
        _condition: BooleanStatement,
        mut true_branch: &mut Statement,
        mut false_branch: &mut Statement,
    ) -> Result<(), Error> {
        // FIXME also rewrite condition when funcs can ret booleans
        let res_true = self.rewrite(funcs, vars, &mut true_branch);
        if res_true.is_err() {
            return res_true;
        }

        let res_false = self.rewrite(funcs, vars, &mut false_branch);
        if res_false.is_err() {
            return res_false;
        }

        Ok(())
    }

    pub fn rewrite_switch(
        &self,
        funcs: &Funcs,
        vars: &Vars,
        _val: RVal,
        vec: &mut Vec<(RVal, Box<Statement>)>,
    ) -> Result<(), Error> {
        for (_, switch_stmt) in vec.iter_mut() {
            let res = self.rewrite(funcs, vars, &mut (*switch_stmt));
            if res.is_err() {
                return res;
            }
        }
        Ok(())
    }

    pub fn rewrite_funcdef(
        &self,
        funcs: &Funcs,
        vars: &Vars,
        _name: &'static str,
        body: &mut Box<Statement>,
    ) -> Result<(), Error> {
        let res = self.rewrite(funcs, vars, &mut (*body));
        if res.is_err() {
            return res;
        }
        Ok(())
    }

    fn rewrite_indirect_invoke(
        &self,
        name: &'static str,
        vec: &Vec<RVal>,
    ) -> Result<Statement, Error> {
        let mut switch_vec = vec![];
        for rval in vec.into_iter() {
            // FIXME remove check (panic)
            match rval.clone() {
                r @ RVal::Var(var) => {
                    switch_vec.push((r, Box::new(Statement::InvokeFunc(var))))
                }
                _ => panic!("IP BUG: num {:?} is not a func name", &rval),
            }
        }
        Ok(Statement::Switch(RVal::Var(name), switch_vec))
    }

    pub fn rewrite_invoke(
        &self,
        funcs: &Funcs,
        vars: &Vars,
        name: &'static str,
    ) -> Result<Statement, Error> {
        match funcs.funcs.get(name) {
            Some(_) => Ok(Statement::InvokeFunc(name)),
            // FIXME remove check (panic)
            None => match vars.vars.get(name) {
                Some(vec) => self.rewrite_indirect_invoke(name, vec),
                None => panic!("IP BUG: missed undef symbol {:?}", &name),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FuncVal;
    use crate::Statement::{
        Assignment, Conditional, FuncDef, InvokeFunc, Print, Sequence, Switch,
    };
    use crate::func_collect::Funcs;

    #[test]
    fn test_print() {
        let mut stmt = Print("hello");
        let check_stmt = stmt.clone();

        let funcs = Funcs::new();
        let vars = Vars::new();
        let rw = Rewriter::new();
        let _ = rw.rewrite(&funcs, &vars, &mut stmt);

        assert_eq!(check_stmt, stmt);
    }

    #[test]
    fn test_assignment() {
        let mut stmt = Assignment("x", RVal::Num(5));
        let check_stmt = stmt.clone();

        let funcs = Funcs::new();
        let mut vars = Vars::new();
        vars.vars.insert("x", vec![RVal::Num(5)]);

        let rw = Rewriter::new();
        let _ = rw.rewrite(&funcs, &vars, &mut stmt);

        assert_eq!(check_stmt, stmt);
    }

    #[test]
    fn test_direct_invoke() {
        let body =
            Box::new(Sequence(vec![Box::new(Assignment("z", RVal::Num(5)))]));
        let mut stmt = Sequence(vec![
            Box::new(FuncDef("foo", body.clone())),
            Box::new(Assignment("x", RVal::Var("foo"))),
            Box::new(InvokeFunc("foo")),
        ]);
        let check_stmt = stmt.clone();

        let mut funcs = Funcs::new();
        funcs.funcs.insert("foo", FuncVal::new(body.clone()));
        let mut vars = Vars::new();
        vars.vars.insert("x", vec![RVal::Var("foo")]);

        let rw = Rewriter::new();
        let _ = rw.rewrite(&funcs, &vars, &mut stmt);

        assert_eq!(check_stmt, stmt);
    }

    #[test]
    fn test_indirect_invoke_single_val() {
        let body =
            Box::new(Sequence(vec![Box::new(Assignment("z", RVal::Num(5)))]));
        let mut stmt = Sequence(vec![
            Box::new(FuncDef("foo", body.clone())),
            Box::new(Assignment("x", RVal::Var("foo"))),
            Box::new(InvokeFunc("x")),
        ]);

        let mut funcs = Funcs::new();
        funcs.funcs.insert("foo", FuncVal::new(body.clone()));
        let mut vars = Vars::new();
        vars.vars.insert("x", vec![RVal::Var("foo")]);

        let rw = Rewriter::new();
        let _ = rw.rewrite(&funcs, &vars, &mut stmt);

        let switch_vec = vec![(RVal::Var("foo"), Box::new(InvokeFunc("foo")))];
        let check_stmt = Sequence(vec![
            Box::new(FuncDef("foo", body)),
            Box::new(Assignment("x", RVal::Var("foo"))),
            Box::new(Switch(RVal::Var("x"), switch_vec)),
        ]);

        assert_eq!(stmt, check_stmt);
    }

    #[test]
    fn test_indirect_invoke_multiple_val() {
        let foo_body =
            Box::new(Sequence(vec![Box::new(Assignment("z", RVal::Num(5)))]));
        let bar_body =
            Box::new(Sequence(vec![Box::new(Assignment("z", RVal::Num(6)))]));
        let mut stmt = Sequence(vec![
            Box::new(FuncDef("foo", foo_body.clone())),
            Box::new(FuncDef("bar", bar_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment("x", RVal::Var("foo"))),
                Box::new(Assignment("x", RVal::Var("bar"))),
            )),
            Box::new(InvokeFunc("x")),
        ]);

        let mut funcs = Funcs::new();
        funcs.funcs.insert("foo", FuncVal::new(foo_body.clone()));
        funcs.funcs.insert("bar", FuncVal::new(bar_body.clone()));
        let mut vars = Vars::new();
        vars.vars
            .insert("x", vec![RVal::Var("bar"), RVal::Var("foo")]);

        let rw = Rewriter::new();
        let _ = rw.rewrite(&funcs, &vars, &mut stmt);

        let switch_vec = vec![
            (RVal::Var("bar"), Box::new(InvokeFunc("bar"))),
            (RVal::Var("foo"), Box::new(InvokeFunc("foo"))),
        ];
        let check_stmt = Sequence(vec![
            Box::new(FuncDef("foo", foo_body.clone())),
            Box::new(FuncDef("bar", bar_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment("x", RVal::Var("foo"))),
                Box::new(Assignment("x", RVal::Var("bar"))),
            )),
            Box::new(Switch(RVal::Var("x"), switch_vec)),
        ]);

        assert_eq!(stmt, check_stmt);
    }
}
