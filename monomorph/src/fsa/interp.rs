//use rustc_data_structures::fx::FxHashSet as HashSet;

use rustc_public::mir::mono::Instance;
use rustc_public::mir::{BasicBlock, Body, LocalDecl, Operand, Terminator, TerminatorKind};
use rustc_public::ty::TyKind;
use rustc_public::ty::RigidTy;
use rustc_public::DefId;

use log::debug;

use crate::fsa::constraints::ConstraintMap;
use crate::fsa::wto::BBDeps;

pub struct InterpPass;

impl InterpPass {
    pub fn new() -> InterpPass {
        Self {}
    }

    pub fn run(
        &self,
        cmap: &mut ConstraintMap,
        cur_scope: DefId,
        instance: Instance,
    ) {
        // Track call stack for debugging
        let mut call_stack = vec![cur_scope];

        self.visit_instance(cmap, &mut call_stack, cur_scope, instance)
    }

    fn visit_instance(
        &self,
        cmap: &mut ConstraintMap,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        instance: Instance,
    ) {
        let body = instance.body().unwrap();
        self.visit_body(cmap, call_stack, cur_scope, &body);
    }

    fn visit_body(
        &self,
        cmap: &mut ConstraintMap,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        body: &Body,
    ) {
        debug!("#############################");
        debug!("\n###### INTERP-ING NEW BODY for func {:?}\n", cur_scope);
        debug!("call_stack: {:?}", call_stack);
        debug!("#############################");

        // If there exists a memoized WTO, use it; otherwise, create it
        let mut bb_deps;
        if let Some(mem_bb_deps) = cmap.wtos.get(&cur_scope) {
            bb_deps = mem_bb_deps.clone();
        } else {
            bb_deps = BBDeps::new(body);
            cmap.wtos.insert(cur_scope, bb_deps.clone());
        }

        // Loop through basic blocks in WTO
        /*
        let mut last_res = None;
        loop {
            if bb_deps.ordering.is_empty() {
                break;
            }
            let bb = bb_deps.ordering.remove(0);
            debug!("\n--NEW BB: {:?}", bb);
            let data = body.basic_blocks.get(bb).unwrap();
            last_res = self.visit_basic_block_data(
                cmap,
                call_stack,
                cur_scope,
                &mut bb_deps,
                body.local_decls.as_slice(),
                &bb,
                data,
            ); //?;
        }
        */

        //Ok(last_res)
    }

    fn visit_basic_block_data(
        &self,
        _cmap: &mut ConstraintMap,
        _call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        _bb_deps: &mut BBDeps,
        _body_locals: &[LocalDecl],
        bb: usize,
        _data: &BasicBlock,
    ) {
        debug!("#############");
        debug!("# visiting BASICBLOCK {:?} for DefId {:?}", bb, cur_scope);
        debug!("#############");
    }

    fn visit_terminator(
        &self,
        _cmap: &mut ConstraintMap,
        term: &Terminator
    ) {
        match &term.kind {
            TerminatorKind::Call {
                func,
                args: _,
                destination: _,
                ..
            } => match func {
                Operand::Constant(co) => match co.const_.ty().kind() {
                    TyKind::RigidTy(rigid_ty) => match rigid_ty {
                        RigidTy::FnDef(defid, genargs) => {
                            let instance = Instance::resolve(defid, &genargs).unwrap();
                            println!("\n--- CALLING {:?}", defid);
                            println!("  - Body: {:#?}", instance.body());
                        }
                        _ => {}
                    }
                    _ => {}
                }
                _ => {}
            }
            _ => {}
        }
    }
}
