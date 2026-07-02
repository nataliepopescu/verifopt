use rustc_public::DefId;
use rustc_public::ty::{GenericArgs, Span};

use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct VOLogger {
    stats_file: File,
    in_reinterp: bool,
    diff: Vec<(
        Span,
        bool,
        Vec<(DefId, Option<GenericArgs>)>,
        Vec<(DefId, Option<GenericArgs>)>,
    )>,
    same: Vec<(
        Span,
        bool,
        Vec<(DefId, Option<GenericArgs>)>,
        Vec<(DefId, Option<GenericArgs>)>,
    )>,
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
            in_reinterp: false,
            diff: Vec::new(),
            same: Vec::new(),
        }
    }

    pub fn set_reinterp(&mut self, state: bool) -> bool {
        let prev = self.in_reinterp;
        self.in_reinterp = state;
        prev
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

        for (term_span, in_re, cha, fsa) in &self.diff {
            write!(
                &mut self.stats_file,
                "Span [{}] {:?}\nCHA ({}): {:?}\nFSA ({}): {:?}\n\n",
                if *in_re { "RE-INTERP" } else { "REAL" },
                term_span,
                cha.len(),
                cha,
                fsa.len(),
                fsa
            )?;
        }

        write!(&mut self.stats_file, "--NOT EXAMPLES--\n",)?;

        for (term_span, in_re, cha, fsa) in &self.same {
            write!(
                &mut self.stats_file,
                "Span: [{}] {:?}\nCHA ({}): {:?}\nFSA ({}): {:?}\n",
                if *in_re { "RE-INTERP" } else { "REAL" },
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
        assoc_fn_impls_cha: &Vec<(DefId, Option<GenericArgs>)>,
        assoc_fn_impls_fsa: &Vec<(DefId, Option<GenericArgs>)>,
    ) {
        self.diff.push((
            *term_span,
            self.in_reinterp,
            assoc_fn_impls_cha.to_vec(),
            assoc_fn_impls_fsa.to_vec(),
        ));
    }

    pub fn update_same(
        &mut self,
        term_span: &Span,
        assoc_fn_impls_cha: &Vec<(DefId, Option<GenericArgs>)>,
        assoc_fn_impls_fsa: &Vec<(DefId, Option<GenericArgs>)>,
    ) {
        self.same.push((
            *term_span,
            self.in_reinterp,
            assoc_fn_impls_cha.to_vec(),
            assoc_fn_impls_fsa.to_vec(),
        ));
    }
}
