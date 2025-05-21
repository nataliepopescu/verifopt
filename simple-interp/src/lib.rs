pub mod func_collector;
pub mod interpreter;
pub mod rewriter;

use crate::func_collector::{Env, FuncCollector};
use crate::interpreter::{Interpreter, Store};
use crate::rewriter::Rewriter;
use thiserror::Error;

use std::fmt;
use std::ops::Not;

/// Define Errors

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

/// Define CFG

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Sequence(Vec<Box<Statement>>),
    Assignment(&'static str, RVal),
    Print(&'static str),
    Conditional(Box<BooleanStatement>, Box<Statement>, Box<Statement>),
    // TODO replace w match
    Switch(RVal, Vec<(RVal, Box<Statement>)>),
    // no args or retvals for now
    FuncDef(&'static str, Box<Statement>),
    InvokeFunc(&'static str),
    // TODO traits
}

#[derive(Debug, Clone, PartialEq)]
pub enum RVal {
    Num(i32),
    Var(&'static str),
}

impl fmt::Display for RVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RVal::Num(num) => write!(f, "{}", num),
            RVal::Var(var) => write!(f, "{}", var),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncVal {
    body: Box<Statement>,
}

impl FuncVal {
    pub fn new(body: Box<Statement>) -> Self {
        Self { body }
    }
}

// intentionally skipping Or, And, Xor, and GreaterThan for simplicity
#[derive(Debug, Clone, PartialEq)]
pub enum BooleanStatement {
    True(),
    False(),
    TrueOrFalse(),
    Not(Box<BooleanStatement>),
    Equals(RVal, RVal),
}

impl Not for BooleanStatement {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            BooleanStatement::True() => BooleanStatement::False(),
            BooleanStatement::False() => BooleanStatement::True(),
            BooleanStatement::TrueOrFalse() => BooleanStatement::TrueOrFalse(),
            BooleanStatement::Not(_) | BooleanStatement::Equals(_, _) => {
                panic!("not implemented yet")
            }
        }
    }
}

impl Not for &BooleanStatement {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            BooleanStatement::True() => &BooleanStatement::False(),
            BooleanStatement::False() => &BooleanStatement::True(),
            BooleanStatement::TrueOrFalse() => &BooleanStatement::TrueOrFalse(),
            BooleanStatement::Not(_) | BooleanStatement::Equals(_, _) => {
                panic!("not implemented yet (&)")
            }
        }
    }
}

/// Define driver

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
