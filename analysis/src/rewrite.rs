use rustc_middle::mir::*;
use rustc_middle::ty::*;
use rustc_span::def_id::{CrateNum, DefId, DefIndex};
use rustc_span::source_map::Spanned;
use rustc_span::{BytePos, Span, SyntaxContext};

use crate::FuncMap;
use crate::constraints::{ConstraintMap, MapKey, VarType};
use crate::core::VerifoptRval;
use crate::core::is_box;
use crate::patch::MirPatch;

// FIXME get dynamically
const INTO_RAW_FN_DEFID: DefId = DefId {
    //index: DefIndex::from_u32(731),
    index: DefIndex::from_u32(749),
    krate: CrateNum::from_u32(3),
};
const EQ_FN_DEFID: DefId = DefId {
    index: DefIndex::from_u32(3216),
    krate: CrateNum::from_u32(2),
};

pub struct RewritePass<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub funcs: &'a FuncMap<'tcx>,
    pub cmap: &'a ConstraintMap<'tcx>,
    pub debug: bool,
}

impl<'a, 'tcx> RewritePass<'a, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        funcs: &'a FuncMap<'tcx>,
        cmap: &'a ConstraintMap<'tcx>,
        debug: bool,
    ) -> RewritePass<'a, 'tcx> {
        Self {
            tcx,
            funcs,
            cmap,
            debug,
        }
    }

    pub fn run(&self, cur_scope: DefId, body: &mut Body<'tcx>) {
        self.visit_body(cur_scope, body);
    }

    pub fn visit_body(&self, cur_scope: DefId, body: &mut Body<'tcx>) {
        if self.debug {
            println!("#############################");
            println!("IN SCOPE: {:?}", cur_scope);
            println!("#############################");
        }
        let mut patch = MirPatch::new(body);
        for (bb, data) in body.basic_blocks.iter_enumerated() {
            // TODO add some notion of WTO
            if data.is_cleanup {
                continue;
            }
            //for stmt in &data.statements {
            //    match &stmt.kind {
            //        StatementKind::Assign(_boxed) => {
            //            //println!("PLACE BEFORE: {:?}", boxed.0);
            //            //println!("rval: {:?}", boxed.1);
            //        }
            //        _ => {}
            //    }
            //}
            match &data.terminator().kind {
                TerminatorKind::Call {
                    func,
                    args: _,
                    destination,
                    ..
                } => {
                    //println!("PLACE BEFORE (term): {:?}", destination);
                    if let Some((defid, genargs)) = func.const_fn_def() {
                        if genargs.len() == 0 {
                            continue;
                        }

                        if self.debug {
                            println!("\n# -------------------------------");
                            println!("# CALL term: {:?}", defid);
                            println!("# -------------------------------");
                            println!("genargs: {:?}", genargs);
                            println!("printed as str: {:?}", genargs.print_as_list());
                        }

                        let genargs_slice = genargs.as_slice();
                        let first = genargs_slice[0];
                        if let Some(ty) = first.as_type() {
                            if ty.is_trait() {
                                if !self.tcx.def_path_debug_str(defid).contains("Animal::speak") {
                                    if self.debug {
                                        println!("SKIPPING: {:?}", defid);
                                    }
                                    continue;
                                }

                                if self.debug {
                                    println!("\nFIRST ARG == TRAIT");
                                    println!("arg: {:?}", first);
                                    println!("func: {:?}", func);
                                    println!("func_defid: {:?}", defid);
                                }

                                let num_bbs = body.basic_blocks.len();
                                self.replace_dynamic_dispatch(
                                    cur_scope,
                                    &mut patch,
                                    &defid,
                                    ty,
                                    *destination,
                                    bb,
                                    data,
                                    num_bbs,
                                );
                            }
                        } else {
                            if self.debug {
                                println!("first arg no type, continuing...");
                            }
                            continue;
                        }
                    }
                }
                // TODO also scan _called_ bodies + rewrite
                _ => {}
            }
        }

        patch.apply(body);
    }

    fn dummy_span(&self) -> Span {
        Span::new(BytePos(0), BytePos(0), SyntaxContext::root(), None)
    }

    fn dummy_source_info(&self) -> SourceInfo {
        SourceInfo {
            span: self.dummy_span(),
            // FIXME use for scoping!
            scope: SourceScope::ZERO,
        }
    }

    fn get_bbs(&self, data: &BasicBlockData<'tcx>) -> (BasicBlock, BasicBlock) {
        let edges = data.terminator().kind.edges();
        let bb_old_next;
        let bb_old_cleanup;
        match edges {
            TerminatorEdges::AssignOnReturn {
                return_, cleanup, ..
            } => {
                if return_.len() > 1 {
                    panic!("RET: multiple return blocks");
                }
                if cleanup.is_none() {
                    panic!("CLN: no cleanup");
                }
                bb_old_next = return_[0];
                bb_old_cleanup = cleanup.unwrap();
            }
            _ => {
                panic!("verifopt: need to set terminator edges");
            }
        }
        (bb_old_next, bb_old_cleanup)
    }

    //fn dyndispatch_retval(
    //    &self,
    //    old_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
    //    term_dst_place: Place<'tcx>,
    //) -> bool {
    //    if old_locals.get(term_dst_place.local).is_some() {
    //        true
    //    } else {
    //        false
    //    }
    //}

    fn get_traitobj_did(&self, ty: Ty<'tcx>) -> DefId {
        let traitobj_did: Option<DefId>;
        match ty.kind() {
            TyKind::Dynamic(rawlist, ..) => {
                if rawlist.len() > 0 {
                    let principal_did_opt = (*rawlist).principal_def_id();
                    if let Some(did) = principal_did_opt {
                        traitobj_did = Some(did);
                    } else {
                        panic!("auto traits only - nothing to replace");
                    }
                } else {
                    traitobj_did = None;
                }
            }
            // realistically can just return, but panicking for now to see
            // if this is ever triggered
            _ => panic!("trait is not Dynamic"),
        }
        if traitobj_did.is_none() {
            panic!("no traitobj_did found");
        }
        traitobj_did.unwrap()
    }

    fn replace_dyndispatch_term_w_goto(
        &self,
        patch: &mut MirPatch<'tcx>,
        cur_bb: BasicBlock,
        new_start: BasicBlock,
    ) {
        // replace term w goto
        patch.patch_terminator(cur_bb, TerminatorKind::Goto { target: new_start });
    }

    fn resolve_first_arg_constraints(&self, first_arg_constraint: &VerifoptRval<'tcx>) -> DefId {
        match first_arg_constraint {
            VerifoptRval::IdkStruct(struct_defid, genarg_vec) => {
                if is_box(*struct_defid) {
                    if let Some(genargs_outer) = genarg_vec {
                        if genargs_outer.len() != 1 {
                            panic!("handle diff genarg len");
                        }
                        let genarg_constraint_vec = &genargs_outer[0];
                        if genarg_constraint_vec.len() != 1 {
                            panic!("handle different genarg constraint len");
                        }
                        let genarg_constraint = &genarg_constraint_vec[0];
                        match genarg_constraint {
                            VerifoptRval::IdkStruct(defid, _) => {
                                if self.debug {
                                    println!("found struct: {:?}", defid);
                                }
                                return *defid;
                            }
                            _ => todo!(),
                        }
                    } else {
                        panic!("no genargs in box...");
                    }
                } else {
                    todo!("struct is not a box: {:?} (\ngenargs: {:?})", struct_defid, genarg_vec);

                }
            }
            VerifoptRval::Ref(boxed) => self.resolve_first_arg_constraints(&*boxed),
            _ => todo!("handle more kinds: {:?}", first_arg_constraint),
        }
    }

    // get the structs that implement the trait being pointed to by the first argument
    fn get_struct_dids(&self, cur_scope: DefId) -> Vec<DefId> {
        let mut structs = vec![];
        // FIXME test simple with local10 instead of local1
        let first_arg = self.cmap.scoped_get(
            Some(cur_scope),
            &MapKey::Place(Place {
                local: Local::from_usize(10),
                projection: List::empty(),
            }),
            false,
        );
        if self.debug {
            println!("\n##### Getting Struct DefIds\n");
            println!("cur_scope: {:?}", cur_scope);
            println!("first arg constraints: {:?}", first_arg);
        }

        if let Some(first_arg_vartype) = first_arg {
            if let VarType::Values(first_arg_constraints) = first_arg_vartype {
                if first_arg_constraints.len() != 1 {
                    todo!("handle diff lens: {:?}", first_arg_constraints.len());
                }
                for first_arg_constraint in first_arg_constraints.iter() {
                    structs.push(self.resolve_first_arg_constraints(first_arg_constraint));
                }
            } else {
                panic!("first arg is a scope...");
            }
        }

        structs
    }

    fn get_impl_dids(
        &self,
        _cur_scope: DefId,
        structs: &Vec<DefId>,
        impltors: &Vec<DefId>,
    ) -> Vec<DefId> {
        let mut impls = vec![];
        for struct_ in structs.iter() {
            if let Some(impl_blocks) = self.funcs.struct_impls.get(struct_) {
                //if self.debug {
                //    println!("impl_blocks: {:?}", impl_blocks);
                //}
                if let Some(assoc) = self.funcs.impl_blocks_to_impls.get(&impl_blocks[0]) {
                    //if self.debug {
                    //    println!("assoc: {:?}", assoc);
                    //}
                    for impltor in impltors {
                        if assoc.contains(impltor) {
                            impls.push(*impltor);
                        }
                    }
                } else {
                    panic!("no");
                }
            } else {
                panic!("no struct impls");
            }
        }
        impls
    }

    // TODO doing duplicate work as in interp, store interp's result somewhere
    // (memoize for each dyn dispatch call)
    fn get_trait_impl_dids(&self, cur_scope: DefId, dynfunc_defid: &DefId) -> Vec<(DefId, DefId)> {
        if self.debug {
            println!("cur_scope: {:?}", cur_scope);
            println!("dynfunc_defid: {:?}", dynfunc_defid);
        }
        let structs = self.get_struct_dids(cur_scope);
        if self.debug {
            println!("structs: {:?}", structs);
        }

        if let Some(impltors) = self
            .funcs
            .trait_fn_impltors
            .lock()
            .unwrap()
            .get(&dynfunc_defid)
        {
            if self.debug {
                println!("impltors: {:?}", impltors);
            }

            let impls = self.get_impl_dids(cur_scope, &structs, impltors);
            if self.debug {
                println!("impls: {:?}", impls);
            }

            return std::iter::zip(structs, impls).collect();
        } else {
            panic!("no impltors");
        }
    }

    #[allow(rustc::usage_of_ty_tykind)]
    fn make_dyn_traitobj_tykind(&self, traitobj_did: DefId) -> TyKind<'tcx> {
        // construct args list (containing dyn Animal)
        let dummy_args: Vec<GenericArg<'tcx>> = Vec::new();
        let pep_list =
            self.tcx
                .mk_poly_existential_predicates(&[Binder::dummy(ExistentialPredicate::Trait(
                    ExistentialTraitRef::new(self.tcx, traitobj_did, dummy_args),
                ))]);

        Dynamic(
            pep_list,
            Region::new_from_kind(self.tcx, RegionKind::ReErased),
        )
    }

    /*
     * let mut _: *mut dyn Animal;
     */
    fn add_mut_dyn_traitobj_temp(&self, patch: &mut MirPatch<'tcx>, traitobj_did: DefId) -> Local {
        let dyn_traitobj_tykind = self.make_dyn_traitobj_tykind(traitobj_did);
        let dyn_traitobj_ty = self.tcx.mk_ty_from_kind(dyn_traitobj_tykind);
        patch.new_temp(
            Ty::new_mut_ptr(self.tcx, dyn_traitobj_ty),
            self.dummy_span(),
        )
    }

    /*
     * let mut _: std::boxed::Box<dyn Animal>;
     */
    fn add_boxed_dyn_traitobj_temp(
        &self,
        patch: &mut MirPatch<'tcx>,
        traitobj_did: DefId,
    ) -> Local {
        let dyn_traitobj_tykind = self.make_dyn_traitobj_tykind(traitobj_did);
        let dyn_traitobj_ty = self.tcx.mk_ty_from_kind(dyn_traitobj_tykind);
        let boxed_dyn_traitobj_ty = Ty::new_box(self.tcx, dyn_traitobj_ty);
        patch.new_temp(boxed_dyn_traitobj_ty, self.dummy_span())
    }

    fn add_into_raw_block(
        &self,
        patch: &mut MirPatch<'tcx>,
        bb_next: BasicBlock,
        bb_cleanup: BasicBlock,
        mut_dyn_traitobj_loc: Local,
        boxed_dyn_traitobj_variant_loc: Local,
        boxed_dyn_traitobj_trait_loc: Local,
        traitobj_did: DefId,
        to_free_opt: Option<Vec<Local>>,
    ) -> BasicBlock {
        let empty_proj_slice: &[ProjectionElem<Local, Ty<'_>>] = &[];
        let empty_proj = self.tcx.mk_place_elems(empty_proj_slice);

        let mut stmts: Vec<Statement<'tcx>> = Vec::new();

        if let Some(to_free_vec) = to_free_opt {
            for to_free in to_free_vec.iter() {
                stmts.push(Statement::new(
                    self.dummy_source_info(),
                    StatementKind::StorageDead(*to_free),
                ));
            }
        }

        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::StorageLive(mut_dyn_traitobj_loc),
        ));
        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::StorageLive(boxed_dyn_traitobj_variant_loc),
        ));
        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::StorageLive(boxed_dyn_traitobj_trait_loc),
        ));

        // TODO const false ?

        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::Assign(Box::new((
                Place {
                    local: boxed_dyn_traitobj_variant_loc,
                    projection: empty_proj,
                },
                Rvalue::Use(Operand::Move(Place {
                    local: boxed_dyn_traitobj_trait_loc,
                    projection: empty_proj,
                })),
            ))),
        ));
        //println!("PLACE AFTER: {:?}", boxed_dyn_traitobj_variant_loc);

        // add terminator
        let dyn_traitobj_tykind = self.make_dyn_traitobj_tykind(traitobj_did);
        let dyn_traitobj_ty = self.tcx.mk_ty_from_kind(dyn_traitobj_tykind);
        let gen_args_ref = self.tcx.mk_args(&[GenericArg::from(dyn_traitobj_ty)]);

        let args: Box<[Spanned<Operand<'tcx>>]> = Box::new([Spanned {
            node: Operand::Move(Place {
                local: boxed_dyn_traitobj_variant_loc,
                projection: empty_proj,
            }),
            span: self.dummy_span(),
        }]);

        let term = Terminator {
            source_info: self.dummy_source_info(),
            kind: TerminatorKind::Call {
                func: Operand::Constant(Box::new(ConstOperand {
                    span: self.dummy_span(),
                    user_ty: None,
                    const_: rustc_middle::mir::Const::Val(
                        ConstValue::ZeroSized,
                        Ty::new_fn_def(self.tcx, INTO_RAW_FN_DEFID, gen_args_ref),
                    ),
                })),
                args,
                destination: Place {
                    local: mut_dyn_traitobj_loc,
                    projection: empty_proj,
                },
                target: Some(bb_next),
                unwind: UnwindAction::Cleanup(bb_cleanup),
                call_source: CallSource::Normal,
                fn_span: self.dummy_span(),
            },
        };
        //println!("PLACE AFTER (term): {:?}", mut_dyn_traitobj_loc);

        let bb_data = BasicBlockData::new_stmts(stmts, Some(term), false);
        patch.new_block(bb_data)
    }

    fn add_goto_block(&self, patch: &mut MirPatch<'tcx>, bb_ret: BasicBlock) -> BasicBlock {
        let term = Terminator {
            source_info: self.dummy_source_info(),
            kind: TerminatorKind::Goto { target: bb_ret },
        };

        let bb_data = BasicBlockData::new(Some(term), false);
        patch.new_block(bb_data)
    }

    fn make_empty_tup(&self) -> Ty<'tcx> {
        let tup_inner: &[Ty<'tcx>] = &[];
        Ty::new_tup(self.tcx, tup_inner)
    }

    /*
     * let mut _: *const ();
     */
    fn add_raw_traitobj_temp(&self, patch: &mut MirPatch<'tcx>) -> Local {
        patch.new_temp(
            Ty::new_imm_ptr(self.tcx, self.make_empty_tup()),
            self.dummy_span(),
        )
    }

    /*
     * let _: &Cat;
     * or
     * let _: &Dog;
     */
    fn add_concretety_ref_temp(&self, patch: &mut MirPatch<'tcx>, struct_did: DefId) -> Local {
        let struct_adt_def = self.tcx.adt_def(struct_did);
        let gen_args: &[GenericArg<'tcx>] = &[];
        let gen_args_ref = self.tcx.mk_args(gen_args);
        patch.new_temp(
            Ty::new_ref(
                self.tcx,
                Region::new_from_kind(self.tcx, RegionKind::ReErased),
                Ty::new_adt(self.tcx, struct_adt_def, gen_args_ref),
                Mutability::Not,
            ),
            self.dummy_span(),
        )
    }

    fn add_speak_block(
        &self,
        patch: &mut MirPatch<'tcx>,
        bb_ret: BasicBlock,
        bb_cleanup: BasicBlock,
        raw_traitobj1_loc: Local,
        raw_traitobj2_loc: Local,
        ret_place: Place<'tcx>,
        concrete_ty_loc: Local,
        concrete_ty_did: DefId,
        speak_fn_did: DefId,
        to_free_opt: Option<Vec<Local>>,
    ) -> BasicBlock {
        let empty_proj_slice: &[ProjectionElem<Local, Ty<'_>>] = &[];
        let empty_proj = self.tcx.mk_place_elems(empty_proj_slice);
        let mut stmts = Vec::new();

        if let Some(to_free_vec) = to_free_opt {
            for to_free in to_free_vec.iter() {
                stmts.push(Statement::new(
                    self.dummy_source_info(),
                    StatementKind::StorageDead(*to_free),
                ));
            }
        }

        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::StorageLive(raw_traitobj2_loc),
        ));
        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::StorageLive(concrete_ty_loc),
        ));
        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::StorageLive(ret_place.local),
        ));

        // copy raw_animal ptr
        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::Assign(Box::new((
                Place {
                    local: raw_traitobj2_loc,
                    projection: empty_proj,
                },
                Rvalue::Use(Operand::Copy(Place {
                    local: raw_traitobj1_loc,
                    projection: empty_proj,
                })),
            ))),
        ));
        //println!("PLACE AFTER: {:?}", raw_traitobj2_loc);

        // transmute raw_animal copy into &concrete_ty
        let struct_adt_def = self.tcx.adt_def(concrete_ty_did);
        let gen_args: &[GenericArg<'tcx>] = &[];
        let gen_args_ref = self.tcx.mk_args(gen_args);

        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::Assign(Box::new((
                Place {
                    local: concrete_ty_loc,
                    projection: empty_proj,
                },
                Rvalue::Cast(
                    CastKind::Transmute,
                    Operand::Move(Place {
                        local: raw_traitobj2_loc,
                        projection: empty_proj,
                    }),
                    Ty::new_ref(
                        self.tcx,
                        Region::new_from_kind(self.tcx, RegionKind::ReErased),
                        Ty::new_adt(self.tcx, struct_adt_def, gen_args_ref),
                        Mutability::Not,
                    ),
                ),
            ))),
        ));
        //println!("PLACE AFTER: {:?}", concrete_ty_loc);

        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::StorageDead(raw_traitobj1_loc),
        ));
        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::StorageDead(raw_traitobj2_loc),
        ));

        // why &*s? try just using result of prev as speak arg

        // construct Cat::speak call
        let empty_proj_slice: &[ProjectionElem<Local, Ty<'_>>] = &[];
        let empty_proj = self.tcx.mk_place_elems(empty_proj_slice);

        let args: Box<[Spanned<Operand<'tcx>>]> = Box::new([Spanned {
            node: Operand::Move(Place {
                local: concrete_ty_loc,
                projection: empty_proj,
            }),
            span: self.dummy_span(),
        }]);

        let gen_args: &[GenericArg<'tcx>] = &[];
        let gen_args_ref = self.tcx.mk_args(gen_args);

        let term = Terminator {
            source_info: self.dummy_source_info(),
            kind: TerminatorKind::Call {
                func: Operand::Constant(Box::new(ConstOperand {
                    span: self.dummy_span(),
                    user_ty: None,
                    const_: rustc_middle::mir::Const::Val(
                        ConstValue::ZeroSized,
                        Ty::new_fn_def(self.tcx, speak_fn_did, gen_args_ref),
                    ),
                })),
                args,
                destination: ret_place,
                target: Some(bb_ret),
                unwind: UnwindAction::Cleanup(bb_cleanup),
                call_source: CallSource::Normal,
                fn_span: self.dummy_span(),
            },
        };
        //println!("PLACE AFTER (term): {:?}", func_ret_loc);

        let bb_data = BasicBlockData::new_stmts(stmts, Some(term), false);
        patch.new_block(bb_data)
    }

    /*
     * let mut _: bool;
     */
    fn add_mut_bool_temp(&self, patch: &mut MirPatch<'tcx>) -> Local {
        patch.new_temp(
            self.tcx.mk_ty_from_kind(rustc_middle::ty::Bool),
            self.dummy_span(),
        )
    }

    fn add_switch_block(
        &self,
        patch: &mut MirPatch<'tcx>,
        bb_eq: BasicBlock,
        bb_neq: BasicBlock,
        eq_res_loc: Local,
    ) -> BasicBlock {
        let empty_proj_slice: &[ProjectionElem<Local, Ty<'_>>] = &[];
        let empty_proj = self.tcx.mk_place_elems(empty_proj_slice);

        let targets = vec![(0u128, bb_neq)].into_iter();

        let term = Terminator {
            source_info: self.dummy_source_info(),
            kind: TerminatorKind::SwitchInt {
                discr: Operand::Move(Place {
                    local: eq_res_loc,
                    projection: empty_proj,
                }),
                targets: SwitchTargets::new(targets, bb_eq),
            },
        };

        let bb_data = BasicBlockData::new(Some(term), false);
        patch.new_block(bb_data)
    }

    fn make_dynmetadata_adt(&self, traitobj_did: DefId) -> Ty<'tcx> {
        // DynMetadata AdtDef
        let dynmetadata_adt_def = self
            .tcx
            .adt_def(self.tcx.lang_items().dyn_metadata().unwrap());

        // GenArgsRef
        let dyn_traitobj_tykind = self.make_dyn_traitobj_tykind(traitobj_did);
        let dyn_traitobj_ty = self.tcx.mk_ty_from_kind(dyn_traitobj_tykind);
        let gen_args_ref = self.tcx.mk_args(&[GenericArg::from(dyn_traitobj_ty)]);

        Ty::new_adt(self.tcx, dynmetadata_adt_def, gen_args_ref)
    }

    fn add_compare_shim(
        &self,
        patch: &mut MirPatch<'tcx>,
        bb_func_call: BasicBlock,
        raw_traitobj1_loc: Local,
        mut_dyn_traitobj_loc: Local,
    ) -> BasicBlock {
        let empty_proj_slice: &[ProjectionElem<Local, Ty<'_>>] = &[];
        let empty_proj = self.tcx.mk_place_elems(empty_proj_slice);

        let mut stmts: Vec<Statement<'tcx>> = Vec::new();

        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::Assign(Box::new((
                Place {
                    local: raw_traitobj1_loc,
                    projection: empty_proj,
                },
                Rvalue::Cast(
                    CastKind::PtrToPtr,
                    Operand::Move(Place {
                        local: mut_dyn_traitobj_loc,
                        projection: empty_proj,
                    }),
                    Ty::new_imm_ptr(self.tcx, self.make_empty_tup()),
                ),
            ))),
        ));

        let term = Terminator {
            source_info: self.dummy_source_info(),
            kind: TerminatorKind::Goto {
                target: bb_func_call,
            },
        };

        let bb_data = BasicBlockData::new_stmts(stmts, Some(term), false);
        patch.new_block(bb_data)
    }

    fn add_compare_vtable_block(
        &self,
        patch: &mut MirPatch<'tcx>,
        bb_first_switch: BasicBlock,
        bb_cleanup: BasicBlock,
        raw_traitobj1_loc: Local,
        mut_dyn_traitobj_loc: Local,
        dynmetadata_traitobj_loc: Local,
        dynmetadata_traitobj_ref_loc: Local,
        dynmetadata_concretety_loc: Local,
        dynmetadata_concretety_ref_loc: Local,
        eq_res_loc: Local,
        traitobj_did: DefId,
        done_copy: bool,
        to_free_opt: Option<Vec<Local>>,
    ) -> BasicBlock {
        let empty_proj_slice: &[ProjectionElem<Local, Ty<'_>>] = &[];
        let empty_proj = self.tcx.mk_place_elems(empty_proj_slice);

        let mut stmts: Vec<Statement<'tcx>> = Vec::new();

        if let Some(to_free_vec) = to_free_opt {
            for to_free in to_free_vec.iter() {
                stmts.push(Statement::new(
                    self.dummy_source_info(),
                    StatementKind::StorageDead(*to_free),
                ));
            }
        }

        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::StorageLive(raw_traitobj1_loc),
        ));

        if !done_copy {
            stmts.push(Statement::new(
                self.dummy_source_info(),
                StatementKind::Assign(Box::new((
                    Place {
                        local: raw_traitobj1_loc,
                        projection: empty_proj,
                    },
                    Rvalue::Cast(
                        CastKind::PtrToPtr,
                        Operand::Move(Place {
                            local: mut_dyn_traitobj_loc,
                            projection: empty_proj,
                        }),
                        Ty::new_imm_ptr(self.tcx, self.make_empty_tup()),
                    ),
                ))),
            ));
        }

        if done_copy {
            stmts.push(Statement::new(
                self.dummy_source_info(),
                StatementKind::StorageDead(mut_dyn_traitobj_loc),
            ));
        }
        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::StorageLive(dynmetadata_traitobj_ref_loc),
        ));
        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::StorageLive(dynmetadata_concretety_ref_loc),
        ));
        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::StorageLive(eq_res_loc),
        ));

        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::Assign(Box::new((
                Place {
                    local: dynmetadata_traitobj_ref_loc,
                    projection: empty_proj,
                },
                Rvalue::Ref(
                    Region::new_from_kind(self.tcx, RegionKind::ReErased),
                    rustc_middle::mir::BorrowKind::Shared,
                    Place {
                        local: dynmetadata_traitobj_loc,
                        projection: empty_proj,
                    },
                ),
            ))),
        ));

        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::Assign(Box::new((
                Place {
                    local: dynmetadata_concretety_ref_loc,
                    projection: empty_proj,
                },
                Rvalue::Ref(
                    Region::new_from_kind(self.tcx, RegionKind::ReErased),
                    rustc_middle::mir::BorrowKind::Shared,
                    Place {
                        local: dynmetadata_concretety_loc,
                        projection: empty_proj,
                    },
                ),
            ))),
        ));

        // add terminator
        let dm_adt = self.make_dynmetadata_adt(traitobj_did);
        let gen_args_ref = self
            .tcx
            .mk_args(&[GenericArg::from(dm_adt), GenericArg::from(dm_adt)]);

        let args: Box<[Spanned<Operand<'tcx>>]> = Box::new([
            Spanned {
                node: Operand::Move(Place {
                    local: dynmetadata_traitobj_ref_loc,
                    projection: empty_proj,
                }),
                span: self.dummy_span(),
            },
            Spanned {
                node: Operand::Move(Place {
                    local: dynmetadata_concretety_ref_loc,
                    projection: empty_proj,
                }),
                span: self.dummy_span(),
            },
        ]);

        let term = Terminator {
            source_info: self.dummy_source_info(),
            kind: TerminatorKind::Call {
                func: Operand::Constant(Box::new(ConstOperand {
                    span: self.dummy_span(),
                    user_ty: None,
                    const_: rustc_middle::mir::Const::Val(
                        ConstValue::ZeroSized,
                        Ty::new_fn_def(self.tcx, EQ_FN_DEFID, gen_args_ref),
                    ),
                })),
                args,
                destination: Place {
                    local: eq_res_loc,
                    projection: empty_proj,
                },
                target: Some(bb_first_switch),
                unwind: UnwindAction::Cleanup(bb_cleanup),
                call_source: CallSource::Normal,
                fn_span: self.dummy_span(),
            },
        };

        let bb_data = BasicBlockData::new_stmts(stmts, Some(term), false);
        patch.new_block(bb_data)
    }

    /*
     * let mut _: &std:ptr::DynMetadata<dyn Animal>;
     */
    fn add_dynmetadata_ref_temp(&self, patch: &mut MirPatch<'tcx>, traitobj_did: DefId) -> Local {
        // add &DynMetadata local to patch
        let dm_adt = self.make_dynmetadata_adt(traitobj_did);
        patch.new_temp(
            Ty::new_ref(
                self.tcx,
                Region::new_from_kind(self.tcx, RegionKind::ReErased),
                dm_adt,
                Mutability::Not,
            ),
            self.dummy_span(),
        )
    }

    #[allow(unused_assignments)]
    fn replace_dynamic_dispatch(
        &self,
        cur_scope: DefId,
        patch: &mut MirPatch<'tcx>,
        dynfunc_defid: &DefId,
        ty: Ty<'tcx>,
        term_dst_place: Place<'tcx>,
        bb: BasicBlock,
        data: &BasicBlockData<'tcx>,
        num_bbs: usize,
    ) {
        if self.debug {
            println!("\n### STARTING\n");
            println!("num_bbs: {:?}", num_bbs);
            println!("cur_bb: {:?}", bb);
            println!("speak dest: {:?}", term_dst_place);
        }

        // get old terminator's edges
        let (bb_old_next, bb_old_cleanup) = self.get_bbs(data);
        if self.debug {
            println!("bb_old_next: {:?}", bb_old_next);
            println!("bb_old_cleanup: {:?}", bb_old_cleanup);
        }

        let traitobj_did = self.get_traitobj_did(ty);
        if self.debug {
            println!("traitobj_did: {:?}", traitobj_did);
        }

        let dids = self.get_trait_impl_dids(cur_scope, dynfunc_defid);
        if self.debug {
            println!("dids: {:?}", dids);
        }

        // FIXME get dynamically
        let traitobj = Local::from_u32(1);

        let mut trait_vtable = None;
        let mut variant_vtable = None;
        let mut trait_vtable_ref = None;
        let mut variant_vtable_ref = None;
        if dids.len() > 1 {
            // FIXME get dynamically
            trait_vtable = Some(Local::from_u32(3));
            variant_vtable = Some(Local::from_u32(4));

            // FIXME do these locals already exist?
            trait_vtable_ref = Some(self.add_dynmetadata_ref_temp(patch, traitobj_did));
            variant_vtable_ref = Some(self.add_dynmetadata_ref_temp(patch, traitobj_did));
        }

        // for into_raw -> speak
        let raw_traitobj1 = self.add_raw_traitobj_temp(patch);
        let mut_dyn_traitobj = self.add_mut_dyn_traitobj_temp(patch, traitobj_did);
        let boxed_dyn_traitobj1 = self.add_boxed_dyn_traitobj_temp(patch, traitobj_did);

        let mut into_raw_target = None;
        for (i, (struct_did, func_did)) in dids.iter().enumerate() {
            // goto (to bb_old_return)
            let bb_variant_ret = self.add_goto_block(patch, bb_old_next);

            // speak
            let raw_traitobj2 = self.add_raw_traitobj_temp(patch);
            let struct_obj = self.add_concretety_ref_temp(patch, *struct_did);
            let bb_variant_speak = self.add_speak_block(
                patch,
                bb_variant_ret,
                bb_old_cleanup,
                raw_traitobj1,
                raw_traitobj2,
                term_dst_place,
                struct_obj,
                *struct_did,
                *func_did,
                None,
            );

            if i > 0 {
                // comparison & switch (if > 1 variant)
                let first_eq_res = self.add_mut_bool_temp(patch);
                // TODO bb order may need to switch?
                let bb_switch =
                    self.add_switch_block(patch, bb_variant_speak, bb_variant_speak, first_eq_res);

                let bb_compare = self.add_compare_vtable_block(
                    patch,
                    bb_switch,
                    bb_old_cleanup,
                    raw_traitobj1,
                    mut_dyn_traitobj,
                    trait_vtable.unwrap(),
                    trait_vtable_ref.unwrap(),
                    variant_vtable.unwrap(),
                    variant_vtable_ref.unwrap(),
                    first_eq_res,
                    traitobj_did,
                    false,
                    Some(vec![boxed_dyn_traitobj1]),
                );
                into_raw_target = Some(bb_compare);
            } else {
                // shim compare block, which does a necessary type cast
                let bb_shim =
                    self.add_compare_shim(patch, bb_variant_speak, raw_traitobj1, mut_dyn_traitobj);
                into_raw_target = Some(bb_shim);
            }
        }

        if into_raw_target.is_none() {
            panic!("no into_raw_target");
        }

        let bb_into_raw = self.add_into_raw_block(
            patch,
            into_raw_target.unwrap(),
            bb_old_cleanup,
            mut_dyn_traitobj,
            boxed_dyn_traitobj1,
            traitobj,
            traitobj_did,
            None,
        );

        // mod cur speak block w/ goto new start (into_raw)
        self.replace_dyndispatch_term_w_goto(patch, bb, bb_into_raw);
    }
}
