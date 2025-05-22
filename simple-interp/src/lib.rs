pub mod func_collect;
pub mod interpret;
pub mod rewrite;

use crate::func_collect::{FuncCollector, Funcs};
use crate::interpret::{Interpreter, Vars};
use crate::rewrite::Rewriter;
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
    rw: Rewriter,
}

impl SimpleInterp {
    pub fn new() -> Self {
        Self {
            fc: FuncCollector::new(),
            ip: Interpreter::new(),
            rw: Rewriter::new(),
        }
    }

    pub fn interp(
        &self,
        stmt: Statement,
    ) -> Result<(Funcs, Vars, Statement), Error> {
        let mut funcs = Funcs::new();
        match self.fc.collect(&mut funcs, stmt.clone()) {
            Ok(fc_res) => {
                let mut vars = Vars::new();
                match self.ip.interp(&funcs, &mut vars, fc_res) {
                    Ok(mut stmt) => {
                        self.rw.rewrite(&funcs, &vars, &mut stmt).unwrap();
                        Ok((funcs, vars, stmt))
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
    use super::*;

    #[test]
    fn test_print() {
        let stmt = Statement::Print("hello");

        let si = SimpleInterp::new();
        let (_, vars, rw_stmt) = si.interp(stmt.clone()).unwrap();

        assert_eq!(vars, Vars::new());
        assert_eq!(rw_stmt, stmt);
    }

    #[test]
    fn test_funcdef() {
        let body = Box::new(Statement::Assignment("x", RVal::Num(5)));
        let stmt = Statement::FuncDef("foo", body.clone());

        let si = SimpleInterp::new();
        let (_, vars, rw_stmt) = si.interp(stmt.clone()).unwrap();

        assert_eq!(vars, Vars::new());
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
        let (_, vars, rw_stmt) = si.interp(stmt.clone()).unwrap();

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Num(5)]);

        assert_eq!(vars, check_vars);
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
        let (_, vars, rw_stmt) = si.interp(stmt.clone()).unwrap();

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Var("foo")]);
        check_vars.vars.insert("z", vec![RVal::Num(5)]);

        let switch_vec =
            vec![(RVal::Var("foo"), Box::new(Statement::InvokeFunc("foo")))];
        let check_stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body)),
            Box::new(Statement::Assignment("x", RVal::Var("foo"))),
            Box::new(Statement::Switch(RVal::Var("x"), switch_vec)),
        ]);

        assert_eq!(vars, check_vars);
        assert_eq!(rw_stmt, check_stmt);
    }
}
