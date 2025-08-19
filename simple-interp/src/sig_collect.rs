use crate::error::Error;
use crate::funcs::Funcs;
use crate::sigs::{SigVal, Sigs};
use std::collections::HashSet;

pub struct SigCollector {}

impl SigCollector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn collect(&self, funcs: &Funcs, sigs: &mut Sigs) -> Result<(), Error> {
        for (func_name, funcvec) in funcs.funcs.iter() {
            for (_tso, func) in funcvec.iter() {
                // FIXME differentiate tso?
                let (_, paramtypes): (Vec<&'static str>, Vec<crate::statement::Type>) =
                    func.params.clone().into_iter().unzip();
                let sig = SigVal::new(paramtypes, func.rettype.clone());
                match sigs.sigs.get(&sig) {
                    Some(existing_funcs) => {
                        let mut func_names = existing_funcs.clone();
                        func_names.insert(func_name);
                        sigs.sigs.insert(sig, func_names);
                    }
                    None => {
                        let mut func_names = HashSet::<&'static str>::new();
                        func_names.insert(func_name);
                        sigs.sigs.insert(sig, func_names);
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod sig_collect_tests;
