use rustc_middle::mir::*;
use rustc_middle::ty::*;
use rustc_span::def_id::{CrateNum, DefId, DefIndex};
use rustc_span::source_map::Spanned;
use rustc_span::{BytePos, Span, SyntaxContext};

use crate::constraints::{ConstraintMap, MapKey, VarType};
use crate::core::{DebugPass, Style, VerifoptRval, is_box, resolve_ty};
use crate::patch::MirPatch;
use crate::{FuncMap, RTAMap};

// FIXME get dynamically
// alloc[7a7c]::boxed::{impl#8}::into_raw
const INTO_RAW_FN_DEFID: DefId = DefId {
    //index: DefIndex::from_u32(731),
    index: DefIndex::from_u32(749),
    krate: CrateNum::from_u32(3),
};
// DefId(2:2583 ~ core[c945]::ptr::metadata::{extern#0}::VTable)
// - defkind == ForeignTy
const VTABLE_TY_DEFID: DefId = DefId {
    index: DefIndex::from_u32(2583),
    krate: CrateNum::from_u32(2),
};

//const FIRST_NONSELF_ARG_IDX: usize = 0;
//const VTABLE_ADDR_IDX: u32 = 2;

pub struct RewritePass<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub funcs: &'a FuncMap<'tcx>,
    pub cmap: &'a ConstraintMap<'tcx>,
    pub inits: &'a RTAMap,
    pub style: Style,
    pub debug: bool,
    // TODO put dynamically-acquired into_raw and eq_fn defids here
}

impl<'a, 'tcx> RewritePass<'a, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        funcs: &'a FuncMap<'tcx>,
        cmap: &'a ConstraintMap<'tcx>,
        inits: &'a RTAMap,
        style: Style,
        which_debug: DebugPass,
    ) -> RewritePass<'a, 'tcx> {
        let mut debug = false;
        if which_debug == DebugPass::Rewrite {
            debug = true;
        }
        Self {
            tcx,
            funcs,
            cmap,
            inits,
            style,
            debug,
        }
    }

    pub fn run(&self, cur_scope: DefId, body: &mut Body<'tcx>) {
        self.visit_body(cur_scope, body);
    }

    #[allow(invalid_reference_casting)]
    fn setup_visit_body(&self, func: &Operand<'tcx>) {
        if self.debug {
            println!("\n######## TRAVERSING!!\n");
        }

        // get defid
        match func {
            Operand::Constant(box co) => match co.const_ {
                rustc_middle::mir::Const::Val(_, ty) => match ty.kind() {
                    TyKind::FnDef(defid, _) => {
                        // get body
                        if !self.tcx.is_mir_available(defid) {
                            if self.debug {
                                println!("# skipping traversal (TODO no mir).....");
                            }
                        } else {
                            let body = self.tcx.optimized_mir(defid);

                            // turn &body _&mut_ body
                            let const_body_ptr: *const Body = &*body;
                            let mut_body_ptr: *mut Body = const_body_ptr as *mut Body;

                            unsafe {
                                self.visit_body(*defid, &mut *mut_body_ptr);
                            }
                        }
                    }
                    _ => panic!("not a FnDef"),
                },
                _ => todo!("not a Val Const"),
            },
            _ => todo!("handle indirect invocations"),
        }
    }

    fn should_rewrite(&self, defid: DefId) -> bool {
        self.tcx.def_path_debug_str(defid).contains("Animal::speak")
            || self.tcx.def_path_debug_str(defid).contains("Animal::visit")
            || self
                .tcx
                .def_path_debug_str(defid)
                .contains("AnimalVisitor::visit_cat")
            || self
                .tcx
                .def_path_debug_str(defid)
                .contains("AnimalVisitor::visit_dog")
    }

    pub fn visit_body(&self, cur_scope: DefId, body: &mut Body<'tcx>) {
        if self.debug {
            println!("#############################");
            println!("IN SCOPE: {:?}", cur_scope);
            println!("#############################");
        }

        // for benchmarks, track "warmup" loop vtable ptr difference per _body_ (not per block)
        let mut warmup = true;
        let mut patch = MirPatch::new(body);
        let num_bbs = body.basic_blocks.len();
        for (bb, data) in body.basic_blocks.iter_enumerated() {
            self.visit_block(cur_scope, num_bbs, &bb, data, &mut patch, &mut warmup);
        }

        if self.debug {
            println!("\n# PATCH: \n\n{:#?}", patch);
        }

        patch.apply(body);

        /*
        if self.debug {
            println!("\n# NEW BODY:\n");

            let locs = &body.local_decls;
            let bbs = &body.basic_blocks;

            println!("num LocalDecls: {:?}", locs.len());
            println!("{{");
            for i in 0..locs.len() {
                println!("-local{:?}", i);
                println!("{:#?}", locs[Local::from_usize(i)]);
            }
            println!("}}");

            println!("num BasicBlocks: {:?}", bbs.len());
            println!("{{");
            for i in 0..bbs.len() {
                println!("-bb{:?}", i);
                println!("{:#?}", bbs[BasicBlock::from_usize(i)]);
            }
            println!("}}");
        }
        */
    }

    #[allow(unused_assignments)]
    fn visit_block(
        &self,
        cur_scope: DefId,
        num_bbs: usize,
        bb: &BasicBlock,
        data: &BasicBlockData<'tcx>,
        mut patch: &mut MirPatch<'tcx>,
        warmup: &mut bool,
    ) {
        if self.debug {
            println!("BB: {:?}", bb);
        }

        // TODO add some notion of WTO?
        if data.is_cleanup {
            //println!("CLEANUP: {:?}", bb);
            return;
        }

        let mut updated_num_blocks = 0;

        match &data.terminator().kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => {
                if let Some((defid, genargs)) = func.const_fn_def() {
                    if genargs.len() == 0 {
                        if self.debug {
                            println!("\n# -------------------------------");
                            println!("# in {:?} of {:?}", bb, cur_scope);
                            println!("# no genargs");
                            println!("# func: {:?}", func);
                            println!("# args: {:?}", args);
                            println!("# dest: {:?}", destination);
                            println!("# -------------------------------");
                        }
                        self.setup_visit_body(func);
                        return;
                    }

                    if self.debug {
                        println!("\n# -------------------------------");
                        println!("# in {:?} of {:?}", bb, cur_scope);
                        println!("# CALL term: {:?}", defid);
                        println!("# -------------------------------");
                        println!("genargs: {:?}", genargs);
                        println!("printed as str: {:?}", genargs.print_as_list());
                    }

                    // can assume that if multiple funcvals, they will have the same
                    // type, so just get the self_opt using the first funcval
                    let funcval = self.funcs.all_funcs.get(&cur_scope).unwrap().clone();
                    if self.debug {
                        println!("funcval: {:?}", funcval);
                    }

                    let genargs_slice = genargs.as_slice();
                    let first = genargs_slice[0];
                    if let Some(ty) = first.as_type() {
                        if ty.is_trait() {
                            if !self.should_rewrite(defid) {
                                if self.debug {
                                    println!("SKIPPING OTHER DYN CALL: {:?}", defid);
                                }
                                return;
                            }

                            if self.debug {
                                println!("\nFIRST ARG == TRAIT");
                                println!("arg: {:?}", first);
                                println!("func: {:?}", func);
                                println!("func_defid: {:?}", defid);
                            }

                            let mut vtable_locs = Vec::new();
                            // vis_one_variant_bench
                            //let bench_def_index = 39;
                            // vis_two_variants_bench
                            let bench_def_index = 46;
                            // no inlining
                            //let bench_def_index = 0;
                            if cur_scope.krate.as_u32() == 0
                                && cur_scope.index.as_u32() == bench_def_index
                            {
                                // different dyn animal vtables for warmup vs run block in
                                // benchmarks (inlined bench())
                                if *warmup == true {
                                    *warmup = false;
                                    // vis_two_variants_bench w prints
                                    //vtable_locs = (264, 5);

                                    // vis_two_variants_bench
                                    vtable_locs.push(5);
                                    vtable_locs.push(214);

                                    // vis_one_variant_bench
                                    //vtable_locs = (165, 5);
                                } else {
                                    // vis_two_variants_bench w prints
                                    //vtable_locs = (269, 5);

                                    // vis_two_variants_bench
                                    vtable_locs.push(5);
                                    vtable_locs.push(219);

                                    // vis_one_variant_bench
                                    //vtable_locs = (170, 5);
                                }
                            } else {
                                if funcval.self_arg.is_some() {
                                    // increase offset b/c arg list now includes `self`
                                    vtable_locs.push(3);
                                    vtable_locs.push(4);
                                    vtable_locs.push(5);
                                } else {
                                    // default offset
                                    vtable_locs.push(2);
                                    vtable_locs.push(3);
                                    vtable_locs.push(4);
                                }
                            }

                            if self.debug {
                                println!("PRE REPLACEMENT");
                                println!("warmup: {:?}", warmup);
                                println!("vtable_locs: {:?}", vtable_locs);
                            }

                            self.replace_dynamic_dispatch(
                                cur_scope,
                                &mut patch,
                                &defid,
                                ty,
                                args,
                                *destination,
                                bb,
                                data,
                                num_bbs,
                                vtable_locs.as_slice(),
                            );

                            // TODO if multiple patches in the same body?
                            // find way to declare/apply each patch in an inner scope
                            // for now assuming only one patch per body

                            if self.debug {
                                println!("POST REPLACEMENT");
                                println!("updated_num_blocks: {:?}", updated_num_blocks);
                                println!("patch.new_blocks.len(): {:?}", patch.new_blocks.len());
                            }

                            // traverse into replacement code
                            if patch.new_blocks.len() - updated_num_blocks > 0 {
                                if self.debug {
                                    println!("TRAVERSING NESTED!!");
                                    println!("num new blocks: {:?}", patch.new_blocks.len());
                                }

                                for (bb_num, data) in patch.new_blocks.clone().iter().enumerate() {
                                    // we've already traversed this part of the patch
                                    if bb_num < updated_num_blocks {
                                        continue;
                                    }
                                    self.visit_block(
                                        cur_scope,
                                        num_bbs + patch.new_blocks.len(),
                                        &BasicBlock::from_usize(bb_num),
                                        data,
                                        patch,
                                        warmup,
                                    );
                                }
                            }

                            updated_num_blocks = patch.new_blocks.len();
                        }
                    } else {
                        if self.debug {
                            println!("first arg no type, continuing...");
                        }
                        return;
                    }

                    //if self.debug {
                    //    println!("\n# -------------------------------");
                    //    println!("# in {:?} of {:?}", bb, cur_scope);
                    //    println!("# POST CONST FN DEF");
                    //    println!("# func: {:?}", func);
                    //    println!("# args: {:?}", args);
                    //    println!("# dest: {:?}", destination);
                    //    println!("# -------------------------------");
                    //}
                } else {
                    todo!();
                    //if self.debug {
                    //    println!("\n# -------------------------------");
                    //    println!("# in {:?} of {:?}", bb, cur_scope);
                    //    println!("# NON CONST FN DEF");
                    //    println!("# func: {:?}", func);
                    //    println!("# args: {:?}", args);
                    //    println!("# dest: {:?}", destination);
                    //    println!("# -------------------------------");
                    //}
                }
            }
            termkind @ _ => {
                if self.debug {
                    println!("\n# -------------------------------");
                    println!("# in {:?} of {:?}", bb, cur_scope);
                    println!("# NON-CALL term: {:?}", termkind);
                    println!("# -------------------------------");
                }
            }
        }
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

    fn get_bbs(&self, data: &BasicBlockData<'tcx>) -> (BasicBlock, UnwindAction) {
        let edges = data.terminator().kind.edges();
        let bb_old_next;
        let unwind_action;
        match edges {
            TerminatorEdges::AssignOnReturn {
                return_, cleanup, ..
            } => {
                if return_.len() > 1 {
                    panic!("RET: multiple return blocks");
                }
                if let Some(cleanup_bb) = cleanup {
                    // cleanup.is_none() {
                    unwind_action = UnwindAction::Cleanup(cleanup_bb);
                } else {
                    unwind_action = UnwindAction::Continue;
                }
                bb_old_next = return_[0];
            }
            _ => {
                panic!("verifopt: need to set terminator edges");
            }
        }
        (bb_old_next, unwind_action)
    }

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

    fn resolve_struct(
        &self,
        struct_defid: &DefId,
        genarg_vec: &Option<Vec<Vec<VerifoptRval<'tcx>>>>,
    ) -> Vec<DefId> {
        if is_box(*struct_defid) {
            if let Some(genargs_outer) = genarg_vec {
                if genargs_outer.len() != 1 {
                    panic!("handle diff genarg len");
                }

                let genarg_constraint_vec = &genargs_outer[0];
                let mut defids = Vec::new();
                for genarg_constraint in genarg_constraint_vec.iter() {
                    match genarg_constraint {
                        VerifoptRval::IdkStruct(defid, _) => {
                            if self.debug {
                                println!("- found struct: {:?}", defid);
                            }
                            if !defids.contains(defid) {
                                if self.debug {
                                    println!("defids vec: {:?}", defids);
                                    println!("adding new defid: {:?}", defid);
                                }
                                defids.push(*defid);
                            } else {
                                if self.debug {
                                    println!("(duplicate)");
                                }
                            }
                        }
                        _ => todo!(),
                    }
                }
                return defids;
            } else {
                panic!("no genargs in box...");
            }
        } else {
            todo!(
                "struct is not a box: {:?} (\ngenargs: {:?})",
                struct_defid,
                genarg_vec
            );
        }
    }

    fn resolve_first_arg_constraints(
        &self,
        first_arg_constraint: &VerifoptRval<'tcx>,
    ) -> Vec<DefId> {
        if self.debug {
            println!("resolving first arg constraint!!");
        }

        match first_arg_constraint {
            VerifoptRval::IdkStruct(struct_defid, genarg_vec) => {
                self.resolve_struct(struct_defid, genarg_vec)
            }
            VerifoptRval::IdkType(ty) => resolve_ty(ty, self.funcs, self.debug),
            VerifoptRval::Ref(boxed) => self.resolve_first_arg_constraints(&*boxed),
            _ => todo!("handle more kinds: {:?}", first_arg_constraint),
        }
    }

    // get the structs that implement the trait being pointed to by the first argument
    fn get_struct_dids(&self, cur_scope: DefId, traitobj: Local) -> Vec<DefId> {
        if self.debug {
            println!("\n##### Getting Struct DefIds\n");
            println!("cur_scope: {:?}", cur_scope);
            println!("traitobj should be at {:?}", traitobj);
            //println!(
            //    "cmap @ scope: {:#?}",
            //    self.cmap.cmap.get(&MapKey::ScopeId(cur_scope))
            //);
        }

        let mut structs = vec![];
        let first_arg = self.cmap.scoped_get(
            Some(cur_scope),
            &MapKey::Place(Place {
                local: traitobj,
                projection: List::empty(),
            }),
            false,
        );
        if self.debug {
            println!("first arg constraints: {:?}", first_arg);
        }

        if let Some(first_arg_vartype) = first_arg {
            if let VarType::Values(first_arg_constraints) = first_arg_vartype {
                for first_arg_constraint in first_arg_constraints.iter() {
                    structs.append(&mut self.resolve_first_arg_constraints(first_arg_constraint));
                }
            } else {
                panic!("first arg is a scope...");
            }
        }
        if self.debug {
            println!(
                "RESOLVED first arg constraints (struct defids): {:?}",
                structs
            );
        }

        structs
    }

    fn get_impl_dids(
        &self,
        _cur_scope: DefId,
        structs: &Vec<DefId>,
        impls: &Vec<DefId>,
    ) -> Vec<DefId> {
        let mut matching_impls = vec![];
        for struct_ in structs.iter() {
            if let Some(impl_blocks) = self.funcs.struct_to_impls.get(struct_) {
                if let Some(assoc) = self.funcs.impl_blocks_to_fn_impls.get(&impl_blocks[0]) {
                    for impl_ in impls {
                        if assoc.contains(impl_) {
                            matching_impls.push(*impl_);
                        }
                    }
                } else {
                    panic!("no");
                }
            } else {
                panic!("no struct impls");
            }
        }
        matching_impls
    }

    fn get_trait_impl_dids_cha(
        &self,
        cur_scope: DefId,
        dynfunc_defid: &DefId,
        traitobj_did: DefId,
    ) -> Vec<(DefId, DefId)> {
        if self.debug {
            println!("cur_scope: {:?}", cur_scope);
            println!("dynfunc_defid: {:?}", dynfunc_defid);
        }
        match self.funcs.trait_to_struct_impls.get(&traitobj_did) {
            Some(structs) => {
                if self.debug {
                    println!("GOT structs: {:?}", structs);
                }

                if let Some(impls) = self
                    .funcs
                    .trait_fn_impls
                    .lock()
                    .unwrap()
                    .get(&dynfunc_defid)
                {
                    if self.debug {
                        println!("GOT impls: {:?}", impls);
                    }

                    return std::iter::zip(structs, impls)
                        .map(|(&x, &y)| (x, y))
                        .collect();
                } else {
                    panic!("no impltors");
                }
            }
            None => panic!("no structs found!"),
        }
    }

    // TODO doing duplicate work as in interp, store interp's result somewhere
    // (memoize for each dyn dispatch call)
    fn get_trait_impl_dids(
        &self,
        cur_scope: DefId,
        dynfunc_defid: &DefId,
        traitobj: Local,
    ) -> Vec<(DefId, DefId)> {
        if self.debug {
            println!("cur_scope: {:?}", cur_scope);
            println!("dynfunc_defid: {:?}", dynfunc_defid);
        }
        let structs = self.get_struct_dids(cur_scope, traitobj);
        if self.debug {
            println!("GOT structs: {:?}", structs);
        }

        if let Some(impls) = self
            .funcs
            .trait_fn_impls
            .lock()
            .unwrap()
            .get(&dynfunc_defid)
        {
            if self.debug {
                println!("impls: {:?}", impls);
            }

            let matching_impls = self.get_impl_dids(cur_scope, &structs, impls);
            if self.debug {
                println!("matching impls: {:?}", matching_impls);
            }

            return std::iter::zip(structs, matching_impls).collect();
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
        unwind_action: UnwindAction,
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
                unwind: unwind_action,
                call_source: CallSource::Normal,
                fn_span: self.dummy_span(),
            },
        };

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

    fn replace_first_arg(
        &self,
        args: &mut Box<[Spanned<Operand<'tcx>>]>,
        concrete_ty_loc: Local,
    ) -> Box<[Spanned<Operand<'tcx>>]> {
        let empty_proj_slice: &[ProjectionElem<Local, Ty<'_>>] = &[];
        let empty_proj = self.tcx.mk_place_elems(empty_proj_slice);
        args[0] = Spanned {
            node: Operand::Move(Place {
                local: concrete_ty_loc,
                projection: empty_proj,
            }),
            span: self.dummy_span(),
        };
        args.clone()
    }

    fn add_speak_block(
        &self,
        patch: &mut MirPatch<'tcx>,
        bb_ret: BasicBlock,
        unwind_action: UnwindAction,
        raw_traitobj1_loc: Local,
        raw_traitobj2_loc: Local,
        ret_place: Place<'tcx>,
        concrete_ty_loc: Local,
        args: &Box<[Spanned<Operand<'tcx>>]>,
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
        let new_args = self.replace_first_arg(&mut args.clone(), concrete_ty_loc);
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
                args: new_args,
                destination: ret_place,
                target: Some(bb_ret),
                unwind: unwind_action,
                call_source: CallSource::Normal,
                fn_span: self.dummy_span(),
            },
        };

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

    /*
     * let mut _: *const std::ptr::metadata::VTable;
     */
    fn make_const_ptr_vtable_adt(&self) -> Ty<'tcx> {
        Ty::new_imm_ptr(self.tcx, Ty::new_foreign(self.tcx, VTABLE_TY_DEFID))
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
        bb_next: BasicBlock,
        raw_traitobj1_loc: Local,
        mut_dyn_traitobj_loc: Local,
        dynmetadata_traitobj_loc: Local,
        dynmetadata_traitobj_vtable_loc: Local,
        dynmetadata_concretety_loc: Local,
        dynmetadata_concretety_vtable_loc: Local,
        eq_res_loc: Local,
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
            StatementKind::StorageLive(dynmetadata_traitobj_vtable_loc),
        ));
        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::StorageLive(dynmetadata_concretety_vtable_loc),
        ));
        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::StorageLive(eq_res_loc),
        ));

        /*
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
        */

        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::Assign(Box::new((
                Place {
                    local: dynmetadata_traitobj_vtable_loc,
                    projection: empty_proj,
                },
                Rvalue::Cast(
                    CastKind::Transmute,
                    Operand::Copy(Place {
                        local: dynmetadata_traitobj_loc,
                        projection: empty_proj,
                    }),
                    self.make_const_ptr_vtable_adt(),
                ),
            ))),
        ));

        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::Assign(Box::new((
                Place {
                    local: dynmetadata_concretety_vtable_loc,
                    projection: empty_proj,
                },
                Rvalue::Cast(
                    CastKind::Transmute,
                    Operand::Copy(Place {
                        local: dynmetadata_concretety_loc,
                        projection: empty_proj,
                    }),
                    self.make_const_ptr_vtable_adt(),
                ),
            ))),
        ));

        stmts.push(Statement::new(
            self.dummy_source_info(),
            StatementKind::Assign(Box::new((
                Place {
                    local: eq_res_loc,
                    projection: empty_proj,
                },
                Rvalue::BinaryOp(
                    BinOp::Eq,
                    Box::new((
                        Operand::Copy(Place {
                            //local: dynmetadata_traitobj_loc,
                            local: dynmetadata_traitobj_vtable_loc,
                            projection: empty_proj,
                        }),
                        Operand::Copy(Place {
                            //local: dynmetadata_concretety_loc,
                            local: dynmetadata_concretety_vtable_loc,
                            projection: empty_proj,
                        }),
                    )),
                ),
            ))),
        ));

        // add terminator

        let term = Terminator {
            source_info: self.dummy_source_info(),
            kind: TerminatorKind::Goto { target: bb_next },
        };

        /*
        let genargs_ref = self.tcx.mk_args(&[]);
        let args: Box<[Spanned<Operand<'tcx>>]> = Box::new([
            Spanned {
                node: Operand::Copy(Place {
                    local: eq_res_loc,
                    //local: dynmetadata_traitobj_vtable_loc,
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
                        Ty::new_fn_def(self.tcx, DEBUG_BOOL_DEFID, genargs_ref),
                    ),
                })),
                args,
                destination: Place {
                    local: eq_res_loc,
                    projection: empty_proj,
                },
                target: Some(bb_next),
                unwind: UnwindAction::Cleanup(bb_cleanup),
                call_source: CallSource::Normal,
                fn_span: self.dummy_span(),
            },
        };
        */

        let bb_data = BasicBlockData::new_stmts(stmts, Some(term), false);
        patch.new_block(bb_data)
    }

    /*
     * let mut _: *const std::ptr::metadata::VTable;
     */
    fn add_const_ptr_vtable_temp(&self, patch: &mut MirPatch<'tcx>) -> Local {
        patch.new_temp(
            Ty::new_imm_ptr(self.tcx, Ty::new_foreign(self.tcx, VTABLE_TY_DEFID)),
            self.dummy_span(),
        )
    }

    /*
     * let mut _: &std:ptr::DynMetadata<dyn Animal>;
     */
    /*
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
    */

    fn get_first_arg_local(&self, args: &Box<[Spanned<Operand<'tcx>>]>) -> Local {
        let op = &(*args)[0].node;
        match op {
            Operand::Copy(place) | Operand::Move(place) => return place.local,
            _ => panic!("first arg is not a copy or move: {:?}", op),
        }
    }

    #[allow(unused_assignments)]
    fn replace_dynamic_dispatch(
        &self,
        cur_scope: DefId,
        patch: &mut MirPatch<'tcx>,
        dynfunc_defid: &DefId,
        ty: Ty<'tcx>,
        //self_arg: Option<Place<'tcx>>,
        args: &Box<[Spanned<Operand<'tcx>>]>,
        term_dst_place: Place<'tcx>,
        bb: &BasicBlock,
        data: &BasicBlockData<'tcx>,
        num_bbs: usize,
        vtable_locs: &[u32],
        //traitobj_vtable: Option<Local>,
    ) {
        if self.debug {
            println!("\n### STARTING\n");
            println!("cur_scope: {:?}", cur_scope);
            //println!("self_arg: {:?}", self_arg);
            println!("num_bbs: {:?}", num_bbs);
            println!("cur_bb: {:?}", bb);
            println!("func dest place: {:?}", term_dst_place);
        }

        let traitobj = self.get_first_arg_local(args);
        if self.debug {
            println!("first arg local: {:?}", traitobj);
            println!("args: {:?}", args);
        }

        // get old terminator's edges
        let (bb_old_next, unwind_action) = self.get_bbs(data);
        if self.debug {
            println!("bb_old_next: {:?}", bb_old_next);
            println!("unwind_action: {:?}", unwind_action);
        }

        let traitobj_did = self.get_traitobj_did(ty);
        if self.debug {
            println!("traitobj_did: {:?}", traitobj_did);
        }

        let mut dids;
        if self.style == Style::CHA {
            dids = self.get_trait_impl_dids_cha(cur_scope, dynfunc_defid, traitobj_did);
        } else if self.style == Style::RTA {
            todo!();
        } else {
            dids = self.get_trait_impl_dids(cur_scope, dynfunc_defid, traitobj);
        }
        if self.debug {
            println!("dids: {:?}", dids);
        }
        dids.reverse();

        let mut traitobj_vtable = None;
        let mut traitobj_vtable_ptr = None;
        if dids.len() > 1 {
            if self.debug {
                println!("- multiple variants case");
                println!("vtable_locs: {:?}", vtable_locs);
            }

            traitobj_vtable = Some(Local::from_u32(vtable_locs[0]));
            // TODO maybe benchmark this route as an alternative, if it is functional?
            //traitobj_vtable_ref = Some(self.add_dynmetadata_ref_temp(patch, traitobj_did));
            traitobj_vtable_ptr = Some(self.add_const_ptr_vtable_temp(patch));

            if self.debug {
                println!("traitobj vtable: {:?}", traitobj_vtable);
            }
        } else {
            if self.debug {
                println!("- single variant case");
            }
        }

        // for into_raw -> speak
        let raw_traitobj1 = self.add_raw_traitobj_temp(patch);
        let mut_dyn_traitobj = self.add_mut_dyn_traitobj_temp(patch, traitobj_did);
        let boxed_dyn_traitobj1 = self.add_boxed_dyn_traitobj_temp(patch, traitobj_did);

        let mut into_raw_target = None;
        let mut bb_last_variant_speak = None;
        for (i, (struct_did, func_did)) in dids.iter().enumerate() {
            if self.debug {
                println!("i: {:?}", i);
                println!("struct_did: {:?}", struct_did);
                println!("func_did: {:?}", func_did);
            }

            // add back old goto
            let bb_variant_ret = self.add_goto_block(patch, bb_old_next);

            // speak (returns to old_next)
            let raw_traitobj2 = self.add_raw_traitobj_temp(patch);
            let struct_obj = self.add_concretety_ref_temp(patch, *struct_did);
            let bb_cur_variant_speak = self.add_speak_block(
                patch,
                bb_variant_ret,
                unwind_action,
                raw_traitobj1,
                raw_traitobj2,
                term_dst_place,
                struct_obj,
                args,
                *struct_did,
                *func_did,
                None,
            );

            if i > 0 {
                // get vtable locations
                let variant_vtable = Some(Local::from_u32(vtable_locs[vtable_locs.len() - i]));
                let variant_vtable_ptr = Some(self.add_const_ptr_vtable_temp(patch));
                if self.debug {
                    println!("variant vtable: {:?}", variant_vtable);
                }

                // comparison & switch (if > 1 variant)
                let eq_res_bool = self.add_mut_bool_temp(patch);
                let bb_switch = self.add_switch_block(
                    patch,
                    bb_last_variant_speak.unwrap(),
                    bb_cur_variant_speak,
                    eq_res_bool,
                );

                let bb_compare = self.add_compare_vtable_block(
                    patch,
                    bb_switch,
                    raw_traitobj1,
                    mut_dyn_traitobj,
                    traitobj_vtable.unwrap(),
                    traitobj_vtable_ptr.unwrap(),
                    variant_vtable.unwrap(),
                    variant_vtable_ptr.unwrap(),
                    eq_res_bool,
                    false,
                    Some(vec![boxed_dyn_traitobj1]),
                );
                into_raw_target = Some(bb_compare);
            } else if dids.len() == 1 {
                // shim compare block, which does a necessary type cast
                let bb_shim = self.add_compare_shim(
                    patch,
                    bb_cur_variant_speak,
                    raw_traitobj1,
                    mut_dyn_traitobj,
                );
                into_raw_target = Some(bb_shim);
            } else {
                if self.debug {
                    //println!("i: {:?}", i);
                    println!("i == 0 && dids.len != 1");
                    println!("dids: {:?}", dids);
                }
            }

            bb_last_variant_speak = Some(bb_cur_variant_speak);
        }

        if into_raw_target.is_none() {
            panic!("no into_raw_target");
        }

        let bb_into_raw = self.add_into_raw_block(
            patch,
            into_raw_target.unwrap(),
            unwind_action,
            mut_dyn_traitobj,
            boxed_dyn_traitobj1,
            traitobj,
            traitobj_did,
            None,
        );

        // mod cur speak block w/ goto new start (into_raw)
        self.replace_dyndispatch_term_w_goto(patch, *bb, bb_into_raw);
    }
}
