use rustc_public::DefId;
use rustc_public::ty::{GenericArgs, Span};

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Error, Write};

pub struct VOLogger {
    stats_file: File,
}

impl VOLogger {
    pub fn new<'a>() -> VOLogger {
        let stats_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(crate::rewrite::stats_path())
            .expect("should be able to open the stats file");

        Self { stats_file }
    }

    pub fn log_stats(
        &mut self,
        dispatch_targets: &HashMap<(DefId, usize, GenericArgs), (Span, Vec<(DefId, Option<GenericArgs>)>)>,
        dispatch_cha: &HashMap<(DefId, usize, GenericArgs), (Span, Vec<(DefId, Option<GenericArgs>)>)>,
        chamax: &HashMap<(DefId, usize, GenericArgs), crate::chamax::ChaMax>,
    ) -> Result<(), Error> {
        let mut diff = Vec::new();
        let mut same = Vec::new();

        for (key, (span, fsa)) in dispatch_targets {
            let cha = dispatch_cha.get(key).cloned().unwrap().1;

            if cha.len() == fsa.len() {
                same.push((key.clone(), *span, cha, fsa.clone()));
            } else {
                diff.push((key.clone(), *span, cha, fsa.clone()));
            }
        }

        write!(
            &mut self.stats_file,
            "STATS:\nNum maybe examples = {:?}\nNum non-examples = {:?}\n",
            diff.len(),
            same.len(),
        )?;

        // FSA vs CHAmax (the whole-program maximum set of instances behind each
        // site's CHA candidates - see chamax.rs). The maybe/non-example split
        // above stays FSA vs CHA; this is extra information.
        let (mut complete, mut incomplete, mut eq, mut lt, mut gt) = (0, 0, 0, 0, 0);
        for (key, (_, fsa)) in dispatch_targets {
            match chamax.get(key) {
                Some(c) if c.incomplete.is_none() => {
                    complete += 1;
                    match fsa.len().cmp(&c.total) {
                        std::cmp::Ordering::Equal => eq += 1,
                        std::cmp::Ordering::Less => lt += 1,
                        std::cmp::Ordering::Greater => gt += 1,
                    }
                }
                _ => incomplete += 1,
            }
        }
        write!(
            &mut self.stats_file,
            "CHAmax summary: complete for {complete} sites (incomplete or unknown for {incomplete}); \
             among complete: FSA == CHAmax at {eq}, FSA < CHAmax at {lt}, FSA > CHAmax at {gt}\n",
        )?;

        write!(&mut self.stats_file, "--MAYBE EXAMPLES--\n")?;
        for (key, span, cha, fsa) in &diff {
            let (caller_def_id, bb_index, caller_genargs) = key;
            write!(
                &mut self.stats_file,
                "Caller: {:?} (bb {}), caller genargs: {:?}\nSpan: {:?}\nCHA ({}): {:?}\nFSA ({}): {:?}\n\n",
                caller_def_id,
                bb_index,
                caller_genargs,
                span,
                cha.len(),
                cha,
                fsa.len(),
                fsa
            )?;
            write_chamax_line(&mut self.stats_file, chamax.get(key))?;
        }

        write!(&mut self.stats_file, "--NOT EXAMPLES--\n")?;
        for (key, span, cha, fsa) in &same {
            let (caller_def_id, bb_index, caller_genargs) = key;
            write!(
                &mut self.stats_file,
                "Caller: {:?} (bb {}), caller genargs: {:?}\nSpan: {:?}\nCHA ({}): {:?}\nFSA ({}): {:?}\n",
                caller_def_id,
                bb_index,
                caller_genargs,
                span,
                cha.len(),
                cha,
                fsa.len(),
                fsa
            )?;
            write_chamax_line(&mut self.stats_file, chamax.get(key))?;
        }

        Ok(())
    }
}

/// The `CHAmax (n[, incomplete: why]): [candidate xN, ...]` line after a site's
/// FSA line. (parse_stats reads exactly the CHA and FSA lines after `Span:`,
/// so this extra line doesn't affect the test harness.)
fn write_chamax_line(out: &mut File, chamax: Option<&crate::chamax::ChaMax>) -> Result<(), Error> {
    match chamax {
        None => writeln!(out, "CHAmax (?): not computed"),
        Some(c) => {
            let incomplete = c.incomplete.as_ref().map_or(String::new(), |why| format!(", incomplete: {why}"));
            let parts: Vec<String> =
                c.per_candidate.iter().map(|(d, n)| format!("{} x{n}", d.name())).collect();
            writeln!(out, "CHAmax ({}{incomplete}): [{}]", c.total, parts.join(", "))
        }
    }
}
