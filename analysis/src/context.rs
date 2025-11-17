use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;
use rustc_data_structures::fx::{FxHashSet as HashSet};

pub struct GlobalContext<'tcx> {
    // The central data structure of the compiler
    pub tcx: TyCtxt<'tcx>,

    // Stores the DefIds that have been already checked, to avoid redundant check
    // TODO when to update: before or after?
    pub checked_def_ids: HashSet<DefId>,
}

impl<'tcx> GlobalContext<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> GlobalContext<'tcx> {
        Self { tcx, checked_def_ids: HashSet::default() }
    }
}

