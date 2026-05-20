use rustc_public::DefId;

use std::fs;

pub struct VOLogger;

impl VOLogger {
    pub fn new() -> VOLogger {
        Self {}
    }

    pub fn log(&self, assoc_fn_impls_cha: &Vec<DefId>, assoc_fn_impls_fsa: &Vec<DefId>) {
        let fstr = format!(
            "\n---\nCHA ({}): {:?}\nFSA ({}): {:?}",
            assoc_fn_impls_cha.len(),
            assoc_fn_impls_cha,
            assoc_fn_impls_fsa.len(),
            assoc_fn_impls_fsa
        );

        fs::write("tool_effectiveness_log", fstr).expect("should write");
    }
}
