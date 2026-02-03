use rustc_index::IndexSlice;
use rustc_middle::mir::*;
//use rustc_middle::ty::fast_reject::SimplifiedType;
use rustc_middle::ty::{List, Ty, TyCtxt, TyKind};
use rustc_span::def_id::{CrateNum, DefId, DefIndex};

use crate::FuncMap;
use crate::constraints::{ConstraintMap, MapKey, VarType};
use crate::core::VerifoptRval;
use crate::core::is_box;
use crate::patch::MirPatch;

//const DUMMY_DEFID: DefId = DefId {
//    index: DefIndex::from_u32(0),
//    krate: CrateNum::from_u32(0),
//};

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
        let mut patch = MirPatch::new(body);
        let old_locals = body.local_decls();
        for (bb, data) in body.basic_blocks.iter_enumerated() {
            match &data.terminator().kind {
                TerminatorKind::Call {
                    func,
                    args: _,
                    destination,
                    ..
                } => {
                    if let Some((defid, rawlist)) = func.const_fn_def() {
                        if rawlist.len() == 0 {
                            continue;
                        }

                        let ty = rawlist.type_at(0);
                        if ty.is_trait() {
                            if !self.tcx.def_path_debug_str(defid).contains("Animal::speak") {
                                if self.debug {
                                    println!("SKIPPING: {:?}", defid);
                                }
                                continue;
                            }

                            if self.debug {
                                println!("FIRST ARG == TRAIT: {:?} ({:?})", func, defid);
                                println!("in scope: {:?}", cur_scope);
                            }

                            let num_bbs = body.basic_blocks.len();
                            self.replace_dynamic_dispatch(
                                cur_scope,
                                &mut patch,
                                &defid,
                                old_locals,
                                ty,
                                *destination,
                                bb,
                                data,
                                num_bbs,
                            );
                        }
                    }
                }
                _ => {}
            }
        }

        patch.apply(body);
    }

    fn get_bbs(&self, data: &BasicBlockData<'tcx>) -> (BasicBlock, BasicBlock) {
        let edges = data.terminator().kind.edges();
        let bb_old_next;
        let bb_old_cleanup;
        match edges {
            TerminatorEdges::AssignOnReturn {
                return_, cleanup, ..
            } => {
                //debug!("edges problems?");
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

    fn dyndispatch_retval(
        &self,
        old_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        term_dst_place: Place<'tcx>,
    ) -> bool {
        if old_locals.get(term_dst_place.local).is_some() {
            true
        } else {
            false
        }
    }

    //fn get_dids(&self, simplified_ty: &SimplifiedType, assoc_items_did: DefId) -> (DefId, DefId) {
    //    let ty_did;
    //    match simplified_ty {
    //        SimplifiedType::Adt(inner_did) => ty_did = inner_did,
    //        _ => panic!("impl is not Adt"),
    //    }

    //    // dummy init value b/c the compiler thinks we can
    //    // proceed with an uninit value despite the `init` flag
    //    let mut init = false;
    //    let mut fn_did = DUMMY_DEFID;
    //    for assoc_item in self
    //        .tcx
    //        .associated_items(assoc_items_did)
    //        .in_definition_order()
    //    {
    //        fn_did = assoc_item.def_id;
    //        init = true;
    //    }
    //    if !init {
    //        panic!("no assoc items!");
    //    }

    //    (*ty_did, fn_did)
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

    fn get_struct_dids(&self, cur_scope: DefId) -> Vec<DefId> {
        let mut structs = vec![];
        let first_arg = self.cmap.scoped_get(
            Some(cur_scope),
            &MapKey::Place(Place {
                local: Local::from_usize(1),
                projection: List::empty(),
            }),
            false,
        );
        if self.debug {
            println!("first arg: {:?}", first_arg);
        }

        if let Some(first_arg_vartype) = first_arg {
            if let VarType::Values(first_arg_constraints) = first_arg_vartype {
                if first_arg_constraints.len() != 1 {
                    todo!("handle diff lens: {:?}", first_arg_constraints.len());
                }
                for first_arg_constraint in first_arg_constraints.iter() {
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
                                            structs.push(*defid);
                                        }
                                        _ => todo!(),
                                    }
                                }
                            } else {
                                todo!("struct is not a box");
                            }
                        }
                        _ => todo!("handle more types"),
                    }
                }
            } else {
                panic!("first arg is a scope...");
            }
        }

        structs
    }

    fn get_impl_dids(
        &self,
        cur_scope: DefId,
        structs: &Vec<DefId>,
        impltors: &Vec<DefId>,
    ) -> Vec<DefId> {
        let mut impls = vec![];
        for struct_ in structs.iter() {
            if let Some(impl_blocks) = self.funcs.struct_impls.get(struct_) {
                println!("impl_blocks: {:?}", impl_blocks);
                if let Some(assoc) = self.funcs.impl_blocks_to_impls.get(&impl_blocks[0]) {
                    println!("assoc: {:?}", assoc);
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
        let structs = self.get_struct_dids(cur_scope);
        if let Some(impltors) = self
            .funcs
            .trait_fn_impltors
            .lock()
            .unwrap()
            .get(&dynfunc_defid)
        {
            let impls = self.get_impl_dids(cur_scope, &structs, impltors);
            return std::iter::zip(structs, impls).collect();
        } else {
            panic!("no impltors");
        }
    }

    fn replace_dynamic_dispatch(
        &self,
        cur_scope: DefId,
        patch: &mut MirPatch<'tcx>,
        dynfunc_defid: &DefId,
        old_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        ty: Ty<'tcx>,
        term_dst_place: Place<'tcx>,
        bb: BasicBlock,
        data: &BasicBlockData<'tcx>,
        num_bbs: usize,
    ) {
        // get old terminator's edges
        let (bb_old_next, bb_old_cleanup) = self.get_bbs(data);
        let has_retval = self.dyndispatch_retval(old_locals, term_dst_place);
        let bb_old_return;
        if has_retval {
            bb_old_return = bb_old_next + 1;
        } else {
            bb_old_return = bb_old_next;
        }

        let bb_into_raw_exp = BasicBlock::from_usize(num_bbs);

        let traitobj_did = self.get_traitobj_did(ty);
        let dids = self.get_trait_impl_dids(cur_scope, dynfunc_defid);

        if self.debug {
            println!("num_bbs: {:?}", num_bbs);
            //println!("bb_old_next: {:?}", bb_old_next);
            //println!("bb_old_cleanup: {:?}", bb_old_cleanup);
            //println!("has_retval: {:?}", has_retval);
            //println!("bb_old_return: {:?}", bb_old_return);
            //println!("bb_into_raw_exp: {:?}", bb_into_raw_exp);
            println!("traitobj_did: {:?}", traitobj_did);
            println!("cur_bb: {:?}", bb);
            println!("new_start: {:?}", bb_into_raw_exp);
            println!("dids: {:?}", dids);
        }

        // FIXME
        //assert_eq!(bb_old_next.as_usize(), 2);
        //assert_eq!(bb_old_cleanup.as_usize(), 4);
        //assert_eq!(bb_into_raw_exp.as_usize(), 6);

        // FIXME get these dynamically
        //let retval = Local::from_u32(0);
        //let animal = Local::from_u32(1);
        //let animal_vtable = Local::from_u32(2);
        //let cat_vtable = Local::from_u32(3);

        // mod cur speak block w/ goto new start (into_raw)
        //self.replace_dyndispatch_term_w_goto(patch, bb, bb_into_raw_exp);

        /*
        // into_raw
        let mut_dyn_traitobj = add_mut_dyn_traitobj_temp(patch, traitobj_did);
        let boxed_dyn_traitobj1 = add_boxed_dyn_traitobj_temp(patch, traitobj_did);

        let bb_first_compare_exp = BasicBlock::from_usize(bb_into_raw_exp.as_usize() + 1);
        let bb_into_raw_act = add_into_raw_block(
            patch,
            bb_first_compare_exp,
            bb_old_cleanup,
            mut_dyn_traitobj,
            boxed_dyn_traitobj1,
            animal,
            traitobj_did,
            None,
        );
        assert_eq!(bb_into_raw_act, bb_into_raw_exp);

        // TODO for loop -> compare/switch blocks (n-1)

        // TODO for loop -> speak/goto blocks (n)

        // first_comparison
        let raw_traitobj1 = add_raw_traitobj_temp(patch);
        let animal_vtable_ref = add_dynmetadata_ref_temp(patch, traitobj_did);
        let cat_vtable_ref = add_dynmetadata_ref_temp(patch, traitobj_did);
        let first_eq_res = add_mut_bool_temp(patch);

        let bb_first_switch_exp = BasicBlock::from_usize(bb_first_compare_exp.as_usize() + 1);
        let bb_first_compare_act = add_compare_vtable_block(
            patch,
            bb_first_switch_exp,
            bb_old_cleanup,
            raw_traitobj1,
            mut_dyn_traitobj,
            animal_vtable,
            animal_vtable_ref,
            cat_vtable,
            cat_vtable_ref,
            first_eq_res,
            traitobj_did,
            false,
            Some(vec![boxed_dyn_traitobj1]),
        );
        assert_eq!(bb_first_compare_act, bb_first_compare_exp);

        // first_switch
        let bb_cat_speak_exp = BasicBlock::from_usize(bb_first_switch_exp.as_usize() + 1);
        let bb_dog_speak_exp = BasicBlock::from_usize(bb_cat_speak_exp.as_usize() + 2);
        let bb_first_switch_act =
            add_switch_block(patch, bb_cat_speak_exp, bb_dog_speak_exp, first_eq_res);
        assert_eq!(bb_first_switch_exp, bb_first_switch_act);

        // first_speak
        let raw_traitobj2 = add_raw_traitobj_temp(patch);
        let cat_obj = add_concretety_ref_temp(patch, cat_did);

        let bb_cat_ret_exp = BasicBlock::from_usize(bb_cat_speak_exp.as_usize() + 1);
        let bb_cat_speak_act = add_speak_block(
            patch,
            bb_cat_ret_exp,
            bb_old_cleanup,
            raw_traitobj1,
            raw_traitobj2,
            retval,
            cat_obj,
            cat_did,
            cat_speak_did,
            None,
        );
        assert_eq!(bb_cat_speak_exp, bb_cat_speak_act);

        // goto (to bb_old_return)
        let bb_cat_ret_act = add_goto_block(patch, bb_old_return);
        assert_eq!(bb_cat_ret_exp, bb_cat_ret_act);

        // second_speak
        let raw_traitobj4 = add_raw_traitobj_temp(patch);
        let dog_obj = add_concretety_ref_temp(patch, dog_did);

        let bb_dog_ret_exp = BasicBlock::from_usize(bb_dog_speak_exp.as_usize() + 1);
        let bb_dog_speak_act = add_speak_block(
            patch,
            bb_dog_ret_exp,
            bb_old_cleanup,
            raw_traitobj1,
            raw_traitobj4,
            retval,
            dog_obj,
            dog_did,
            dog_speak_did,
            None,
        );
        assert_eq!(bb_dog_speak_exp, bb_dog_speak_act);

        // goto (to bb_old_return)
        let bb_dog_ret_act = add_goto_block(patch, bb_old_return);
        assert_eq!(bb_dog_ret_exp, bb_dog_ret_act);
        */
    }
}
