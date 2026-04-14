use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::ty::{GenericArg, TyCtxt, TyKind};
use rustc_span::source_map::Spanned;

use crate::core::FuncVal;
use crate::func_collect::FuncMap;

pub struct CHAPass<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub funcs: &'a FuncMap<'tcx>,
    pub debug: bool,
}

impl<'a, 'tcx> CHAPass<'a, 'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, funcs: &'a FuncMap<'tcx>, debug: bool) -> CHAPass<'a, 'tcx> {
        Self { tcx, funcs, debug }
    }

    pub fn run(&self, cur_scope: DefId, body: &'tcx Body<'tcx>) {
        // track call stack for debugging
        let mut call_stack = vec![cur_scope];
        self.visit_body(&mut call_stack, cur_scope, body);
    }

    fn visit_body(&self, call_stack: &mut Vec<DefId>, cur_scope: DefId, body: &'tcx Body<'tcx>) {
        // just visit bbs in order since there is no dependent information we need to track
        for bb in body.basic_blocks.reverse_postorder() {
            let data = body.basic_blocks.get(*bb).unwrap();
            if data.is_cleanup {
                continue;
            }

            if self.debug {
                println!("\n--NEW BB: {:?}", bb);
            }

            self.visit_basic_block_data(call_stack, cur_scope, &bb, data);
        }
    }

    fn visit_basic_block_data(
        &self,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        bb: &BasicBlock,
        data: &BasicBlockData<'tcx>,
    ) {
        if self.debug {
            println!("#############################");
            println!("# visiting BASICBLOCK for {:?}", cur_scope);
            println!("#############################");
        }

        // ignoring statements b/c these are really only used to populate the CMap in VerifOpt
        //for (_, stmt) in data.statements.iter().enumerate() {
        //    if self.debug {
        //        println!("\n# -------------------------------");
        //        println!("# visiting STATEMENT in {:?} of defid {:?}", bb, cur_scope);
        //        println!("# -------------------------------");
        //    }
        //    self.visit_statement(stmt);
        //}

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
        //if self.debug {
        //    println!(
        //        "\n### SETTING UP FUNC CALL (RESOLVING ARGS) for func {:?}\n",
        //        funcval.def_id
        //    );
        //    println!("args: {:?}\n", args);
        //}

        //self.resolve_args(&mut cmap, cur_scope, funcval, args);

        //if funcval.param_generics.is_some() {
        //    if self.debug {
        //        println!("\n## RESOLVING GENERICS IN ARGTYS...\n");
        //    }
        //    self.resolve_generic_argtys(&mut cmap, cur_scope, body_locals, funcval, func_genargs);
        //}

        //if funcval.ret_generics.is_some() {
        //    if self.debug {
        //        println!("\n## RESOLVING GENERICS IN RETTY...\n");
        //    }
        //    self.resolve_generic_retty(
        //        &mut cmap,
        //        cur_scope,
        //        body_locals,
        //        funcval,
        //        func_genargs,
        //        destination,
        //    );
        //}

        // visit callee
        let callee_body = self.tcx.optimized_mir(funcval.def_id);
        call_stack.push(funcval.def_id);
        self.visit_body(call_stack, funcval.def_id, callee_body)
    }
}
