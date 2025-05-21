use crate::{Error, FuncVal, Statement};
use std::collections::HashMap;

/// Define collector state

#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    // FIXME pub ok?
    pub funcs: HashMap<&'static str, FuncVal>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::<&'static str, FuncVal>::new(),
        }
    }
}

pub struct FuncCollector {}

/// Implement collector

impl FuncCollector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn collect(
        &self,
        env: Env,
        stmt: Statement,
    ) -> Result<(Env, Statement), Error> {
        match stmt {
            Statement::Sequence(stmt_vec) => self.collect_seq(env, stmt_vec),
            Statement::FuncDef(name, body) => {
                self.collect_funcdef(env, name, body)
            }
            _ => Ok((env, stmt)),
        }
    }

    pub fn collect_seq(
        &self,
        env: Env,
        stmt_vec: Vec<Box<Statement>>,
    ) -> Result<(Env, Statement), Error> {
        let mut cur_env = env;
        for stmt in stmt_vec.iter() {
            let res = self.collect(cur_env, *stmt.clone());
            if res.is_err() {
                return res;
            }
            cur_env = res.unwrap().0;
        }
        Ok((cur_env, Statement::Sequence(stmt_vec)))
    }

    pub fn collect_funcdef(
        &self,
        env: Env,
        name: &'static str,
        body: Box<Statement>,
    ) -> Result<(Env, Statement), Error> {
        match env.funcs.get(name) {
            Some(_) => {
                return Err(Error::FuncAlreadyExists(name));
            }
            None => {
                let mut new_env = env.clone();
                new_env.funcs.insert(name, FuncVal::new(body.clone()));
                Ok((new_env, Statement::FuncDef(name, Box::new(*body))))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FuncVal, RVal, Statement};

    #[test]
    fn test_print() {
        let fc = FuncCollector::new();
        let stmt = Statement::Print("hello");
        let res = fc.collect(Env::new(), stmt.clone());

        let env = Env::new();
        assert_eq!(res.unwrap(), (env, stmt));
    }

    #[test]
    fn test_assign_num() {
        let fc = FuncCollector::new();
        let stmt = Statement::Assignment("x", RVal::Num(5));
        let res = fc.collect(Env::new(), stmt.clone());

        let env = Env::new();
        assert_eq!(res.unwrap(), (env, stmt));
    }

    #[test]
    fn test_seq() {
        let fc = FuncCollector::new();
        let stmt_vec = vec![Box::new(Statement::Assignment("x", RVal::Num(5)))];
        let stmt = Statement::Sequence(stmt_vec);
        let res = fc.collect(Env::new(), stmt.clone());

        let env = Env::new();
        assert_eq!(res.unwrap(), (env, stmt));
    }

    #[test]
    fn test_nested_seq() {
        let fc = FuncCollector::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Sequence(vec![Box::new(
                Statement::Assignment("x", RVal::Num(5)),
            )])),
            Box::new(Statement::Sequence(vec![Box::new(
                Statement::Assignment("y", RVal::Num(6)),
            )])),
        ]);
        let res = fc.collect(Env::new(), stmt.clone());

        let env = Env::new();
        assert_eq!(res.unwrap(), (env, stmt));
    }

    #[test]
    fn test_var_undef() {
        let fc = FuncCollector::new();
        let stmt = Statement::Sequence(vec![Box::new(Statement::Assignment(
            "x",
            RVal::Var("y"),
        ))]);
        let res = fc.collect(Env::new(), stmt.clone());

        let env = Env::new();
        assert_eq!(res.unwrap(), (env, stmt));
    }

    #[test]
    fn test_assign_var() {
        let fc = FuncCollector::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("y", RVal::Var("x"))),
        ]);
        let res = fc.collect(Env::new(), stmt.clone());

        let env = Env::new();
        assert_eq!(res.unwrap(), (env, stmt));
    }

    #[test]
    fn test_funcdef() {
        let fc = FuncCollector::new();
        let body = Box::new(Statement::Assignment("x", RVal::Num(5)));
        let stmt = Statement::FuncDef("foo", body.clone());
        let res = fc.collect(Env::new(), stmt.clone());

        let mut env = Env::new();
        env.funcs.insert("foo", FuncVal::new(body));
        assert_eq!(res.unwrap(), (env, stmt));
    }

    #[test]
    fn test_funcdef_err() {
        let fc = FuncCollector::new();
        let body = Box::new(Statement::Assignment("x", RVal::Num(5)));
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body.clone())),
            Box::new(Statement::FuncDef("foo", body.clone())),
        ]);
        let res = fc.collect(Env::new(), stmt.clone());

        assert_eq!(res, Err(Error::FuncAlreadyExists("foo")));
    }

    #[test]
    fn test_assign_funcptr() {
        let fc = FuncCollector::new();
        let body = Box::new(Statement::Assignment("y", RVal::Num(5)));
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body.clone())),
            Box::new(Statement::Assignment("x", RVal::Var("foo"))),
        ]);
        let res = fc.collect(Env::new(), stmt.clone());

        let mut env = Env::new();
        env.funcs.insert("foo", FuncVal::new(body.clone()));
        assert_eq!(res.unwrap(), (env, stmt));
    }
}
