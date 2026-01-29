//use rustc_hir::TyKind;
//use rustc_hir::FnRetTy::*;
use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_hir::def::DefKind;
//use rustc_hir::def::Res;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::ty::{Generics, InstanceKind, List, TyCtxt, TyKind};

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
    // trait -> implementors of that trait (i.e. structs)
    pub trait_impltors: HashMap<DefId, Vec<DefId>>,
    // struct defid -> generics
    pub struct_generics: HashMap<DefId, Generics>,
    // TODO
    // struct -> [impl blocks]
    pub struct_impls: HashMap<DefId, Vec<DefId>>,
    // impl blocks -> [impl fns/methods]
    pub impl_blocks_to_impls: HashMap<DefId, Vec<DefId>>,
}

impl<'tcx> FuncMap<'tcx> {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::default(),
            trait_fn_impltors: Arc::new(Mutex::new(HashMap::default())),
            assocfns_to_traits: Arc::new(Mutex::new(HashMap::default())),
            trait_impltors: HashMap::default(),
            struct_generics: HashMap::default(),
            struct_impls: HashMap::default(),
            impl_blocks_to_impls: HashMap::default(),
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

    pub fn handle_struct(&self, funcs: &mut FuncMap<'tcx>, def_id: DefId, debug: bool) {
        if debug {
            println!("generics: {:#?}", self.tcx.generics_of(def_id));
        }
        funcs
            .struct_generics
            .insert(def_id, self.tcx.generics_of(def_id).clone());
    }

    pub fn handle_trait(&self, funcs: &mut FuncMap<'tcx>, def_id: DefId, debug: bool) {
        if debug {
            println!("generics: {:#?}", self.tcx.generics_of(def_id));
        }

        match funcs.trait_impltors.get(&def_id) {
            Some(_) => {}
            None => {
                funcs.trait_impltors.insert(def_id, vec![]);
            }
        }

        for impl_defid in self.tcx.all_impls(def_id) {
            let impltors = self.tcx.impl_item_implementor_ids(impl_defid);
            //if debug {
            //    println!("impl_defid: {:?}", impl_defid);
            //    println!("impltors: {:?}", impltors);
            //}

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

    pub fn handle_trait_impl(&self, funcs: &mut FuncMap<'tcx>, def_id: DefId, debug: bool) {
        // TODO might be useful once we encounter default trait implementations, to see
        // exactly _which_ functions are being implemented/overriden
        //println!("assoc_items: {:#?}", self.tcx.associated_items(def_id));

        let trait_ref = self.tcx.impl_trait_header(def_id).trait_ref.skip_binder();
        let trait_defid = trait_ref.def_id;
        if debug {
            println!("trait defid: {:?}", trait_defid);
        }
        let arglen = trait_ref.args.len();
        if arglen == 1 {
            let impl_struct = trait_ref.args.as_slice()[0];
            if debug {
                println!("impl_struct: {:?}", impl_struct.as_type().unwrap().kind());
            }

            match impl_struct.as_type().unwrap().kind() {
                TyKind::Adt(def, _) => {
                    let struct_defid = def.did();

                    if debug {
                        println!("defid: {:?}", struct_defid);
                    }

                    // add struct -> impl block pairing
                    match funcs.struct_impls.get(&struct_defid) {
                        Some(other_impls) => {
                            let mut updated_impls = other_impls.clone();
                            updated_impls.push(def_id);
                            funcs.struct_impls.insert(struct_defid, updated_impls);
                        }
                        None => {
                            funcs.struct_impls.insert(struct_defid, vec![def_id]);
                        }
                    }

                    match funcs.trait_impltors.get(&trait_defid) {
                        Some(vec_impltors) => {
                            //if debug {
                            //    println!("adding impltor to trait-struct map");
                            //}
                            let mut new_impltors = vec_impltors.clone();
                            new_impltors.push(def.did());
                            funcs.trait_impltors.insert(trait_defid, new_impltors);
                        }
                        None => {
                            //if debug {
                            //    println!("trait doesn't exist in map yet, adding now");
                            //}
                            funcs.trait_impltors.insert(trait_defid, vec![def.did()]);
                        }
                    }
                }
                _ => {
                    if debug {
                        println!("other kind")
                    }
                }
            }
        } else if arglen == 0 {
            panic!("arg len 0");
        } else {
            if debug {
                println!("arg len > 1: {:?}", arglen);
            }
        }
    }

    pub fn handle_fn(&self, funcs: &mut FuncMap<'tcx>, def_id: DefId, debug: bool) {
        // TODO for AssocFns, might be useful to have a field describing if it has a
        // default implementation or not

        if debug {
            println!("generics: {:#?}", self.tcx.generics_of(def_id));
        }

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
        let mut arg_types = None;
        if mir_avail {
            let mut arg_types_inner = vec![];
            let body = self.tcx.instance_mir(InstanceKind::Item(def_id));

            for (i, loc) in body.local_decls.clone().into_iter_enumerated() {
                let idx = i.as_usize();
                if idx == 0 || idx > num_args + 1 {
                    continue;
                }
                arg_types_inner.push(loc.ty);
            }

            arg_types = Some(arg_types_inner);

            if debug {
                println!("arg_types: {:?}", arg_types);
                println!("MIR Body: \n{:#?}", body);
            }
        }

        let mut is_intrinsic = false;
        if self.tcx.intrinsic_raw(def_id).is_some() {
            is_intrinsic = true;
        }

        let sig = self.tcx.fn_sig(def_id);
        // FIXME skip_binder() generally incorrect but in this instance the return type
        // is not generic so I think it is ok
        // CORRECTION: binders are not for simple generics but rather https://rustc-dev-guide.rust-lang.org/ty_module/binders.html
        let retty = sig.skip_binder().skip_binder().output();
        // TODO ty has an is_never() method which we can use to not execute panic
        // methods

        let funcval = FuncVal::new(
            def_id,
            is_intrinsic,
            is_method,
            arg_names,
            arg_types,
            Some(retty),
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

    pub fn collect_funcs(&self, funcs: &mut FuncMap<'tcx>, debug: bool) {
        // TODO try past 4
        let num_crates = 4u32;
        //let num_crates = self.tcx.used_crates(()).len();
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

                match def_kind {
                    DefKind::Trait => self.handle_trait(funcs, def_id, debug),
                    DefKind::Struct => self.handle_struct(funcs, def_id, debug),
                    DefKind::Impl { of_trait: true } => {
                        self.handle_trait_impl(funcs, def_id, debug)
                    }
                    DefKind::Fn | DefKind::AssocFn => self.handle_fn(funcs, def_id, debug),
                    _ => {}
                }
            }
        }
    }

    pub fn run(&self, funcs: &mut FuncMap<'tcx>, debug: bool) {
        self.collect_funcs(funcs, debug);
    }
}
