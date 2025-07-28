use crate::error::Error;

use std::fmt;
use std::ops::Not;

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
        bool,
        Vec<(&'static str, Type)>,
        Option<Box<Type>>,
        Box<Statement>,
    ),
    InvokeFunc(&'static str, Vec<&'static str>),
    InvokeTraitFunc(&'static str, TraitStructTup, Vec<&'static str>),
    Struct(&'static str, Vec<Type>, Vec<&'static str>),
    // traits without associated types for now
    // (trait name, func names, funcs decls)
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
    IdkNum(),
    // TODO add strings
    Struct(&'static str, Vec<RVal>),
    IdkStruct(&'static str),
    Var(&'static str),
    IdkVar(),
}

pub type TraitStructTup = (&'static str, &'static str);
pub type TraitStructOpt = Option<TraitStructTup>;
pub type FuncName = &'static str;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncDecl {
    pub is_method: bool,
    pub params: Vec<(&'static str, Type)>,
    pub rettype: Option<Box<Type>>,
}

impl FuncDecl {
    pub fn new(
        is_method: bool,
        params: Vec<(&'static str, Type)>,
        rettype: Option<Box<Type>>,
    ) -> Self {
        Self {
            is_method,
            params,
            rettype,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncVal {
    pub is_method: bool,
    pub params: Vec<(&'static str, Type)>,
    pub rettype: Option<Box<Type>>,
    pub body: Box<Statement>,
}

impl FuncVal {
    pub fn new(
        is_method: bool,
        params: Vec<(&'static str, Type)>,
        rettype: Option<Box<Type>>,
        body: Box<Statement>,
    ) -> Self {
        Self {
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
pub enum BooleanStatement {
    True(),
    False(),
    TrueOrFalse(),
    Not(Box<BooleanStatement>),
    Equals(RVal, RVal),
}

pub trait Merge<T> {
    fn merge(&self) -> Result<Option<T>, Error>;
}

impl fmt::Display for RVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RVal::Num(num) => write!(f, "{}", num),
            RVal::IdkNum() => write!(f, "IDK-Num"),
            RVal::Var(var) => write!(f, "{}", var),
            RVal::IdkVar() => write!(f, "IDK-Var"),
            RVal::Struct(name, field_values) => {
                let mut fv_string: String = "".to_owned();
                for field_value in field_values.iter() {
                    fv_string.push_str(&field_value.to_string());
                }
                write!(f, "{} : {}", name, fv_string)
            }
            RVal::IdkStruct(name) => write!(f, "IDK-Struct({})", name),
        }
    }
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

