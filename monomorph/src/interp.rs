use rustc_public::DefId;
use rustc_public::mir::mono::{Instance, InstanceKind};
use rustc_public::mir::{
    BasicBlock, Body, ConstOperand, Local, LocalDecl, Operand, Place, Statement, StatementKind,
    Successors, SwitchTargets, Terminator, TerminatorKind,
};
use rustc_public::ty::{
    BoundVariableKind, ClosureDef, ClosureKind, ConstantKind, FnDef, GenericArgs, IntTy, PolyFnSig,
    RigidTy, Span, TyKind,
};

use log::debug;

use crate::VOLogger;
use crate::common::{is_wrapper_type, log_call_stack, log_mir, log_scope};
use crate::constraints::{Constraints, InterpStore, MapKey, MapValue, Merge, VOID, VORval};
use crate::constraints::{merge_stores, unique_append};
use crate::convert::RvalConverter;
use crate::error::Error;
use crate::sig_collect::{SigStore, SigVal};
use crate::trait_collect::TraitStore;
use crate::wto::BBDeps;

pub struct InterpPass<'a> {
    pub sigstore: &'a SigStore,
    pub tstore: &'a TraitStore,
    pub converter: RvalConverter,
}

impl<'a> InterpPass<'a> {
    pub fn new(sigstore: &'a SigStore, tstore: &'a TraitStore) -> InterpPass<'a> {
        Self {
            sigstore,
            tstore,
            converter: RvalConverter::new(),
        }
    }

    fn check_call_stack(&self, call_stack: &Vec<VOID>, cur_scope: &VOID) {
        let last_item = call_stack[call_stack.len() - 1].clone();
        if *cur_scope != last_item {
            debug!("cur_scope: {:?}", cur_scope.0.name());
            debug!("last call_stack item: {:?}", last_item);
            log_call_stack(call_stack);
            panic!("call stack out of sorts (cur_scope does not match last call_stack elem)");
        }
    }

    pub fn run(
        &self,
        logger: &mut VOLogger,
        istore: &mut InterpStore,
        start_instance: Instance,
    ) -> Result<Option<Constraints>, Error> {
        let start_scope = (start_instance, GenericArgs(vec![]));
        let mut call_stack = vec![start_scope.clone()];

        // Initialize InterpStore with entry_fn's substore
        let entry_fn_istore = InterpStore::new();
        istore.cmap.insert(
            MapKey::ScopeId(start_scope.clone()),
            Box::new(MapValue::Store(entry_fn_istore, None)),
        );

        self.visit_instance(logger, istore, &mut call_stack, &start_scope)
    }

    fn visit_instance(
        &self,
        logger: &mut VOLogger,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
    ) -> Result<Option<Constraints>, Error> {
        let body = cur_scope.0.body().unwrap();
        debug!("\n\n\n#############################");
        debug!(
            "###### INTERP-ING NEW BODY for func {:?}",
            cur_scope.0.name()
        );
        log_call_stack(call_stack);
        //log_mir(&body);
        debug!("#############################\n\n");

        self.check_call_stack(call_stack, cur_scope);

        self.visit_body(logger, istore, call_stack, cur_scope, &body)
    }

    fn visit_body(
        &self,
        logger: &mut VOLogger,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        body: &Body,
    ) -> Result<Option<Constraints>, Error> {
        // If there exists a memoized WTO, use it; otherwise, create it
        let mut bb_deps;
        if let Some(mem_bb_deps) = istore.wtos.get(cur_scope) {
            bb_deps = mem_bb_deps.clone();
            debug!("OLD ordering: {:?}", bb_deps.ordering);
        } else {
            bb_deps = BBDeps::new(body);
            istore.wtos.insert(cur_scope.clone(), bb_deps.clone());
            debug!("NEW ordering: {:?}", bb_deps.ordering);
        }

        // Loop through basic blocks in WTO
        let mut last_res = None;
        let num_bbs = bb_deps.ordering.len();
        loop {
            debug!("\n\nGETTING NEXT BB");
            debug!("pre-pop ordering: {:?}", bb_deps.ordering);
            if bb_deps.ordering.is_empty() {
                break;
            }
            let bb = bb_deps.ordering.remove(0);
            debug!("--NEW BB: {:?}", bb);
            let data = body.blocks.get(bb).unwrap();
            last_res = self.visit_basic_block(
                logger,
                istore,
                call_stack,
                cur_scope,
                body.locals(),
                &mut bb_deps,
                num_bbs,
                bb,
                data,
            )?;
        }

        Ok(last_res)
    }

    fn visit_basic_block(
        &self,
        logger: &mut VOLogger,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        bb_deps: &mut BBDeps,
        num_bbs: usize,
        bb: usize,
        data: &BasicBlock,
    ) -> Result<Option<Constraints>, Error> {
        debug!("\n##########");
        debug!(
            "# visiting BASICBLOCK {:?} ({:?}/{:?}) for {:?}",
            bb,
            bb + 1,
            num_bbs,
            cur_scope.0.name()
        );
        debug!("##########");

        self.check_call_stack(call_stack, cur_scope);

        let num_stmts = data.statements.len();
        for (i, stmt) in data.statements.iter().enumerate() {
            debug!(
                "\n\n# visiting STATEMENT {:?} ({:?}/{:?}) in BB{:?} for {:?}",
                i,
                i + 1,
                num_stmts,
                bb,
                cur_scope.0.name()
            );
            self.visit_statement(istore, cur_scope, local_decls, stmt);
        }

        debug!(
            "\n\n# visiting TERMINATOR in BB{:?} for {:?}",
            bb,
            cur_scope.0.name()
        );

        let res = self.visit_terminator(
            logger,
            istore,
            call_stack,
            cur_scope,
            local_decls,
            bb_deps,
            bb,
            &data.terminator,
        )?;

        bb_deps.mark_visited(bb, cur_scope);

        Ok(res)
    }

    fn visit_statement(
        &self,
        istore: &mut InterpStore,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        stmt: &Statement,
    ) {
        match &stmt.kind {
            // Only interp assignments to track type constraint changes
            StatementKind::Assign(place, rvalue) => {
                debug!("start assignment!");
                log_scope(cur_scope);
                debug!("stmt: {:?}", &stmt.kind);
                debug!("dest ty: {:?}", place.ty(local_decls).unwrap());
                debug!("place: {:?}", place);
                debug!("rval: {:?}", rvalue);
                //debug!("locals: {:?}", local_decls);

                // convert MIR Rvalue to VORval
                let constraints = self.converter.convert(istore, cur_scope, rvalue);
                debug!("CONVERTED CONSTRAINTS: {:?}", constraints);

                // add resolved constraints to istore
                istore.scoped_update(
                    cur_scope,
                    MapKey::Local(place.local),
                    Box::new(MapValue::Constraints(constraints)),
                );
            }
            _ => {}
        }
    }

    fn visit_terminator(
        &self,
        logger: &mut VOLogger,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        bb_deps: &mut BBDeps,
        bb: usize,
        term: &Terminator,
    ) -> Result<Option<Constraints>, Error> {
        debug!("TERM KIND: {:?}", &term.kind);
        match &term.kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => match func {
                Operand::Constant(co) => self.interp_direct_call(
                    logger,
                    &term.span,
                    istore,
                    call_stack,
                    cur_scope,
                    local_decls,
                    co,
                    args,
                    destination,
                ),
                Operand::Copy(place) | Operand::Move(place) => self.interp_indirect_call(
                    logger,
                    &term.span,
                    istore,
                    call_stack,
                    cur_scope,
                    local_decls,
                    place,
                    args,
                    destination,
                ),
                _ => todo!("calling runtime check operand?"),
            },
            TerminatorKind::Return => self.interp_return(istore, call_stack, cur_scope),
            TerminatorKind::SwitchInt { discr, targets } => {
                self.interp_switchint(istore, cur_scope, local_decls, bb, bb_deps, discr, targets)
            }
            TerminatorKind::Assert { .. }
            | TerminatorKind::Drop { .. }
            | TerminatorKind::Goto { .. } => Ok(None),
            _ => todo!("other term kind"),
        }
    }

    fn interp_indirect_call(
        &self,
        logger: &mut VOLogger,
        term_span: &Span,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        place: &Place,
        args: &Vec<Operand>,
        destination: &Place,
    ) -> Result<Option<Constraints>, Error> {
        debug!("INTERPING INDIRECT CALL");
        let mut ret_constraints = Vec::new();
        match istore.scoped_get(cur_scope, &MapKey::Local(place.local), false) {
            Some(val) => match val {
                MapValue::Constraints(constraints) => {
                    debug!("found constraints!: {:?}", constraints);
                    for constraint in &constraints {
                        match self.interp_constraint_as_fn(
                            logger,
                            term_span,
                            istore,
                            call_stack,
                            cur_scope,
                            local_decls,
                            &constraint,
                            args,
                        ) {
                            Ok(Some(constraints)) => {
                                unique_append(&mut ret_constraints, constraints)
                            }
                            Ok(None) => {}
                            e @ Err(_) => panic!("interping constraint as fn, got error: {:?}", e),
                        }
                    }
                }
                _ => panic!("trying to indirectly call a scope - invalid"),
            },
            None => panic!("fnptr value not found in cmap"),
        }

        // Set destination local to value in cmap
        debug!(
            "RET FROM INDIRECT FUNC CALL ret_constraints: {:?}",
            ret_constraints
        );
        log_scope(cur_scope);
        debug!("destination: {:?}", destination);
        istore.scoped_update(
            cur_scope,
            MapKey::Local(destination.local),
            Box::new(MapValue::Constraints(ret_constraints)),
        );

        Ok(None)
    }

    fn interp_constraint_as_fn(
        &self,
        logger: &mut VOLogger,
        term_span: &Span,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        constraint: &VORval,
        args: &Vec<Operand>,
    ) -> Result<Option<Constraints>, Error> {
        match constraint {
            /*
            VORval::IdkType(ty) => match ty.kind() {
                TyKind::RigidTy(rigid_ty) => match rigid_ty {
                    RigidTy::FnDef(fndef, genargs) => self.interp_fn_def(
                        logger,
                        term_span,
                        istore,
                        call_stack,
                        cur_scope,
                        local_decls,
                        fndef,
                        &genargs,
                        args,
                    ),
                    RigidTy::FnPtr(sig) => self.interp_fn_ptr(
                        logger,
                        term_span,
                        istore,
                        call_stack,
                        cur_scope,
                        local_decls,
                        sig,
                        args,
                    ),
                    RigidTy::Closure(cdef, genargs) => self.interp_closure(
                        logger,
                        term_span,
                        istore,
                        call_stack,
                        cur_scope,
                        local_decls,
                        cdef,
                        &genargs,
                        args,
                    ),
                    other @ _ => panic!("other rigidty (not a fn ptr!): {:?}", other),
                },
                _ => todo!("other tykind"),
            },
            */
            VORval::FnDef(fndef, genargs) => self.interp_fn_def(
                logger,
                term_span,
                istore,
                call_stack,
                cur_scope,
                local_decls,
                *fndef,
                &genargs,
                args,
            ),
            VORval::FnPtr(_) => todo!("fnptr vorval: {:?}", constraint),
            VORval::Closure(cdef, genargs) => self.interp_closure(
                logger,
                term_span,
                istore,
                call_stack,
                cur_scope,
                local_decls,
                *cdef,
                &genargs,
                args,
            ),
            _ => panic!("other vorval interp as fn?: {:?}", constraint),
        }
    }

    fn interp_fn_ptr(
        &self,
        _logger: &mut VOLogger,
        _term_span: &Span,
        _istore: &mut InterpStore,
        _call_stack: &mut Vec<VOID>,
        _cur_scope: &VOID,
        _local_decls: &[LocalDecl],
        sig: PolyFnSig,
        _args: &Vec<Operand>,
    ) -> Result<Option<Constraints>, Error> {
        debug!("sig!: {:?}", sig);
        let sigval = SigVal::new(&self.converter, &sig);
        match self.sigstore.sigs.get(&sigval) {
            Some(fndef_vec) => {
                debug!("got fndef_vec!: {:?}", fndef_vec);
                debug!("num candidate fns: {:?}", fndef_vec.len());

                // Don't want to pollute our results too much, so if we have too
                // many possible fns to call, just fallback to retty
                if fndef_vec.len() > 5 {
                    debug!("TOO MANY CANDIDATES - falling back to retty");
                    self.retty_fallback(sig)
                // Otherwise, interpret the candidate functions
                } else {
                    debug!("INTERPRET CANDIDATES");
                    todo!();
                    /*
                    let mut ret_constraints = Vec::new();
                    for fndef in fndef_vec {
                        debug!("TRY FUNC: {:?}", fndef);
                        // FIXME no genargs??
                        let genargs = GenericArgs(vec![]);
                        match self.interp_fn_ptr(
                            logger,
                            term_span,
                            istore,
                            call_stack,
                            cur_scope,
                            local_decls,
                            *fndef,
                            &genargs,
                            args,
                        ) {
                            Ok(Some(constraints)) => unique_append(&mut ret_constraints, constraints),
                            Ok(None) => {}
                            e @ Err(_) => panic!("got error: {:?}", e),
                        }
                    }
                    debug!("ret_constraints: {:?}", ret_constraints);
                    todo!("FN PTR (interp'd candidates)");
                    */
                }
            }
            None => {
                debug!("sigval: {:?}", sigval);
                debug!("no candidate fns to call, using retty fallback");
                self.retty_fallback(sig)
            }
        }
    }

    fn interp_closure(
        &self,
        logger: &mut VOLogger,
        _term_span: &Span,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        cdef: ClosureDef,
        genargs: &GenericArgs,
        args: &Vec<Operand>,
    ) -> Result<Option<Constraints>, Error> {
        debug!("INTERPING CLOSURE");
        debug!("cdef: {:?}", cdef);
        debug!("args: {:?}", args);
        let closure_kind = self.get_closure_kind(&genargs);
        debug!("genargs: {:?}", genargs);
        debug!("CLOSURE KIND: {:?}", closure_kind);
        if let Some(body) = cdef.body() {
            debug!("RESOLVING CLOSURE INSTANCE");
            let instance = Instance::resolve_closure(cdef, &genargs, closure_kind).unwrap();
            debug!("instance: {:?}", instance);
            debug!("closure body..");
            log_mir(&body);

            // FIXME may not need to clone if only one thing to interpret?
            //let mut istore_clone = istore.clone();
            let new_scope = (instance, genargs.clone());
            call_stack.push(new_scope.clone());
            self.resolve_args(
                //&mut istore_clone,
                istore,
                cur_scope,
                local_decls,
                &new_scope,
                args,
                &genargs,
                true,
            );
            //self.visit_instance(logger, &mut istore_clone, call_stack, &new_scope)
            self.visit_instance(logger, istore, call_stack, &new_scope)
        } else {
            todo!("closure has no body");
        }
    }

    fn get_closure_kind(&self, genargs: &GenericArgs) -> ClosureKind {
        if genargs.0.is_empty() {
            panic!("no closure kind in genargs (empty)");
        }

        match genargs.0[0].expect_ty().kind() {
            // Rustc encodings for closure kinds
            TyKind::RigidTy(RigidTy::Int(IntTy::I8)) => ClosureKind::Fn,
            TyKind::RigidTy(RigidTy::Int(IntTy::I16)) => ClosureKind::FnMut,
            TyKind::RigidTy(RigidTy::Int(IntTy::I32)) => ClosureKind::FnOnce,
            other @ _ => panic!("first genarg is unexpected: {:?}", other),
        }
    }

    fn interp_direct_call(
        &self,
        logger: &mut VOLogger,
        term_span: &Span,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        co: &ConstOperand,
        args: &Vec<Operand>,
        destination: &Place,
    ) -> Result<Option<Constraints>, Error> {
        let ret_constraints = match co.const_.ty().kind() {
            TyKind::RigidTy(rigid_ty) => match rigid_ty {
                RigidTy::FnDef(fndef, genargs) => self.interp_fn_def(
                    logger,
                    term_span,
                    istore,
                    call_stack,
                    cur_scope,
                    local_decls,
                    fndef,
                    &genargs,
                    args,
                ),
                other @ _ => todo!("different RigidTy: {:?}", other),
            },
            kind @ _ => todo!("funccall const is another kind: {:?}", kind),
        };

        // Set destination local to value in cmap
        debug!("RET FROM FUNC CALL ret_constraints: {:?}", ret_constraints);
        log_scope(cur_scope);
        debug!("destination: {:?}", destination);
        match ret_constraints {
            Ok(Some(constraints)) => {
                istore.scoped_update(
                    cur_scope,
                    MapKey::Local(destination.local),
                    Box::new(MapValue::Constraints(constraints)),
                );
            }
            Ok(None) => {}
            _ => panic!("error in constraints"),
        }

        Ok(None)
    }

    /// Interpret a function call from a FnDef object
    fn interp_fn_def(
        &self,
        logger: &mut VOLogger,
        term_span: &Span,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        fndef: FnDef,
        genargs: &GenericArgs,
        args: &Vec<Operand>,
    ) -> Result<Option<Constraints>, Error> {
        let instance = Instance::resolve(fndef, genargs).unwrap();
        let new_scope = (instance, genargs.clone());
        debug!("instance def: {:?}", instance.def);
        debug!("--- CALLING {:?}", fndef);
        log_scope(cur_scope);
        match instance.kind {
            InstanceKind::Item => {
                debug!("regular static funccall");
                self.interp_static_call(
                    logger,
                    istore,
                    call_stack,
                    cur_scope,
                    local_decls,
                    &new_scope,
                    fndef,
                    args,
                    &genargs,
                )
            }
            InstanceKind::Virtual { .. } => {
                debug!("virtual funccall");
                self.interp_virtual_call(
                    logger,
                    term_span,
                    istore,
                    call_stack,
                    cur_scope,
                    local_decls,
                    &fndef,
                    &genargs,
                    args,
                )
            }
            InstanceKind::Shim => {
                debug!("shim funccall");
                self.interp_static_call(
                    logger,
                    istore,
                    call_stack,
                    cur_scope,
                    local_decls,
                    &new_scope,
                    fndef,
                    args,
                    &genargs,
                )
            }
            InstanceKind::Intrinsic => {
                debug!("intrinsic funccall");
                self.retty_fallback(fndef.fn_sig())
            }
        }
    }

    fn interp_static_call(
        &self,
        logger: &mut VOLogger,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        caller_scope: &VOID,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        fndef: FnDef,
        args: &Vec<Operand>,
        genargs: &GenericArgs,
    ) -> Result<Option<Constraints>, Error> {
        debug!("INTERP STATIC CALL");
        if cur_scope.0.has_body() {
            //let mut istore_clone = istore.clone();
            call_stack.push(cur_scope.clone());
            self.resolve_args(
                //&mut istore_clone,
                istore,
                caller_scope,
                local_decls,
                cur_scope,
                args,
                genargs,
                false,
            );
            //self.visit_instance(logger, &mut istore_clone, call_stack, cur_scope)
            self.visit_instance(logger, istore, call_stack, cur_scope)
        } else {
            debug!("no body, so not visiting/not updating call stack");
            self.retty_fallback(fndef.fn_sig())
        }
    }

    fn resolve_args(
        &self,
        istore: &mut InterpStore,
        caller_scope: &VOID,
        local_decls: &[LocalDecl],
        callee_scope: &VOID,
        //fndef: FnDef,
        args: &Vec<Operand>,
        _genargs: &GenericArgs,
        is_closure: bool,
    ) {
        debug!("RESOLVING ARGS");
        debug!("IS TARGET FN CLOSURE?: {}", is_closure);
        let mut new_substore = InterpStore::new();
        let key = MapKey::ScopeId(callee_scope.clone());

        //let sig = fndef.fn_sig();
        //debug!("fn_sig: {:?}", sig);
        //self.check_sig_boundvars(&sig);

        // Resolve generics + add arg values into new substore
        self.resolve_args_helper(
            istore,
            &mut new_substore,
            caller_scope,
            local_decls,
            args,
            //genargs,
            is_closure,
        );

        // Merge new substore into existing substore at this scopeId
        let store;
        match istore.cmap.get(&key) {
            Some(box MapValue::Store(old_substore, old_es)) => {
                store = merge_stores(
                    old_substore,
                    old_es,
                    &new_substore,
                    &Some(vec![caller_scope.clone()]),
                );
            }
            Some(_) => panic!("got constraint, expected store"),
            None => store = (new_substore, Some(vec![caller_scope.clone()])),
        }

        // Add new substore in top-level store
        istore
            .cmap
            .insert(key, Box::new(MapValue::Store(store.0, store.1)));
    }

    fn resolve_args_helper(
        &self,
        istore: &InterpStore,
        new_substore: &mut InterpStore,
        caller_scope: &VOID,
        local_decls: &[LocalDecl],
        args: &Vec<Operand>,
        //genargs: &GenericArgs,
        is_closure: bool,
    ) {
        //debug!("generic args: {:?}", genargs);
        for (i, arg) in args.into_iter().enumerate() {
            debug!("\narg position: {:?}", i);
            debug!("arg: {:?}", arg);
            let arg_constraints =
                self.resolve_arg(istore, caller_scope, local_decls, arg, is_closure);
            debug!("arg constraints: {:?}\n", arg_constraints);

            let local = if is_closure {
                i + 2
            } else {
                i + 1
            };
            new_substore.cmap.insert(
                MapKey::Local(local),
                Box::new(MapValue::Constraints(arg_constraints)),
            );
        }
    }

    fn resolve_arg(
        &self,
        istore: &InterpStore,
        caller_scope: &VOID,
        _local_decls: &[LocalDecl],
        arg: &Operand,
        is_closure: bool,
    ) -> Constraints {
        match arg {
            Operand::Copy(place) | Operand::Move(place) => {
                match istore.scoped_get(caller_scope, &MapKey::Local(place.local), is_closure) {
                    Some(val) => match val {
                        MapValue::Constraints(constraints) => constraints,
                        _ => panic!("arg is a scope"),
                    },
                    None => {
                        debug!("place {:?} DNE in cmap, widen to type", place,);
                        vec![]
                    }
                }
            }
            // TODO can maybe get a more precise VORval depending on kind
            Operand::Constant(const_op) => match const_op.const_.kind() {
                ConstantKind::Allocated(alloc) => match alloc.read_uint() {
                    // FIXME
                    Ok(val) => {
                        debug!("ALLOC CONST");
                        // Only use the constval if this is supposed to be used as an integer
                        match const_op.const_.ty().kind() {
                            TyKind::RigidTy(rigidty) => match rigidty {
                                RigidTy::Bool | RigidTy::Int(_) | RigidTy::Uint(_) => {
                                    vec![VORval::Scalar(Some(val))]
                                }
                                _ => vec![],
                            },
                            _ => todo!(),
                        }
                    }
                    _ => vec![],
                },
                ConstantKind::ZeroSized => vec![],
                other @ _ => todo!("arg is another constant kind: {:?}", other),
            },
            _ => todo!("runtime check arg"),
        }
    }

    fn check_sig_boundvars(&self, sig: &PolyFnSig) {
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
    }

    fn retty_fallback(&self, sig: PolyFnSig) -> Result<Option<Constraints>, Error> {
        debug!("fn_sig: {:?}", sig);
        self.check_sig_boundvars(&sig);
        debug!("output: {:?}", sig.value.output());

        // Return output type as VORval (widening)
        let ret_constraints = vec![];
        Ok(Some(ret_constraints))
    }

    fn interp_virtual_call(
        &self,
        logger: &mut VOLogger,
        term_span: &Span,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        fndef: &FnDef,
        genargs: &GenericArgs,
        args: &Vec<Operand>,
    ) -> Result<Option<Constraints>, Error> {
        // Steps:
        // - Get trait that this function is associated with
        //   - tstore.assoc_fn_traits (Map<AssocFn, Trait>)
        // - Get concrete type constraints for trait object
        //   - istore (FSA) / tstore (CHA / RTA)
        // - Get every concrete type constraint's impl of this function
        //   - tstore.struct_assoc_fns (Map<(Struct, Trait), FnImpls>)

        debug!("DYNAMIC DISPATCH");
        debug!("args: {:?}", args);

        let trait_defid = self.get_trait_defid(&fndef.0);
        debug!("trait_defid: {:?}", trait_defid);

        let assoc_fn_impls_cha = self.get_impls_cha(&fndef.0, &trait_defid);
        debug!("------- assoc_fn_impls_cha: {:?}", assoc_fn_impls_cha);

        let assoc_fn_impls_fsa = self.get_impls_fsa(istore, cur_scope, &fndef.0, args);
        debug!("------- assoc_fn_impls_fsa: {:?}", assoc_fn_impls_fsa);

        if assoc_fn_impls_cha != assoc_fn_impls_fsa {
            debug!("SET OF IMPLS DIFFER");
            // Log this dynamic dispatch!
            match logger.log_found(term_span, &assoc_fn_impls_cha, &assoc_fn_impls_fsa) {
                e @ Err(_) => panic!("logging error: {:?}", e),
                _ => {}
            }
        } else {
            debug!("SET OF IMPLS SAME");
            // Log this dynamic dispatch :(
            match logger.log_not_found(term_span, &assoc_fn_impls_cha, &assoc_fn_impls_fsa) {
                e @ Err(_) => panic!("logging error: {:?}", e),
                _ => {}
            }
        }

        self.simulate_static_calls(
            logger,
            istore,
            call_stack,
            cur_scope,
            local_decls,
            //assoc_fn_impls_cha,
            assoc_fn_impls_fsa,
            genargs,
            args,
        )
    }

    fn get_trait_defid(&self, assoc_fn_defid: &DefId) -> DefId {
        // Get trait that this function is associated with
        match self.tstore.assoc_fn_traits.get(assoc_fn_defid) {
            Some(trait_defid) => *trait_defid,
            None => panic!("assoc fn {:?} does not point to trait", assoc_fn_defid),
        }
    }

    fn get_impls_from_defids(
        &self,
        assoc_fn_defid: &DefId,
        constraint_defids: &Vec<DefId>,
    ) -> Vec<DefId> {
        // Get every concrete type constraint's impl of this function
        let mut assoc_fn_impls = Vec::new();
        for defid in constraint_defids {
            match self.tstore.struct_assoc_fns.get(&(*defid, *assoc_fn_defid)) {
                Some(assoc_fn_impl) => assoc_fn_impls.append(&mut assoc_fn_impl.clone()),
                None => panic!(
                    "struct/container ({:?}, {:?}) pair does not point to assoc fn",
                    defid, assoc_fn_defid
                ),
            }
        }
        assoc_fn_impls
    }

    fn get_impls_cha(&self, assoc_fn_defid: &DefId, trait_defid: &DefId) -> Vec<DefId> {
        debug!("GETTING CHA IMPLS");
        let constraint_defids = self.get_cha_tyconstraint_defids(&trait_defid);
        debug!("constraint defids: {:?}", constraint_defids);
        self.get_impls_from_defids(assoc_fn_defid, &constraint_defids)
    }

    fn get_cha_tyconstraint_defids(&self, trait_defid: &DefId) -> Vec<DefId> {
        // Get concrete type constraints for trait object
        match self.tstore.trait_structs.get(trait_defid) {
            Some(tyconstraints) => tyconstraints.to_vec(),
            None => panic!("trait {:?} does not point to any structs", trait_defid),
        }
    }

    fn get_impls_fsa(
        &self,
        istore: &InterpStore,
        cur_scope: &VOID,
        assoc_fn_defid: &DefId,
        args: &Vec<Operand>,
    ) -> Vec<DefId> {
        debug!("GETTING FSA IMPLS");
        let local = self.get_traitobj_local(args);
        debug!("local: {:?}", local);
        let tyconstraints = self.get_fsa_tyconstraints(istore, cur_scope, local);
        debug!("tyconstraints: {:?}", tyconstraints);
        let constraint_defids = self.get_fsa_constraint_defids(&tyconstraints);
        debug!("constraint defids: {:?}", constraint_defids);
        self.get_impls_from_defids(assoc_fn_defid, &constraint_defids)
    }

    fn get_traitobj_local(&self, args: &Vec<Operand>) -> Local {
        match &args[0] {
            Operand::Copy(place) | Operand::Move(place) => {
                if !place.projection.is_empty() {
                    panic!("traitobj place has projections");
                }

                place.local
            }
            _ => panic!("unexpected operand: {:?}", args[0]),
        }
    }

    fn get_fsa_tyconstraints(
        &self,
        istore: &InterpStore,
        cur_scope: &VOID,
        local: Local,
    ) -> Constraints {
        // Get concrete type constraints for trait object
        match istore.scoped_get(cur_scope, &MapKey::Local(local), false) {
            Some(val) => match val {
                MapValue::Constraints(tyconstraints) => tyconstraints,
                MapValue::Store(..) => panic!("local {:?} refers to a scope", local),
            },
            None => panic!("local {:?} has no constraints", local),
        }
    }

    fn get_fsa_constraint_defids(&self, tyconstraints: &Constraints) -> Vec<DefId> {
        let mut defids = Vec::new();
        for vorval in tyconstraints {
            unique_append(&mut defids, self.resolve_defid(vorval));
        }
        defids
    }

    fn resolve_defid(&self, vorval: &VORval) -> Vec<DefId> {
        match vorval {
            VORval::Adt(adtdef, genargs) => {
                if is_wrapper_type(&adtdef.0) {
                    //let genargs = genargs.clone().unwrap();
                    if genargs.is_none() {
                        panic!("no genargs");
                    }
                    if genargs.as_ref().unwrap().len() > 1 {
                        debug!("more than 1 genarg in wrapper type");
                    }
                    self.resolve_defid(&genargs.as_ref().unwrap()[0])
                } else {
                    vec![adtdef.0]
                }
            }
            VORval::Idk(inner_vec) => {
                let mut defids = Vec::new();
                for vorval in inner_vec {
                    unique_append(&mut defids, self.resolve_defid(vorval));
                }
                defids
            }
            _ => panic!("other vorval: {:?}", vorval),
        }
    }

    fn simulate_static_calls(
        &self,
        logger: &mut VOLogger,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        assoc_fn_impls: Vec<DefId>,
        genargs: &GenericArgs,
        args: &Vec<Operand>,
    ) -> Result<Option<Constraints>, Error> {
        let mut results = Vec::<Option<Constraints>>::new();
        let mut istore_vec = Vec::new();

        for assoc_fn_impl in assoc_fn_impls {
            let fndef = FnDef(assoc_fn_impl);
            let instance = Instance::resolve(fndef, &genargs).unwrap();
            debug!("instance def: {:?}", instance.def);
            debug!("a converted static instance: {:?}", instance);

            let mut istore_clone = istore.clone();
            let mut call_stack_clone = call_stack.clone();
            let callee_scope = (instance, genargs.clone());
            call_stack_clone.push(callee_scope.clone());

            self.resolve_args(
                &mut istore_clone,
                cur_scope,
                local_decls,
                &callee_scope,
                args,
                &genargs,
                false,
            );
            results.push(self.visit_instance(
                logger,
                &mut istore_clone,
                &mut call_stack_clone,
                &callee_scope,
            )?);

            istore_vec.push(istore_clone);
        }

        self.merge_istores_and_set(istore, &mut istore_vec);
        self.merge_results_and_ret(&mut results)
    }

    fn merge_istores_and_set(&self, istore: &mut InterpStore, istore_vec: &mut Vec<InterpStore>) {
        match istore_vec.merge() {
            Ok(Some(merged_istore)) => {
                debug!("merged istore: {:?}", merged_istore);
                *istore = merged_istore;
            }
            Ok(None) => panic!("istores empty?"),
        }
    }

    fn merge_results_and_ret(
        &self,
        results: &mut Vec<Option<Constraints>>,
    ) -> Result<Option<Constraints>, Error> {
        // Filter out None constraints and unwrap all Some options
        debug!("results PRE filter: {:?}", results);
        let filtered_results: Vec<Constraints> = results
            .into_iter()
            .filter(|option| option.is_none())
            .map(|x| x.clone().unwrap())
            .collect();
        debug!("results POST filter: {:?}", filtered_results);

        match filtered_results.merge() {
            Ok(Some(merged_constraints)) => {
                debug!("merged constraints: {:?}", merged_constraints);
                return Ok(Some(merged_constraints));
            }
            Ok(None) => Ok(None),
        }
    }

    fn interp_switchint(
        &self,
        istore: &mut InterpStore,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        bb: usize,
        bb_deps: &mut BBDeps,
        discr: &Operand,
        targets: &SwitchTargets,
    ) -> Result<Option<Constraints>, Error> {
        debug!("SWITCHINT");
        debug!("discr: {:?}", discr);
        debug!("targets: {:?}", targets);

        match discr {
            Operand::Copy(place) | Operand::Move(place) => {
                match istore.scoped_get(cur_scope, &MapKey::Local(place.local), false) {
                    Some(val) => match val {
                        MapValue::Constraints(constraints) => {
                            // Create a byte-map for finding statically-impossible successors
                            let mut discr_vals_uninit =
                                Box::<[u8]>::new_zeroed_slice(targets.len());
                            let discr_vals = discr_vals_uninit.write_filled(0);
                            debug!("PRE discr_vals: {:?}", discr_vals);

                            // Check expected type of discriminant
                            debug!("expected ty: {:?}", place.ty(local_decls));
                            debug!("constraints: {:?}", constraints);

                            // Populate byte-map with possible branch values, based on constraints
                            self.set_bytemap(&constraints, targets, discr_vals);
                            debug!("POST discr_vals: {:?}", discr_vals);

                            self.prune_switchint_targets(
                                bb,
                                bb_deps,
                                &targets.all_targets(),
                                discr_vals,
                            );
                        }
                        _ => panic!("cannot switch on scope"),
                    },
                    None => {
                        debug!("local DNE, cannot prune any targets");
                    }
                }
            }
            Operand::Constant(_co) => {}
            _ => debug!("runtime checks op (ignoring)"),
        }

        Ok(None)
    }

    fn set_bytemap(
        &self,
        constraints: &Constraints,
        //branches: impl Iterator<Item = (u128, BasicBlockIdx)>,
        targets: &SwitchTargets,
        discr_vals: &mut [u8],
    ) {
        if constraints.is_empty() {
            debug!("NO CONSTRAINTS FOR BYTEMAP");
            // Increment all branch counters (since no statically-known discr value)
            for (i, _) in targets.branches().enumerate() {
                discr_vals[usize::try_from(i).unwrap()] += 1;
            }
            return;
        }

        debug!("USING CONSTRAINTS TO SET BYTEMAP");
        for constraint in constraints {
            debug!("setting bytemap for constraint: {:?}", constraint);
            match constraint {
                VORval::Scalar(num_opt) => {
                    debug!("scalar discr val: {:?}", num_opt);

                    if let Some(num) = num_opt {
                        // Increment matching branch counters
                        let mut set = false;
                        for (i, (val, _bb)) in targets.branches().enumerate() {
                            if *num == val {
                                discr_vals[usize::try_from(i).unwrap()] += 1;
                                set = true;
                            }
                        }
                        if !set {
                            discr_vals[discr_vals.len() - 1] += 1;
                        }
                    } else {
                        // Increment all branch counters (since no statically-known discr value)
                        for (i, _) in targets.branches().enumerate() {
                            discr_vals[usize::try_from(i).unwrap()] += 1;
                        }
                        discr_vals[discr_vals.len() - 1] += 1;
                    }
                }
                _ => {
                    // Increment all branch counters (since no statically-known discr value)
                    for (i, _) in targets.branches().enumerate() {
                        discr_vals[usize::try_from(i).unwrap()] += 1;
                    }
                    discr_vals[discr_vals.len() - 1] += 1;
                }
            }
        }
    }

    fn prune_switchint_targets(
        &self,
        bb: usize,
        bb_deps: &mut BBDeps,
        targets: &Successors,
        discr_vals: &mut [u8],
    ) {
        let prunable = self.get_prunable_targets(discr_vals);

        debug!("all targets: {:?}", targets);
        debug!("prunable: {:?}", prunable);

        if let Some(prune_idxs) = prunable {
            for prune_idx in prune_idxs {
                bb_deps.prune(bb, targets[prune_idx]);
            }
        }
    }

    fn get_prunable_targets(&self, discr_vals: &mut [u8]) -> Option<Vec<usize>> {
        // If the last value (otherwise branch) is > 0, cannot prune anything
        if discr_vals[discr_vals.len() - 1] > 0 {
            return None;
        }

        let mut poss_idxs = Vec::new();
        let mut imposs_idxs = Vec::new();
        for i in 0..discr_vals.len() {
            if discr_vals[i] > 0 {
                poss_idxs.push(i);
            } else {
                imposs_idxs.push(i);
            }
        }
        debug!("possible indices: {:?}", poss_idxs);
        debug!("impossible indices: {:?}", imposs_idxs);

        // If no possible indices, error?
        if poss_idxs.is_empty() {
            panic!("no possible branches");
        }

        // If some impossible indices, prune
        if !imposs_idxs.is_empty() {
            return Some(imposs_idxs);
        }

        // Some possible branches without any impossible branches -> cannot prune
        None
    }

    fn interp_return(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
    ) -> Result<Option<Constraints>, Error> {
        let popped = call_stack.pop();
        if popped.clone().unwrap() != *cur_scope {
            debug!("\npopped: {:?}", popped.unwrap().0.name());
            debug!("cur_scope: {:?}", cur_scope.0.name());
            log_call_stack(call_stack);
            panic!("call stack out of sorts");
        }

        debug!("\n\n\n#############################");
        debug!("RETURNING from scope {:?}...", cur_scope.0.name());
        log_call_stack(call_stack);
        debug!("#############################\n\n");

        // Get and "return" the constraints at Place(0)
        match istore.scoped_get(cur_scope, &MapKey::Local(0), false) {
            Some(retval) => match retval {
                MapValue::Constraints(retval_constraints) => {
                    debug!("\n###### RETURNING constraints:");
                    debug!("\t{:?}\n\n", retval_constraints);
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
