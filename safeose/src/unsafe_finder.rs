use rustc_public::mir::mono::{Instance, InstanceKind};
use rustc_public::mir::{Body, ConstOperand, Place, Operand, Terminator, TerminatorKind};
use rustc_public::ty::{FnDef, GenericArgs, RigidTy, Span, TyKind};
use rustc_public::{CrateDef, DefId};

use im::HashMap as ImHashMap;
use log::debug;
use std::cell::RefCell;

pub struct UnsafeFinder {
    pub unsafe_uses: RefCell<ImHashMap<(DefId, usize), (Span, Vec<FnDef>)>>,
}

impl UnsafeFinder {
    pub fn new() -> UnsafeFinder {
        Self {
            unsafe_uses: ImHashMap::new().into(),
        }
    }

    pub fn run(&self, start_instance: Instance) {
        let body = start_instance.body().unwrap();
        self.visit_body(&start_instance, &body);
    }

    fn visit_body(
        &self,
        cur_scope: &Instance,
        body: &Body,
    ) {
        // naively iterate through all blocks and check all terminators for calls to unsafe fns
        // TODO:
        // - caching/memoization
        // - flow-senstivity
        // - resolve call ambiguities
        // - unsafe _blocks_
        for (bb, data) in body.blocks.clone().into_iter().enumerate() {
            self.visit_terminator(
                cur_scope,
                bb,
                &data.terminator,
            );
        }
    }

    fn visit_terminator(
        &self,
        cur_scope: &Instance,
        bb: usize,
        term: &Terminator,
    ) {
        //debug!("TERM KIND: {:?}", &term.kind);
        match &term.kind {
            TerminatorKind::Call {
                func,
                ..
            } => match func {
                Operand::Constant(co) => self.visit_direct_call(&term.span, cur_scope, bb, co),
                Operand::Copy(place) | Operand::Move(place) => self.visit_indirect_call(&term.span, cur_scope, place),
                _ => todo!("runtime checks"),
            }
            _ => {}
        }
    }

    fn visit_direct_call(
        &self,
        term_span: &Span,
        cur_scope: &Instance,
        bb: usize,
        co: &ConstOperand,
    ) {
        match co.const_.ty().kind() {
            TyKind::RigidTy(rigid_ty) => match rigid_ty {
                RigidTy::FnDef(fndef, genargs) => self.visit_fn_def(
                    term_span,
                    cur_scope,
                    bb,
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
        term_span: &Span,
        caller_scope: &Instance,
        bb: usize,
        fndef: FnDef,
        genargs: &GenericArgs,
    ) {
        // check fn sig for unsafety
        match fndef.fn_sig().value.safety {
            rustc_public::mir::Safety::Unsafe => {
                // add unsafe fn to our list
                let mut u = self.unsafe_uses.borrow_mut();
                let key = (caller_scope.def.def_id(), bb);
                let entry = u.entry(key).or_insert((*term_span, Vec::new()));
                if !entry.1.contains(&fndef) {
                    entry.1.push(fndef);
                }

                debug!("FOUND UNSAFE FN: {:?} (called from {:?})", fndef, key);
            }
            _ => {}
        }

        // visit body
        let instance = match Instance::resolve(fndef, genargs) {
            Ok(instance_) => instance_,
            Err(_) => todo!("error resolving instance for {:?}", fndef),
        };

        let fetchable_body = matches!(instance.kind, InstanceKind::Item | InstanceKind::Shim)
            && instance.has_body();

        if !fetchable_body {
            panic!("no body to fetch for {:?}, need fallback", instance);
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
        _term_span: &Span,
        _cur_scope: &Instance,
        _place: &Place,
    ) {
        todo!("indirect call");
    }
}
