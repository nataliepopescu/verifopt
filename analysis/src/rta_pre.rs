use rustc_hir::def_id::DefId;
use rustc_middle::mir::Body;
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

    fn visit_body(&self, _inits: &mut RTAMap, cur_scope: &DefId, _body: &'tcx Body<'tcx>) {
        if self.debug {
            println!("## Visiting body for {:?}", cur_scope);
        }

        // TODO what are constructors? and when are they called?
        // what about Box::<Cat>::new()? is that a constructor for Box or Cat?
        // assoc fns also constructors... `new()`, but maybe also others
    }

    fn collect_inits(&self, inits: &mut RTAMap) {
        if self.debug {
            println!(
                "# Iterating through up to {:?} funcs...\n",
                self.funcs.all_funcs.len()
            );
        }
        let mut ctr = 0;

        // flow-insensitive: just iterating through all potential functions
        for (func_defid, _) in self.funcs.all_funcs.iter() {
            if self.tcx.is_mir_available(func_defid) {
                self.visit_body(inits, func_defid, self.tcx.optimized_mir(func_defid));
                ctr += 1;
            }
        }

        if self.debug {
            println!("\n# Iterated through {:?} funcs with MIR available", ctr);
        }
    }

    pub fn run(&self, inits: &mut RTAMap) {
        self.collect_inits(inits);
        if self.debug {
            println!("\n##### DONE w RTACollectPass #####\n");
        }
    }
}
