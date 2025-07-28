use crate::statement::{FuncName, Type};
use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Clone, PartialEq)]
pub struct Sigs {
    pub sigs: HashMap<SigVal, HashSet<FuncName>>,
}

impl Sigs {
    pub fn new() -> Self {
        Self {
            sigs: HashMap::<SigVal, HashSet<FuncName>>::new(),
        }
    }
}
