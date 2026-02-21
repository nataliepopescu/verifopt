//use rustc_hir::TyKind;
//use rustc_hir::FnRetTy::*;
use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_hir::def::DefKind;
//use rustc_hir::def::Res;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::ty::{
    GenericArg, GenericArgKind, Generics, InstanceKind, List, ParamTy, Ty, TyCtxt, TyKind,
};

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
    // trait -> structs
    pub trait_impltors: HashMap<DefId, Vec<DefId>>,
    // struct defid -> generics
    pub struct_generics: HashMap<DefId, Generics>,
    // struct -> [impl blocks]
    // FIXME this only covers explicit trait implementations;
    // missing auto/blanket implementations
    pub struct_impls: HashMap<DefId, Vec<DefId>>,
    // impl blocks -> [impl fns/methods]
    pub impl_blocks_to_impls: HashMap<DefId, Vec<DefId>>,
    // ty -> defid map
    // TODO currently just adding struct defs
    //pub ty_to_defids: HashMap<Ty<'tcx>, DefId>,
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
            //ty_to_defids: HashMap::default(),
        }
    }
}

pub struct FuncCollectPass<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub debug: bool,
}

impl<'tcx> FuncCollectPass<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, debug: bool) -> FuncCollectPass<'tcx> {
        Self { tcx, debug }
    }

    pub fn handle_struct(&self, funcs: &mut FuncMap<'tcx>, def_id: DefId) {
        if self.debug {
            println!("generics: {:#?}", self.tcx.generics_of(def_id));
        }
        funcs
            .struct_generics
            .insert(def_id, self.tcx.generics_of(def_id).clone());
    }

    pub fn handle_trait(&self, funcs: &mut FuncMap<'tcx>, def_id: DefId) {
        if self.debug {
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
            //if self.debug {
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

    pub fn store_struct_impl(
        &self,
        funcs: &mut FuncMap<'tcx>,
        def_id: DefId,
        trait_defid: DefId,
        impl_struct: GenericArg<'tcx>,
    ) {
        match impl_struct.as_type().unwrap().kind() {
            TyKind::Adt(def, _) => {
                let struct_defid = def.did();

                if self.debug {
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
                        //if self.debug {
                        //    println!("adding impltor to trait-struct map");
                        //}
                        let mut new_impltors = vec_impltors.clone();
                        new_impltors.push(def.did());
                        funcs.trait_impltors.insert(trait_defid, new_impltors);
                    }
                    None => {
                        //if self.debug {
                        //    println!("trait doesn't exist in map yet, adding now");
                        //}
                        funcs.trait_impltors.insert(trait_defid, vec![def.did()]);
                    }
                }
            }
            _ => {
                if self.debug {
                    println!("other kind")
                }
            }
        }
    }

    pub fn handle_trait_impl(&self, funcs: &mut FuncMap<'tcx>, def_id: DefId) {
        // TODO might be useful once we encounter default trait implementations, to see
        // exactly _which_ functions are being implemented/overriden
        //println!("assoc_items: {:#?}", self.tcx.associated_items(def_id));

        let trait_ref = self.tcx.impl_trait_header(def_id).trait_ref.skip_binder();
        let trait_defid = trait_ref.def_id;
        if self.debug {
            println!("trait defid: {:?}", trait_defid);
            println!("args: {:#?}", trait_ref.args);
        }
        let arglen = trait_ref.args.len();
        if arglen >= 1 {
            let impl_struct = trait_ref.args.as_slice()[0];
            if self.debug {
                println!("arg len: {:?}", arglen);
                println!("impl_struct: {:?}", impl_struct.as_type().unwrap().kind());
            }
            self.store_struct_impl(funcs, def_id, trait_defid, impl_struct);
        } else if arglen == 0 {
            panic!("arg len 0");
        }
    }

    fn get_params(&self, ty: &Ty<'tcx>) -> Option<Vec<ParamTy>> {
        match ty.kind() {
            TyKind::Param(param) => {
                if self.debug {
                    println!("arg has ty param: {:?}", param);
                }
                return Some(vec![*param]);
            }
            TyKind::Slice(ty) => {
                if self.debug {
                    println!("slice arg ty: {:?}", ty);
                }
                return self.get_params(ty);
            }
            TyKind::Ref(_, ty, _) => {
                if self.debug {
                    println!("ref arg ty: {:?}", ty);
                }
                return self.get_params(ty);
            }
            TyKind::Adt(_, genargs) => {
                if self.debug {
                    println!("adt genargs: {:?}", genargs);
                }
                let mut params = vec![];
                for genarg in genargs.as_slice().iter() {
                    if self.debug {
                        println!("LOOP genarg: {:?}", genarg);
                    }
                    match genarg.kind() {
                        GenericArgKind::Lifetime(_) => {
                            if self.debug {
                                println!("skipping lifetime arg...");
                            }
                            continue;
                        }
                        GenericArgKind::Type(ty) => {
                            if let Some(inner_params) = self.get_params(&ty) {
                                if self.debug {
                                    println!("ty arg contains: {:?}", inner_params);
                                }
                                for param in inner_params.iter() {
                                    params.push(*param);
                                }
                            }
                        }
                        GenericArgKind::Const(_) => {
                            if self.debug {
                                println!("skipping const arg...");
                            }
                            continue;
                        }
                    }
                }
                if params.len() == 0 {
                    None
                } else {
                    Some(params)
                }
            }
            _ => {
                if self.debug {
                    println!("different ty: {:?}", ty.kind());
                }
                return None;
            }
        }
    }

    pub fn handle_fn(&self, funcs: &mut FuncMap<'tcx>, def_id: DefId) {
        // TODO for AssocFns, might be useful to have a field describing if it has a
        // default implementation or not

        let impl_of_assoc = self.tcx.impl_of_assoc(def_id);
        let generics_of = self.tcx.generics_of(def_id);

        if self.debug {
            println!("impl of assoc: {:?}", impl_of_assoc);
            println!("generics: {:#?}", generics_of);
        }

        if let Some(impl_defid) = impl_of_assoc {
            match funcs.impl_blocks_to_impls.get(&impl_defid) {
                Some(existing_assoc) => {
                    if self.debug {
                        println!("existing assoc fns in impl block: {:?}", existing_assoc);
                    }
                    let mut updated_assoc = existing_assoc.clone();
                    updated_assoc.push(def_id);
                    funcs.impl_blocks_to_impls.insert(impl_defid, updated_assoc);
                }
                None => {
                    if self.debug {
                        println!("first assoc fn in impl block");
                    }
                    funcs.impl_blocks_to_impls.insert(impl_defid, vec![def_id]);
                }
            }
        }

        let arg_idents = self.tcx.fn_arg_idents(def_id);
        let num_args = arg_idents.len();

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
        let mut arg_generics = None;
        if mir_avail {
            let mut arg_types_inner = vec![];
            let mut arg_generics_inner = vec![];
            let body = self.tcx.instance_mir(InstanceKind::Item(def_id));

            for (i, loc) in body.local_decls.clone().into_iter_enumerated() {
                let idx = i.as_usize();
                if idx == 0 || idx > num_args {
                    continue;
                }
                arg_types_inner.push(loc.ty);
                if self.debug {
                    println!("idx: {:?}", idx);
                    println!("local (arg) type: {:?}", loc.ty);
                }

                match self.get_params(&loc.ty) {
                    Some(param_vec) => {
                        for param in param_vec {
                            arg_generics_inner.push(param);
                        }
                    }
                    _ => {}
                }
            }

            arg_types = Some(arg_types_inner);
            if arg_generics_inner.len() > 0 {
                arg_generics = Some(arg_generics_inner);
            }

            if self.debug {
                println!("arg_count: {:?}", body.arg_count);
                println!("arg_types: {:?}", arg_types);
                println!("arg_generics: {:?}", arg_generics);
                println!("----Start MIR Body----");

                let locs = &body.local_decls;
                let bbs = &body.basic_blocks;

                println!("num LocalDecls: {:?}", locs.len());
                println!("{{");
                for i in 0..locs.len() {
                    println!("-local{:?}", i);
                    println!("{:#?}", locs[Local::from_usize(i)]);
                }
                println!("}}");

                println!("num BasicBlocks: {:?}", bbs.len());
                println!("{{");
                for i in 0..bbs.len() {
                    println!("-bb{:?}", i);
                    println!("{:#?}", bbs[BasicBlock::from_usize(i)]);
                }
                println!("}}");

                println!("----End MIR Body----");
            }
        } else {
            if self.debug {
                println!("no mir :(");
            }
        }

        let mut self_arg = None;
        if num_args > 0 && generics_of.has_self {
            self_arg = Some(arg_names[0]);
            if self.debug {
                println!("has self!");
                println!("self type: {:?}", self_arg);
            }
        }

        let mut is_intrinsic = false;
        if self.tcx.intrinsic_raw(def_id).is_some() {
            is_intrinsic = true;
            if self.debug {
                println!("is intrinsic");
            }
        } else {
            if self.debug {
                println!("not intrinsic");
            }
        }

        let sig = self.tcx.fn_sig(def_id);
        // FIXME skip_binder() generally incorrect but in this instance the return type
        // is not generic so maybe fine (for now)
        // CORRECTION: binders are not for simple generics but rather lifetime generics
        // https://rustc-dev-guide.rust-lang.org/ty_module/binders.html
        // TODO use bound_vars field in inner Binder to determine if safe
        // (outer == EarlyBinder, no such field)
        let rettype = sig.skip_binder().skip_binder().output();
        if self.debug {
            println!("sig: {:?}", sig);
            println!("rettype: {:?}", rettype);
            //println!("ret impl trait?: {:?}", self.tcx.collect_return_position_impl_trait_in_trait_tys(def_id));
        }
        let mut ret_did = None;
        let mut ret_generic = None;
        match rettype.kind() {
            TyKind::Adt(def, adt_genargs) => {
                if self.debug {
                    println!("adt_def def_id: {:?}", def.did());
                    println!("adt genargs: {:?}", adt_genargs);
                }
                ret_did = Some(def.did());
                if adt_genargs.len() > 0 {
                    match adt_genargs[0].kind() {
                        //GenericArgKind::Const(c) => match c.kind() {
                        //    ConstKind::Param(param_const) => {
                        //        if self.debug {
                        //            println!("rettype has const param: {:?}", param_const);
                        //        }
                        //        ret_generic =
                        //            Some(ParamTy::new(param_const.index, param_const.name));
                        //    }
                        //    _ => {
                        //        if self.debug {
                        //            println!("not a param: {:?}", c.kind());
                        //        }
                        //    }
                        //},
                        GenericArgKind::Type(ty) => match ty.kind() {
                            TyKind::Param(param) => {
                                if self.debug {
                                    println!("rettype has ty param: {:?}", param);
                                }
                                ret_generic = Some(*param);
                            }
                            _ => {}
                        },
                        _ => {
                            if self.debug {
                                println!("genarg is not a ty: {:?}", adt_genargs[0].kind());
                            }
                        }
                    }
                }
            }
            TyKind::Param(param) => {
                if self.debug {
                    println!("rettype == param: {:?}", param);
                }
                ret_generic = Some(*param);
            }
            _ => {}
        }
        // TODO ty has an is_never() method which we can use to not execute panic methods

        let funcval = FuncVal::new(
            def_id,
            is_intrinsic,
            self_arg,
            arg_names,
            arg_types,
            arg_generics,
            Some(rettype),
            ret_did,
            ret_generic,
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

    pub fn collect_funcs(&self, funcs: &mut FuncMap<'tcx>) {
        // TODO try past 4
        //let num_crates = 4u32;
        let num_crates = self.tcx.used_crates(()).len() as u32;
        if self.debug {
            println!("num_crates: {:?}", num_crates);
        }
        for crate_num in 0u32..num_crates + 1 {
            if self.debug {
                println!("\n\ncrate_num: {:?}\n", crate_num);
            }
            for def_index in 0..u32::MAX {
                // simple (no bmark): limit = 25
                // simple (bmark): limit = 29
                // one_variant (no bmark): limit = 21
                // one_variant (bmark): limit = 26
                // two_variants (no bmark): limit = 26
                if crate_num == 0 && def_index >= 26
                    || crate_num == 1 && def_index >= 19549
                    || crate_num == 2 && def_index >= 78916
                    || crate_num == 3 && def_index >= 12636
                    || crate_num == 4 && def_index >= 4970
                    || crate_num == 5 && def_index >= 12217
                    || crate_num == 6 && def_index >= 3
                    || crate_num == 7 && def_index >= 94
                    || crate_num == 8 && def_index >= 513
                    || crate_num == 9 && def_index >= 71
                    || crate_num == 10 && def_index >= 3492
                    || crate_num == 11 && def_index >= 3
                    || crate_num == 12 && def_index >= 351
                    || crate_num == 13 && def_index >= 317
                    || crate_num == 14 && def_index >= 4
                    || crate_num == 15 && def_index >= 636
                    || crate_num == 16 && def_index >= 11666
                    || crate_num == 17 && def_index >= 21753
                    || crate_num == 18 && def_index >= 2174
                    || crate_num == 19 && def_index >= 27
                {
                    break;
                }

                let def_id = DefId {
                    index: def_index.into(),
                    krate: crate_num.into(),
                };
                let def_kind = self.tcx.def_kind(def_id);

                if self.debug {
                    println!("\nnew def_index");
                    println!("crate_num: {:?}", crate_num);
                    println!("def_index: {:?}", def_index);
                    println!("forged defid: {:?}", def_id);
                    println!("def_kind: {:?}", def_kind);
                }

                match def_kind {
                    DefKind::Trait => self.handle_trait(funcs, def_id),
                    DefKind::Struct => self.handle_struct(funcs, def_id),
                    DefKind::Impl { of_trait: true } => self.handle_trait_impl(funcs, def_id),
                    DefKind::Fn | DefKind::AssocFn => self.handle_fn(funcs, def_id),
                    _ => {}
                }
            }
        }
    }

    pub fn run(&self, funcs: &mut FuncMap<'tcx>) {
        self.collect_funcs(funcs);
    }
}
