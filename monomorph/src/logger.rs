use rustc_public::DefId;
use rustc_public::ty::Span;

use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct VOLogger {
    f_file: File,
    nf_file: File,
    stats_file: File,
    num_found: usize,
    num_not_found: usize,
}

impl VOLogger {
    pub fn new<'a>(f_filename: &'a str, nf_filename: &'a str) -> VOLogger {
        let f_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(f_filename)
            .expect("should be able to open file");
        let nf_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(nf_filename)
            .expect("should be able to open file");
        let stats_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("stats")
            .expect("should be able to open file");
        Self {
            f_file,
            nf_file,
            stats_file,
            num_found: 0,
            num_not_found: 0,
        }
    }

    pub fn log_stats(&mut self) -> Result<(), std::io::Error> {
        write!(
            &mut self.stats_file,
            "STATS:\nNum potential examples = \t{:?}\nNum non-examples = \t\t{:?}",
            self.num_found, self.num_not_found,
        )?;
        Ok(())
    }

    pub fn log_found(
        &mut self,
        term_span: &Span,
        assoc_fn_impls_cha: &Vec<DefId>,
        assoc_fn_impls_fsa: &Vec<DefId>,
    ) -> Result<(), std::io::Error> {
        self.num_found += 1;
        write!(
            &mut self.f_file,
            "Span: {:?}\nCHA ({}): {:?}\nFSA ({}): {:?}",
            term_span,
            assoc_fn_impls_cha.len(),
            assoc_fn_impls_cha,
            assoc_fn_impls_fsa.len(),
            assoc_fn_impls_fsa
        )?;
        Ok(())
    }

    pub fn log_not_found(
        &mut self,
        term_span: &Span,
        assoc_fn_impls_cha: &Vec<DefId>,
        assoc_fn_impls_fsa: &Vec<DefId>,
    ) -> Result<(), std::io::Error> {
        self.num_not_found += 1;
        write!(
            &mut self.nf_file,
            "Span: {:?}\nCHA ({}): {:?}\nFSA ({}): {:?}",
            term_span,
            assoc_fn_impls_cha.len(),
            assoc_fn_impls_cha,
            assoc_fn_impls_fsa.len(),
            assoc_fn_impls_fsa
        )?;
        Ok(())
    }
}
