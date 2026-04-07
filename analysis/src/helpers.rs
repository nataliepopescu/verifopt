use rustc_hir::def_id::DefId;
use rustc_middle::ty::ExistentialPredicate;
use rustc_middle::ty::{GenericArg, GenericArgKind, ParamTy, Ty, TyKind};

use crate::FuncMap;

pub fn is_box(def_id: DefId) -> bool {
    // FIXME does this ever change....
    def_id.krate.as_usize() == 3 && def_id.index.as_usize() == 662
}

pub fn is_fn_trait(def_id: DefId) -> bool {
    // FIXME does this ever change....
    // FnOnce DefId
    def_id.krate.as_usize() == 2 && def_id.index.as_usize() == 4203
    // FnMut DefId
    //|| def_id.krate.as_usize() == 2 && def_id.index.as_usize() == 4203
    // Fn DefId
    || def_id.krate.as_usize() == 2 && def_id.index.as_usize() == 4197
}

//pub fn is_option(def_id: DefId) -> bool {
//    // FIXME does this ever change....
//    def_id.krate.as_usize() == 2 && def_id.index.as_usize() == 49010
//}

fn get_params_from_genarg<'tcx>(genarg: &GenericArg, debug: bool) -> Vec<ParamTy> {
    let mut params = Vec::new();
    match genarg.kind() {
        GenericArgKind::Type(ty) => {
            if let Some(inner_params) = get_params_from_ty(&ty, debug) {
                if debug {
                    println!("ty arg contains: {:?}", inner_params);
                }
                for param in inner_params.iter() {
                    params.push(*param);
                }
            }
        }
        _ => {}
    }
    params
}

pub fn get_params_from_ty<'tcx>(ty: &Ty<'tcx>, debug: bool) -> Option<Vec<ParamTy>> {
    match ty.kind() {
        TyKind::Param(param) => {
            if debug {
                println!("arg has ty param: {:?}", param);
            }
            return Some(vec![*param]);
        }
        TyKind::Slice(ty) => {
            if debug {
                println!("slice arg ty: {:?}", ty);
            }
            return get_params_from_ty(ty, debug);
        }
        TyKind::Ref(_, ty, _) => {
            if debug {
                println!("ref arg ty: {:?}", ty);
            }
            return get_params_from_ty(ty, debug);
        }
        TyKind::Adt(_, genargs) => {
            if debug {
                println!("adt genargs: {:?}", genargs);
            }
            let mut params = Vec::new();
            for genarg in genargs.as_slice().iter() {
                if debug {
                    println!("LOOP genarg: {:?}", genarg);
                }
                params.append(&mut get_params_from_genarg(genarg, debug));
            }
            if params.len() == 0 {
                None
            } else {
                Some(params)
            }
        }
        TyKind::Closure(closure_defid, genargs) => {
            if debug {
                println!("closure defid: {:?}", closure_defid);
            }
            let mut params = Vec::new();
            for genarg in genargs.as_slice().iter() {
                if debug {
                    println!("LOOP genarg: {:?}", genarg);
                }
                params.append(&mut get_params_from_genarg(genarg, debug));
            }
            if params.len() == 0 {
                None
            } else {
                Some(params)
            }
        }
        TyKind::FnPtr(bound_sig, _header) => {
            if debug {
                println!("FNPTR");
                println!("bound_sig: {:?}", bound_sig);
            }
            // TODO
            return None;
        }
        TyKind::Tuple(_) => {
            if debug {
                println!("TUPLE");
            }
            // TODO
            return None;
        }
        _ => {
            if debug {
                println!("diff ty: {:?}", ty.kind());
            }
            return None;
        }
    }
}

pub fn resolve_ty<'tcx>(ty: &Ty<'tcx>, funcs: &FuncMap<'tcx>, debug: bool) -> Vec<DefId> {
    match ty.kind() {
        TyKind::RawPtr(ty, _mut) => resolve_ty(ty, funcs, debug),
        TyKind::Dynamic(list, _) => {
            if debug {
                println!("dyn");
                println!("list: {:?}", list);
            }
            let mut defids = Vec::new();
            for inner in list.iter() {
                if debug {
                    println!("inner: {:?}", inner);
                    println!("inner.skip_binder: {:?}", inner.skip_binder());
                }
                match inner.skip_binder() {
                    ExistentialPredicate::Trait(etraitref) => {
                        if debug {
                            println!("etraitref: {:?}", etraitref);
                            println!("etraitref defid: {:?}", etraitref.def_id);
                        }
                        match funcs.trait_to_struct_impls.get(&etraitref.def_id) {
                            Some(structs) => {
                                for struct_ in structs {
                                    defids.push(*struct_);
                                }
                            }
                            None => panic!("no structs to return: {:?}", etraitref.def_id),
                        }
                    }
                    _ => println!("other"),
                }
            }
            return defids;
        }
        TyKind::Adt(adtdef, genargs) => {
            if debug {
                println!("adt");
            }
            resolve_adt(&adtdef.did(), genargs, funcs, debug)
        }
        _ => {
            if debug {
                println!("other: {:?}", ty.kind());
            }
            //todo!();
            vec![]
        }
    }
}

pub fn resolve_adt<'tcx>(
    adt_defid: &DefId,
    genargs: &[GenericArg<'tcx>],
    funcs: &FuncMap<'tcx>,
    debug: bool,
) -> Vec<DefId> {
    if debug {
        println!("adtdefid: {:?}", adt_defid);
        println!("genargs: {:?}", genargs);
    }
    if is_box(*adt_defid) {
        let mut defids = Vec::new();
        for genarg in genargs {
            defids.append(&mut resolve_genarg(genarg, funcs, debug));
        }
        return defids;
    } else {
        if debug {
            println!(
                "adt is not a box: {:?} \n\t(genargs: {:?})",
                adt_defid, genargs
            );
        }
        return vec![*adt_defid];
    }
}

pub fn resolve_genarg<'tcx>(
    genarg: &GenericArg<'tcx>,
    funcs: &FuncMap<'tcx>,
    debug: bool,
) -> Vec<DefId> {
    match genarg.kind() {
        GenericArgKind::Type(ty) => resolve_ty(&ty, funcs, debug),
        _ => todo!(),
    }
}
