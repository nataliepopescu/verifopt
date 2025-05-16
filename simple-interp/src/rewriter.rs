use crate::Error;
use crate::interpreter::{BooleanStatement, RVal, Statement, Store};

pub struct Rewriter {
    // note immutability
    store: Store,
}

/// Implement rewriter

// FIXME never returns Err, refactor out Result return type

impl Rewriter {
    pub fn new(store: Store) -> Self {
        Self { store }
    }

    pub fn rewrite(&self, stmt: Statement) -> Result<Statement, Error> {
        match stmt {
            // FIXME when funcs have retvals + can print result
            Statement::Print(_) => Ok(stmt),
            // FIXME when funcs have retvals + can be assigned
            Statement::Assignment(_, _) => Ok(stmt),
            Statement::Sequence(stmt_vec) => self.rewrite_seq(stmt_vec),
            Statement::Conditional(condition, true_branch, false_branch) => {
                self.rewrite_conditional(
                    *condition,
                    *true_branch,
                    *false_branch,
                )
            }
            Statement::Switch(val, vec) => self.rewrite_switch(val, vec),
            Statement::FuncDef(name, body) => self.rewrite_funcdef(name, body),
            Statement::InvokeFunc(name) => self.rewrite_invoke(name),
        }
    }

    pub fn rewrite_seq(
        &self,
        stmt_vec: Vec<Box<Statement>>,
    ) -> Result<Statement, Error> {
        let mut new_stmt_vec = vec![];
        for stmt in stmt_vec.iter() {
            let res = self.rewrite(*stmt.clone());
            if res.is_err() {
                return res;
            }
            new_stmt_vec.push(Box::new(res.unwrap()));
        }
        Ok(Statement::Sequence(new_stmt_vec))
    }

    pub fn rewrite_conditional(
        &self,
        condition: BooleanStatement,
        true_branch: Statement,
        false_branch: Statement,
    ) -> Result<Statement, Error> {
        // FIXME also rewrite condition when funcs can ret booleans
        let res_true = self.rewrite(true_branch);
        if res_true.is_err() {
            return res_true;
        }

        let res_false = self.rewrite(false_branch);
        if res_false.is_err() {
            return res_false;
        }

        Ok(Statement::Conditional(
            Box::new(condition),
            Box::new(res_true.unwrap()),
            Box::new(res_false.unwrap()),
        ))
    }

    pub fn rewrite_switch(
        &self,
        val: RVal,
        vec: Vec<(RVal, Box<Statement>)>,
    ) -> Result<Statement, Error> {
        let mut new_vec = vec![];
        for (switch_val, switch_stmt) in vec.into_iter() {
            let res = self.rewrite(*switch_stmt);
            if res.is_err() {
                return res;
            }
            new_vec.push((switch_val, Box::new(res.unwrap())));
        }
        Ok(Statement::Switch(val, new_vec))
    }

    pub fn rewrite_funcdef(
        &self,
        name: &'static str,
        body: Box<Statement>,
    ) -> Result<Statement, Error> {
        let res = self.rewrite(*body);
        if res.is_err() {
            return res;
        }
        Ok(Statement::FuncDef(name, Box::new(res.unwrap())))
    }

    fn rewrite_indirect_invoke(
        &self,
        name: &'static str,
        vec: &Vec<RVal>,
    ) -> Result<Statement, Error> {
        let mut switch_vec = vec![];
        for rval in vec.into_iter() {
            match rval.clone() {
                RVal::Var(var) => switch_vec
                    .push((rval.clone(), Box::new(Statement::InvokeFunc(var)))),
                _ => panic!("IP BUG: num is not a func name"),
            }
        }
        Ok(Statement::Switch(RVal::Var(name), switch_vec))
    }

    pub fn rewrite_invoke(
        &self,
        name: &'static str,
    ) -> Result<Statement, Error> {
        match self.store.clone().funcs.get(name) {
            Some(_) => Ok(Statement::InvokeFunc(name)),
            None => match self.store.clone().vars.get(name) {
                Some(vec) => self.rewrite_indirect_invoke(name, vec),
                None => panic!("IP BUG: missed undef symbol"),
            },
        }
    }
}

mod tests {
    use super::*;
    use crate::func_collector::Env;
    use crate::interpreter::FuncVal;

    #[test]
    fn test_print() {
        let stmt = Statement::Print("hello");

        let rw = Rewriter::new(Store::new());
        let res = rw.rewrite(stmt.clone());

        assert_eq!(res.unwrap(), stmt);
    }

    #[test]
    fn test_assignment() {
        let stmt = Statement::Assignment("x", RVal::Num(5));

        let mut store = Store::new();
        store.vars.insert("x", vec![RVal::Num(5)]);

        let rw = Rewriter::new(store);
        let res = rw.rewrite(stmt.clone());

        assert_eq!(res.unwrap(), stmt);
    }

    #[test]
    fn test_direct_invoke() {
        let body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("z", RVal::Num(5)),
        )]));
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body.clone())),
            Box::new(Statement::Assignment("x", RVal::Var("foo"))),
            Box::new(Statement::InvokeFunc("foo")),
        ]);

        let mut env = Env::new();
        env.funcs.insert("foo", FuncVal::new(body.clone()));
        let mut store = Store::new_with_func_symbols(env.clone());
        store.vars.insert("x", vec![RVal::Var("foo")]);

        let rw = Rewriter::new(store);
        let res = rw.rewrite(stmt.clone());

        assert_eq!(res.unwrap(), stmt.clone());
    }

    #[test]
    fn test_indirect_invoke_single_val() {
        let body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("z", RVal::Num(5)),
        )]));
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body.clone())),
            Box::new(Statement::Assignment("x", RVal::Var("foo"))),
            Box::new(Statement::InvokeFunc("x")),
        ]);

        let mut env = Env::new();
        env.funcs.insert("foo", FuncVal::new(body.clone()));
        let mut store = Store::new_with_func_symbols(env.clone());
        store.vars.insert("x", vec![RVal::Var("foo")]);

        let rw = Rewriter::new(store);
        let res = rw.rewrite(stmt);

        let switch_vec =
            vec![(RVal::Var("foo"), Box::new(Statement::InvokeFunc("foo")))];
        let check_stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body)),
            Box::new(Statement::Assignment("x", RVal::Var("foo"))),
            Box::new(Statement::Switch(RVal::Var("x"), switch_vec)),
        ]);

        assert_eq!(res.unwrap(), check_stmt);
    }

    #[test]
    fn test_indirect_invoke_multiple_val() {
        let foo_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("z", RVal::Num(5)),
        )]));
        let bar_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("z", RVal::Num(6)),
        )]));
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", foo_body.clone())),
            Box::new(Statement::FuncDef("bar", bar_body.clone())),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Statement::Assignment("x", RVal::Var("foo"))),
                Box::new(Statement::Assignment("x", RVal::Var("bar"))),
            )),
            Box::new(Statement::InvokeFunc("x")),
        ]);

        let mut env = Env::new();
        env.funcs.insert("foo", FuncVal::new(foo_body.clone()));
        env.funcs.insert("bar", FuncVal::new(bar_body.clone()));
        let mut store = Store::new_with_func_symbols(env.clone());
        store
            .vars
            .insert("x", vec![RVal::Var("bar"), RVal::Var("foo")]);

        let rw = Rewriter::new(store);
        let res = rw.rewrite(stmt);

        let switch_vec = vec![
            (RVal::Var("bar"), Box::new(Statement::InvokeFunc("bar"))),
            (RVal::Var("foo"), Box::new(Statement::InvokeFunc("foo"))),
        ];
        let check_stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", foo_body.clone())),
            Box::new(Statement::FuncDef("bar", bar_body.clone())),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Statement::Assignment("x", RVal::Var("foo"))),
                Box::new(Statement::Assignment("x", RVal::Var("bar"))),
            )),
            Box::new(Statement::Switch(RVal::Var("x"), switch_vec)),
        ]);

        assert_eq!(res.unwrap(), check_stmt);
    }
}
