use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::ty::TyCtxt;
use rustc_data_structures::fx::{FxHashMap as HashMap};

use crate::core::{FuncName, FuncVal};

#[derive(Debug, Clone)]
pub struct FuncMap {
    // omitting TraitStructOpt unless needed
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
    pub funcs: &'a mut FuncMap,
}

impl<'a, 'tcx> FuncCollectPass<'a, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        entry_func: DefId,
        funcs: &'a mut FuncMap,
    ) -> FuncCollectPass<'a, 'tcx> {
        Self { tcx, entry_func, funcs }
    }

    pub fn run(&mut self, _body: &Body<'tcx>) {
    }
}

