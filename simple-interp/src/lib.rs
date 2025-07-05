pub mod func_collect;
pub mod interpret;
pub mod rewrite;
pub mod sig_collect;
pub mod ssa;

use crate::func_collect::{FuncCollector, Funcs};
use crate::interpret::{ConstraintMap, Interpreter};
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
    #[error("Invalid RVal for Assignment.")]
    InvalidAssignmentRVal(),
    #[error("Cannot assign var to a return value of None.")]
    CannotAssignNoneRetval(),
    #[error("{0} is not a function.")]
    NotAFunction(&'static str),
    #[error("Symbol {0} already exists.")]
    SymbolAlreadyExists(&'static str),
    #[error("Attempting to merge constraints of different types.")]
    TypesDiffer(),
    #[error("Scope backpointers differ.")]
    BackpointersDiffer(),
    #[error("Switching on a function pointer, not a value.")]
    NoSwitchOnFuncPtr(),
    #[error("Cannot perform merge on Vec with no elements.")]
    VecSize(),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
    Sequence(Vec<Box<Statement>>),
    Assignment(&'static str, Box<AssignmentRVal>),
    Print(&'static str),
    Conditional(Box<BooleanStatement>, Box<Statement>, Box<Statement>),
    Switch(RVal, Vec<(RVal, Box<Statement>)>),
    Return(RVal),
    FuncDef(
        &'static str,
        Vec<(&'static str, Type)>,
        Option<Box<Type>>,
        Box<Statement>,
    ),
    InvokeFunc(&'static str, Vec<&'static str>),
    Struct(&'static str, Vec<Type>, Vec<&'static str>),
    // traits without associated types for now
    // (trait name, func names, funcs defs)
    TraitDecl(&'static str, Vec<&'static str>, Vec<FuncDecl>),
    // (trait name, struct name, func names, funcs impls)
    TraitImpl(&'static str, &'static str, Vec<&'static str>, Vec<FuncVal>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssignmentRVal {
    RVal(RVal),
    Statement(Box<Statement>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RVal {
    Num(i32),
    // TODO add strings
    Struct(&'static str, Vec<RVal>),
    Var(&'static str),
}

impl fmt::Display for RVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RVal::Num(num) => write!(f, "{}", num),
            RVal::Var(var) => write!(f, "{}", var),
            RVal::Struct(name, field_values) => {
                let mut fv_string: String = "".to_owned();
                for field_value in field_values.iter() {
                    fv_string.push_str(&field_value.to_string());
                }
                write!(f, "{} : {}", name, fv_string)
            }
        }
    }
}

pub type TraitStructOpt = Option<(&'static str, &'static str)>;
pub type FuncName = &'static str;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncDecl {
    params: Vec<(&'static str, Type)>,
    rettype: Option<Box<Type>>,
}

impl FuncDecl {
    pub fn new(
        params: Vec<(&'static str, Type)>,
        rettype: Option<Box<Type>>,
    ) -> Self {
        Self { params, rettype }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncVal {
    params: Vec<(&'static str, Type)>,
    rettype: Option<Box<Type>>,
    body: Box<Statement>,
}

impl FuncVal {
    pub fn new(
        params: Vec<(&'static str, Type)>,
        rettype: Option<Box<Type>>,
        body: Box<Statement>,
    ) -> Self {
        Self {
            params,
            rettype,
            body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int(),
    Struct(&'static str),
    DynTrait(&'static str),
    Func(Vec<Type>, Option<Box<Type>>),
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

pub trait Merge<T> {
    fn merge(&self) -> Result<Option<T>, Error>;
}

// intentionally skipping Or, And, Xor, and GreaterThan for simplicity
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    pub fn interp(&self, mut stmt: Statement) -> Result<Statement, Error> {
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

        let mut cmap = ConstraintMap::new();
        let res4 = self.interpreter.interp(&funcs, &mut cmap, None, &stmt);
        if res4.is_err() {
            return Err(res4.err().unwrap());
        }

        //println!("\n-----------------------------------");
        //println!("PHASE 4: Flow Interpretation");
        //println!("-----------------------------------");
        //println!("\n1. Function symbols table (from PHASE 2)");
        //println!(
        //    "\n2. Flow-sensitive variable symbols table: \n\n{:#?}",
        //    &cmap
        //);
        //println!("\n3. Original program statement");

        let res5 = self
            .rewriter
            .rewrite(&funcs, &cmap, &sigs, None, &mut stmt, true);
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

        Ok(stmt)
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
        let rw_stmt = si.interp(stmt.clone()).unwrap();

        assert_eq!(rw_stmt, stmt);
    }

    #[test]
    fn test_funcdef() {
        let body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        let stmt = FuncDef("foo", vec![], None, body.clone());

        let si = SimpleInterp::new();
        let rw_stmt = si.interp(stmt.clone()).unwrap();

        assert_eq!(rw_stmt, stmt);
    }

    #[test]
    fn test_direct_invoke() {
        let body = Box::new(Sequence(vec![Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], None, body)),
            Box::new(InvokeFunc("foo", vec![])),
        ]);

        let si = SimpleInterp::new();
        let rw_stmt = si.interp(stmt.clone()).unwrap();

        assert_eq!(rw_stmt, stmt);
    }

    #[test]
    fn test_indirect_invoke() {
        let body = Box::new(Sequence(vec![Box::new(Assignment(
            "z",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], None, body.clone())),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]);

        let si = SimpleInterp::new();
        let rw_stmt = si.interp(stmt).unwrap();

        let switch_vec =
            vec![(RVal::Var("foo"), Box::new(InvokeFunc("foo", vec![])))];
        let check_stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], None, body)),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(Switch(RVal::Var("x"), switch_vec)),
        ]);

        assert_eq!(rw_stmt, check_stmt);
    }

    #[test]
    fn test_direct_invoke_uncertain() {
        let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        let bar_body = Box::new(Sequence(vec![Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(6))),
        ))]));

        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], None, foo_body)),
            Box::new(FuncDef("bar", vec![], None, bar_body)),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(InvokeFunc("foo", vec![])),
                Box::new(InvokeFunc("bar", vec![])),
            )),
        ]);

        let si = SimpleInterp::new();
        let rw_stmt = si.interp(stmt.clone()).unwrap();

        assert_eq!(rw_stmt, stmt);
    }

    #[test]
    fn overall_test_indirect_invoke_uncertain() {
        let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
            "y",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))]));
        let bar_body = Box::new(Sequence(vec![Box::new(Assignment(
            "z",
            Box::new(AssignmentRVal::RVal(RVal::Num(6))),
        ))]));

        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], None, foo_body.clone())),
            Box::new(FuncDef("bar", vec![], None, bar_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("bar"))),
                )),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]);

        let si = SimpleInterp::new();
        let rw_stmt = si.interp(stmt).unwrap();

        let switch_vec = vec![
            (RVal::Var("bar"), Box::new(InvokeFunc("bar", vec![]))),
            (RVal::Var("foo"), Box::new(InvokeFunc("foo", vec![]))),
        ];
        let check_stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], None, foo_body)),
            Box::new(FuncDef("bar", vec![], None, bar_body)),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("bar"))),
                )),
            )),
            Box::new(Switch(RVal::Var("x"), switch_vec)),
        ]);

        assert_eq!(rw_stmt, check_stmt);
    }

    #[test]
    fn test_nested_direct_calls_no_args() {
        let bar_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        let foo_body = Box::new(Sequence(vec![
            Box::new(FuncDef("bar", vec![], None, bar_body.clone())),
            Box::new(InvokeFunc("bar", vec![])),
        ]));
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], None, foo_body.clone())),
            Box::new(InvokeFunc("foo", vec![])),
        ]);
        let check_stmt = stmt.clone();

        let si = SimpleInterp::new();
        let rw_stmt = si.interp(stmt).unwrap();

        assert_eq!(rw_stmt, check_stmt);
    }

    #[test]
    fn test_nested_indirect_calls_no_args() {
        let baz_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(1))),
        ));
        let qux_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(2))),
        ));
        let baz2_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(3))),
        ));
        let qux2_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(4))),
        ));
        let foo_body = Box::new(Sequence(vec![
            Box::new(FuncDef("baz", vec![], None, baz_body.clone())),
            Box::new(FuncDef("qux", vec![], None, qux_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("baz"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("qux"))),
                )),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]));
        let bar_body = Box::new(Sequence(vec![
            Box::new(FuncDef("baz2", vec![], None, baz2_body.clone())),
            Box::new(FuncDef("qux2", vec![], None, qux2_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("baz2"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("qux2"))),
                )),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]));

        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], None, foo_body.clone())),
            Box::new(FuncDef("bar", vec![], None, bar_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("bar"))),
                )),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]);

        let foo_switch_vec = vec![
            (RVal::Var("baz"), Box::new(InvokeFunc("baz", vec![]))),
            (RVal::Var("qux"), Box::new(InvokeFunc("qux", vec![]))),
        ];
        let check_foo_body = Box::new(Sequence(vec![
            Box::new(FuncDef("baz", vec![], None, baz_body.clone())),
            Box::new(FuncDef("qux", vec![], None, qux_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("baz"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("qux"))),
                )),
            )),
            Box::new(Switch(RVal::Var("x"), foo_switch_vec)),
        ]));
        let bar_switch_vec = vec![
            (RVal::Var("baz2"), Box::new(InvokeFunc("baz2", vec![]))),
            (RVal::Var("qux2"), Box::new(InvokeFunc("qux2", vec![]))),
        ];
        let check_bar_body = Box::new(Sequence(vec![
            Box::new(FuncDef("baz2", vec![], None, baz2_body.clone())),
            Box::new(FuncDef("qux2", vec![], None, qux2_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("baz2"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("qux2"))),
                )),
            )),
            Box::new(Switch(RVal::Var("x"), bar_switch_vec)),
        ]));
        let switch_vec = vec![
            (RVal::Var("bar"), Box::new(InvokeFunc("bar", vec![]))),
            (RVal::Var("foo"), Box::new(InvokeFunc("foo", vec![]))),
        ];
        let check_stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], None, check_foo_body.clone())),
            Box::new(FuncDef("bar", vec![], None, check_bar_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("bar"))),
                )),
            )),
            Box::new(Switch(RVal::Var("x"), switch_vec)),
        ]);

        let si = SimpleInterp::new();
        let rw_stmt = si.interp(stmt).unwrap();

        assert_eq!(rw_stmt, check_stmt);
    }
}
