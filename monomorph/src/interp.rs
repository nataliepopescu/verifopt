//use rustc_data_structures::fx::FxHashSet as HashSet;

use rustc_public::DefId;
use rustc_public::mir::mono::{Instance, InstanceDef, InstanceKind};
use rustc_public::mir::{
    BasicBlock, Body, ConstOperand, LocalDecl, Operand, Place, Statement, StatementKind,
    Terminator, TerminatorKind,
};
use rustc_public::ty::{BoundVariableKind, FnDef, GenericArgs, RigidTy, TyKind};

use log::debug;

use crate::constraints::{Constraints, InterpStore, MapKey, MapValue, ScopeId}; //, VerifoptRval};
use crate::convert::RvalConverter;
use crate::error::Error;
use crate::trait_collect::TraitStore;
use crate::wto::BBDeps;

pub struct InterpPass<'a> {
    pub tstore: &'a TraitStore,
    pub converter: RvalConverter<'a>,
}

impl<'a> InterpPass<'a> {
    pub fn new(tstore: &'a TraitStore) -> InterpPass<'a> {
        Self {
            tstore,
            converter: RvalConverter::new(tstore),
        }
    }

    pub fn run(
        &self,
        istore: &mut InterpStore,
        start_scope: DefId,
        instance: Instance,
    ) -> Result<Option<Constraints>, Error> {
        let scope_id = (start_scope, instance.def);
        let mut call_stack = vec![scope_id];

        // Initialize InterpStore with entry_fn's substore
        let entry_fn_istore = InterpStore::new();
        istore.cmap.insert(
            MapKey::ScopeId(scope_id),
            Box::new(MapValue::Store(vec![entry_fn_istore])),
        );

        self.visit_instance(istore, &mut call_stack, scope_id, instance)
    }

    fn visit_instance(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<ScopeId>,
        cur_scope: ScopeId,
        instance: Instance,
    ) -> Result<Option<Constraints>, Error> {
        let body = instance.body().unwrap();
        debug!("#############################");
        debug!("###### INTERP-ING NEW BODY for func {:?}", cur_scope);
        debug!("full call_stack: {:#?}", call_stack);
        debug!("START BODY");
        debug!("{:#?}", body);
        debug!("END BODY");
        debug!("#############################");

        self.visit_body(istore, call_stack, cur_scope, &body)
    }

    fn visit_body(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<ScopeId>,
        cur_scope: ScopeId,
        body: &Body,
    ) -> Result<Option<Constraints>, Error> {
        // If there exists a memoized WTO, use it; otherwise, create it
        let mut bb_deps;
        if let Some(mem_bb_deps) = istore.wtos.get(&cur_scope) {
            bb_deps = mem_bb_deps.clone();
            debug!("OLD ordering: {:?}", bb_deps.ordering);
        } else {
            bb_deps = BBDeps::new(body);
            istore.wtos.insert(cur_scope, bb_deps.clone());
            debug!("NEW ordering: {:?}", bb_deps.ordering);
        }

        // Loop through basic blocks in WTO
        let mut last_res = None;
        loop {
            if bb_deps.ordering.is_empty() {
                break;
            }
            let bb = bb_deps.ordering.remove(0);
            debug!("--NEW BB: {:?}", bb);
            let data = body.blocks.get(bb).unwrap();
            last_res = self.visit_basic_block(
                istore,
                call_stack,
                cur_scope,
                &mut bb_deps,
                body.locals(),
                bb,
                data,
            )?;
        }

        Ok(last_res)
    }

    fn visit_basic_block(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<ScopeId>,
        cur_scope: ScopeId,
        bb_deps: &mut BBDeps,
        body_locals: &[LocalDecl],
        bb: usize,
        data: &BasicBlock,
    ) -> Result<Option<Constraints>, Error> {
        debug!("#############");
        debug!("# visiting BASICBLOCK {:?} for {:?}", bb, cur_scope);
        debug!("#############");

        for (i, stmt) in data.statements.iter().enumerate() {
            debug!(
                "# visiting STATEMENT {:?} in BB{:?} for {:?}",
                i, bb, cur_scope
            );
            self.visit_statement(istore, call_stack, cur_scope, body_locals, stmt);
        }

        debug!("# visiting TERMINATOR in BB{:?} for {:?}", bb, cur_scope);
        let res = self.visit_terminator(
            istore,
            call_stack,
            cur_scope,
            //bb_deps,
            //body_locals,
            bb,
            &data.terminator,
        )?;

        bb_deps.mark_visited(bb, cur_scope);

        Ok(res)
    }

    fn visit_statement(
        &self,
        istore: &mut InterpStore,
        _call_stack: &mut Vec<ScopeId>,
        cur_scope: ScopeId,
        _body_locals: &[LocalDecl],
        stmt: &Statement,
    ) {
        match &stmt.kind {
            // Only interp assignments to track type constraint changes
            StatementKind::Assign(place, rvalue) => {
                debug!("start assignment!");
                debug!("cur_scope: {:?}", cur_scope);
                debug!("place: {:?}", place);
                debug!("rval: {:?}", rvalue);

                // convert Rvalue to VerifoptRval
                let constraints = self.converter.convert(istore, cur_scope, rvalue);
                debug!("CONVERTED CONSTRAINTS: {:?}", constraints);

                // add resolved constraints to istore
                istore.scoped_update(
                    cur_scope,
                    MapKey::Place(place.clone()),
                    Box::new(MapValue::Constraints(constraints)),
                );
            }
            _ => {}
        }
    }

    fn visit_terminator(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<ScopeId>,
        cur_scope: ScopeId,
        _bb: usize,
        term: &Terminator,
    ) -> Result<Option<Constraints>, Error> {
        match &term.kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => match func {
                Operand::Constant(co) => self.interp_direct_call(
                    istore,
                    call_stack,
                    cur_scope,
                    //body_locals,
                    co,
                    args,
                    destination,
                ),
                _ => todo!("handle indirect function invocations"),
            },
            TerminatorKind::Return => self.interp_return(istore, call_stack, cur_scope),
            //TerminatorKind::SwitchInt { discr, targets } => {
            //    self.interp_switchint(istore, bb, bb_deps, cur_scope, discr, targets)
            //}
            //TerminatorKind::TailCall { .. } => todo!("impl tail calls"),
            _ => Ok(None),
        }
    }

    fn interp_direct_call(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<ScopeId>,
        cur_scope: ScopeId,
        co: &ConstOperand,
        _args: &Vec<Operand>,
        _destination: &Place,
    ) -> Result<Option<Constraints>, Error> {
        match co.const_.ty().kind() {
            TyKind::RigidTy(rigid_ty) => match rigid_ty {
                RigidTy::FnDef(fndef, genargs) => {
                    let instance = Instance::resolve(fndef, &genargs).unwrap();
                    debug!("instance def: {:?}", instance.def);
                    debug!("--- CALLING {:?}", fndef);
                    match instance.kind {
                        InstanceKind::Item => {
                            debug!("regular static funccall");
                            self.interp_static_call(istore, call_stack, instance, fndef)
                        }
                        InstanceKind::Virtual { .. } => {
                            debug!("virtual funccall");
                            self.interp_virtual_call(
                                istore, call_stack, cur_scope, &fndef, &genargs,
                            )
                        }
                        InstanceKind::Intrinsic => {
                            debug!("intrinsic funccall");
                            self.retty_fallback(fndef)
                        }
                        InstanceKind::Shim => todo!("shim funccall"),
                    }
                }
                other @ _ => todo!("different RigidTy: {:?}", other),
            },
            kind @ _ => todo!("funccall const is another kind: {:?}", kind),
        }
    }

    fn interp_static_call(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<(DefId, InstanceDef)>,
        instance: Instance,
        fndef: FnDef,
    ) -> Result<Option<Constraints>, Error> {
        if instance.has_body() {
            let mut istore_clone = istore.clone();
            let scope_id = (fndef.0, instance.def);
            call_stack.push(scope_id);
            self.visit_instance(&mut istore_clone, call_stack, scope_id, instance)
        } else {
            debug!("no body");
            self.retty_fallback(fndef)
        }
    }

    fn retty_fallback(&self, fndef: FnDef) -> Result<Option<Constraints>, Error> {
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

        // FIXME how to return constraints?
        Ok(None)
    }

    fn interp_virtual_call(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<ScopeId>,
        cur_scope: ScopeId,
        fndef: &FnDef,
        genargs: &GenericArgs,
    ) -> Result<Option<Constraints>, Error> {
        // Steps:
        // - Get trait that this function is associated with
        //   - tstore.assoc_fn_traits (Map<AssocFn, Trait>)
        // - Get concrete type constraints for trait object
        //   - istore / tstore.trait_structs (CHA / RTA)
        // - Get every concrete type constraint's impl of this function
        //   - tstore.struct_assoc_fns (Map<(Struct, Trait), FnImpls>)

        // Get trait that this function is associated with
        let trait_defid = match self.tstore.assoc_fn_traits.get(&fndef.0) {
            Some(trait_defid_) => trait_defid_,
            None => panic!("assoc fn {:?} does not point to trait", fndef.0),
        };
        debug!("trait_defid: {:?}", trait_defid);

        // Get concrete type constraints for trait object
        let tyconstraints = match self.tstore.trait_structs.get(&trait_defid) {
            Some(tyconstraints_) => tyconstraints_,
            None => panic!("trait {:?} does not point to any structs", trait_defid),
        };
        debug!("tyconstraints: {:?}", tyconstraints);

        // Get every concrete type constraint's impl of this function
        let mut assoc_fn_impls = Vec::new();
        for &tyconstraint in tyconstraints {
            match self.tstore.struct_assoc_fns.get(&(tyconstraint, fndef.0)) {
                Some(assoc_fn_impl) => assoc_fn_impls.append(&mut assoc_fn_impl.clone()),
                None => panic!(
                    "struct/container ({:?}, {:?}) pair does not point to assoc fn",
                    tyconstraint, fndef.0
                ),
            }
        }
        debug!("assoc_fn_impls: {:?}", assoc_fn_impls);

        self.simulate_static_calls(istore, call_stack, cur_scope, assoc_fn_impls, genargs)
    }

    fn simulate_static_calls(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<ScopeId>,
        _cur_scope: ScopeId,
        assoc_fn_impls: Vec<DefId>,
        genargs: &GenericArgs,
    ) -> Result<Option<Constraints>, Error> {
        let mut static_results = Vec::<Option<Constraints>>::new();
        for assoc_fn_impl in assoc_fn_impls {
            let fndef = FnDef(assoc_fn_impl);
            let instance = Instance::resolve(fndef, &genargs).unwrap();
            debug!("instance def: {:?}", instance.def);
            debug!("a converted static instance: {:?}", instance);
            let mut call_stack_clone = call_stack.clone();
            let scope_id = (assoc_fn_impl, instance.def);
            call_stack_clone.push(scope_id);
            static_results.push(self.visit_instance(
                istore,
                &mut call_stack_clone,
                scope_id,
                instance,
            )?);
        }

        // FIXME merge
        Ok(static_results[0].clone())
    }

    fn interp_return(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<ScopeId>,
        cur_scope: ScopeId,
    ) -> Result<Option<Constraints>, Error> {
        debug!("RETURNING from scope {:?}...", cur_scope);
        //debug!("callstack PRE POP: {:?}", call_stack);
        let popped = call_stack.pop();
        if popped.unwrap() != cur_scope {
            panic!("call stack out of sorts");
        }
        //debug!("callstack POST POP: {:?}", call_stack);

        // Get and "return" the constraints at Place(0)
        match istore.scoped_get(
            cur_scope,
            &MapKey::Place(Place {
                local: 0,
                projection: Vec::new(),
            }),
        ) {
            Some(retval) => match retval {
                MapValue::Constraints(retval_constraints) => {
                    debug!("returning constraints: {:?}", retval_constraints);
                    Ok(Some(retval_constraints))
                }
                _ => panic!("should not be returning a scope"),
            },
            None => {
                // TODO Double check that nothing _needs_ to be returned (for interp correctness)

                Ok(None)
            }
        }
    }
}
