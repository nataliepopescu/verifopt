use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use rustc_public::DefId;
use rustc_public::mir::mono::{Instance, InstanceKind};
use rustc_public::mir::{
    BasicBlock, Body, BorrowKind, ConstOperand, LocalDecl, Mutability, NonDivergingIntrinsic,
    Operand, Place, ProjectionElem, Rvalue, Statement, StatementKind, Successors, SwitchTargets,
    Terminator, TerminatorKind,
};
use rustc_public::ty::{
    AdtDef, BoundVariableKind, ClosureDef, ClosureKind, FnDef, GenericArgKind, GenericArgs, IntTy,
    PolyFnSig, RigidTy, Span, Ty, TyKind,
};

use log::{debug, error};

use crate::VOLogger;
use crate::common::{log_call_stack, log_scope};
use crate::constraints::{
    ArgSet, Constraint, Constraints, InterpStore, Location, MapKey, MapValue, 
    RunningConstraint, SummaryKey, TraitObjConstraint, TraitObjTy, VOID, summary_key,
};
use crate::constraints::{unique_append, unique_push};
use crate::convert::RvalConverter;
use crate::error::Error;
use crate::merge::Merge;
use crate::merge::merge_stores;
use crate::sig_collect::{SigStore, SigVal};
use crate::trait_collect::TraitStore;
use crate::wto::BBDeps;

const MAX_DEPTH: u32 = 50;

pub struct InterpPass<'a> {
    pub sigstore: &'a SigStore,
    pub tstore: &'a TraitStore,
    pub converter: RvalConverter<'a>,
    pub dispatch_targets: RefCell<HashMap<Span, Vec<(DefId, Option<GenericArgs>)>>>,
    pub dispatch_cha: RefCell<HashMap<Span, Vec<(DefId, Option<GenericArgs>)>>>,

    pub summaries: RefCell<HashMap<SummaryKey, Constraints>>,
    pub in_queue: RefCell<HashSet<SummaryKey>>,
    pub key_stack: RefCell<Vec<SummaryKey>>,
    pub wq: RefCell<HashMap<SummaryKey, Vec<(VOID, Vec<Constraints>, Vec<VOID>)>>>,
    pub rec_depth: RefCell<u32>,
    pub dependencies: RefCell<HashMap<Span, HashSet<VOID>>>,
    pub incomplete: RefCell<HashSet<VOID>>,
}

impl<'a> InterpPass<'a> {
    pub fn new(sigstore: &'a SigStore, tstore: &'a TraitStore) -> InterpPass<'a> {
        Self {
            sigstore,
            tstore,
            converter: RvalConverter::new(tstore),
            dispatch_targets: HashMap::new().into(),
            dispatch_cha: HashMap::new().into(),
            wq: HashMap::new().into(),
            summaries: HashMap::new().into(),
            in_queue: HashSet::new().into(),
            key_stack: Vec::new().into(),
            rec_depth: 0.into(),
            dependencies: HashMap::new().into(),
            incomplete: HashSet::new().into(),
        }
    }

    fn check_call_stack(&self, call_stack: &Vec<VOID>, scope: &VOID) {
        let last_item = call_stack[call_stack.len() - 1].clone();
        if *scope != last_item {
            //debug!("scope: {:?}", scope.0.name());
            //debug!("last call_stack item: {:?}", last_item);
            log_call_stack(call_stack);
            panic!("call stack out of sorts (scope does not match last call_stack elem)");
        }
    }

    fn prepare_call(&self, call_stack: &mut Vec<VOID>, key: &SummaryKey) {
        call_stack.push(key.0.clone());
        self.key_stack.borrow_mut().push(key.clone());
    }

    fn prepare_return(&self, call_stack: &mut Vec<VOID>) -> Option<VOID> {
        self.key_stack.borrow_mut().pop();
        call_stack.pop()
    }

    pub fn run(
        &self,
        logger: &mut VOLogger,
        istore: &mut InterpStore,
        start_instance: Instance,
    ) -> Result<Option<ConstraintsAndFields>, Error> {
        let start_scope = (start_instance, GenericArgs(vec![]));
        let mut call_stack = vec![start_scope.clone()];

        self.key_stack
            .borrow_mut()
            .push((start_scope.clone(), ArgSet::new(&[])));

        // Initialize InterpStore with entry_fn's substore
        let entry_fn_istore = InterpStore::new();
        istore.cmap.insert(
            MapKey::ScopeId(start_scope.clone()),
            Box::new(MapValue::Store(entry_fn_istore, None)),
        );

        self.visit_body(
            logger,
            istore,
            &mut call_stack,
            &start_scope,
            &self.get_body(&start_scope),
        )
    }

    fn get_body(&self, cur_scope: &VOID) -> Body {
        cur_scope.0.body().unwrap()
    }

    fn visit_body(
        &self,
        logger: &mut VOLogger,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        body: &Body,
    ) -> Result<Option<ConstraintsAndFields>, Error> {
        debug!("\n\n\n#############################");
        debug!(
            "###### INTERP-ING NEW BODY for func {:?}",
            cur_scope.0.name()
        );
        log_call_stack(call_stack);
        //log_mir(&body);
        debug!("#############################\n\n");

        self.check_call_stack(call_stack, cur_scope);

        // If there exists a memoized WTO, use it; otherwise, create it
        let mut bb_deps;
        if let Some(mem_bb_deps) = istore.wtos.get(cur_scope) {
            bb_deps = mem_bb_deps.clone();
            if !bb_deps.has_ret {
                // This block does not explicitly return - likely a panicker, thus we can skip
                self.prepare_return(call_stack);
                return Ok(None);
            }
            debug!("OLD ordering: {:?}", bb_deps.ordering);
        } else {
            bb_deps = BBDeps::new(body);
            if !bb_deps.has_ret {
                // This block does not explicitly return - likely a panicker, thus we can skip
                self.prepare_return(call_stack);
                return Ok(None);
            }
            istore.wtos.insert(cur_scope.clone(), bb_deps.clone());
            debug!("NEW ordering: {:?}", bb_deps.ordering);
        }

        // Loop through basic blocks in WTO
        let mut last_res = None;
        let num_bbs = bb_deps.ordering.len();
        loop {
            //debug!("\n\nGETTING NEXT BB");
            //debug!("pre-pop ordering: {:?}", bb_deps.ordering);
            if bb_deps.ordering.is_empty() {
                //debug!("DONE INTERPING");
                break;
            }
            let bb = bb_deps.ordering.remove(0);
            debug!("\n\n--NEW BB: {:?}", bb);
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
    ) -> Result<Option<ConstraintsAndFields>, Error> {
        //debug!("\n##########");
        debug!(
            "# visiting BASICBLOCK {:?} ({:?}/{:?}) for {:?}",
            bb,
            bb + 1,
            num_bbs,
            cur_scope.0.name()
        );
        //debug!("##########");

        self.check_call_stack(call_stack, cur_scope);

        let num_stmts = data.statements.len();
        for (i, stmt) in data.statements.iter().enumerate() {
            debug!(
                "\n\n# visiting STATEMENT {:?} ({:?}/{:?}) in BB{:?} for {:?}\n\nSPAN: {:?}",
                i,
                i + 1,
                num_stmts,
                bb,
                cur_scope.0.name(),
                stmt.span,
            );
            self.visit_statement(istore, cur_scope, local_decls, stmt);
        }

        debug!(
            "\n\n# visiting TERMINATOR in BB{:?} for {:?}\n\nSPAN: {:?}",
            bb,
            cur_scope.0.name(),
            &data.terminator.span,
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

    /// If this type contains one or more (RigidTy) Dynamics, return the associated TraitObjTys
    /// (i.e. the Dynamic info converted into VerifOpt semantics)
    fn contains_dyn(&self, ty: &Ty) -> Option<Vec<TraitObjTy>> {
        match ty.kind() {
            TyKind::RigidTy(rigidty) => match rigidty {
                RigidTy::Dynamic(trait_vec, _) => {
                    let mut desttys = Vec::new();
                    for trait_ in trait_vec {
                        unique_push(
                            &mut desttys,
                            TraitObjTy::new_from_bound_existential(&trait_),
                        );
                    }
                    return Some(desttys);
                }
                RigidTy::Adt(_def, genargs) => {
                    for genarg in genargs.0 {
                        match genarg {
                            GenericArgKind::Type(ty) => {
                                let maybe_dyn = self.contains_dyn(&ty);
                                if maybe_dyn.is_some() {
                                    return maybe_dyn;
                                }
                            }
                            _ => continue,
                        }
                    }
                }
                RigidTy::Tuple(ty_vec) => {
                    for ty in ty_vec {
                        let maybe_dyn = self.contains_dyn(&ty);
                        if maybe_dyn.is_some() {
                            return maybe_dyn;
                        }
                    }
                }
                RigidTy::Array(ty, _)
                | RigidTy::Slice(ty)
                | RigidTy::Pat(ty, _)
                | RigidTy::RawPtr(ty, _)
                | RigidTy::Ref(_, ty, _) => {
                    let maybe_dyn = self.contains_dyn(&ty);
                    if maybe_dyn.is_some() {
                        return maybe_dyn;
                    }
                }
                _ => {} //debug!("LVAL TY IS OTHER RIGIDTY"),
            },
            _ => {} //debug!("LVAL TY IS OTHER TYKIND"),
        }

        None
    }

    /// If any of the constraints contain a type that implements one of the Traits listed in
    /// `maybe_trait_destty`, copy those types and put them into the TraitObjConstraint field
    /// of the constraint, leaving the RunningConstraint field unchanged
    fn pull_traitobjs_from_constraints(
        &self,
        maybe_trait_destty: &Option<Vec<TraitObjTy>>,
        old_constraints: Constraints,
    ) -> Constraints {
        let mut constraints = Vec::new();
        for constraint in old_constraints {
            match self
                .converter
                .get_any_traitobj(maybe_trait_destty, &constraint)
            {
                // Add into traitobj constraint
                toc @ Some(_) => match constraint {
                    Constraint { toc: None, cfc } => {
                        constraints.push(Constraint::new(toc, cfc));
                    }
                    Constraint {
                        toc: Some(ref existing_toc),
                        cfc: ref _cfc,
                    } => {
                        //debug!("existing_toc: {:?}", existing_toc);
                        if *existing_toc != toc.unwrap() {
                            todo!("update existing TOC");
                        } else {
                            constraints.push(constraint);
                        }
                    }
                },
                // Push constraint unchanged
                None => {
                    constraints.push(constraint);
                }
            }
        }
        constraints
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
                //debug!("stmt: {:?}", &stmt.kind);
                debug!("place: {:?}", place);
                debug!("rval: {:?}", rvalue);

                //let rval_ty = rvalue.ty(local_decls).unwrap();
                //debug!("rval ty: {:?}", rval_ty);

                //debug!("CHECKING FOR DYN IN PLACE TY");
                let dest_ty = place.ty(local_decls).unwrap();
                let maybe_trait_destty = self.contains_dyn(&dest_ty);
                //debug!("lval ty: {:?}", dest_ty);
                //debug!("maybe_trait_destty? {:?}", maybe_trait_destty);

                // convert MIR Rvalue to Constraint
                let (constraints, maybe_fields) =
                    if let Rvalue::Ref(_region, bk, to) = rvalue.clone() {
                        let to = match to.projection.as_slice() {
                            [ProjectionElem::Deref] => Place {
                                local: to.local,
                                projection: vec![],
                            },
                            _ => to.clone(),
                        };
                        istore.add_ref(
                            (place.clone(), cur_scope.clone()),
                            (to, cur_scope.clone()),
                            if matches!(bk, BorrowKind::Mut { .. }) {
                                Mutability::Mut
                            } else {
                                Mutability::Not
                            },
                        );
                        (vec![], None)
                    } else {
                        self.converter.convert(
                            istore,
                            &Location::new(),
                            local_decls,
                            cur_scope,
                            &dest_ty,
                            rvalue,
                        )
                    };

                let final_constraints =
                    self.pull_traitobjs_from_constraints(&maybe_trait_destty, constraints.clone());
                //if final_constraints != constraints {
                //    debug!("pull success");
                //} else {
                //    debug!("pull not needed");
                //}
                debug!("FINAL (PULLED) CONSTRAINTS: {:?}", final_constraints);

                // Add resolved constraints to istore
                istore.scoped_update(
                    cur_scope,
                    MapKey::Var(place.clone()),
                    Box::new(MapValue::Constraints(final_constraints)),
                );

                if let Some(field_places) = maybe_fields {
                    debug!("STORING FIELD PROJECTIONS TOO: {:?}", field_places);

                    // Store operand (field) constraints into projected places in istore
                    for field_place in field_places {
                        let final_op_constraints = self.pull_traitobjs_from_constraints(
                            &maybe_trait_destty,
                            field_place.1.clone(),
                        );
                        let field_place = Place {
                            local: place.local,
                            projection: field_place.0.clone(),
                        };

                        istore.link_adt_field(&(place.clone(), cur_scope.clone()), &field_place);

                        istore.scoped_update(
                            cur_scope,
                            MapKey::Var(field_place),
                            Box::new(MapValue::Constraints(final_op_constraints)),
                        );
                    }
                }
            }
            StatementKind::FakeRead(_, _)
            | StatementKind::StorageLive(_)
            | StatementKind::StorageDead(_) => {}
            StatementKind::Intrinsic(ndi) => match ndi {
                NonDivergingIntrinsic::Assume(_) => {}
                NonDivergingIntrinsic::CopyNonOverlapping(_) => todo!(),
            },
            _ => todo!("new statement kind: {:?}", &stmt.kind),
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
    ) -> Result<Option<ConstraintsAndFields>, Error> {
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
            TerminatorKind::Return => self.interp_return(logger, istore, call_stack, cur_scope),
            TerminatorKind::SwitchInt { discr, targets } => {
                self.interp_switchint(istore, cur_scope, local_decls, bb, bb_deps, discr, targets)
            }
            TerminatorKind::Assert { .. }
            | TerminatorKind::Drop { .. }
            | TerminatorKind::Goto { .. } => Ok(None),
            iasm @ TerminatorKind::InlineAsm {
                ..
                //template,
                //operands,
                //options,
                //line_spans,
                //destination,
                //unwind
            } => {
                debug!("iasm: {:#?}", iasm);
                // TODO do not interp, try to get rettype
                todo!("inline asm");
            }
            _ => todo!("other term kind: {:?}", &term.kind),
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
    ) -> Result<Option<ConstraintsAndFields>, Error> {
        //debug!("INTERPING INDIRECT CALL");

        //debug!("CHECKING FOR DYN IN PLACE TY");
        let dest_ty = place.ty(local_decls).unwrap();
        let maybe_trait_destty = self.contains_dyn(&dest_ty);
        //debug!("lval ty: {:?}", dest_ty);
        //debug!("dyn lval ty? {:?}", maybe_trait_destty);

        let mut ret_constraints = ConstraintsAndFields::empty(cur_scope.clone());
        match istore.scoped_get(cur_scope, &MapKey::Var(place.clone()), false) {
            Some(val) => match val {
                MapValue::Constraints(constraints) => {
                    for constraint in &constraints {
                        match constraint {
                            Constraint {
                                toc: _,
                                cfc: Some(cf),
                            } => match self.interp_constraint_as_fn(
                                logger,
                                term_span,
                                istore,
                                call_stack,
                                cur_scope,
                                local_decls,
                                &cf,
                                args,
                            ) {
                                Ok(Some(constraints)) => {
                                    ret_constraints.update(constraints);
                                }
                                Ok(None) => {}
                                e @ Err(_) => {
                                    panic!("interping constraint as fn, got error: {:?}", e)
                                }
                            },
                            _ => panic!("got traitobj when expected fn-type thing"),
                        }
                    }
                }
                _ => panic!("trying to indirectly call a scope - invalid"),
            },
            None => panic!("fnptr value not found in cmap"),
        }

        // Set destination local to value in cmap
        //debug!(
        //    "RET FROM INDIRECT FUNC CALL ret_constraints: {:?}",
        //    ret_constraints
        //);
        //log_scope(cur_scope);
        //debug!("destination: {:?}", destination);
        let constraints =
            self.pull_traitobjs_from_constraints(&maybe_trait_destty, ret_constraints.constraints);
        istore.scoped_update(
            cur_scope,
            MapKey::Var(destination.clone()),
            Box::new(MapValue::Constraints(constraints)),
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
        constraint: &RunningConstraint,
        args: &Vec<Operand>,
    ) -> Result<Option<ConstraintsAndFields>, Error> {
        match constraint {
            RunningConstraint::FnDef(fndef, genargs) => self.interp_fn_def(
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
            RunningConstraint::FnPtr(sigval) => self.interp_fn_ptr(
                logger,
                term_span,
                istore,
                call_stack,
                cur_scope,
                local_decls,
                sigval,
                args,
            ),
            RunningConstraint::Closure(cdef, genargs) => self.interp_closure(
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
        sigval: &SigVal,
        _args: &Vec<Operand>,
    ) -> Result<Option<ConstraintsAndFields>, Error> {
        //debug!("INTERPING FN PTR");
        //debug!("sigval!: {:?}", sigval);
        match self.sigstore.sigs.get(&sigval) {
            Some(fndef_vec) => {
                //debug!("num candidate fns: {:?}", fndef_vec.len());

                // Don't want to pollute our results too much, so if we have too
                // many possible fns to call, just fallback to retty
                if fndef_vec.len() > 5 {
                    debug!("TOO MANY CANDIDATES - falling back to retty");
                    self.retty_fallback_from_sigval(sigval)
                // Otherwise, interpret the candidate functions
                } else {
                    //debug!("INTERPRET CANDIDATES");
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
                //debug!("sigval: {:?}", sigval);
                //debug!("no candidate fns to call, using retty fallback");
                self.retty_fallback_from_sigval(sigval)
            }
        }
    }

    fn interp_closure(
        &self,
        logger: &mut VOLogger,
        term_span: &Span,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        cdef: ClosureDef,
        genargs: &GenericArgs,
        args: &Vec<Operand>,
    ) -> Result<Option<ConstraintsAndFields>, Error> {
        //debug!("INTERPING CLOSURE");
        //debug!("cdef: {:?}", cdef);
        //debug!("args: {:?}", args);
        //debug!("genargs: {:?}", genargs);
        let closure_kind = self.get_closure_kind(&genargs);
        //debug!("CLOSURE KIND: {:?}", closure_kind);
        if let Some(_body) = cdef.body() {
            let instance = Instance::resolve_closure(cdef, &genargs, closure_kind).unwrap();
            //debug!("instance: {:?}", instance);

            let new_scope = (instance, genargs.clone());
            let body = self.get_body(&new_scope);
            // FIXME body vs _body?

            let key = summary_key(
                self,
                new_scope.clone(),
                istore,
                term_span,
                cur_scope,
                &body,
                local_decls,
                args,
                true,
            );

            self.resolve_args(
                istore,
                term_span,
                cur_scope,
                &body,
                &new_scope,
                local_decls,
                args,
                &genargs,
                true,
            );
            self.prepare_call(call_stack, &key);
            self.visit_body(logger, istore, call_stack, &new_scope, &body)
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
    ) -> Result<Option<ConstraintsAndFields>, Error> {
        debug!("DIRECT CALL");

        //debug!("CHECKING FOR DYN IN PLACE TY");
        let dest_ty = destination.ty(local_decls).unwrap();
        let maybe_trait_destty = self.contains_dyn(&dest_ty);
        //debug!("lval ty: {:?}", dest_ty);
        //debug!("dyn lval ty? {:?}", maybe_trait_destty);

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
                RigidTy::FnPtr(poly_sig) => {
                    let sigval = SigVal::new_from_poly(&poly_sig);
                    self.interp_fn_ptr(
                        logger,
                        term_span,
                        istore,
                        call_stack,
                        cur_scope,
                        local_decls,
                        &sigval,
                        args,
                    )
                }
                other @ _ => todo!("different RigidTy: {:?}", other),
            },
            kind @ _ => todo!("funccall const is another kind: {:?}", kind),
        };

        // Set destination local to value in cmap
        debug!("RET FROM FUNC CALL ret_constraints: {:?}", ret_constraints);
        log_scope(cur_scope);
        debug!("destination: {:?}", destination);

        match ret_constraints {
            Ok(Some(constraints_)) => {
                // Make sure that if we're widening the concrete type of a function's return value
                // (e.g. assigning a Cat to a dyn Animal), we pull out the relevant traitobj info
                let constraints = self.pull_traitobjs_from_constraints(
                    &maybe_trait_destty,
                    constraints_.constraints.clone(),
                );
                istore.scoped_update(
                    cur_scope,
                    MapKey::Var(destination.clone()),
                    Box::new(MapValue::Constraints(constraints)),
                );

                // Propagate field projections by getting projections from the return value in the CALLEE's scope
                debug!("constraints.constraints: {:?}", constraints_.constraints);
                for field in &constraints_.fields {
                    debug!("field: {:?}", field);
                    match istore
                        .field_map
                        .get(&(field.clone(), constraints_.scope.clone()))
                    {
                        Some(places) => {
                            for place in places {
                                debug!("place: {:?}", place);
                                let field_constraints = istore
                                    .scoped_get(
                                        &constraints_.scope,
                                        &MapKey::Var(field.clone()),
                                        false,
                                    )
                                    .unwrap();
                                debug!("field constraints: {:?}", field_constraints);
                            }
                        }
                        None => todo!("ruh roh"),
                    }
                }
                todo!("fields: {:?}", constraints_.fields);
            }
            Ok(None) => {}
            err @ Err(Error::RecurseLimit(_)) => return err,
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
    ) -> Result<Option<ConstraintsAndFields>, Error> {
        debug!("trying to resolve instance");
        debug!("fndef: {:?}", fndef);
        debug!("genargs: {:?}", genargs);
        debug!("args: {:?}", args);
        let instance = match Instance::resolve(fndef, genargs) {
            Ok(instance_) => instance_,
            Err(_) => {
                // Support instances we can't resolve without more info
                // (i.e. this is a declaration, not an implementation,
                // We likely got here b/c we interpreted a trait func w a default implementation
                // that calls a trait func without a default implementation)
                //
                // The "more info" being:
                // - is this a trait method without an implementation?
                // - if so, who implements it? execute those implementations
                // This tends to be stuff that dynamic dispatch does for us anyway

                //let trait_ = self.tstore.assoc_fn_traits.get(&fndef.0).unwrap();
                //let structs = self.tstore.trait_structs.get(&trait_).unwrap();
                //for struct_ in structs {
                //    let impls = self.tstore.struct_assoc_fns.get(&(*struct_, fndef.0)).unwrap();
                //    debug!("impls: {:?}", impls);
                //}
                return self.interp_virtual_call(
                    logger,
                    term_span,
                    istore,
                    call_stack,
                    cur_scope,
                    local_decls,
                    &fndef,
                    &genargs,
                    args,
                );
            }
        };

        let new_scope = (instance, genargs.clone());
        debug!("--- CALLING {:?}", fndef);
        log_scope(cur_scope);
        //debug!("FNDEF GENARGS: {:?}", genargs);

        // checking for recursive stack depths of > 50
        if *self.rec_depth.borrow() > MAX_DEPTH {
            return Err(Error::RecurseLimit(MAX_DEPTH));
            // return self.retty_fallback_from_poly(fndef.fn_sig());
        }

        if !new_scope.0.has_body() {
            return self.dispatch_call(
                logger,
                term_span,
                istore,
                call_stack,
                cur_scope,
                &new_scope,
                local_decls,
                fndef,
                args,
                genargs,
                instance,
                Vec::new(),
            );
        }

        let new_cs: Vec<Constraints> = self
            .collect_resolved_args(
                istore,
                term_span,
                cur_scope,
                &self.get_body(&new_scope),
                local_decls,
                args,
                false,
            )
            .into_iter()
            .map(|(cs, _)| cs)
            .collect();

        if call_stack.contains(&new_scope) {
            let new_key = (new_scope.clone(), ArgSet::new(&new_cs));

            if let Some(cs) = self.summaries.borrow().get(&new_key).cloned() {
                debug!("\trecursive call, hit summary");
                return Ok(Some(cs));
            }

            debug!("\trecursive call, queueing...");

            let retty = self
                .retty_fallback_from_poly(fndef.fn_sig())?
                .unwrap_or_default();
            self.summaries
                .borrow_mut()
                .insert(new_key.clone(), retty.clone());

            let cur_key = self.key_stack.borrow().last().cloned().unwrap();

            self.wq.borrow_mut().entry(cur_key).or_default().push((
                new_scope.clone(),
                new_cs,
                call_stack.clone(),
            ));

            return Ok(Some(retty));
        }

        self.dispatch_call(
            logger,
            term_span,
            istore,
            call_stack,
            cur_scope,
            &new_scope,
            local_decls,
            fndef,
            args,
            genargs,
            instance,
            new_cs,
        )
    }

    fn dispatch_call(
        &self,
        logger: &mut VOLogger,
        term_span: &Span,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        new_scope: &VOID,
        local_decls: &[LocalDecl],
        fndef: FnDef,
        args: &Vec<Operand>,
        genargs: &GenericArgs,
        instance: Instance,
        new_cs: Vec<Constraints>,
    ) -> Result<Option<Constraints>, Error> {
        match instance.kind {
            InstanceKind::Item => {
                //debug!("regular static funccall");
                self.interp_static_call(
                    logger,
                    term_span,
                    istore,
                    call_stack,
                    cur_scope,
                    &new_scope,
                    local_decls,
                    fndef,
                    args,
                    &genargs,
                    &new_cs,
                )
            }
            InstanceKind::Virtual { .. } => {
                //debug!("virtual funccall");
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
                //debug!("shim funccall");
                self.interp_static_call(
                    logger,
                    term_span,
                    istore,
                    call_stack,
                    cur_scope,
                    &new_scope,
                    local_decls,
                    fndef,
                    args,
                    &genargs,
                    &new_cs,
                )
            }
            InstanceKind::Intrinsic => {
                //debug!("intrinsic funccall");
                self.retty_fallback_from_poly(fndef.fn_sig())
            }
        }
    }

    fn interp_static_call(
        &self,
        logger: &mut VOLogger,
        term_span: &Span,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        caller_scope: &VOID,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        fndef: FnDef,
        args: &Vec<Operand>,
        genargs: &GenericArgs,
        cur_cs: &Vec<Constraints>,
    ) -> Result<Option<Constraints>, Error> {
        debug!("INTERP STATIC CALL");
        debug!("fndef: {:?}", fndef);
        debug!("genargs: {:?}", genargs);

        if cur_scope.0.has_body() {
            debug!("HAS BODY");
            let body = self.get_body(cur_scope);
            let key = (cur_scope.clone(), ArgSet::new(cur_cs));
            self.resolve_args(
                istore,
                term_span,
                caller_scope,
                &body,
                cur_scope,
                local_decls,
                args,
                genargs,
                false,
            );
            self.prepare_call(call_stack, &key);
            self.visit_body(logger, istore, call_stack, cur_scope, &body)
        } else {
            // No body, so not visiting/updating call stack
            debug!("NO BODY");
            self.retty_fallback_from_poly(fndef.fn_sig())
        }
    }

    /// Create new subscope for the callee function, and put the resolved argument constraints into
    /// it. Then, if another subscope already exists for this callee, merge it with the new one and update
    fn resolve_args(
        &self,
        istore: &mut InterpStore,
        term_span: &Span,
        caller_scope: &VOID,
        body: &Body,
        callee_scope: &VOID,
        local_decls: &[LocalDecl],
        //fndef: FnDef,
        args: &Vec<Operand>,
        _genargs: &GenericArgs,
        is_closure: bool,
    ) {
        //debug!("RESOLVING ARGS");
        //debug!("IS TARGET FN CLOSURE?: {}", is_closure);

        let mut new_substore = InterpStore::new();
        let key = MapKey::ScopeId(callee_scope.clone());

        // Resolve generics + add arg values into new substore
        self.resolve_args_helper(
            istore,
            term_span,
            &mut new_substore,
            caller_scope,
            callee_scope,
            body,
            local_decls,
            args,
            is_closure,
        );

        // Merge new substore into existing substore at this scopeId
        let store;
        match istore.cmap.get(&key) {
            Some(box MapValue::Store(old_substore, old_es)) => {
                store = merge_stores(
                    &old_substore,
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

    pub fn collect_resolved_args(
        &self,
        istore: &InterpStore,
        term_span: &Span,
        caller_scope: &VOID,
        body: &Body,
        local_decls: &[LocalDecl],
        args: &Vec<Operand>,
        is_closure: bool,
    ) -> Vec<(Constraints, Option<Vec<(Place, Constraints)>>)> {
        let arg_count = body.arg_locals().len();
        let mut resolved = Vec::new();

        for (i, arg) in args.into_iter().enumerate() {
            let local = if is_closure { i + 2 } else { i + 1 };
            let place = Place {
                local,
                projection: vec![], // FIXME should this ever _not_ be empty?
            };

            //debug!("[CALL] CHECKING FOR DYN IN ARG TY");
            let maybe_trait_argty = if i > arg_count - 1 {
                //debug!("arg count is surpassed: {:?}", arg_count);
                None
            } else {
                let arg_ty = place.ty(body.locals()).unwrap();
                //debug!("arg ty: {:?}", arg_ty);
                self.contains_dyn(&arg_ty)
            };
            //debug!("(call) dyn arg ty? {:?}", maybe_trait_argty);

            resolved.push(self.resolve_arg(
                istore,
                term_span,
                caller_scope,
                &maybe_trait_argty,
                local_decls,
                arg,
                is_closure,
            ));
        }

        return resolved;
    }

    /// For each argument:
    /// - get the local to put constraints into
    /// - check if any arg types are traitobjects that we should translate existing concrete
    /// constraints into
    /// - resolve the argument into constraints (given constraints in our store)
    /// - update the substore for the callee
    fn resolve_args_helper(
        &self,
        istore: &mut InterpStore,
        term_span: &Span,
        new_substore: &mut InterpStore,
        caller_scope: &VOID,
        callee_scope: &VOID,
        body: &Body,
        local_decls: &[LocalDecl],
        args: &Vec<Operand>,
        is_closure: bool,
    ) {
        let resolved = self.collect_resolved_args(
            istore,
            term_span,
            caller_scope,
            body,
            local_decls,
            args,
            is_closure,
        );

        for (i, (constraints, maybe_fields)) in resolved.into_iter().enumerate() {
            debug!("\narg position: {:?}", i);
            let local = if is_closure { i + 2 } else { i + 1 };
            let place = Place {
                local,
                projection: vec![], // FIXME should this ever _not_ be empty?
            };

            let arg_ty = place.ty(body.locals()).unwrap();
            if let TyKind::RigidTy(RigidTy::Ref(_, _, mt)) = arg_ty.kind() {
                if let Operand::Copy(caller_place) | Operand::Move(caller_place) = &args[i] {
                    istore.add_ref(
                        (place.clone(), callee_scope.clone()),
                        (caller_place.clone(), caller_scope.clone()),
                        mt,
                    );
                }
            }

            debug!("resolved arg constraints: {:?}", constraints);
            debug!("arg place in new scope: {:?}\n", place);

            // Copy found constraints into new scope cmap
            new_substore.cmap.insert(
                MapKey::Var(place.clone()),
                Box::new(MapValue::Constraints(constraints)),
            );

            if let Some(fields) = maybe_fields {
                for field in fields {
                    let field_place = Place {
                        local: place.local,
                        projection: field.0.projection,
                    };
                    new_substore.cmap.insert(
                        MapKey::Var(field_place),
                        Box::new(MapValue::Constraints(field.1)),
                    );
                }
            }
        }
    }

    /// If constraints exist for the argument local, return those (potentially transforming/pulling
    /// out traitobj constraints if this arg contains a traitobj).
    /// If the arg is a constant, return the constraints gotten by converting the type into
    /// VerifOpt constraints.
    fn resolve_arg(
        &self,
        istore: &InterpStore,
        _term_span: &Span,
        caller_scope: &VOID,
        maybe_trait_argty: &Option<Vec<TraitObjTy>>,
        local_decls: &[LocalDecl],
        arg: &Operand,
        is_closure: bool,
    ) -> (Constraints, Option<Vec<(Place, Constraints)>>) {
        // FIXME implementation is similar to convert::convert_place()
        match arg {
            Operand::Copy(place) | Operand::Move(place) => {
                match self.get_place_constraints(
                    istore,
                    caller_scope,
                    maybe_trait_argty,
                    place,
                    is_closure,
                ) {
                    Some(constraints) => {
                        // Get any field projections
                        match istore.field_map.get(&(place.clone(), caller_scope.clone())) {
                            Some(field_places) => {
                                let mut fields = Vec::new();
                                for field_place in field_places {
                                    match self.get_place_constraints(
                                        istore,
                                        caller_scope,
                                        maybe_trait_argty,
                                        &field_place,
                                        is_closure,
                                    ) {
                                        Some(field_constraints) => {
                                            //debug!(
                                            //    "[ResolveArg] field_constraints: {:?}",
                                            //    field_constraints
                                            //);
                                            fields.push((
                                                // field place
                                                field_place.clone(),
                                                // field constraints
                                                field_constraints,
                                            ));
                                        }
                                        // No fields
                                        None => debug!("not a field"),
                                    }
                                }
                                if fields.is_empty() {
                                    (constraints, None)
                                } else {
                                    (constraints, Some(fields))
                                }
                            }
                            None => {
                                //debug!("NO FIELDS @ ({:?}, {:?})", place, caller_scope);
                                (constraints, None)
                            }
                        }
                    }
                    None => {
                        //debug!("place {:?} DNE in cmap, widen to type", place);
                        let (_maybe_traitobjty, constraint) = self
                            .converter
                            .convert_ty(&Location::new(), &place.ty(local_decls).unwrap());
                        (vec![constraint], None)
                    }
                }
            }
            // TODO can maybe get a more precise VORval depending on kind
            Operand::Constant(const_op) => (
                self.converter.convert_const(&Location::new(), &const_op),
                None,
            ),
            _ => todo!("runtime check arg"),
        }
    }

    fn get_place_constraints(
        &self,
        istore: &InterpStore,
        caller_scope: &VOID,
        maybe_trait_argty: &Option<Vec<TraitObjTy>>,
        place: &Place,
        is_closure: bool,
    ) -> Option<Constraints> {
        match istore.scoped_get(caller_scope, &MapKey::Var(place.clone()), is_closure) {
            Some(val) => match val {
                MapValue::Constraints(constraints_) => {
                    Some(self.pull_traitobjs_from_constraints(maybe_trait_argty, constraints_))
                }
                _ => panic!("arg is a scope"),
            },
            _ => None,
        }
    }

    fn check_sig_boundvars(&self, sig: &PolyFnSig) {
        if !sig.bound_vars.is_empty() {
            // Might not be safe to just skip binder
            //debug!("Bound vars - cannot just skip binder in call resolution");
            for bound_var in sig.bound_vars.iter() {
                match bound_var {
                    BoundVariableKind::Ty(_) => todo!("ty"),
                    BoundVariableKind::Const => todo!("const"),
                    BoundVariableKind::Region(_) => {}
                }
            }
        }
    }

    fn retty_fallback_from_poly(
        &self,
        sig: PolyFnSig,
    ) -> Result<Option<ConstraintsAndFields>, Error> {
        //debug!("fn_sig: {:?}", sig);
        self.check_sig_boundvars(&sig);
        //debug!("output: {:?}", sig.value.output());

        // Return output type that matches type info (widening)
        //let ret_constraints = vec![];
        todo!();
        //Ok(Some(ret_constraints))
    }

    fn retty_fallback_from_sigval(
        &self,
        sigval: &SigVal,
    ) -> Result<Option<ConstraintsAndFields>, Error> {
        //debug!("sigval: {:?}", sigval);
        if !sigval.bound_tys.is_empty() {
            todo!();
        }

        // Return output type that matches type info (widening)
        //let ret_constraints = vec![];
        todo!();
        //Ok(Some(ret_constraints))
    }

    /// Interpret dynamic dispatch.
    ///
    /// First determine the set of calls as determined by CHA.
    ///
    /// Then determine the set of calls as determined by FSA.
    ///
    /// Comparing these sets will help us determine where FSA might win over CHA (or other
    /// baselines TBD)
    ///
    /// Then continue interpretation given the FSA candidate function set.
    fn interp_virtual_call(
        &self,
        logger: &mut VOLogger,
        term_span: &Span,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        caller_scope: &VOID,
        //callee_scope: Option<VOID>,
        local_decls: &[LocalDecl],
        fndef: &FnDef,
        genargs: &GenericArgs,
        args: &Vec<Operand>,
    ) -> Result<Option<ConstraintsAndFields>, Error> {
        debug!("\nDYNAMIC CALL - fndef: {:?}\n", fndef);
        log_scope(caller_scope);
        debug!("args: {:?}", args);
        debug!("genargs: {:?}", genargs);

        // Get trait that this function is associated with
        // - tstore.assoc_fn_traits (Map<AssocFn, Trait>)
        let trait_defid = self.get_trait_defid(&fndef.0);
        debug!("trait_defid: {:?}", trait_defid);

        // Get concrete type constraints for trait object
        // - istore (FSA) / tstore (CHA / RTA)
        // Get every concrete type constraint's impl of this function
        // - tstore.struct_assoc_fns (Map<(Struct, Trait), FnImpls>)
        let assoc_fn_impls_cha;
        if let Some(cha_impls) = self.dispatch_cha.borrow().get(term_span) {
            assoc_fn_impls_cha = cha_impls.clone();
        } else {
            assoc_fn_impls_cha = self.get_impls_cha(&fndef.0, &trait_defid);
        }
        debug!(
            "CHA impls (len={:?}): {:?}",
            assoc_fn_impls_cha.len(),
            assoc_fn_impls_cha
        );

        for (cha_impl, _) in &assoc_fn_impls_cha {
            if *cha_impl == fndef.0 {
                debug!("CHA set has the virtual dyn function");
            }
        }

        let (is_closure, mut assoc_fn_impls_fsa) = self.get_impls_fsa(
            istore,
            term_span,
            caller_scope,
            &trait_defid,
            &fndef.0,
            args,
        );
        debug!(
            "FSA impls (len={:?}): {:?}",
            assoc_fn_impls_fsa.len(),
            assoc_fn_impls_fsa
        );

        for fsa_impl in &assoc_fn_impls_fsa {
            if !assoc_fn_impls_cha.contains(&fsa_impl) {
                error!("CHA missing impl: {:?}", fsa_impl);
            }
        }

        if assoc_fn_impls_fsa.is_empty() {
            debug!("nothing to call, FSA set is empty, falling back to CHA");
            assoc_fn_impls_fsa = assoc_fn_impls_cha.clone();
        }

        // Log CHA vs FSA diffs
        if assoc_fn_impls_cha != assoc_fn_impls_fsa {
            debug!(
                "\n\nDYNAMIC DISPATCH - SET OF IMPLS DIFFER [Trait {:?}]: (CHA:FSA) = ({:?}:{:?})\tFNDEF = {:?}\n",
                trait_defid,
                assoc_fn_impls_cha.len(),
                assoc_fn_impls_fsa.len(),
                fndef,
            );
        } else {
            debug!(
                "\n\nDYNAMIC DISPATCH - SET OF IMPLS SAME [Trait {:?}]: (CHA:FSA) = ({:?}:{:?})\tFNDEF = {:?}\n",
                trait_defid,
                assoc_fn_impls_cha.len(),
                assoc_fn_impls_fsa.len(),
                fndef,
            );
        }
        debug!("term_span: {:?}", term_span);

        self.dispatch_cha
            .borrow_mut()
            .entry(*term_span)
            .or_insert_with(|| assoc_fn_impls_cha.clone());

        // collect possible calls (mostly for recursion)
        let dt = &mut self.dispatch_targets.borrow_mut();
        let entry = dt.entry(*term_span).or_default();
        for f in &assoc_fn_impls_fsa {
            if !entry.contains(f) {
                entry.push(f.clone());
            }
        }

        let ds = &mut self.dependencies.borrow_mut();
        let entry = ds.entry(*term_span).or_default();
        for c in call_stack.iter() {
            entry.insert(c.clone());
        }

        self.simulate_static_calls(
            logger,
            term_span,
            istore,
            call_stack,
            caller_scope,
            local_decls,
            assoc_fn_impls_fsa,
            genargs,
            args,
            is_closure,
        )
    }

    fn get_trait_defid(&self, assoc_fn_defid: &DefId) -> DefId {
        // Get trait that this function is associated with
        match self.tstore.assoc_fn_traits.get(assoc_fn_defid) {
            Some(trait_defid) => *trait_defid,
            None => panic!("assoc fn {:?} does not point to trait", assoc_fn_defid),
        }
    }

    /// Returns a set of candidate functions (implementation DefIds) given an input set of types
    /// (constraint DefIds)
    ///
    /// If there are no candidates based on input constraints, and this is on the FSA path, add the default
    /// implementation to the returned candidate function set, if there exists one.
    /// For CHA, add the default implementation (if it exists) no matter what.
    fn get_impls_from_defids(
        &self,
        //_callee_scope: &VOID,
        assoc_fn_defid: &DefId,
        constraint_defids: &Vec<(DefId, Option<GenericArgs>)>,
        _fsa: bool,
    ) -> Vec<(DefId, Option<GenericArgs>)> {
        //debug!("\nGETTING IMPL DEFIDS FROM TYPE DEFIDS");

        let mut assoc_fn_impls = Vec::new();

        // CHA-collected defid genargs are None, while FSA-collected defid genargs might be Some().
        // Get every concrete type constraint's impl of this function
        for (i, (defid, genargs)) in constraint_defids.iter().enumerate() {
            debug!("i: {:?}", i);
            debug!("defid: {:?}", defid);
            debug!("genargs: {:?}", genargs);
            match self.tstore.struct_assoc_fns.get(&(*defid, *assoc_fn_defid)) {
                Some(assoc_fn_impl) => {
                    debug!("assoc_fn_impl: {:?}", assoc_fn_impl);
                    unique_append(
                        &mut assoc_fn_impls,
                        assoc_fn_impl
                            .clone()
                            .into_iter()
                            .map(|x| (x, genargs.clone()))
                            .collect(),
                    );
                }
                None => {
                    if FnDef(*defid).body().is_some() {
                        // This is a callable item, push the impl defid
                        debug!("callable defid! {:?}", defid);
                        unique_push(&mut assoc_fn_impls, (*defid, genargs.clone()));
                    }
                }
            }
        }

        // If the supposedly-virtual function we want to call has a body, and there were some
        // candidate objects that do not implement said function, and this is FSA
        // OR if this is CHA, then add this "default" implmentation
        //if FnDef(*assoc_fn_defid).body().is_some() && (!constraint_defids.is_empty() || !fsa) {
        //    debug!("ADDING DEFAULT IMPL");
        //    unique_push(&mut assoc_fn_impls, (*assoc_fn_defid, None));
        //}

        assoc_fn_impls
    }

    fn get_impls_cha(
        &self,
        //callee_scope: &VOID,
        assoc_fn_defid: &DefId,
        trait_defid: &DefId,
    ) -> Vec<(DefId, Option<GenericArgs>)> {
        debug!("\n\nGETTING CHA IMPLS");
        let constraint_defids = self.get_cha_tyconstraint_defids(&trait_defid);
        debug!(
            "constraint defids ({:?} total): {:?}",
            constraint_defids.len(),
            constraint_defids
        );
        self.get_impls_from_defids(assoc_fn_defid, &constraint_defids, false)
    }

    fn get_cha_tyconstraint_defids(
        &self,
        trait_defid: &DefId,
    ) -> Vec<(DefId, Option<GenericArgs>)> {
        // Get concrete type constraints for trait object
        match self.tstore.trait_structs.get(trait_defid) {
            Some(tyconstraints) => tyconstraints
                .into_iter()
                .map(|x| (x.clone(), None))
                .collect(),
            None => panic!("trait {:?} does not point to any structs", trait_defid),
        }
    }

    fn get_impls_fsa(
        &self,
        istore: &InterpStore,
        term_span: &Span,
        caller_scope: &VOID,
        //callee_scope: &VOID,
        trait_defid: &DefId,
        assoc_fn_defid: &DefId,
        args: &Vec<Operand>,
    ) -> (bool, Vec<(DefId, Option<GenericArgs>)>) {
        debug!("\n\nGETTING FSA IMPLS");
        let place = self.get_traitobj_place(args);
        debug!("traitobj place: {:?}", place);
        let tyconstraints = self.get_fsa_tyconstraints(istore, caller_scope, place);
        debug!("tyconstraints: {:?}", tyconstraints);
        let (is_closure, constraint_defids) =
            self.get_fsa_constraint_defids(term_span, trait_defid, &tyconstraints);
        debug!(
            "constraint defids ({:?} total): {:?}",
            constraint_defids.len(),
            constraint_defids
        );
        (
            is_closure,
            self.get_impls_from_defids(assoc_fn_defid, &constraint_defids, true),
        )
    }

    fn get_traitobj_place(&self, args: &Vec<Operand>) -> Place {
        match &args[0] {
            Operand::Copy(place) | Operand::Move(place) => {
                if !place.projection.is_empty() {
                    panic!("traitobj place has projections");
                }

                place.clone()
            }
            _ => panic!("unexpected operand: {:?}", args[0]),
        }
    }

    fn get_fsa_tyconstraints(
        &self,
        istore: &InterpStore,
        caller_scope: &VOID,
        place: Place,
    ) -> Constraints {
        // Get concrete type constraints for trait object
        match istore.scoped_get(caller_scope, &MapKey::Var(place.clone()), false) {
            Some(val) => match val {
                MapValue::Constraints(tyconstraints) => tyconstraints,
                MapValue::Store(..) => panic!("place {:?} refers to a scope", place),
            },
            None => panic!("place {:?} has no constraints", place),
        }
    }

    /// For each concrete type constraint, if it contains a type that implements the trait of the
    /// traitobject we are dispatching on, return that type's DefId (FIXME remove: along with its generic args)
    ///
    /// This will later be used to get that type's implementation of the function-to-dispatch
    fn get_fsa_constraint_defids(
        &self,
        term_span: &Span,
        trait_defid: &DefId,
        tyconstraints: &Constraints,
    ) -> (bool, Vec<(DefId, Option<GenericArgs>)>) {
        let mut defids = Vec::new();
        let mut is_closure = false;
        //debug!("\nGETTING CONSTRAINT DEFIDS");
        for constraint in tyconstraints {
            //debug!("constraint: {:?}", constraint);
            let (is_closure_, res) = self.resolve_defid(term_span, trait_defid, constraint);
            is_closure = is_closure_;
            unique_append(&mut defids, res);
        }
        (is_closure, defids)
    }

    /// If a concrete type constraint contains a type that implements the trait of the
    /// traitobject we are dispatching on, return that type's DefId (FIXME remove: along with its generic args)
    ///
    /// This will later be used to get that type's implementation of the function-to-dispatch
    fn resolve_defid(
        &self,
        term_span: &Span,
        trait_defid: &DefId,
        constraint: &Constraint,
    ) -> (bool, Vec<(DefId, Option<GenericArgs>)>) {
        //debug!("\nRESOLVE DEFID");
        match constraint {
            Constraint {
                toc: Some(toc_),
                cfc: _,
            } => {
                if *trait_defid != toc_.0.def.0 {
                    todo!("traits don't match");
                }

                match toc_ {
                    (_, TraitObjConstraint::Adt(adtdef, genargs)) => {
                        self.resolve_adt_helper(term_span, trait_defid, adtdef, genargs)
                    }
                    (_, TraitObjConstraint::Closure(cdef, genargs)) => {
                        if genargs.0.is_empty() {
                            (true, vec![(cdef.0, None)])
                        } else {
                            (true, vec![(cdef.0, Some(genargs.clone()))])
                        }
                    }
                }
            }
            // FIXME by the time we get here, shouldn't TOC be correctly populated? why are we
            // looking inside CFC?
            Constraint {
                toc: None,
                cfc: Some(_cfc),
            } => {
                todo!("TOC not correctly populated");
                /*
                match cfc {
                    RunningConstraint::Adt(adtdef, genargs) => {
                        self.resolve_adt_helper(term_span, trait_defid, adtdef, genargs)
                    }
                    RunningConstraint::Closure(cdef, genargs) => {
                        (true, vec![(cdef.0, Some(genargs.clone()))])
                    }
                    RunningConstraint::Scalar(_) => (false, vec![]),
                    // If this is truly a Dynamic constraint that we cannot resolve to any concrete
                    // types, then return nothing here so that if needed, we may fallback to
                    // another, coarser resolution mechanism
                    RunningConstraint::Dynamic(_) => (false, vec![]),
                    _ => todo!("{:?}", cfc),
                }
                */
            }
            _ => {
                panic!("no constraints");
            }
        }
    }

    fn resolve_adt_helper(
        &self,
        term_span: &Span,
        trait_defid: &DefId,
        adtdef: &AdtDef,
        genargs: &GenericArgs,
    ) -> (bool, Vec<(DefId, Option<GenericArgs>)>) {
        let mut resvec = Vec::new();
        match self.tstore.struct_traits.get(&adtdef.0) {
            // Does this ADT implement the desired trait? If so, add to vec
            Some(traits) => {
                if traits.contains(trait_defid) {
                    if genargs.0.is_empty() {
                        unique_push(&mut resvec, (adtdef.0, None));
                    } else {
                        unique_push(&mut resvec, (adtdef.0, Some(genargs.clone())));
                    }
                }
            }
            None => {}
        }

        // Also search in genargs for an implementing type
        for genarg in &genargs.0 {
            match self.converter.convert_genarg(&Location::new(), &genarg) {
                Some(genarg_constraint) => {
                    let (_is_closure, inner_resvec) =
                        self.resolve_defid(term_span, trait_defid, &genarg_constraint);
                    unique_append(&mut resvec, inner_resvec);
                }
                _ => {}
            }
        }

        (false, resvec)
    }

    /// For each of the FSA candidate functions to call, resolve into a monomorphic
    /// instance and interpret as if it were a static call
    fn simulate_static_calls(
        &self,
        logger: &mut VOLogger,
        term_span: &Span,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        assoc_fn_impls: Vec<(DefId, Option<GenericArgs>)>,
        method_genargs: &GenericArgs,
        args: &Vec<Operand>,
        is_closure: bool,
    ) -> Result<Option<ConstraintsAndFields>, Error> {
        let mut results = Vec::<Option<ConstraintsAndFields>>::new();
        let mut istore_vec = Vec::new();

        debug!("\nSIMULATING STATIC CALL(S)");
        let len = assoc_fn_impls.len();

        for (i, (assoc_fn_impl, adt_genargs)) in assoc_fn_impls.iter().enumerate() {
            debug!(
                "\n---ITER {:?} out of {:?} ({:?}/{:?})",
                i,
                len - 1,
                i + 1,
                len
            );
            debug!("assoc_fn_impl defid: {:?}", assoc_fn_impl);
            debug!("adt genargs: {:?}", adt_genargs);
            debug!("method genargs: {:?}", method_genargs);

            let genargs = if is_closure && adt_genargs.is_some() {
                debug!("--concatenating adt + method genargs");
                GenericArgs(
                    adt_genargs
                        .clone()
                        .unwrap()
                        .0
                        .iter()
                        .chain(method_genargs.0.iter())
                        .cloned()
                        .collect(),
                )
            } else if !is_closure && adt_genargs.is_some() {
                debug!("--adt genargs only");
                adt_genargs.clone().unwrap()
            } else {
                debug!("--method genargs only");
                method_genargs.clone()
            };
            debug!("TOTAL genargs: {:?}", genargs);

            // TODO different resolves for fn_ptr / closure

            let fndef = FnDef(*assoc_fn_impl);
            let instance_ = Instance::resolve(fndef, &genargs).unwrap();
            //debug!("og instance: {:?}", instance_);
            let (is_virtual, instance) = match instance_.kind {
                // Likely a default trait method implementation, convert to a concrete InstanceKind
                // so we can interpret it
                InstanceKind::Virtual { .. } => {
                    debug!("virtual instance! (default impl)");
                    (
                        true,
                        Instance {
                            kind: InstanceKind::Item,
                            def: instance_.def,
                        },
                    )
                }
                _ => {
                    debug!("non-virtual instance: {:?}", instance_.kind);
                    (false, instance_)
                }
            };
            debug!("a converted static instance: {:?}", instance);
            let callee_scope = (instance, genargs.clone());

            if call_stack.contains(&callee_scope) {
                debug!("\tpossible infinite call!");
                results.push(self.retty_fallback_from_poly(fndef.fn_sig()).unwrap());
            } else {
                let mut istore_clone = istore.clone();
                let mut call_stack_clone = call_stack.clone();

                if !instance.has_body() {
                    panic!("instance has no body");
                }
                let body = if is_virtual {
                    // FIXME not monomorphized
                    debug!("getting default func body");
                    fndef.body().unwrap()
                } else {
                    //debug!("getting func body");
                    self.get_body(&callee_scope)
                };

                let key = summary_key(
                    self,
                    callee_scope.clone(),
                    istore,
                    term_span,
                    cur_scope,
                    &body,
                    local_decls,
                    args,
                    is_closure,
                );

                self.resolve_args(
                    &mut istore_clone,
                    term_span,
                    cur_scope,
                    &body,
                    &callee_scope,
                    local_decls,
                    args,
                    &genargs,
                    is_closure,
                );
                self.prepare_call(&mut call_stack_clone, &key);
                results.push(self.visit_body(
                    logger,
                    &mut istore_clone,
                    &mut call_stack_clone,
                    &callee_scope,
                    &body,
                )?);

                istore_vec.push(istore_clone);
            }
        }

        self.merge_istores_and_set(istore, &mut istore_vec);
        self.merge_results_and_ret(&mut results)
    }

    fn merge_istores_and_set(&self, istore: &mut InterpStore, istore_vec: &mut Vec<InterpStore>) {
        match istore_vec.merge() {
            Ok(Some(merged_istore)) => {
                *istore = merged_istore;
            }
            Ok(None) => panic!("istores empty?"),
            Err(_) => panic!(),
        }
    }

    fn merge_results_and_ret(
        &self,
        results: &mut Vec<Option<ConstraintsAndFields>>,
    ) -> Result<Option<ConstraintsAndFields>, Error> {
        // Filter out None constraints and unwrap all Some options
        let filtered_results: Vec<ConstraintsAndFields> = results
            .into_iter()
            .filter(|option| option.is_some())
            .map(|x| x.clone().unwrap())
            .collect();

        match filtered_results.merge() {
            Ok(Some(merged_constraints)) => {
                return Ok(Some(merged_constraints));
            }
            Ok(None) => Ok(None),
            Err(_) => panic!(),
        }
    }

    fn interp_switchint(
        &self,
        istore: &mut InterpStore,
        cur_scope: &VOID,
        _local_decls: &[LocalDecl],
        bb: usize,
        bb_deps: &mut BBDeps,
        discr: &Operand,
        targets: &SwitchTargets,
    ) -> Result<Option<ConstraintsAndFields>, Error> {
        //debug!("SWITCHINT");
        //debug!("discr: {:?}", discr);
        //debug!("targets: {:?}", targets);

        match discr {
            Operand::Copy(place) | Operand::Move(place) => {
                match istore.scoped_get(cur_scope, &MapKey::Var(place.clone()), false) {
                    Some(val) => match val {
                        MapValue::Constraints(constraints) => {
                            // Create a byte-map for finding statically-impossible successors
                            let mut discr_vals_uninit =
                                Box::<[u8]>::new_zeroed_slice(targets.len());
                            let discr_vals = discr_vals_uninit.write_filled(0);
                            //debug!("PRE discr_vals: {:?}", discr_vals);

                            // Check expected type of discriminant
                            //debug!("expected ty: {:?}", place.ty(local_decls));
                            //debug!("constraints: {:?}", constraints);

                            // Populate byte-map with possible branch values, based on constraints
                            self.set_bytemap(&constraints, targets, discr_vals);
                            //debug!("POST discr_vals: {:?}", discr_vals);

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
                        //debug!("local DNE, cannot prune any targets");
                    }
                }
            }
            Operand::Constant(_co) => {}
            _ => {} //debug!("runtime checks op (ignoring)"),
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
            // Increment all branch counters (since no statically-known discr value)
            for (i, _) in targets.branches().enumerate() {
                discr_vals[usize::try_from(i).unwrap()] += 1;
            }
            discr_vals[discr_vals.len() - 1] += 1;
            return;
        }

        //debug!("USING CONSTRAINTS TO SET BYTEMAP");
        for constraint in constraints {
            //debug!("setting bytemap for constraint: {:?}", constraint);
            match constraint {
                Constraint {
                    toc: _,
                    cfc: Some(RunningConstraint::Scalar(num_opt)),
                } => {
                    //debug!("scalar discr val: {:?}", num_opt);

                    if let Some(num) = num_opt {
                        // Increment matching branch counters
                        let mut set = false;
                        for (i, (val, _bb)) in targets.branches().enumerate() {
                            if *num == val.try_into().unwrap() {
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
        let prunable_indices_opt = self.get_prunable_indices(discr_vals);

        if let Some(prunable_indices) = prunable_indices_opt {
            // Some prunable items point to bbs that are also pointed to by non-prunable items,
            // so need to check that prunable bbs are ONLY pointed to by prunable items
            let mut keep_targets = Vec::new();
            let mut prunable_targets = Vec::new();

            // First collect all targets to keep
            for (i, target) in targets.iter().enumerate() {
                if !prunable_indices.contains(&i) {
                    keep_targets.push(target.clone());
                }
            }

            // Then collect all targets we might be able to prune
            for prune_idx in prunable_indices {
                prunable_targets.push(targets[prune_idx]);
            }

            // Finally, prune all targets that are _not_ in the list of targets to keep
            for prunable_target in prunable_targets {
                if keep_targets.contains(&prunable_target) {
                    continue;
                }

                bb_deps.prune(bb, prunable_target);
            }
        }
    }

    fn get_prunable_indices(&self, discr_vals: &mut [u8]) -> Option<Vec<usize>> {
        // If the last value (otherwise branch) is > 0, cannot prune anything
        // if discr_vals[discr_vals.len() - 1] > 0 {
        //     return None;
        // }

        let mut poss_idxs = Vec::new();
        let mut imposs_idxs = Vec::new();
        for i in 0..discr_vals.len() {
            if discr_vals[i] > 0 {
                poss_idxs.push(i);
            } else {
                imposs_idxs.push(i);
            }
        }

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

    fn reinterp_recursive(
        &self,
        logger: &mut VOLogger,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        callee_scope: &VOID,
        callee_cs: &[Constraints],
    ) -> Result<Option<Constraints>, Error> {
        let mut substore = InterpStore::new();
        for (i, cs) in callee_cs.iter().enumerate() {
            let place = Place {
                local: i + 1,
                projection: vec![],
            };
            substore.cmap.insert(
                MapKey::Var(place.clone()),
                Box::new(MapValue::Constraints(cs.clone())),
            );
        }
        istore.cmap.insert(
            MapKey::ScopeId(callee_scope.clone()),
            Box::new(MapValue::Store(substore, None)),
        );

        let key = (callee_scope.clone(), ArgSet::new(&callee_cs));
        self.prepare_call(call_stack, &key);
        let body = self.get_body(callee_scope);
        let refined = self.visit_body(logger, istore, call_stack, callee_scope, &body)?;

        // publish refined summary for widened constraints
        self.summaries
            .borrow_mut()
            .insert(key.clone(), refined.clone().unwrap_or_default());

        Ok(refined)
    }

    fn interp_return(
        &self,
        logger: &mut VOLogger,
        istore: &mut InterpStore,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
    ) -> Result<Option<Constraints>, Error> {
        debug!("\n\n\n#############################");
        debug!("RETURNING from scope {:?}...", cur_scope.0.name());
        log_call_stack(call_stack);
        debug!("#############################\n\n");

        let ret_place = Place {
            local: 0,
            projection: vec![],
        };
        let projections = istore
            .field_map
            .get(&(ret_place.clone(), cur_scope.clone()))
            .unwrap();
        debug!("projections?: {:?}", projections);

        // Get and "return" the constraints at Place(0)
        let retval = match istore.scoped_get(cur_scope, &MapKey::Var(ret_place), false) {
            Some(retval) => match retval {
                MapValue::Constraints(retval_constraints) => {
                    debug!("\n###### RETURNING constraints:");
                    debug!("\t{:?}\n\n", retval_constraints);
                    Some(retval_constraints)
                }
                _ => panic!("should not be returning a scope"),
            },
            None => {
                // TODO Double check that nothing _needs_ to be returned (for interp correctness)
                None
            }
        };

        let key = self.key_stack.borrow().last().cloned().unwrap();

        let old_scope = self.prepare_return(call_stack);
        if old_scope.clone().unwrap() != *cur_scope {
            //debug!("\nold_scope: {:?}", old_scope.unwrap().0.name());
            //debug!("cur_scope: {:?}", cur_scope.0.name());
            log_call_stack(call_stack);
            panic!("call stack out of sorts");
        }

        let queued = self.wq.borrow_mut().remove(&key).unwrap_or_default();

        if self.in_queue.borrow().contains(&key) || queued.is_empty() {
            // use summary version OR
            // no recursive calls, no recursed interp needed
            return Ok(retval);
        }

        // about to be queueing, set to prevent infinite recursion
        self.in_queue.borrow_mut().insert(key.clone());

        // janky method to preserve stores conflicting on voids but not keys
        let saved: Vec<(VOID, Option<Box<MapValue>>)> = queued
            .iter()
            .map(|(scope, _, _)| {
                (
                    scope.clone(),
                    istore.cmap.get(&MapKey::ScopeId(scope.clone())).cloned(),
                )
            })
            .collect();

        *self.rec_depth.borrow_mut() += 1;
        for (scope, constraints, stack) in queued {
            debug!("\treinterp queued recursive {:?}", scope.0.name());

            let depth = call_stack.len();

            // reevaluate recursive calls
            let restored = stack;
            let res = self.reinterp_recursive(
                logger,
                istore,
                &mut restored.clone(),
                &scope,
                &constraints,
            );

            if matches!(res, Err(Error::RecurseLimit(_))) {
                // truncate to before call on error
                call_stack.truncate(depth);
                self.key_stack.borrow_mut().truncate(depth);

                self.incomplete.borrow_mut().insert(cur_scope.clone());

                *self.rec_depth.borrow_mut() -= 1;
                return res;
            }
        }
        *self.rec_depth.borrow_mut() -= 1;

        for (scope, old) in saved {
            match old {
                Some(v) => {
                    istore.cmap.insert(MapKey::ScopeId(scope), v);
                }
                None => {
                    istore.cmap.remove(&MapKey::ScopeId(scope));
                }
            }
        }

        // final traverse after recursive wq drained
        let body = self.get_body(cur_scope);

        // constrain arguments
        let mut substore = InterpStore::new();
        for (i, arg_set) in key.1.args.iter().enumerate() {
            let place = Place {
                local: i + 1,
                projection: vec![],
            };
            let cs: Constraints = arg_set.iter().cloned().collect();
            substore
                .cmap
                .insert(MapKey::Var(place), Box::new(MapValue::Constraints(cs)));
        }
        istore.cmap.insert(
            MapKey::ScopeId(cur_scope.clone()),
            Box::new(MapValue::Store(substore, None)),
        );

        self.prepare_call(call_stack, &key);
        let res = self.visit_body(logger, istore, call_stack, cur_scope, &body);

        res
    }
}
