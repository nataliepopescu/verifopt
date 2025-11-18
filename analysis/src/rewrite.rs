use rustc_middle::mir::*;
use rustc_middle::mir::visit::MutVisitor;
use rustc_middle::ty::TyCtxt;

use crate::context::GlobalContext;
use crate::constraints::ConstraintMap;

pub struct RewritePass<'a, 'tcx> {
   pub context: &'a GlobalContext<'a, 'tcx>,
   pub cmap: ConstraintMap<'tcx>,
}

impl<'a, 'tcx> RewritePass<'a, 'tcx> {
    pub fn new(
        context: &'a GlobalContext<'a, 'tcx>, 
        cmap: ConstraintMap<'tcx>
    ) -> RewritePass<'a, 'tcx> {
        Self { context, cmap }
    }

    pub fn run(&self, _body: &mut Body<'tcx>) {
    }
}

impl<'a, 'tcx> MutVisitor<'tcx> for RewritePass<'a, 'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.context.tcx
    }
}
