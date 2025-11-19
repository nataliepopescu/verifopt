use rustc_hir::TyKind;
use rustc_hir::FnRetTy::*;
use rustc_hir::def::Res;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::ty::TyCtxt;
use rustc_data_structures::fx::{FxHashMap as HashMap};

use crate::core::{FuncName, FuncVal};

#[derive(Debug, Clone)]
pub struct FuncMap {
    // omitting TraitStructOpt unless useful
    pub funcs: HashMap<FuncName, Vec<FuncVal>>,
}

impl<'a, 'tcx> FuncMap {
    pub fn new() -> Self {
        Self { funcs: HashMap::default() }
    }
}

pub struct FuncCollectPass<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub entry_func: DefId,
    pub func_map: &'a mut FuncMap,
}

impl<'a, 'tcx> FuncCollectPass<'a, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        entry_func: DefId,
        func_map: &'a mut FuncMap,
    ) -> FuncCollectPass<'a, 'tcx> {
        Self { tcx, entry_func, func_map }
    }

    fn get_type_res(&self, tykind: TyKind) -> Option<Res> {
        match tykind {
            rustc_hir::TyKind::Ref(_lifetime, mut_ty) => {
                self.get_type_res(mut_ty.ty.kind)
            },
            rustc_hir::TyKind::Path(qpath) => {
                match qpath {
                    rustc_hir::QPath::Resolved(ty_opt, path) => {
                        Some(path.res)
                    },
                    _ => None,
                }
            },
            _ => None,
        }
    }

    pub fn run(&mut self) {
        for def_id in self.tcx.hir_body_owners() {
            let def_kind = self.tcx.def_kind(def_id);
            if def_kind == DefKind::Fn || def_kind == DefKind::AssocFn {
                let item_name = self.tcx.item_name(def_id.to_def_id());

                let arg_idents = self.tcx.fn_arg_idents(def_id);
                let arg_names = arg_idents.into_iter().map(|x| x.unwrap().name).collect();

                let mut arg_types = vec![];
                let hir_id = self.tcx.local_def_id_to_hir_id(def_id);
                let decl = self.tcx.hir_fn_decl_by_hir_id(hir_id).unwrap();
                for input in decl.inputs {
                    let res_opt = self.get_type_res(input.kind);
                    if let Some(res) = res_opt {
                        arg_types.push(res);
                    }
                }

                let is_method;
                let arg_types_slice = arg_types.as_slice();
                if arg_types_slice.len() > 0 {
                    match arg_types_slice[0] {
                        Res::SelfTyAlias { .. } => is_method = true,
                        _ => is_method = false,
                    }
                } else {
                    is_method = false;
                }

                let rettype;
                match decl.output {
                    DefaultReturn(_) => rettype = None,
                    Return(ty) => {
                        rettype = self.get_type_res(ty.kind);
                    }
                }

                let funcval = FuncVal::new(
                    def_id.into(), item_name, is_method, arg_names, arg_types, rettype
                );

                // TODO would TraitStructOpt be useful?

                let vec_to_insert: Vec<FuncVal>;
                match self.func_map.funcs.get_mut(&item_name) {
                    Some(mut func_vec) => {
                        func_vec.push(funcval);
                        vec_to_insert = func_vec.to_vec();
                    },
                    None => {
                        vec_to_insert = vec![funcval];
                        // TODO handle nested func decls
                    }
                }
                self.func_map.funcs.insert(item_name, vec_to_insert);
            }
        }
    }
}

