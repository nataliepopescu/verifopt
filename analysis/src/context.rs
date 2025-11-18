use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::ty::TyCtxt;
//use rustc_data_structures::fx::{FxHashSet as HashSet};

use crate::func_collect::FuncMap;
use crate::constraints::ConstraintMap;

pub struct GlobalContext<'a, 'tcx> {
    // The central data structure of the compiler
    pub tcx: TyCtxt<'tcx>,

    // Stores the DefIds that have been already checked, to avoid redundant check
    // TODO when to update: before or after?
    //pub checked_def_ids: HashSet<DefId>,

    // The entry function DefId
    pub entry_func: DefId,

    // The entry function MIR Body
    pub mir_body: &'a Body<'tcx>,

    // Map of function names to their values
    pub functions: FuncMap,

    // Map of function signatures to their names
    //pub signatures: SigMap,

    // Map of all type constraints
    pub cmap: ConstraintMap<'tcx>,
}

impl<'a, 'tcx> GlobalContext<'a, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        entry_func: DefId,
        mir_body: &'a Body<'tcx>,
        //cmap: ConstraintMap<'tcx>,
    ) -> GlobalContext<'a, 'tcx> {
        Self { 
            tcx, 
            //checked_def_ids: HashSet::default(),
            entry_func,
            mir_body,
            functions: FuncMap::new(),
            //signatures: SigMap::new(),
            cmap: ConstraintMap::new(),
        }
    }
}

