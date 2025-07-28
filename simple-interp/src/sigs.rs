use crate::{FuncName, SigVal};
use std::collections::{HashMap, HashSet};

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

