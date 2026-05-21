use rustc_public::DefId;
use rustc_public::ty::Span;

use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct VOLogger {
    pub file: File,
}

impl VOLogger {
    pub fn new<'a>(filename: &'a str) -> VOLogger {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filename)
            .expect("should be able to open file");
        Self { file }
    }

    pub fn log(&mut self, term_span: &Span, assoc_fn_impls_cha: &Vec<DefId>, assoc_fn_impls_fsa: &Vec<DefId>) -> Result<(), std::io::Error> {
        write!(&mut self.file, "Span: {:?}\nCHA ({}): {:?}\nFSA ({}): {:?}", term_span, assoc_fn_impls_cha.len(), assoc_fn_impls_cha, assoc_fn_impls_fsa.len(), assoc_fn_impls_fsa)?;
        Ok(())
    }
}
