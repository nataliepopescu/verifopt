use rustc_hir::TyKind;
use rustc_hir::FnRetTy::*;
use rustc_hir::def::Res;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::ty::{InstanceKind, List, TyCtxt};
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

pub struct FuncCollectPass<'tcx> {
    pub tcx: TyCtxt<'tcx>,
}

impl<'tcx> FuncCollectPass<'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
    ) -> FuncCollectPass<'tcx> {
        Self { tcx }
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

    pub fn run(
        &self,
        funcs: &mut FuncMap<'tcx>,
    ) {
        // FIXME try past 4
        //let num_crates = self.tcx.used_crates(()).len();
        let num_crates = 4u32;
        for crate_num in 0u32..num_crates + 1 {
            //println!("\n\ncrate_num: {:?}\n", crate_num);
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
                { break }

                //println!("\nnew def_index");
                //println!("crate_num: {:?}", crate_num);
                //println!("def_index: {:?}", def_index);
                let def_id = DefId {
                    index: def_index.into(),
                    krate: crate_num.into()
                };
                let def_kind = self.tcx.def_kind(def_id);
                //println!("\nforged defid: {:?}", def_id);
                //println!("def_kind: {:?}", def_kind);

                if def_kind == DefKind::Fn || def_kind == DefKind::AssocFn {
                    //println!("fn_sig: {:?}", self.tcx.fn_sig(def_id));
                    let arg_idents = self.tcx.fn_arg_idents(def_id);
                    let num_args = arg_idents.len();
                    let mut is_method = false;
                    if num_args > 0 {
                        if let Some(first) = arg_idents[0] && first.is_special() {
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

                    //println!("fn_arg_idents: {:?}", arg_idents);
                    //let mir_avail = self.tcx.is_mir_available(def_id);
                    //if mir_avail {
                    //    println!("mir available...");
                    //    //println!("Body: \n{:?}", self.tcx.instance_mir(InstanceKind::Item(def_id)));
                    //} else {
                    //    println!("mir NOT available...");
                    //}

                    let funcval = FuncVal::new(def_id, is_method, arg_names);
                    let vec_to_insert: Vec<FuncVal>;
                    match funcs.funcs.get_mut(&def_id) {
                        Some(func_vec) => {
                            func_vec.push(funcval);
                            vec_to_insert = func_vec.to_vec();
                        },
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

/*
    pub fn run_old(&mut self) {
        for loc_def_id in self.tcx.hir_body_owners() {
            let loc_def_kind = self.tcx.def_kind(loc_def_id);
            if loc_def_kind == DefKind::Fn || loc_def_kind == DefKind::AssocFn {
                // get DefId
                let def_id = loc_def_id.to_def_id();

                // get function name
                let item_name = self.tcx.item_name(def_id);

                // get argument types
                let mut arg_types = vec![];
                let hir_id = self.tcx.local_def_id_to_hir_id(loc_def_id);
                let decl = self.tcx.hir_fn_decl_by_hir_id(hir_id).unwrap();
                for input in decl.inputs {
                    let res_opt = self.get_type_res(input.kind);
                    if let Some(res) = res_opt {
                        arg_types.push(res);
                    }
                }

                // get argument names
                let mut arg_names = vec![];
                for i in 1..arg_types.len() + 1 {
                    let arg_place = Place {
                        local: Local::from_usize(i),
                        projection: List::empty(),
                    };
                    arg_names.push(arg_place);
                }

                // is method?
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

                // get return type
                let rettype;
                match decl.output {
                    DefaultReturn(_) => rettype = None,
                    Return(ty) => {
                        rettype = self.get_type_res(ty.kind);
                    }
                }

                let funcval = FuncVal::new(
                    def_id, item_name, is_method, arg_names, arg_types, rettype
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
*/
}

