use rustc_data_structures::fx::FxHashMap as HashMap;
//use rustc_data_structures::fx::FxHashSet as HashSet;
use rustc_public::DefId;
//use rustc_public::mir::mono::Instance;
use rustc_public::ty::{
    BoundRegionKind, BoundTyKind, BoundVariableKind, FnDef, ForeignItemKind, PolyFnSig, Ty,
};

//use log::debug;
use std::panic;

use crate::TraitStore;
use crate::common::log_mir;
use crate::convert::RvalConverter;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct SigVal {
    pub bound_tys: Vec<(DefId, String)>,
    pub bound_regions: Vec<(DefId, String)>,
    pub inputs: Vec<Ty>,
    pub output: Ty,
}

impl SigVal {
    pub fn new_from_poly(polysig: &PolyFnSig) -> SigVal {
        let mut bound_tys = Vec::new();
        let mut bound_regions = Vec::new();
        if !polysig.bound_vars.is_empty() {
            for bound_var in &polysig.bound_vars {
                match bound_var {
                    BoundVariableKind::Ty(ty) => match ty {
                        BoundTyKind::Param(def, s) => bound_tys.push((def.0, s.clone())),
                        _ => {}
                    },
                    BoundVariableKind::Region(region) => match region {
                        BoundRegionKind::BrNamed(def, s) => bound_regions.push((def.0, s.clone())),
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        let sig = polysig.clone().skip_binder();
        let inputs = sig.inputs().to_vec();
        let output = sig.output();

        //debug!("NUM INPUTS: {:?}", inputs.len());
        //debug!("INPUTS: {:?}", inputs);
        //debug!("OUTPUT: {:?}", output);
        //debug!("BOUND_TYS: {:?}", bound_tys);
        //debug!("BOUND_REGIONS: {:?}", bound_regions);

        Self {
            bound_tys,
            bound_regions,
            inputs,
            output,
        }
    }
}

// If have inputs/outputs, will probably have generics that get resolved later
pub struct SigStore {
    pub sigs: HashMap<SigVal, Vec<FnDef>>,
}

impl SigStore {
    pub fn new() -> SigStore {
        Self {
            sigs: HashMap::default(),
        }
    }
}

pub struct SigCollectPass<'a> {
    pub converter: RvalConverter<'a>,
}

impl<'a> SigCollectPass<'a> {
    pub fn new(tstore: &'a TraitStore) -> SigCollectPass<'a> {
        Self {
            converter: RvalConverter::new(tstore),
        }
    }

    pub fn run(&self, sigstore: &mut SigStore) {
        //debug!("FUNC SIG PASS");
        //self.test();
        self.collect_function_sigs(sigstore);
        //debug!("DONE FUNC SIG PASS");
    }

    /*
    fn test(&self) {
        let krates: Vec<_> = rustc_public::all_local_items();
            //.into_iter()
            //.filter_map(|item| Instance::try_from(item).ok())
            //.collect();
        for krate in krates {
            debug!("\nkrate?: {:?}", krate); //instance.name(), instance);
        }
        panic!("done")
    }
    */

    fn collect_function_sigs(&self, sigstore: &mut SigStore) {
        let mut all_crates = rustc_public::external_crates().clone();
        all_crates.insert(0, rustc_public::local_crate());
        //let len = all_crates.len();
        for (_i, krate) in all_crates.into_iter().enumerate() {
            //debug!("krate#{:?} ({:?}/{:?})", i, i + 1, len);
            //debug!("krate: {:?}", krate);

            // Non-crate-local FnDefs
            for fndef in krate.fn_defs() {
                //debug!("NOT FOREIGN");
                self.process_fndef(sigstore, &fndef);
            }

            // Foreign FnDefs
            for foreign_mod in krate.foreign_modules() {
                for foreign_item in foreign_mod.module().items() {
                    match foreign_item.kind() {
                        ForeignItemKind::Fn(fndef) => {
                            //debug!("FOREIGN");
                            self.process_fndef(sigstore, &fndef);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn process_fndef(&self, sigstore: &mut SigStore, fndef: &FnDef) {
        let sig = fndef.fn_sig();
        //debug!("fndef: {:?}", fndef);
        let sigval = SigVal::new_from_poly(&sig);
        //debug!("sigval: {:?}", sigval);
        match sigstore.sigs.get_mut(&sigval) {
            Some(fn_vec) => {
                if !fn_vec.contains(&fndef) {
                    fn_vec.push(*fndef);
                }
            }
            None => {
                sigstore.sigs.insert(sigval, vec![*fndef]);
            }
        }

        // FIXME false -> true if want to print func bodies
        if fndef.has_body() && false {
            //debug!("HAS BODY");
            match panic::catch_unwind(|| fndef.body()) {
                Ok(body) => {
                    log_mir(&body.unwrap());
                }
                _ => {} //debug!("ERROR GETTING BODY"),
            }
        } else {
            //debug!("NO BODY");
        }
    }
}
