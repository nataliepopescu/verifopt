pub mod func_collector;
pub mod interpreter;
pub mod rewriter;

use crate::func_collector::{Env, FuncCollector};
use crate::interpreter::{Interpreter, RVal, Statement, Store};
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
    interp: Interpreter,
}

impl SimpleInterp {
    pub fn new() -> Self {
        Self {
            fc: FuncCollector::new(),
            interp: Interpreter::new(),
        }
    }

    pub fn interp(
        &self,
        stmt: Statement,
    ) -> (Option<Env>, Result<(Store, Statement), Error>) {
        match self.fc.collect(Env::new(), stmt.clone()) {
            Ok(fc_res) => {
                let store = Store::new_with_func_symbols(fc_res.0.clone());
                (Some(fc_res.0), self.interp.interp(store, fc_res.1))
            }
            Err(err) => (None, Err(err)),
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
        let (_, res) = si.interp(stmt.clone());

        assert_eq!(res.unwrap(), (Store::new(), stmt));
    }

    #[test]
    fn test_funcdef() {
        let body = Box::new(Statement::Assignment("x", RVal::Num(5)));
        let stmt = Statement::FuncDef("foo", body.clone());

        let si = SimpleInterp::new();
        let (Some(env), res) = si.interp(stmt.clone()) else {
            panic!("env is none")
        };

        assert_eq!(res.unwrap(), (Store::new_with_func_symbols(env), stmt));
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
        let (Some(env), res) = si.interp(stmt.clone()) else {
            panic!("env is none")
        };

        let mut store = Store::new_with_func_symbols(env);
        store.vars.insert("x", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    #[test]
    fn test_indirect_invoke() {
        let body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("z", RVal::Num(5)),
        )]));
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body)),
            Box::new(Statement::Assignment("x", RVal::Var("foo"))),
            Box::new(Statement::InvokeFunc("x")),
        ]);

        let si = SimpleInterp::new();
        let (Some(env), res) = si.interp(stmt.clone()) else {
            panic!("env is none")
        };

        let mut store = Store::new_with_func_symbols(env);
        store.vars.insert("x", vec![RVal::Var("foo")]);
        store.vars.insert("z", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }
}
