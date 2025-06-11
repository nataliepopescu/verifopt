pub mod func_collect;
pub mod interpret;
pub mod rewrite;
pub mod ssa;

use crate::func_collect::{FuncCollector, Funcs};
use crate::interpret::{Interpreter, Vars};
use crate::rewrite::Rewriter;
use crate::ssa::{SSAChecker, Symbols};
use thiserror::Error;

use std::fmt;
use std::ops::Not;

#[derive(Clone, Debug, PartialEq, Error)]
pub enum Error {
    #[error("Symbol {0} is undefined")]
    UndefinedSymbol(&'static str),
    #[error("{0} cannot be compared to {1}.")]
    IncomparableTypes(RVal, RVal),
    #[error("{0} is not a function.")]
    NotAFunction(&'static str),
    #[error("Symbol {0} already exists.")]
    SymbolAlreadyExists(&'static str),
    #[error("Cannot perform merge on Vec with less than two elements")]
    VecSize(),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Sequence(Vec<Box<Statement>>),
    Assignment(&'static str, RVal),
    Print(&'static str),
    Conditional(Box<BooleanStatement>, Box<Statement>, Box<Statement>),
    Switch(RVal, Vec<(RVal, Box<Statement>)>),
    // no args or retvals for now
    FuncDef(&'static str, Box<Statement>),
    InvokeFunc(&'static str),
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

pub struct SimpleInterp {
    sc: SSAChecker,
    fc: FuncCollector,
    ip: Interpreter,
    rw: Rewriter,
}

impl SimpleInterp {
    pub fn new() -> Self {
        Self {
            sc: SSAChecker::new(),
            fc: FuncCollector::new(),
            ip: Interpreter::new(),
            rw: Rewriter::new(),
        }
    }

    pub fn interp(
        &self,
        mut stmt: Statement,
    ) -> Result<(Vars, Statement), Error> {
        //println!("\nOriginal program statement: \n\n{:#?}", &stmt);

        let mut symbols = Symbols::new();
        let res1 = self.sc.check(&mut symbols, &stmt);
        if res1.is_err() {
            return Err(res1.err().unwrap());
        }

        //println!("\n-----------------------------------");
        //println!("PHASE 1: SSA Check");
        //println!("-----------------------------------");
        //println!("\n1. General symbols set: \n\n{:#?}", &symbols);
        //println!("\n2. Original program statement");

        let mut funcs = Funcs::new();
        let res2 = self.fc.collect(&mut funcs, &stmt);
        if res2.is_err() {
            return Err(res2.err().unwrap());
        }

        //println!("\n-----------------------------------");
        //println!("PHASE 2: Function Collection");
        //println!("-----------------------------------");
        //println!("\n1. Function symbols table: \n\n{:#?}", &funcs);
        //println!("\n2. Original program statement");

        let mut vars = Vars::new();
        let res3 = self.ip.interp(&funcs, &mut vars, &stmt);
        if res3.is_err() {
            return Err(res3.err().unwrap());
        }

        //println!("\n-----------------------------------");
        //println!("PHASE 3: Flow Interpretation");
        //println!("-----------------------------------");
        //println!("\n1. Function symbols table (from PHASE 2)");
        //println!(
        //    "\n2. Flow-sensitive variable symbols table: \n\n{:#?}",
        //    &vars
        //);
        //println!("\n3. Original program statement");

        let res4 = self.rw.rewrite(&funcs, &vars, &mut stmt);
        if res4.is_err() {
            return Err(res4.err().unwrap());
        }

        //println!("\n-----------------------------------");
        //println!("PHASE 4: Switch-Case Rewrite");
        //println!("-----------------------------------");
        //println!("\n1. Function symbols table (from PHASE 2)");
        //println!("\n2. Flow-sensitive variable symbols table (from PHASE
        // 3)"); println!("\n3. (Maybe) modified program statement:
        // \n\n{:#?}", &stmt);

        Ok((vars, stmt))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Statement::{
        Assignment, Conditional, FuncDef, InvokeFunc, Print, Sequence, Switch,
    };

    #[test]
    fn test_print() {
        let stmt = Print("hello");

        let si = SimpleInterp::new();
        let (vars, rw_stmt) = si.interp(stmt.clone()).unwrap();

        assert_eq!(vars, Vars::new());
        assert_eq!(rw_stmt, stmt);
    }

    #[test]
    fn test_funcdef() {
        let body = Box::new(Assignment("x", RVal::Num(5)));
        let stmt = FuncDef("foo", body.clone());

        let si = SimpleInterp::new();
        let (vars, rw_stmt) = si.interp(stmt.clone()).unwrap();

        assert_eq!(vars, Vars::new());
        assert_eq!(rw_stmt, stmt);
    }

    #[test]
    fn test_direct_invoke() {
        let body =
            Box::new(Sequence(vec![Box::new(Assignment("x", RVal::Num(5)))]));
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", body)),
            Box::new(InvokeFunc("foo")),
        ]);

        let si = SimpleInterp::new();
        let (vars, rw_stmt) = si.interp(stmt.clone()).unwrap();

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Num(5)]);

        assert_eq!(vars, check_vars);
        assert_eq!(rw_stmt, stmt);
    }

    #[test]
    fn test_indirect_invoke() {
        let body =
            Box::new(Sequence(vec![Box::new(Assignment("z", RVal::Num(5)))]));
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", body.clone())),
            Box::new(Assignment("x", RVal::Var("foo"))),
            Box::new(InvokeFunc("x")),
        ]);

        let si = SimpleInterp::new();
        let (vars, rw_stmt) = si.interp(stmt).unwrap();

        let mut check_vars = Vars::new();
        check_vars.vars.insert("x", vec![RVal::Var("foo")]);
        check_vars.vars.insert("z", vec![RVal::Num(5)]);

        let switch_vec = vec![(RVal::Var("foo"), Box::new(InvokeFunc("foo")))];
        let check_stmt = Sequence(vec![
            Box::new(FuncDef("foo", body)),
            Box::new(Assignment("x", RVal::Var("foo"))),
            Box::new(Switch(RVal::Var("x"), switch_vec)),
        ]);

        assert_eq!(vars, check_vars);
        assert_eq!(rw_stmt, check_stmt);
    }

    #[test]
    fn test_direct_invoke_uncertain() {
        let foo_body =
            Box::new(Sequence(vec![Box::new(Assignment("x", RVal::Num(5)))]));
        let bar_body =
            Box::new(Sequence(vec![Box::new(Assignment("x", RVal::Num(6)))]));

        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", foo_body)),
            Box::new(FuncDef("bar", bar_body)),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(InvokeFunc("foo")),
                Box::new(InvokeFunc("bar")),
            )),
        ]);

        let si = SimpleInterp::new();
        let (vars, rw_stmt) = si.interp(stmt.clone()).unwrap();

        let mut check_vars = Vars::new();
        check_vars
            .vars
            .insert("x", vec![RVal::Num(6), RVal::Num(5)]);

        assert_eq!(vars, check_vars);
        assert_eq!(rw_stmt, stmt);
    }

    #[test]
    fn overall_test_indirect_invoke_uncertain() {
        let foo_body =
            Box::new(Sequence(vec![Box::new(Assignment("y", RVal::Num(5)))]));
        let bar_body =
            Box::new(Sequence(vec![Box::new(Assignment("z", RVal::Num(6)))]));

        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", foo_body.clone())),
            Box::new(FuncDef("bar", bar_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment("x", RVal::Var("foo"))),
                Box::new(Assignment("x", RVal::Var("bar"))),
            )),
            Box::new(InvokeFunc("x")),
        ]);

        let si = SimpleInterp::new();
        let (vars, rw_stmt) = si.interp(stmt).unwrap();

        let mut check_vars = Vars::new();
        check_vars
            .vars
            .insert("x", vec![RVal::Var("bar"), RVal::Var("foo")]);
        check_vars.vars.insert("y", vec![RVal::Num(5)]);
        check_vars.vars.insert("z", vec![RVal::Num(6)]);

        let switch_vec = vec![
            (RVal::Var("bar"), Box::new(InvokeFunc("bar"))),
            (RVal::Var("foo"), Box::new(InvokeFunc("foo"))),
        ];
        let check_stmt = Sequence(vec![
            Box::new(FuncDef("foo", foo_body)),
            Box::new(FuncDef("bar", bar_body)),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment("x", RVal::Var("foo"))),
                Box::new(Assignment("x", RVal::Var("bar"))),
            )),
            Box::new(Switch(RVal::Var("x"), switch_vec)),
        ]);

        assert_eq!(vars, check_vars);
        assert_eq!(rw_stmt, check_stmt);
    }
}
