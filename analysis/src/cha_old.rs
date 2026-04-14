use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::ty::{GenericArg, TyCtxt, TyKind};
use rustc_span::source_map::Spanned;

use crate::core::{DebugPass, FuncVal};
use crate::func_collect::FuncMap;

pub struct CHAPass<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub funcs: &'a FuncMap<'tcx>,
    pub debug: bool,
}

impl<'a, 'tcx> CHAPass<'a, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        funcs: &'a FuncMap<'tcx>,
        which_debug: DebugPass,
    ) -> CHAPass<'a, 'tcx> {
        let mut debug = false;
        if which_debug == DebugPass::Interp {
            debug = true;
        }
        Self { tcx, funcs, debug }
    }

    pub fn run(&self, cur_scope: DefId, body: &'tcx Body<'tcx>) {
        // track call stack for debugging
        let mut call_stack = vec![cur_scope];
        self.visit_body(&mut call_stack, cur_scope, body);
    }

    fn visit_body(&self, call_stack: &mut Vec<DefId>, cur_scope: DefId, body: &'tcx Body<'tcx>) {
        // just visit bbs in order since there is no dependent information we need to track
        for (bb, data) in body.basic_blocks.iter_enumerated() {
            if data.is_cleanup {
                continue;
            }

            if self.debug {
                println!("\n--NEW BB: {:?}", bb);
            }

            self.visit_block(call_stack, cur_scope, num_bbs, &bb, data, &mut patch, &mut warmup);
        }
    }

    fn visit_block(
        &self,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        num_bbs: usize,
        bb: &BasicBlock,
        data: &BasicBlockData<'tcx>,
        mut patch: &mut MirPatch<'tcx>,
        warmup: &mut bool,
    ) {
        if self.debug {
            println!("#############################");
            println!("# visiting BASICBLOCK for {:?}", cur_scope);
            println!("#############################");
        }

        if let Some(term) = &data.terminator {
            if self.debug {
                println!("\n# -------------------------------");
                println!("# visiting TERM in {:?} of defid {:?}", bb, cur_scope);
                println!("# -------------------------------");
            }
            self.visit_terminator(call_stack, cur_scope, bb, &term);
        }
    }

    fn visit_terminator(
        &self,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        bb: &BasicBlock,
        terminator: &Terminator<'tcx>,
    ) {
        match &terminator.kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => self.visit_func_call(call_stack, cur_scope, func, args, destination),
            TerminatorKind::TailCall { .. } => todo!("impl tail calls"),
            _ => {}
        }
    }

    fn visit_func_call(
        &self,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        func: &Operand<'tcx>,
        args: &Box<[Spanned<Operand<'tcx>>]>,
        destination: &Place<'tcx>,
    ) {
        if self.debug {
            println!("\n-----------");
            println!("call!");
            println!("call_stack: {:#?}", call_stack);
            println!("cur scope: {:?}", cur_scope);
            println!("func: {:?}", func);
            println!("args: {:?}", args);
            println!("destination place: {:?}", destination);
        }

        match func {
            Operand::Constant(box co) => {
                self.visit_direct_func_call(call_stack, cur_scope, co, args, destination)
            }
            _ => todo!("handle indirect invocations"),
        }
    }

    fn visit_direct_func_call(
        &self,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        co: &ConstOperand<'tcx>,
        args: &Box<[Spanned<Operand<'tcx>>]>,
        destination: &Place<'tcx>,
    ) {
        match co.const_ {
            Const::Val(_, ty) => match ty.kind() {
                TyKind::FnDef(def_id, func_genargs) => match self.funcs.funcs.get(def_id) {
                    Some(funcval_vec) => {
                        if funcval_vec.len() != 1 {
                            todo!("unexpected number of functions: {:?}", funcval_vec.len());
                        }

                        for funcval in funcval_vec.iter() {
                            if self.debug {
                                println!("defid: {:?}", def_id);
                                println!("funcval: {:?}", funcval);
                                println!("func genargs: {:?}", func_genargs);
                            }

                            if funcval.is_intrinsic {
                                if self.debug {
                                    println!("\n### FUNC IS INTRINSIC {:?}\n", def_id);
                                }
                            } else if funcval.is_closure {
                                if self.debug {
                                    todo!("\n### FUNC IS CLOSURE: {:?}\n", def_id);
                                }
                            } else if !self.tcx.is_mir_available(def_id) {
                                if self.debug {
                                    println!("MIR NOT AVAILABLE for {:?}", def_id);
                                }

                                let mutex = self.funcs.assoc_fns_to_trait.lock().unwrap();
                                match mutex.get(def_id) {
                                    Some(trait_def_id) => {
                                        let trait_def_id_clone = trait_def_id.clone();
                                        std::mem::drop(mutex);

                                        if self.debug {
                                            println!("\n### DYN DISPATCH TO {:?}\n", def_id);
                                        }

                                        self.visit_dyn_dispatch(
                                            call_stack,
                                            cur_scope,
                                            funcval,
                                            func_genargs.as_slice(),
                                            &trait_def_id_clone,
                                            args,
                                            destination,
                                        );
                                    }
                                    None => {
                                        std::mem::drop(mutex);

                                        if self.debug {
                                            println!("\n### UNDEF FN / RES {:?}\n", def_id);
                                        }
                                    }
                                }
                            } else {
                                if self.debug {
                                    println!("\n### STATIC DISPATCH TO: {:?}", def_id);
                                }

                                self.visit_static_dispatch(
                                    call_stack,
                                    cur_scope,
                                    funcval,
                                    func_genargs.as_slice(),
                                    args,
                                    destination,
                                );
                            }
                        }
                    }
                    None => {
                        panic!("no such function: {:?}", def_id);
                    }
                },
                _ => panic!("not a FnDef"),
            },
            _ => todo!("not a Val Const"),
        }
    }

    fn visit_dyn_dispatch(
        &self,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        assoc_funcval: &FuncVal<'tcx>,
        func_genargs: &[GenericArg<'tcx>],
        trait_def_id: &DefId,
        args: &Box<[Spanned<Operand<'tcx>>]>,
        destination: &Place<'tcx>,
    ) {
        if self.debug {
            println!("found in trait: {:?}", trait_def_id);
        }

        if args.len() == 0 {
            if self.debug {
                println!("argments list empty: cannot determine self type");
            }
            return;
        }

        if assoc_funcval.self_arg.is_none() {
            if self.debug {
                println!("no self type: {:?}", assoc_funcval);
            }
            return;
        }

        // get the concrete implementations of this (trait's) assoc_fn
        let mutex = self.funcs.trait_fn_impls.lock().unwrap();
        if let Some(impls_) = mutex.get(&assoc_funcval.def_id) {
            let impls = impls_.clone();
            std::mem::drop(mutex);

            // get the first (`self`) arg place
            match args[0].node {
                Operand::Copy(place) | Operand::Move(place) => {
                    if self.debug {
                        println!("first arg: {:?}", args[0]);
                        println!("place: {:?}", place);
                        println!("impls: {:#?}", impls);
                        //println!(
                        //    "structs: {:#?}",
                        //    self.funcs.trait_to_struct_impls.get(trait_def_id)
                        //);
                    }
                    self.setup_rewrite();
                }
                _ => {}
            }
        } else {
            std::mem::drop(mutex);
            panic!("missing implementors");
        }
    }

    fn visit_static_dispatch(
        &self,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        funcval: &FuncVal<'tcx>,
        func_genargs: &[GenericArg<'tcx>],
        args: &Box<[Spanned<Operand<'tcx>>]>,
        destination: &Place<'tcx>,
    ) {
        // visit callee
        let callee_body = self.tcx.optimized_mir(funcval.def_id);
        call_stack.push(funcval.def_id);
        self.visit_body(call_stack, funcval.def_id, callee_body)
    }

    fn setup_rewrite(&self, cur_scope: DefId) {
        // optionally include the `should_rewrite` method just in case we happen to rewrite
        // something else when comparing to VerifOpt

        if self.debug {
            println!("PRE REPLACEMENT");
            println!("warmup: {:?}", warmup);
            println!("vtable_locs: {:?}", vtable_locs);
        }
        self.replace_dynamic_dispatch();
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
}
