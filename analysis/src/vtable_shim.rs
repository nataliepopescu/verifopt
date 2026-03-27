use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;

use std::ptr::DynMetadata;

use crate::FuncMap;

#[derive(Debug)]
pub struct VtableMap<Dyn: ?Sized> {
    vtables: HashMap<DefId, DynMetadata<Dyn>>,
}

impl<Dyn> VtableMap<Dyn> {
    pub fn new() -> VtableMap<Dyn> {
        Self {
            vtables: HashMap::default(),
        }
    }
}

pub struct VtableShimPass<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub funcs: &'a FuncMap<'tcx>,
    pub debug: bool,
}

impl<'a, 'tcx, Dyn> VtableShimPass<'a, 'tcx> where Dyn: ?Sized {
    pub fn new(tcx: TyCtxt<'tcx>, funcs: &'a FuncMap<'tcx>, debug: bool) -> VtableShimPass<'a, 'tcx> {
        Self { tcx, funcs, debug }
    }

    pub fn run(&self, _vtables: &mut VtableMap<Dyn>) {
        if self.debug {
            println!("in VTABLE SHIM PASS");
        }
    }
}
