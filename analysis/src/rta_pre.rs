use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;

use crate::FuncMap;
use crate::core::DebugPass;

pub struct RTAMap {
    pub struct_inits: Vec<DefId>,
}

impl RTAMap {
    pub fn new() -> RTAMap {
        Self {
            struct_inits: Vec::<DefId>::new(),
        }
    }
}

pub struct RTACollectPass<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub funcs: &'a FuncMap<'tcx>,
    pub debug: bool,
}

impl<'a, 'tcx> RTACollectPass<'a, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        funcs: &'a FuncMap<'tcx>,
        which_debug: DebugPass,
    ) -> RTACollectPass<'a, 'tcx> {
        let mut debug = false;
        if which_debug == DebugPass::RTA {
            debug = true;
        }
        Self { tcx, funcs, debug }
    }

    fn collect_inits(&self, inits: &mut RTAMap) {}

    pub fn run(&self, inits: &mut RTAMap) {
        self.collect_inits(inits);
    }
}
