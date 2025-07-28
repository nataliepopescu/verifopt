use crate::statement::{FuncName, FuncVal, TraitStructOpt};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Funcs {
    pub funcs: HashMap<FuncName, Vec<(TraitStructOpt, FuncVal)>>,
}

impl Funcs {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::<FuncName, Vec<(TraitStructOpt, FuncVal)>>::new(),
        }
    }
}
