use rustc_data_structures::fx::FxHashMap as HashMap;
//use rustc_data_structures::fx::FxHashSet as HashSet;
use rustc_public::DefId;
use rustc_public::ty::{BoundVariableKind, FnDef, FnSig, RigidTy, Ty, TyKind};

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
        self.collect_function_sigs(sigstore);
    }

    fn collect_function_sigs(&self, sigstore: &mut SigStore) {
        for item in rustc_public::all_local_items() {
            debug!("local item: {:?}", item);
            match item.ty().kind() {
                TyKind::RigidTy(rigid_ty) => match rigid_ty {
                    RigidTy::FnDef(fndef, genargs) => {
                        if !genargs.0.is_empty() {
                            todo!();
                        }
                        self.process_fn(sigstore, &fndef);
                    }
                    _ => todo!(),
                },
                _ => todo!(),
            }
        }
        for krate in rustc_public::external_crates() {
            debug!("krate: {:?}", krate);
            for fndef in krate.fn_defs() {
                self.process_fn(sigstore, &fndef);
            }
        }
    }

    fn process_fn(&self, sigstore: &mut SigStore, fndef: &FnDef) {
        let sig = fndef.fn_sig();
        debug!("\n\nfndef: {:?}", fndef);
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
