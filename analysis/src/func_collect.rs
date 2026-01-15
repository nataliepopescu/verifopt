use rustc_hir::{FnDecl, TyKind};
use rustc_hir::FnRetTy::*;
use rustc_hir::def::Res;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::ty::{List, TyCtxt};
use rustc_data_structures::fx::{FxHashMap as HashMap};

use crate::core::FuncVal;

#[derive(Debug, Clone)]
pub struct FuncMap<'tcx> {
    // omitting TraitStructOpt unless useful
    pub funcs: HashMap<DefId, Vec<FuncVal<'tcx>>>,
}

impl<'tcx> FuncMap<'tcx> {
    pub fn new() -> Self {
        Self { funcs: HashMap::default() }
    }
}

pub struct FuncCollectPass<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub func_map: &'a mut FuncMap<'tcx>,
}

impl<'a, 'tcx> FuncCollectPass<'a, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        func_map: &'a mut FuncMap<'tcx>,
    ) -> FuncCollectPass<'a, 'tcx> {
        Self { tcx, func_map }
    }

    fn get_type_res(&self, tykind: TyKind) -> Option<Res> {
        match tykind {
            rustc_hir::TyKind::Ref(_lifetime, mut_ty) => {
                self.get_type_res(mut_ty.ty.kind)
            },
            rustc_hir::TyKind::Path(qpath) => {
                match qpath {
                    rustc_hir::QPath::Resolved(_ty_opt, path) => {
                        Some(path.res)
                    },
                    _ => None,
                }
            },
            _ => None,
        }
    }

    pub(crate) fn get_arg_types(&self, decl: &'tcx FnDecl<'tcx>) -> Vec<Res> {
        let mut arg_types = vec![];
        for input in decl.inputs {
            let res_opt = self.get_type_res(input.kind);
            if let Some(res) = res_opt {
                arg_types.push(res);
            }
        }
        arg_types
    }

    pub(crate) fn get_arg_names(len: usize) -> Vec<Place<'tcx>> {
        let mut arg_names = vec![];
        for i in 1..len + 1 {
            let arg_place = Place {
                local: Local::from_usize(i),
                projection: List::empty(),
            };
            arg_names.push(arg_place);
        }
        arg_names
    }

    pub(crate) fn is_method(len: usize, arg_types_slice: &[Res]) -> bool {
        if len > 0 {
            match arg_types_slice[0] {
                Res::SelfTyAlias { .. } => return true,
                _ => return false,
            }
        } else {
            return false;
        }
    }

    pub(crate) fn get_ret_type(&self, decl: &'tcx FnDecl<'tcx>) -> Option<Res> {
        match decl.output {
            DefaultReturn(_) => return None,
            Return(ty) => return self.get_type_res(ty.kind),
        }
    }

    //pub(crate) fn add_funcval(&'tcx mut self, def_id: DefId, funcval: FuncVal<'tcx>) {
    //    let vec_to_insert: Vec<FuncVal<'tcx>>;
    //    match self.func_map.funcs.get_mut(&def_id) {
    //        Some(func_vec) => {
    //            func_vec.push(funcval);
    //            vec_to_insert = func_vec.to_vec();
    //        },
    //        None => {
    //            vec_to_insert = vec![funcval];
    //            // TODO handle nested func decls
    //        }
    //    }
    //    self.func_map.funcs.insert(def_id, vec_to_insert);
    //}

    pub fn run(&mut self) {
        for loc_def_id in self.tcx.hir_body_owners() {
            let loc_def_kind = self.tcx.def_kind(loc_def_id);
            if loc_def_kind == DefKind::Fn || loc_def_kind == DefKind::AssocFn {
                let def_id = loc_def_id.to_def_id();
                // TODO how to get hir_id from non-local def_id
                // how is this query implemented / why hir_ids only for 
                // local_def_ids?
                let hir_id = self.tcx.local_def_id_to_hir_id(loc_def_id);
                let decl = self.tcx.hir_fn_decl_by_hir_id(hir_id).unwrap();

                let func_name = self.tcx.item_name(def_id);
                let arg_types = self.get_arg_types(decl);
                let arg_len = arg_types.len();
                let arg_names = Self::get_arg_names(arg_len);
                let is_method = Self::is_method(arg_len, arg_types.clone().as_slice());
                let rettype = self.get_ret_type(decl);

                let funcval = FuncVal::new(
                    def_id, func_name, is_method, arg_names, arg_types, rettype
                );

                // TODO would TraitStructOpt be useful?

                let vec_to_insert: Vec<FuncVal<'tcx>>;
                match self.func_map.funcs.get_mut(&def_id) {
                    Some(func_vec) => {
                        func_vec.push(funcval);
                        vec_to_insert = func_vec.to_vec();
                    },
                    None => {
                        vec_to_insert = vec![funcval];
                        // TODO handle nested func decls
                    }
                }
                self.func_map.funcs.insert(def_id, vec_to_insert);
            }
        }
    }
}

