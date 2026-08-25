use rustc_public::mir::mono::{Instance, InstanceKind};
use rustc_public::mir::{Body, ConstOperand, Place, Operand, Terminator, TerminatorKind};
use rustc_public::ty::{FnDef, GenericArgs, RigidTy, TyKind};

use log::debug;

pub struct UnsafeFinder;

impl UnsafeFinder {
    pub fn new() -> UnsafeFinder {
        Self {}
    }

    pub fn run(&self, start_instance: Instance) {
        //let mut call_stack = vec![start_instance.clone()];
        let body = start_instance.body().unwrap();
        self.visit_body(&start_instance, &body);
    }

    fn visit_body(
        &self,
        //call_stack: &mut Vec<Instance>,
        cur_scope: &Instance,
        body: &Body,
    ) {
        // naively iterate through all blocks and check all terminators
        // TODO:
        // - caching/memoization
        // - flow-senstivity
        // - resolve call ambiguities
        for bb in &body.blocks {
            self.visit_terminator(
                //call_stack,
                cur_scope,
                &bb.terminator,
            );
        }
    }

    fn visit_terminator(
        &self,
        //call_stack: &mut Vec<Instance>,
        cur_scope: &Instance,
        term: &Terminator,
    ) {
        debug!("TERM KIND: {:?}", &term.kind);
        match &term.kind {
            TerminatorKind::Call {
                func,
                ..
            } => match func {
                Operand::Constant(co) => self.visit_direct_call(co),
                Operand::Copy(place) | Operand::Move(place) => self.visit_indirect_call(cur_scope, place),
                _ => todo!("runtime checks"),
            }
            _ => {}
        }
    }

    fn visit_direct_call(
        &self,
        co: &ConstOperand,
    ) {
        match co.const_.ty().kind() {
            TyKind::RigidTy(rigid_ty) => match rigid_ty {
                RigidTy::FnDef(fndef, genargs) => self.visit_fn_def(
                    fndef,
                    &genargs,
                ),
                RigidTy::FnPtr(_poly_sig) => {
                    todo!();
                    //let sigval = SigVal::new_from_poly(&poly_sig);
                    //self.interp_fn_ptr(
                    //    cur_scope,
                    //    &sigval,
                    //    args,
                    //)
                }
                other @ _ => todo!("different RigidTy: {:?}", other),
            },
            kind @ _ => todo!("funccall const is another kind: {:?}", kind),
        }

    }

    fn visit_fn_def(
        &self,
        fndef: FnDef,
        genargs: &GenericArgs,
    ) {
        let instance = match Instance::resolve(fndef, genargs) {
            Ok(instance_) => instance_,
            Err(_) => todo!("error resolving instance for {:?}", fndef),
        };

        let fetchable_body = matches!(instance.kind, InstanceKind::Item | InstanceKind::Shim)
            && instance.has_body();

        if !fetchable_body {
            panic!("no body to fetch for {:?}", instance);
        }

        self.dispatch_call(&instance);
    }

    fn dispatch_call(
        &self,
        new_scope: &Instance,
    ) {
        match new_scope.kind {
            InstanceKind::Item | InstanceKind::Shim => {
                self.visit_static_call(new_scope);
            }
            InstanceKind::Virtual { .. } => todo!("virtual call"),
            InstanceKind::Intrinsic => todo!("instrinsic call"),
                //self.retty_fallback_from_poly(fndef.fn_sig()),
        }
    }

    fn visit_static_call(
        &self,
        cur_scope: &Instance,
    ) {
        if cur_scope.has_body() {
            let body = cur_scope.body().unwrap();
            self.visit_body(cur_scope, &body);
        } else {
            todo!("called fn has no body");
        }
    }

    fn visit_indirect_call(
        &self,
        _cur_scope: &Instance,
        _place: &Place,
    ) {
        todo!("indirect call");
    }
}
