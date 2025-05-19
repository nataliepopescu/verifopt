pub mod func_collector;
pub mod interpreter;
pub mod rewriter;

use crate::func_collector::{Env, FuncCollector};
use crate::interpreter::{Interpreter, RVal, Statement, Store};
use crate::rewriter::Rewriter;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Error)]
pub enum Error {
    #[error("Symbol {0} is undefined")]
    UndefinedSymbol(&'static str),
    #[error("{0} cannot be compared to {1}.")]
    IncomparableTypes(RVal, RVal),
    #[error("{0} is not a function.")]
    NotAFunction(&'static str),
    #[error("Function {0} already exists")]
    FuncAlreadyExists(&'static str),
    #[error("Variable {0} already exists")]
    VarAlreadyExists(&'static str),
    #[error("Cannot perform merge on Vec with less than two elements")]
    VecSize(),
}

pub struct SimpleInterp {
    fc: FuncCollector,
    ip: Interpreter,
}

impl SimpleInterp {
    pub fn new() -> Self {
        Self {
            fc: FuncCollector::new(),
            ip: Interpreter::new(),
        }
    }

    pub fn interp(
        &self,
        stmt: Statement,
    ) -> Result<(Env, Store, Statement), Error> {
        match self.fc.collect(Env::new(), stmt.clone()) {
            Ok(fc_res) => {
                let store = Store::new_with_func_symbols(fc_res.0.clone());
                match self.ip.interp(store, fc_res.1) {
                    Ok((store, stmt)) => {
                        // FIXME better API?
                        let rw = Rewriter::new(store.clone());
                        let rw_stmt = rw.rewrite(stmt.clone()).unwrap();
                        Ok((fc_res.0, store, rw_stmt))
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::interpreter::RVal;
    use super::*;

    #[test]
    fn test_print() {
        let stmt = Statement::Print("hello");

        let si = SimpleInterp::new();
        let (_, store, rw_stmt) = si.interp(stmt.clone()).unwrap();

        assert_eq!(store, Store::new());
        assert_eq!(rw_stmt, stmt);
    }

    #[test]
    fn test_funcdef() {
        let body = Box::new(Statement::Assignment("x", RVal::Num(5)));
        let stmt = Statement::FuncDef("foo", body.clone());

        let si = SimpleInterp::new();
        let (env, store, rw_stmt) = si.interp(stmt.clone()).unwrap();

        assert_eq!(store, Store::new_with_func_symbols(env));
        assert_eq!(rw_stmt, stmt);
    }

    #[test]
    fn test_direct_invoke() {
        let body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body)),
            Box::new(Statement::InvokeFunc("foo")),
        ]);

        let si = SimpleInterp::new();
        let (env, store, rw_stmt) = si.interp(stmt.clone()).unwrap();

        let mut check_store = Store::new_with_func_symbols(env);
        check_store.vars.insert("x", vec![RVal::Num(5)]);

        assert_eq!(store, check_store);
        assert_eq!(rw_stmt, stmt);
    }

    #[test]
    fn test_indirect_invoke() {
        let body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("z", RVal::Num(5)),
        )]));
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body.clone())),
            Box::new(Statement::Assignment("x", RVal::Var("foo"))),
            Box::new(Statement::InvokeFunc("x")),
        ]);

        let si = SimpleInterp::new();
        let (env, store, rw_stmt) = si.interp(stmt.clone()).unwrap();

        let mut check_store = Store::new_with_func_symbols(env);
        check_store.vars.insert("x", vec![RVal::Var("foo")]);
        check_store.vars.insert("z", vec![RVal::Num(5)]);

        let switch_vec =
            vec![(RVal::Var("foo"), Box::new(Statement::InvokeFunc("foo")))];
        let check_stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body)),
            Box::new(Statement::Assignment("x", RVal::Var("foo"))),
            Box::new(Statement::Switch(RVal::Var("x"), switch_vec)),
        ]);

        assert_eq!(store, check_store);
        assert_eq!(rw_stmt, check_stmt);
    }
}
