use rustc_data_structures::fx::FxHashMap as HashMap;
//use rustc_data_structures::fx::FxHashSet as HashSet;
use rustc_public::DefId;
use rustc_public::mir::mono::Instance;
use rustc_public::ty::{BoundVariableKind, FnDef, FnSig, ForeignItemKind, Ty};

use log::debug;

//use crate::constraints::VORval;
use crate::convert::RvalConverter;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct SigVal {
    inputs: Vec<Ty>,
    output: Ty,
}

impl SigVal {
    pub fn new(_converter: &RvalConverter, sig: &FnSig) -> SigVal {
        //let mut inputs = Vec::new();
        //for input in sig.inputs() {
        //    inputs.push(converter.convert_ty(input));
        //}
        //let output = converter.convert_ty(&sig.output());
        let inputs = sig.inputs().to_vec();
        let output = sig.output();
        debug!("NUM INPUTS: {:?}", inputs.len());
        Self { inputs, output }
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
        self.test_mono();
        self.collect_function_sigs(sigstore);
    }

    fn test_mono(&self) {
        let instances: Vec<Instance> = rustc_public::all_local_items()
            .into_iter()
            .filter_map(|item| Instance::try_from(item).ok())
            .collect();
        for instance in instances {
            debug!("\nmono?: {:?}\n{:?}", instance.name(), instance);
        }
    }

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
        if !sig.bound_vars.is_empty() {
            for bound_var in &sig.bound_vars {
                match bound_var {
                    BoundVariableKind::Ty(_) => todo!(),
                    _ => {}
                }
            }
        }

        let sigval = SigVal::new(&self.converter, &sig.skip_binder());
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
