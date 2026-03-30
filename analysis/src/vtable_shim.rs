use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_hir::def_id::{CrateNum, DefId, DefIndex};
use rustc_middle::ty::TyCtxt;

//use std::ptr::DynMetadata;

use crate::FuncMap;

#[derive(Debug)]
pub struct Vtables {
    pub vtable_consts: HashMap<DefId, usize>,
}

impl Vtables {
    pub fn new() -> Vtables {
        Self {
            vtable_consts: HashMap::default(),
        }
    }
}

#[derive(Debug)]
pub struct VtableMap {
    pub traits: HashMap<DefId, Vtables>,
}

impl VtableMap {
    pub fn new() -> VtableMap {
        Self {
            traits: HashMap::default(),
        }
    }
}

pub struct VtableShimPass<'a, 'tcx> {
    pub _tcx: TyCtxt<'tcx>,
    pub _funcs: &'a FuncMap<'tcx>,
    pub debug: bool,
}

impl<'a, 'tcx> VtableShimPass<'a, 'tcx> {
    pub fn new(_tcx: TyCtxt<'tcx>, _funcs: &'a FuncMap<'tcx>, debug: bool) -> VtableShimPass<'a, 'tcx> {
        Self { _tcx, _funcs, debug }
    }

    pub fn run(&self, vtable_consts: &mut VtableMap) {
        if self.debug {
            println!("in VTABLE SHIM PASS");
        }

        // TODO like func_collect, iterate through all DefIds and handle
        // def_kind Struct or Trait

        // Animal Trait
        let a_defid = DefId {
            index: DefIndex::from_u32(4),
            krate: CrateNum::from_u32(0),
        };
        let mut a_inner = Vtables::new();

        // Cat Struct
        let cat_defid = DefId {
            index: DefIndex::from_u32(7),
            krate: CrateNum::from_u32(0),
        };
        let cat_vtid = 9838263505978427528;
        a_inner.vtable_consts.insert(cat_defid, cat_vtid);
        vtable_consts.traits.insert(a_defid, a_inner);

        // AnimalVisitor Trait
        let av_defid = DefId {
            index: DefIndex::from_u32(23),
            krate: CrateNum::from_u32(0),
        };
        let mut av_inner = Vtables::new();

        // Visitor1 Struct
        let vis1_defid = DefId {
            index: DefIndex::from_u32(26),
            krate: CrateNum::from_u32(0),
        };
        let vis1_vtid = 6148914691236517205;
        av_inner.vtable_consts.insert(vis1_defid, vis1_vtid);
        vtable_consts.traits.insert(av_defid, av_inner);

        if self.debug {
            println!("\n### vtables: \n\n{:#?}", vtable_consts.traits);
        }

        //let traits: Vec<_> = self.funcs.trait_to_struct_impls.clone().into_keys().collect();
        //for trait_ in traits {
        //    let structs = self.funcs.trait_to_struct_impls.get(&trait_);

        //    if self.debug {
        //        println!("\n### Structs for Trait {:?}", trait_);
        //        //println!("\n{:#?}", structs);
        //    }

        //    for struct_ in structs {
        //    }
        //}
    }
}
