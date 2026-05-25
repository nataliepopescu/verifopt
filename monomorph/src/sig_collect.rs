use rustc_data_structures::fx::FxHashMap as HashMap;
//use rustc_data_structures::fx::FxHashSet as HashSet;
use rustc_public::DefId;
//use rustc_public::mir::mono::Instance;
use rustc_public::ty::{
    BoundRegionKind, BoundTyKind, BoundVariableKind, FnDef, ForeignItemKind, PolyFnSig, Ty,
};

use log::debug;

//use crate::constraints::VORval;
use crate::convert::RvalConverter;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct SigVal {
    inputs: Vec<Ty>,
    output: Ty,
    bound_tys: Vec<(DefId, String)>,
    bound_regions: Vec<(DefId, String)>,
}

impl SigVal {
    pub fn new(_converter: &RvalConverter, sig: &PolyFnSig) -> SigVal {
        let mut bound_tys = Vec::new();
        let mut bound_regions = Vec::new();
        if !sig.bound_vars.is_empty() {
            for bound_var in &sig.bound_vars {
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

        let sigval = sig.clone().skip_binder();
        let inputs = sigval.inputs().to_vec();
        let output = sigval.output();

        debug!("NUM INPUTS: {:?}", inputs.len());
        debug!("INPUTS: {:?}", inputs);
        debug!("OUTPUT: {:?}", output);
        debug!("BOUND_TYS: {:?}", bound_tys);
        debug!("BOUND_REGIONS: {:?}", bound_regions);

        Self {
            inputs,
            output,
            bound_tys,
            bound_regions,
        }
    }
}

// If have inputs/outputs, will probably have generics that get resolved later
pub struct SigStore {
    pub sigs: HashMap<SigVal, Vec<DefId>>,
}

impl SigStore {
    pub fn new() -> SigStore {
        Self {
            sigs: HashMap::default(),
        }
    }
}

pub struct SigCollectPass {
    pub converter: RvalConverter,
}

impl SigCollectPass {
    pub fn new() -> SigCollectPass {
        Self {
            converter: RvalConverter::new(),
        }
    }

    pub fn run(&self, sigstore: &mut SigStore) {
        //self.test();
        self.collect_function_sigs(sigstore);
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
        for krate in all_crates {
            debug!("krate: {:?}", krate);

            // Non-crate-local FnDefs
            for fndef in krate.fn_defs() {
                debug!("\n\n");
                self.process_fndef(sigstore, &fndef);
            }

            // Foreign FnDefs
            for foreign_mod in krate.foreign_modules() {
                for foreign_item in foreign_mod.module().items() {
                    match foreign_item.kind() {
                        ForeignItemKind::Fn(fndef) => {
                            debug!("\n\n");
                            debug!("FOREIGN!");
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
        debug!("fndef: {:?}", fndef);
        let sigval = SigVal::new(&self.converter, &sig);
        debug!("sigval: {:?}", sigval);
        match sigstore.sigs.get_mut(&sigval) {
            Some(fn_vec) => {
                if !fn_vec.contains(&fndef.0) {
                    fn_vec.push(fndef.0);
                }
            }
            None => {
                sigstore.sigs.insert(sigval, vec![fndef.0]);
            }
        }
    }
}
