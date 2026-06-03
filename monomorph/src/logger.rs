use rustc_public::DefId;
use rustc_public::ty::Span;

use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct VOLogger {
    stats_file: File,
    diff: Vec<(Span, Vec<DefId>, Vec<DefId>)>,
    same: Vec<(Span, Vec<DefId>, Vec<DefId>)>,
}

impl VOLogger {
    pub fn new<'a>(_f_filename: &'a str, _nf_filename: &'a str) -> VOLogger {
        let stats_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("stats")
            .expect("should be able to open file");
        Self {
            stats_file,
            diff: Vec::new(),
            same: Vec::new(),
        }
    }

    pub fn log_stats(&mut self) -> Result<(), std::io::Error> {
        let num_found = self.diff.len();
        let num_not_found = self.same.len();

        write!(
            &mut self.stats_file,
            "STATS:\nNum maybe examples = {:?}\nNum non-examples = {:?}\n",
            num_found, num_not_found,
        )?;

        write!(&mut self.stats_file, "--MAYBE EXAMPLES--\n",)?;

        for (term_span, cha, fsa) in &self.diff {
            write!(
                &mut self.stats_file,
                "Span: {:?}\nCHA ({}): {:?}\nFSA ({}): {:?}\n",
                term_span,
                cha.len(),
                cha,
                fsa.len(),
                fsa
            )?;
        }

        write!(&mut self.stats_file, "--NOT EXAMPLES--\n",)?;

        for (term_span, cha, fsa) in &self.same {
            write!(
                &mut self.stats_file,
                "Span: {:?}\nCHA ({}): {:?}\nFSA ({}): {:?}\n",
                term_span,
                cha.len(),
                cha,
                fsa.len(),
                fsa
            )?;
        }

        Ok(())
    }

    pub fn update_diff(
        &mut self,
        term_span: &Span,
        assoc_fn_impls_cha: &Vec<DefId>,
        assoc_fn_impls_fsa: &Vec<DefId>,
    ) {
        self.diff.push((
            *term_span,
            assoc_fn_impls_cha.to_vec(),
            assoc_fn_impls_fsa.to_vec(),
        ));
    }

    pub fn update_same(
        &mut self,
        term_span: &Span,
        assoc_fn_impls_cha: &Vec<DefId>,
        assoc_fn_impls_fsa: &Vec<DefId>,
    ) {
        self.same.push((
            *term_span,
            assoc_fn_impls_cha.to_vec(),
            assoc_fn_impls_fsa.to_vec(),
        ));
    }
}
