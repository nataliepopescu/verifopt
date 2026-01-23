//use rustc_hir::TyKind;
//use rustc_hir::FnRetTy::*;
use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_hir::def::DefKind;
//use rustc_hir::def::Res;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::ty::{List, TyCtxt};

use crate::core::FuncVal;

use std::sync::{Arc, Mutex};

// omitting TraitStructOpt unless useful
#[derive(Debug, Clone)]
pub struct FuncMap<'tcx> {
    // all fns, trait-related or not
    pub funcs: HashMap<DefId, Vec<FuncVal<'tcx>>>,
    // assoc fn of a trait -> implementors of that assoc fn
    pub trait_fn_impltors: Arc<Mutex<HashMap<DefId, Vec<DefId>>>>,
    // assoc fn of a trait -> that trait
    pub assocfns_to_traits: Arc<Mutex<HashMap<DefId, DefId>>>,
}

impl<'tcx> FuncMap<'tcx> {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::default(),
            trait_fn_impltors: Arc::new(Mutex::new(HashMap::default())),
            assocfns_to_traits: Arc::new(Mutex::new(HashMap::default())),
        }
    }
}

pub struct FuncCollectPass<'tcx> {
    pub tcx: TyCtxt<'tcx>,
}

impl<'tcx> FuncCollectPass<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> FuncCollectPass<'tcx> {
        Self { tcx }
    }

    pub fn collect_funcs(&self, funcs: &mut FuncMap<'tcx>, debug: bool) {
        // TODO try past 4
        //let num_crates = self.tcx.used_crates(()).len();
        let num_crates = 4u32;
        for crate_num in 0u32..num_crates + 1 {
            if debug {
                println!("\n\ncrate_num: {:?}\n", crate_num);
            }
            for def_index in 0..u32::MAX {
                if crate_num == 0 && def_index >= 22
                    || crate_num == 1 && def_index >= 19549
                    || crate_num == 2 && def_index >= 78916
                    || crate_num == 3 && def_index >= 12636
                    || crate_num == 4 && def_index >= 4970
                //|| crate_num == 5 && def_index >= 12217
                //|| crate_num == 6 && def_index >= 3
                //|| crate_num == 7 && def_index >= 94
                //|| crate_num == 8 && def_index >= 513
                //|| crate_num == 9 && def_index >= 71
                {
                    break;
                }

                let def_id = DefId {
                    index: def_index.into(),
                    krate: crate_num.into(),
                };
                let def_kind = self.tcx.def_kind(def_id);

                if debug {
                    println!("\nnew def_index");
                    println!("crate_num: {:?}", crate_num);
                    println!("def_index: {:?}", def_index);
                    println!("forged defid: {:?}", def_id);
                    println!("def_kind: {:?}", def_kind);
                }

                if def_kind == DefKind::Trait {
                    if debug {
                        println!("trait");
                    }
                    for impl_defid in self.tcx.all_impls(def_id) {
                        let impltors = self.tcx.impl_item_implementor_ids(impl_defid);
                        if debug {
                            println!("impltors: {:?}", impltors);
                        }

                        impltors.items().all(|(key, val)| {
                            let mut trait_map_lock = funcs.trait_fn_impltors.lock().unwrap();
                            match trait_map_lock.get_mut(key) {
                                Some(existing_vals) => {
                                    let mut new_vals = existing_vals.clone();
                                    new_vals.push(val.clone());
                                    trait_map_lock.insert(key.clone(), new_vals.to_vec());
                                }
                                None => {
                                    trait_map_lock.insert(key.clone(), vec![val.clone()]);
                                }
                            }

                            // add for reverse search
                            funcs
                                .assocfns_to_traits
                                .lock()
                                .unwrap()
                                .insert(*key, def_id);

                            true
                        });
                    }
                }

                if def_kind == DefKind::AssocFn {
                    if debug {
                        println!("assocfn");
                    }
                }

                if def_kind == DefKind::Fn || def_kind == DefKind::AssocFn {
                    //println!("fn_sig: {:?}", self.tcx.fn_sig(def_id));
                    let arg_idents = self.tcx.fn_arg_idents(def_id);
                    let num_args = arg_idents.len();
                    let mut is_method = false;
                    if num_args > 0 {
                        // FIXME more granular check? i.e. check actual `self` str?
                        if let Some(first) = arg_idents[0]
                            && first.is_reserved()
                        {
                            is_method = true;
                        }
                    }
                    let mut arg_names = vec![];
                    for i in 1..num_args + 1 {
                        let arg_place = Place {
                            local: Local::from_usize(i),
                            projection: List::empty(),
                        };
                        arg_names.push(arg_place);
                    }

                    let mir_avail = self.tcx.is_mir_available(def_id);
                    if mir_avail {
                        //println!("mir available...");
                        //println!("Body: \n{:?}", self.tcx.instance_mir(InstanceKind::Item(def_id)));
                    } else {
                        //println!("mir NOT available...");
                    }

                    let mut is_intrinsic = false;
                    if self.tcx.intrinsic_raw(def_id).is_some() {
                        //println!("intrinsic");
                        is_intrinsic = true;
                    }

                    let sig = self.tcx.fn_sig(def_id);
                    // FIXME skip_binder() generally incorrect but in this instance the return type
                    // is not generic so I think it is ok
                    let funcval = FuncVal::new(
                        def_id,
                        is_intrinsic,
                        is_method,
                        arg_names,
                        Some(sig.skip_binder().skip_binder().output()),
                    );
                    let vec_to_insert: Vec<FuncVal>;
                    match funcs.funcs.get_mut(&def_id) {
                        Some(func_vec) => {
                            func_vec.push(funcval);
                            vec_to_insert = func_vec.to_vec();
                        }
                        None => {
                            vec_to_insert = vec![funcval];
                            // TODO handle nested func decls
                        }
                    }
                    funcs.funcs.insert(def_id, vec_to_insert);
                }
            }
        }
    }

    pub fn run(&self, funcs: &mut FuncMap<'tcx>, debug: bool) {
        self.collect_funcs(funcs, debug);
    }
}
