use crate::error::Error;

use std::ops::Not;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
    Sequence(Vec<Box<Statement>>),
    Assignment(&'static str, Box<AssignmentRVal>),
    Print(&'static str),
    Conditional(Box<BStatement>, Box<Statement>, Box<Statement>),
    Switch(RVal, Vec<(RVal, Box<Statement>)>),
    Return(RVal),
    FuncDecl(FuncDecl),
    FuncDef(FuncVal),
    InvokeFunc(&'static str, Vec<&'static str>),
    // only used _after_ rewrite
    InvokeTraitFunc(&'static str, TraitStructTup, Vec<&'static str>),
    Struct(&'static str, Vec<Type>, Vec<&'static str>),
    // traits without associated types for now
    // (trait name, funcs decls)
    TraitDecl(&'static str, Vec<FuncDecl>),
    // (trait name, struct name, funcs impls)
    TraitImpl(&'static str, &'static str, Vec<FuncVal>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssignmentRVal {
    RVal(RVal),
    Statement(Box<Statement>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RVal {
    Num(i32),
    IdkNum(),
    // TODO add strings
    Struct(&'static str, Vec<RVal>, Vec<&'static str>),
    IdkStruct(&'static str),
    Var(&'static str),
    IdkVar(),
}

pub type TraitStructTup = (&'static str, &'static str);
pub type TraitStructOpt = Option<TraitStructTup>;
pub type FuncName = &'static str;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncDecl {
    pub name: &'static str,
    pub is_method: bool,
    pub params: Vec<(&'static str, Type)>,
    pub rettype: Option<Box<Type>>,
}

impl FuncDecl {
    pub fn new(
        name: &'static str,
        is_method: bool,
        params: Vec<(&'static str, Type)>,
        rettype: Option<Box<Type>>,
    ) -> Self {
        Self {
            name,
            is_method,
            params,
            rettype,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncVal {
    pub name: &'static str,
    pub is_method: bool,
    pub params: Vec<(&'static str, Type)>,
    pub rettype: Option<Box<Type>>,
    pub body: Box<Statement>,
}

impl FuncVal {
    pub fn new(
        name: &'static str,
        is_method: bool,
        params: Vec<(&'static str, Type)>,
        rettype: Option<Box<Type>>,
        body: Box<Statement>,
    ) -> Self {
        Self {
            name,
            is_method,
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
    // FIXME add is_method bool?
    Func(Vec<Type>, Option<Box<Type>>),
    Idk(),
}

// intentionally skipping Or, And, Xor, and GreaterThan for simplicity
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BStatement {
    True(),
    False(),
    TrueOrFalse(),
    Not(Box<BStatement>),
    Equals(RVal, RVal),
}

pub trait Merge<T> {
    fn merge(&self) -> Result<Option<T>, Error>;
}

impl Not for BStatement {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            BStatement::True() => BStatement::False(),
            BStatement::False() => BStatement::True(),
            BStatement::TrueOrFalse() => BStatement::TrueOrFalse(),
            BStatement::Not(_) | BStatement::Equals(_, _) => {
                panic!("not implemented yet")
            }
        }
    }
}

impl Not for &BStatement {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            BStatement::True() => &BStatement::False(),
            BStatement::False() => &BStatement::True(),
            BStatement::TrueOrFalse() => &BStatement::TrueOrFalse(),
            BStatement::Not(_) | BStatement::Equals(_, _) => {
                panic!("not implemented yet (&)")
            }
        }
    }
}
