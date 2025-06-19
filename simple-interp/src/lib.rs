pub mod func_collect;
pub mod interpret;
pub mod rewrite;
pub mod sig_collect;
pub mod ssa;

use crate::func_collect::{FuncCollector, Funcs};
use crate::interpret::{Interpreter, Vars};
use crate::rewrite::Rewriter;
use crate::sig_collect::{SigCollector, Sigs};
use crate::ssa::{SSAChecker, Symbols};
use thiserror::Error;

use std::fmt;
use std::ops::Not;

#[derive(Clone, Debug, PartialEq, Error)]
pub enum Error {
    #[error("Scope {0} is undefined,")]
    UndefinedScope(&'static str),
    #[error("{0} is not a scope.")]
    NotAScope(&'static str),
    #[error("Symbol {0} is undefined.")]
    UndefinedSymbol(&'static str),
    #[error("VarTypes cannot be compared.")]
    IncomparableVarTypes(),
    #[error("{0} cannot be compared to {1}.")]
    IncomparableTypes(RVal, RVal),
    #[error("Inconsistent return types.")]
    InconsistentReturnTypes(),
    #[error("{0} is not a function.")]
    NotAFunction(&'static str),
    #[error("Symbol {0} already exists.")]
    SymbolAlreadyExists(&'static str),
    #[error("Cannot perform merge on Vec with no elements.")]
    VecSize(),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int(),
    Func(Vec<Type>, Box<Type>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Sequence(Vec<Box<Statement>>),
    Assignment(&'static str, RVal),
    Print(&'static str),
    Conditional(Box<BooleanStatement>, Box<Statement>, Box<Statement>),
    Switch(RVal, Vec<(RVal, Box<Statement>)>),
    Return(RVal),
    FuncDef(
        &'static str,
        Vec<Type>,
        Vec<&'static str>,
        Option<Box<Type>>,
        Box<Statement>,
    ),
    InvokeFunc(&'static str, Vec<&'static str>),
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
    paramtypes: Vec<Type>,
    params: Vec<&'static str>,
    rettype: Option<Box<Type>>,
    body: Box<Statement>,
}

impl FuncVal {
    pub fn new(
        paramtypes: Vec<Type>,
        params: Vec<&'static str>,
        rettype: Option<Box<Type>>,
        body: Box<Statement>,
    ) -> Self {
        Self {
            paramtypes,
            params,
            rettype,
            body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SigVal {
    paramtypes: Vec<Type>,
    rettype: Option<Box<Type>>,
}

impl SigVal {
    pub fn new(paramtypes: Vec<Type>, rettype: Option<Box<Type>>) -> Self {
        Self {
            paramtypes,
            rettype,
        }
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
    ssa_checker: SSAChecker,
    func_collector: FuncCollector,
    sig_collector: SigCollector,
    interpreter: Interpreter,
    rewriter: Rewriter,
}

impl SimpleInterp {
    pub fn new() -> Self {
        Self {
            ssa_checker: SSAChecker::new(),
            func_collector: FuncCollector::new(),
            sig_collector: SigCollector::new(),
            interpreter: Interpreter::new(),
            rewriter: Rewriter::new(),
        }
    }

    pub fn interp(
        &self,
        mut stmt: Statement,
    ) -> Result<(Vars, Statement), Error> {
        //println!("\nOriginal program statement: \n\n{:#?}", &stmt);

        let mut symbols = Symbols::new();
        let res1 = self.ssa_checker.check(&mut symbols, &stmt);
        if res1.is_err() {
            return Err(res1.err().unwrap());
        }

        //println!("\n-----------------------------------");
        //println!("PHASE 1: SSA Check");
        //println!("-----------------------------------");
        //println!("\n1. General symbols set: \n\n{:#?}", &symbols);
        //println!("\n2. Original program statement");

        let mut funcs = Funcs::new();
        let res2 = self.func_collector.collect(&mut funcs, &stmt);
        if res2.is_err() {
            return Err(res2.err().unwrap());
        }

        //println!("\n-----------------------------------");
        //println!("PHASE 2: Function Collection");
        //println!("-----------------------------------");
        //println!("\n1. Function symbols table: \n\n{:#?}", &funcs);
        //println!("\n2. Original program statement");

        let mut sigs = Sigs::new();
        let res3 = self.sig_collector.collect(&funcs, &mut sigs);
        if res3.is_err() {
            return Err(res3.err().unwrap());
        }

        //println!("\n-----------------------------------");
        //println!("PHASE 3: Signature Collection");
        //println!("-----------------------------------");
        //println!("\n1. Function symbols table (from PHASE 2)");
        //println!("\n2. Function signatures table: \n\n{:#?}", &sigs);

        let mut vars = Vars::new();
        let res4 = self.interpreter.interp(&funcs, &mut vars, None, &stmt);
        if res4.is_err() {
            return Err(res4.err().unwrap());
        }

        //println!("\n-----------------------------------");
        //println!("PHASE 4: Flow Interpretation");
        //println!("-----------------------------------");
        //println!("\n1. Function symbols table (from PHASE 2)");
        //println!(
        //    "\n2. Flow-sensitive variable symbols table: \n\n{:#?}",
        //    &vars
        //);
        //println!("\n3. Original program statement");

        let res5 = self.rewriter.rewrite(&funcs, &vars, None, &mut stmt);
        if res5.is_err() {
            return Err(res5.err().unwrap());
        }

        //println!("\n-----------------------------------");
        //println!("PHASE 5: Switch-Case Rewrite");
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
    use crate::interpret::VarType;

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
        let stmt = FuncDef("foo", vec![], vec![], None, body.clone());

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
            Box::new(FuncDef("foo", vec![], vec![], None, body)),
            Box::new(InvokeFunc("foo", vec![])),
        ]);

        let si = SimpleInterp::new();
        let (vars, rw_stmt) = si.interp(stmt.clone()).unwrap();

        let mut check_vars = Vars::new();
        let mut foo_vars = Vars::new();
        foo_vars
            .vars
            .insert("x", Box::new(VarType::Values(vec![RVal::Num(5)])));
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));

        assert_eq!(vars, check_vars);
        assert_eq!(rw_stmt, stmt);
    }

    #[test]
    fn test_indirect_invoke() {
        let body =
            Box::new(Sequence(vec![Box::new(Assignment("z", RVal::Num(5)))]));
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, body.clone())),
            Box::new(Assignment("x", RVal::Var("foo"))),
            Box::new(InvokeFunc("x", vec![])),
        ]);

        let si = SimpleInterp::new();
        let (vars, rw_stmt) = si.interp(stmt).unwrap();

        let mut check_vars = Vars::new();
        let mut foo_vars = Vars::new();
        foo_vars
            .vars
            .insert("z", Box::new(VarType::Values(vec![RVal::Num(5)])));
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));
        check_vars
            .vars
            .insert("x", Box::new(VarType::Values(vec![RVal::Var("foo")])));

        let switch_vec =
            vec![(RVal::Var("foo"), Box::new(InvokeFunc("foo", vec![])))];
        let check_stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, body)),
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
            Box::new(FuncDef("foo", vec![], vec![], None, foo_body)),
            Box::new(FuncDef("bar", vec![], vec![], None, bar_body)),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(InvokeFunc("foo", vec![])),
                Box::new(InvokeFunc("bar", vec![])),
            )),
        ]);

        let si = SimpleInterp::new();
        let (vars, rw_stmt) = si.interp(stmt.clone()).unwrap();

        let mut check_vars = Vars::new();
        let mut foo_vars = Vars::new();
        let mut bar_vars = Vars::new();
        foo_vars
            .vars
            .insert("x", Box::new(VarType::Values(vec![RVal::Num(5)])));
        bar_vars
            .vars
            .insert("x", Box::new(VarType::Values(vec![RVal::Num(6)])));
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));
        check_vars
            .vars
            .insert("bar", Box::new(VarType::Scope(None, bar_vars)));

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
            Box::new(FuncDef("foo", vec![], vec![], None, foo_body.clone())),
            Box::new(FuncDef("bar", vec![], vec![], None, bar_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment("x", RVal::Var("foo"))),
                Box::new(Assignment("x", RVal::Var("bar"))),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]);

        let si = SimpleInterp::new();
        let (vars, rw_stmt) = si.interp(stmt).unwrap();

        let mut check_vars = Vars::new();
        let mut foo_vars = Vars::new();
        let mut bar_vars = Vars::new();
        foo_vars
            .vars
            .insert("y", Box::new(VarType::Values(vec![RVal::Num(5)])));
        bar_vars
            .vars
            .insert("z", Box::new(VarType::Values(vec![RVal::Num(6)])));
        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(vec![RVal::Var("foo"), RVal::Var("bar")])),
        );
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));
        check_vars
            .vars
            .insert("bar", Box::new(VarType::Scope(None, bar_vars)));

        let switch_vec = vec![
            (RVal::Var("foo"), Box::new(InvokeFunc("foo", vec![]))),
            (RVal::Var("bar"), Box::new(InvokeFunc("bar", vec![]))),
        ];
        let check_stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, foo_body)),
            Box::new(FuncDef("bar", vec![], vec![], None, bar_body)),
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

    #[test]
    fn test_nested_direct_calls_no_args() {
        let bar_body = Box::new(Assignment("x", RVal::Num(5)));
        let foo_body = Box::new(Sequence(vec![
            Box::new(FuncDef("bar", vec![], vec![], None, bar_body.clone())),
            Box::new(InvokeFunc("bar", vec![])),
        ]));
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, foo_body.clone())),
            Box::new(InvokeFunc("foo", vec![])),
        ]);
        let check_stmt = stmt.clone();

        let si = SimpleInterp::new();
        let (vars, rw_stmt) = si.interp(stmt).unwrap();

        let mut check_vars = Vars::new();
        let foo_vars = Vars::new();
        let mut bar_vars = Vars::new();
        bar_vars
            .vars
            .insert("x", Box::new(VarType::Values(vec![RVal::Num(5)])));
        check_vars
            .vars
            .insert("bar", Box::new(VarType::Scope(Some("foo"), bar_vars)));
        check_vars
            .vars
            .insert("foo", Box::new(VarType::Scope(None, foo_vars)));

        assert_eq!(vars, check_vars);
        assert_eq!(rw_stmt, check_stmt);
    }

    #[test]
    fn test_nested_indirect_calls_no_args() {
        let baz_body = Box::new(Assignment("x", RVal::Num(1)));
        let qux_body = Box::new(Assignment("x", RVal::Num(2)));
        let baz2_body = Box::new(Assignment("x", RVal::Num(3)));
        let qux2_body = Box::new(Assignment("x", RVal::Num(4)));
        let foo_body = Box::new(Sequence(vec![
            Box::new(FuncDef("baz", vec![], vec![], None, baz_body.clone())),
            Box::new(FuncDef("qux", vec![], vec![], None, qux_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment("x", RVal::Var("baz"))),
                Box::new(Assignment("x", RVal::Var("qux"))),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]));
        let bar_body = Box::new(Sequence(vec![
            Box::new(FuncDef("baz2", vec![], vec![], None, baz2_body.clone())),
            Box::new(FuncDef("qux2", vec![], vec![], None, qux2_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment("x", RVal::Var("baz2"))),
                Box::new(Assignment("x", RVal::Var("qux2"))),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]));

        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, foo_body.clone())),
            Box::new(FuncDef("bar", vec![], vec![], None, bar_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment("x", RVal::Var("foo"))),
                Box::new(Assignment("x", RVal::Var("bar"))),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]);

        let foo_switch_vec = vec![
            (RVal::Var("baz"), Box::new(InvokeFunc("baz", vec![]))),
            (RVal::Var("qux"), Box::new(InvokeFunc("qux", vec![]))),
        ];
        let check_foo_body = Box::new(Sequence(vec![
            Box::new(FuncDef("baz", vec![], vec![], None, baz_body.clone())),
            Box::new(FuncDef("qux", vec![], vec![], None, qux_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment("x", RVal::Var("baz"))),
                Box::new(Assignment("x", RVal::Var("qux"))),
            )),
            Box::new(Switch(RVal::Var("x"), foo_switch_vec)),
        ]));
        let bar_switch_vec = vec![
            (RVal::Var("baz2"), Box::new(InvokeFunc("baz2", vec![]))),
            (RVal::Var("qux2"), Box::new(InvokeFunc("qux2", vec![]))),
        ];
        let check_bar_body = Box::new(Sequence(vec![
            Box::new(FuncDef("baz2", vec![], vec![], None, baz2_body.clone())),
            Box::new(FuncDef("qux2", vec![], vec![], None, qux2_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment("x", RVal::Var("baz2"))),
                Box::new(Assignment("x", RVal::Var("qux2"))),
            )),
            Box::new(Switch(RVal::Var("x"), bar_switch_vec)),
        ]));
        let switch_vec = vec![
            (RVal::Var("foo"), Box::new(InvokeFunc("foo", vec![]))),
            (RVal::Var("bar"), Box::new(InvokeFunc("bar", vec![]))),
        ];
        let check_stmt = Sequence(vec![
            Box::new(FuncDef(
                "foo",
                vec![],
                vec![],
                None,
                check_foo_body.clone(),
            )),
            Box::new(FuncDef(
                "bar",
                vec![],
                vec![],
                None,
                check_bar_body.clone(),
            )),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment("x", RVal::Var("foo"))),
                Box::new(Assignment("x", RVal::Var("bar"))),
            )),
            Box::new(Switch(RVal::Var("x"), switch_vec)),
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

        let si = SimpleInterp::new();
        let (vars, rw_stmt) = si.interp(stmt).unwrap();

        let mut check_vars = Vars::new();
        let mut foo_vars = Vars::new();
        let mut bar_vars = Vars::new();
        let mut baz_vars = Vars::new();
        let mut qux_vars = Vars::new();
        let mut baz2_vars = Vars::new();
        let mut qux2_vars = Vars::new();

        baz_vars
            .vars
            .insert("x", Box::new(VarType::Values(vec![RVal::Num(1)])));
        qux_vars
            .vars
            .insert("x", Box::new(VarType::Values(vec![RVal::Num(2)])));
        baz2_vars
            .vars
            .insert("x", Box::new(VarType::Values(vec![RVal::Num(3)])));
        qux2_vars
            .vars
            .insert("x", Box::new(VarType::Values(vec![RVal::Num(4)])));

        foo_vars.vars.insert(
            "x",
            Box::new(VarType::Values(vec![RVal::Var("baz"), RVal::Var("qux")])),
        );
        bar_vars.vars.insert(
            "x",
            Box::new(VarType::Values(vec![
                RVal::Var("baz2"),
                RVal::Var("qux2"),
            ])),
        );

        check_vars.vars.insert(
            "x",
            Box::new(VarType::Values(vec![RVal::Var("foo"), RVal::Var("bar")])),
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

        assert_eq!(vars, check_vars);
        assert_eq!(rw_stmt, check_stmt);
    }
}
