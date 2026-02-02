use rustc_middle::mir::*;
//use rustc_middle::mir::visit::MutVisitor;
use rustc_index::IndexSlice;
use rustc_middle::ty::{Ty, TyCtxt, TyKind};
use rustc_mir_transform::patch::MirPatch;
use rustc_span::def_id::DefId;

use crate::FuncMap;
use crate::constraints::ConstraintMap;

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

    pub fn run(&self, body: &mut Body<'tcx>) {
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
                            if self.debug {
                                println!("FIRST ARG == TRAIT: {:?} ({:?})", func, defid);
                            }

                            let num_bbs = body.basic_blocks.len();
                            self.replace_dynamic_dispatch(
                                &mut patch,
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

    fn replace_dynamic_dispatch(
        &self,
        patch: &mut MirPatch<'tcx>,
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

        /*
        let impls = tcx.trait_impls_of(traitobj_did);
        let nb_impls_dids = impls.non_blanket_impls();
        let impls_keys: Vec<_> = nb_impls_dids.keys().collect();
        let impls_vals: Vec<_> = nb_impls_dids.values().collect();
        let (cat_did, cat_speak_did) =
            get_dids(tcx, impls_keys.get(0).unwrap(), impls_vals.get(0).unwrap().as_slice()[0]);
        let (dog_did, dog_speak_did) =
            get_dids(tcx, impls_keys.get(1).unwrap(), impls_vals.get(1).unwrap().as_slice()[0]);

        assert_eq!(bb_old_next.as_usize(), 1);
        assert_eq!(bb_old_cleanup.as_usize(), 3);
        assert_eq!(bb_into_raw_exp.as_usize(), 5);

        let retval = Local::from_u32(0);
        let animal = Local::from_u32(1);
        let animal_vtable = Local::from_u32(2);
        let cat_vtable = Local::from_u32(3);

        // mod cur speak block w/ goto new start (into_raw)
        replace_dyndispatch_term_w_goto(patch, bb, bb_into_raw_exp);

        // into_raw
        let mut_dyn_traitobj = add_mut_dyn_traitobj_temp(tcx, patch, traitobj_did);
        let boxed_dyn_traitobj1 = add_boxed_dyn_traitobj_temp(tcx, patch, traitobj_did);

        let bb_first_compare_exp = BasicBlock::from_usize(bb_into_raw_exp.as_usize() + 1);
        let bb_into_raw_act = add_into_raw_block(
            tcx,
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
        let raw_traitobj1 = add_raw_traitobj_temp(tcx, patch);
        let animal_vtable_ref = add_dynmetadata_ref_temp(tcx, patch, traitobj_did);
        let cat_vtable_ref = add_dynmetadata_ref_temp(tcx, patch, traitobj_did);
        let first_eq_res = add_mut_bool_temp(tcx, patch);

        let bb_first_switch_exp = BasicBlock::from_usize(bb_first_compare_exp.as_usize() + 1);
        let bb_first_compare_act = add_compare_vtable_block(
            tcx,
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
            add_switch_block(tcx, patch, bb_cat_speak_exp, bb_dog_speak_exp, first_eq_res);
        assert_eq!(bb_first_switch_exp, bb_first_switch_act);

        // first_speak
        let raw_traitobj2 = add_raw_traitobj_temp(tcx, patch);
        let cat_obj = add_concretety_ref_temp(tcx, patch, cat_did);

        let bb_cat_ret_exp = BasicBlock::from_usize(bb_cat_speak_exp.as_usize() + 1);
        let bb_cat_speak_act = add_speak_block(
            tcx,
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
        let raw_traitobj4 = add_raw_traitobj_temp(tcx, patch);
        let dog_obj = add_concretety_ref_temp(tcx, patch, dog_did);

        let bb_dog_ret_exp = BasicBlock::from_usize(bb_dog_speak_exp.as_usize() + 1);
        let bb_dog_speak_act = add_speak_block(
            tcx,
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
