//use rustc_data_structures::fx::FxHashSet as HashSet;

use rustc_public::DefId;
use rustc_public::mir::mono::Instance;
use rustc_public::mir::{
    BasicBlock, Body, ConstOperand, LocalDecl, Operand, Place, Terminator, TerminatorKind,
};
use rustc_public::ty::RigidTy;
use rustc_public::ty::TyKind;

use log::debug;

use crate::fsa::constraints::ConstraintMap;
use crate::fsa::wto::BBDeps;

pub struct InterpPass;

impl InterpPass {
    pub fn new() -> InterpPass {
        Self {}
    }

    pub fn run(&self, cmap: &mut ConstraintMap, cur_scope: DefId, instance: Instance) {
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
        debug!("###### INTERP-ING NEW BODY for func {:?}", cur_scope);
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
        //let mut last_res = None;
        loop {
            if bb_deps.ordering.is_empty() {
                break;
            }
            let bb = bb_deps.ordering.remove(0);
            debug!("--NEW BB: {:?}", bb);
            let data = body.blocks.get(bb).unwrap();
            //last_res =
            self.visit_basic_block(
                cmap,
                call_stack,
                cur_scope,
                &mut bb_deps,
                //body.local_decls(), //.as_slice(),
                bb,
                data,
            ); //?;
        }

        //Ok(last_res)
    }

    fn visit_basic_block(
        &self,
        cmap: &mut ConstraintMap,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        bb_deps: &mut BBDeps,
        //_body_locals: &[LocalDecl],
        bb: usize,
        data: &BasicBlock,
    ) {
        debug!("#############");
        debug!("# visiting BASICBLOCK {:?} for {:?}", bb, cur_scope);
        debug!("#############");

        for stmt in data.statements.iter() {
            debug!("# visiting STATEMENT in BB{:?} for {:?}", bb, cur_scope);
            //debug!("{:?}", stmt);
            //self.visit_statement
        }

        self.visit_terminator(
            cmap,
            call_stack,
            cur_scope,
            //bb_deps,
            //body_locals,
            bb,
            &data.terminator,
        );

        bb_deps.mark_visited(bb, cur_scope);
    }

    fn visit_terminator(
        &self,
        cmap: &mut ConstraintMap,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        bb: usize,
        term: &Terminator,
    ) {
        match &term.kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => match func {
                Operand::Constant(co) => self.interp_direct_call(
                    cmap,
                    call_stack,
                    cur_scope,
                    //body_locals,
                    co,
                    args,
                    destination,
                ),
                _ => todo!("handle indirect function invocations"),
            },
            //TerminatorKind::Return => self.interp_return(cmap, call_stack, cur_scope),
            //TerminatorKind::SwitchInt { discr, targets } => {
            //    self.interp_switchint(cmap, bb, bb_deps, cur_scope, discr, targets)
            //}
            //TerminatorKind::TailCall { .. } => todo!("impl tail calls"),
            //_ => Ok(None),
            _ => {}
        }
    }

    fn interp_direct_call(
        &self,
        cmap: &mut ConstraintMap,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        co: &ConstOperand,
        args: &Vec<Operand>,
        destination: &Place,
    ) {
        match co.const_.ty().kind() {
            TyKind::RigidTy(rigid_ty) => match rigid_ty {
                RigidTy::FnDef(defid, genargs) => {
                    let instance = Instance::resolve(defid, &genargs).unwrap();
                    debug!("--- CALLING {:?}", defid);
                    debug!("START BODY");
                    debug!("{:?}", instance.body());
                    debug!("END BODY");
                }
                _ => {}
            },
            _ => {}
        }
    }
}
