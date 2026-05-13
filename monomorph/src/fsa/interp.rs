//use rustc_data_structures::fx::FxHashSet as HashSet;

use rustc_public::DefId;
use rustc_public::mir::mono::{Instance, InstanceDef, InstanceKind};
use rustc_public::mir::{
    BasicBlock, Body, ConstOperand, LocalDecl, Operand, Place, Terminator, TerminatorKind,
};
use rustc_public::ty::{BoundVariableKind, FnDef, GenericArgs, RigidTy, TyKind};

use log::debug;

use crate::fsa::constraints::ConstraintMap;
use crate::fsa::func_collect::FuncMap;
use crate::fsa::wto::BBDeps;

pub struct InterpPass<'a> {
    pub fmap: &'a FuncMap,
}

impl<'a> InterpPass<'a> {
    pub fn new(fmap: &'a FuncMap) -> InterpPass<'a> {
        Self { fmap }
    }

    pub fn run(&self, cmap: &mut ConstraintMap, start_scope: DefId, instance: Instance) {
        // Track call stack for cur_scope + debugging
        let mut call_stack = vec![(start_scope, instance.def)];

        self.visit_instance(cmap, &mut call_stack, start_scope, instance)
    }

    fn visit_instance(
        &self,
        cmap: &mut ConstraintMap,
        call_stack: &mut Vec<(DefId, InstanceDef)>,
        cur_scope: DefId,
        instance: Instance,
    ) {
        let body = instance.body().unwrap();
        debug!("#############################");
        debug!("###### INTERP-ING NEW BODY for func {:?}", cur_scope);
        debug!("full call_stack: {:?}", call_stack);
        debug!("instance def: {:?}", instance.def);
        debug!("START BODY");
        debug!("{:#?}", body);
        debug!("END BODY");
        debug!("#############################");

        self.visit_body(cmap, call_stack, cur_scope, instance.def, &body);
    }

    fn visit_body(
        &self,
        cmap: &mut ConstraintMap,
        call_stack: &mut Vec<(DefId, InstanceDef)>,
        cur_scope: DefId,
        instance_def: InstanceDef,
        body: &Body,
    ) {
        // If there exists a memoized WTO, use it; otherwise, create it
        let mut bb_deps;
        if let Some(mem_bb_deps) = cmap.wtos.get(&(cur_scope, instance_def)) {
            bb_deps = mem_bb_deps.clone();
            debug!("OLD ordering: {:?}", bb_deps.ordering);
        } else {
            bb_deps = BBDeps::new(body);
            cmap.wtos.insert((cur_scope, instance_def), bb_deps.clone());
            debug!("NEW ordering: {:?}", bb_deps.ordering);
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
                instance_def,
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
        call_stack: &mut Vec<(DefId, InstanceDef)>,
        cur_scope: DefId,
        instance_def: InstanceDef,
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
            instance_def,
            //bb_deps,
            //body_locals,
            bb,
            &data.terminator,
        );

        bb_deps.mark_visited(bb, cur_scope, instance_def);
    }

    fn visit_terminator(
        &self,
        cmap: &mut ConstraintMap,
        call_stack: &mut Vec<(DefId, InstanceDef)>,
        cur_scope: DefId,
        instance_def: InstanceDef,
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
                    instance_def,
                    //body_locals,
                    co,
                    args,
                    destination,
                ),
                _ => todo!("handle indirect function invocations"),
            },
            TerminatorKind::Return => self.interp_return(cmap, call_stack, cur_scope, instance_def),
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
        call_stack: &mut Vec<(DefId, InstanceDef)>,
        cur_scope: DefId,
        instance_def: InstanceDef,
        co: &ConstOperand,
        args: &Vec<Operand>,
        destination: &Place,
    ) {
        match co.const_.ty().kind() {
            TyKind::RigidTy(rigid_ty) => match rigid_ty {
                RigidTy::FnDef(fndef, genargs) => {
                    let instance = Instance::resolve(fndef, &genargs).unwrap();
                    debug!("instance def: {:?}", instance.def);
                    debug!("--- CALLING {:?}", fndef);
                    match instance.kind {
                        InstanceKind::Item => {
                            debug!("regular funccall");
                            if instance.has_body() {
                                call_stack.push((fndef.0, instance.def));
                                self.visit_instance(cmap, call_stack, fndef.0, instance);
                            } else {
                                debug!("no body");
                                self.retty_fallback(fndef);
                            }
                        }
                        InstanceKind::Virtual { .. } => {
                            debug!("virtual funccall");
                            self.interp_virtual_call(cmap, call_stack, cur_scope, &fndef, &genargs);
                        }
                        InstanceKind::Intrinsic => {
                            debug!("intrinsic funccall");
                            self.retty_fallback(fndef);
                        }
                        InstanceKind::Shim => todo!("shim funccall"),
                    }
                }
                other @ _ => todo!("different RigidTy: {:?}", other),
            },
            _ => {}
        }
    }

    fn retty_fallback(&self, fndef: FnDef) {
        let sig = fndef.fn_sig();
        debug!("fn_sig: {:?}", sig);

        if !sig.bound_vars.is_empty() {
            // Might not be safe to just skip binder
            debug!("Bound vars - cannot just skip binder in call resolution");
            for bound_var in sig.bound_vars.iter() {
                match bound_var {
                    BoundVariableKind::Ty(_) => todo!("ty"),
                    BoundVariableKind::Const => todo!("const"),
                    BoundVariableKind::Region(_) => {}
                }
            }
        }

        debug!("output: {:?}", sig.value.output());
    }

    fn interp_virtual_call(
        &self,
        cmap: &mut ConstraintMap,
        call_stack: &mut Vec<(DefId, InstanceDef)>,
        cur_scope: DefId,
        fndef: &FnDef,
        genargs: &GenericArgs,
    ) {
        // Steps:
        // - Get trait that this function is associated with
        //   - fmap.assoc_fn_traits (Map<AssocFn, Trait>)
        // - Get concrete type constraints for trait object
        //   - cmap / fmap.trait_structs (CHA / RTA)
        // - Get every concrete type constraint's impl of this function
        //   - fmap.struct_assoc_fns (Map<(Struct, Trait), FnImpls>)

        // Get trait that this function is associated with
        let trait_defid = match self.fmap.assoc_fn_traits.get(&fndef.0) {
            Some(trait_defid_) => trait_defid_,
            None => panic!("assoc fn {:?} does not point to trait", fndef.0),
        };
        debug!("trait_defid: {:?}", trait_defid);

        // Get concrete type constraints for trait object
        let tyconstraints = match self.fmap.trait_structs.get(&trait_defid) {
            Some(tyconstraints_) => tyconstraints_,
            None => panic!("trait {:?} does not point to any structs", trait_defid),
        };
        debug!("tyconstraints: {:?}", tyconstraints);

        // Get every concrete type constraint's impl of this function
        let mut assoc_fn_impls = Vec::new();
        for &tyconstraint in tyconstraints {
            match self.fmap.struct_assoc_fns.get(&(tyconstraint, fndef.0)) {
                Some(assoc_fn_impl) => assoc_fn_impls.append(&mut assoc_fn_impl.clone()),
                None => panic!(
                    "struct/container ({:?}, {:?}) pair does not point to assoc fn",
                    tyconstraint, fndef.0
                ),
            }
        }
        debug!("assoc_fn_impls: {:?}", assoc_fn_impls);

        self.simulate_static_calls(cmap, call_stack, cur_scope, assoc_fn_impls, genargs);
    }

    fn simulate_static_calls(
        &self,
        cmap: &mut ConstraintMap,
        call_stack: &mut Vec<(DefId, InstanceDef)>,
        cur_scope: DefId,
        assoc_fn_impls: Vec<DefId>,
        genargs: &GenericArgs,
    ) {
        for assoc_fn_impl in assoc_fn_impls {
            let fndef = FnDef(assoc_fn_impl);
            let instance = Instance::resolve(fndef, &genargs).unwrap();
            debug!("instance def: {:?}", instance.def);
            debug!("a converted static instance: {:?}", instance);
            let mut call_stack_clone = call_stack.clone();
            call_stack_clone.push((assoc_fn_impl, instance.def));
            self.visit_instance(cmap, &mut call_stack_clone, assoc_fn_impl, instance);
        }
    }

    fn interp_return(
        &self,
        cmap: &mut ConstraintMap,
        call_stack: &mut Vec<(DefId, InstanceDef)>,
        cur_scope: DefId,
        instance_def: InstanceDef,
    ) {
        debug!("RETURNING from scope {:?} (instance def {:?})...", cur_scope, instance_def);
        debug!("callstack PRE POP: {:?}", call_stack);
        let popped = call_stack.pop();
        if popped.unwrap() != (cur_scope, instance_def) {
            panic!("call stack out of sorts");
        }
        debug!("callstack POST POP: {:?}", call_stack);

        // TODO return actual val
    }
}
