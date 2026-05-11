use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_public::DefId;

use crate::fsa::wto::BBDeps;

#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintMap {
    pub wtos: HashMap<DefId, BBDeps>,
}

impl ConstraintMap {
    pub fn new() -> ConstraintMap {
        Self {
            wtos: HashMap::default(),
        }
    }
}

