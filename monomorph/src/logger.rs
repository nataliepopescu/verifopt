use rustc_public::DefId;
use rustc_public::ty::{GenericArgs, Span};

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Error, Write};

pub struct VOLogger {
    stats_file: File,
}

impl VOLogger {
    pub fn new<'a>(_f_filename: &'a str, _nf_filename: &'a str) -> VOLogger {
        let stats_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("stats")
            .expect("should be able to open file");

        Self { stats_file }
    }

    pub fn log_stats(
        &mut self,
        dispatch_targets: &HashMap<(DefId, usize), (Span, Vec<(DefId, Option<GenericArgs>)>)>,
        dispatch_cha: &HashMap<(DefId, usize), (Span, Vec<(DefId, Option<GenericArgs>)>)>,
    ) -> Result<(), Error> {
        let mut diff = Vec::new();
        let mut same = Vec::new();

        for (key, (span, fsa)) in dispatch_targets {
            let cha = dispatch_cha.get(key).cloned().unwrap().1;

            if cha.len() == fsa.len() {
                same.push((*span, cha, fsa.clone()));
            } else {
                diff.push((*span, cha, fsa.clone()));
            }
        }

        write!(
            &mut self.stats_file,
            "STATS:\nNum maybe examples = {:?}\nNum non-examples = {:?}\n",
            diff.len(),
            same.len(),
        )?;

        write!(&mut self.stats_file, "--MAYBE EXAMPLES--\n")?;
        for (span, cha, fsa) in &diff {
            write!(
                &mut self.stats_file,
                "Span: {:?}\nCHA ({}): {:?}\nFSA ({}): {:?}\n\n",
                span,
                cha.len(),
                cha,
                fsa.len(),
                fsa
            )?;
        }

        write!(&mut self.stats_file, "--NOT EXAMPLES--\n")?;
        for (span, cha, fsa) in &same {
            write!(
                &mut self.stats_file,
                "Span: {:?}\nCHA ({}): {:?}\nFSA ({}): {:?}\n",
                span,
                cha.len(),
                cha,
                fsa.len(),
                fsa
            )?;
        }

        Ok(())
    }
}
