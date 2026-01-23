use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::mir::interpret::Scalar; 
use rustc_middle::ty::{List, TyCtxt, TyKind};
use rustc_data_structures::fx::{FxHashSet as HashSet};
//use rustc_data_structures::packed::Pu128;
use rustc_index::IndexSlice;
use rustc_span::source_map::Spanned;

use crate::func_collect::FuncMap;
use crate::constraints::{Constraints, ConstraintMap, MapKey, VarType};
use crate::core::{FuncVal, VerifoptRval};
use crate::wto::BBDeps;
use crate::error::Error;

pub struct InterpPass<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub funcs: &'a FuncMap<'tcx>,
}

impl<'a, 'tcx> InterpPass<'a, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        funcs: &'a FuncMap<'tcx>,
    ) -> InterpPass<'a, 'tcx> {
        Self { tcx, funcs }
    }

    pub fn run(
        &self, 
        cmap: &mut ConstraintMap<'tcx>, 
        prev_scope: Option<DefId>,
        cur_scope: DefId,
        body: &'tcx Body<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        // set up main / entry-point scope
        let locals = body.local_decls.as_slice();
        let ty = locals[Local::from_usize(0)].ty;
        let mut set = HashSet::default();
        set.insert(VerifoptRval::IdkType(ty));
        let mut main_cmap = ConstraintMap::new();

        main_cmap.cmap.insert(
            MapKey::Place(Place {
                local: Local::from_usize(0),
                projection: List::empty(),
            }),
            Box::new(VarType::Values(set)),
        );

        cmap.cmap.insert(
            MapKey::ScopeId(cur_scope),
            Box::new(VarType::SubScope(
                prev_scope,
                vec![(
                    Box::new("main functype"),
                    main_cmap,
                )],
            )),
        );

        // start interpretation
        self.visit_body(cmap, prev_scope, cur_scope, body)
    }

    fn visit_body(
        &self, 
        cmap: &mut ConstraintMap<'tcx>, 
        prev_scope: Option<DefId>,
        cur_scope: DefId,
        body: &'tcx Body<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        // FIXME how do loops affect this order?

        println!("\n###### INTERP-ING NEW BODY for func {:?}\n", cur_scope);
        println!("prev_scope: {:?}", prev_scope);
        println!("cur_scope: {:?}", cur_scope);

        // get Weak Topological Ordering of function body
        // TODO memoize this ordering
        let mut bb_deps = BBDeps::new(body);
        let mut last_res = None;
        loop {
            println!("LOOPING");
            if bb_deps.ordering.is_empty() {
                println!("empty");
                break;
            }
            let bb = bb_deps.ordering.remove(0);
            println!("--NEW BB: {:?}", bb);
            let data = body.basic_blocks.get(bb).unwrap();
            last_res = self.visit_basic_block_data(cmap, &mut bb_deps, body.local_decls.as_slice(), prev_scope, cur_scope, &bb, data)?;
        }

        Ok(last_res)
    }

    fn visit_basic_block_data(
        &self,
        cmap: &mut ConstraintMap<'tcx>, 
        bb_deps: &mut BBDeps<'tcx>,
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        prev_scope: Option<DefId>,
        cur_scope: DefId,
        bb: &BasicBlock,
        data: &BasicBlockData<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        let mut last_res = None;

        println!("# visiting BASICBLOCK for {:?}", cur_scope);

        for (_, stmt) in data.statements.iter().enumerate() {
            println!("# visiting STATEMENT");
            last_res = self.visit_statement(cmap, body_locals, cur_scope, stmt)?;
        }

        if let Some(term) = &data.terminator {
            println!("# visiting TERM");
            last_res = self.visit_terminator(cmap, bb, bb_deps, prev_scope, cur_scope, &term)?;
        }

        bb_deps.mark_visited(bb);

        Ok(last_res)
    }

    fn visit_statement(
        &self,
        cmap: &mut ConstraintMap<'tcx>, 
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        cur_scope: DefId,
        statement: &Statement<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        match statement.kind {
            StatementKind::Assign(box (place, ref rvalue)) => {
                println!("assignment!");
                println!("place: {:?}", place);
                println!("rval: {:?}", rvalue);

                let mut set = HashSet::default();
                set.insert(VerifoptRval::from_rvalue(body_locals, rvalue));

                cmap.scoped_set(
                    Some(cur_scope),
                    MapKey::Place(place),
                    Box::new(VarType::Values(set)),
                );
                println!("~~~CMAP: {:?}", cmap);

                Ok(None)
            },
            _ => Ok(None),
        }
    }

    fn interp_func_call(
        &self,
        cmap: &mut ConstraintMap<'tcx>, 
        prev_scope: Option<DefId>,
        cur_scope: DefId,
        func: &Operand<'tcx>,
        args: &Box<[Spanned<Operand<'tcx>>]>,
        destination: &Place<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        println!("\n-----------");
        println!("call!");
        println!("func: {:?}", func);
        println!("args: {:?}", args);
        println!("destination place: {:?}", destination);

        match func {
            Operand::Constant(box co) => {
                match co.const_ {
                    Const::Val(_, ty) => {
                        match ty.kind() {
                            TyKind::FnDef(def_id, _) => {
                                match self.funcs.funcs.get(def_id) {
                                    Some(funcval_vec) => {
                                        let mut res_vec = vec![];
                                        let mut cmap_vec = vec![];

                                        for funcval in funcval_vec.iter() {
                                            if funcval.is_intrinsic {
                                                println!("\n### FUNC IS INTRINSIC {:?}\n", funcval.def_id);
                                                if let Some(rettype) = funcval.rettype {
                                                    let mut constraints = HashSet::default();
                                                    constraints.insert(VerifoptRval::IdkType(rettype));
                                                    res_vec.push(constraints);
                                                }
                                            } else if !self.tcx.is_mir_available(def_id) {
                                                println!("MIR NOT AVAILABLE for {:?}", def_id);
                                                match self.funcs.assocfns_to_traits.lock().unwrap().get(def_id) {
                                                    Some(trait_def_id) => {
                                                        println!("found in trait: {:?}", trait_def_id);
                                                        match self.funcs.trait_impls.lock().unwrap().get(def_id) {
                                                            Some(impltors) => {
                                                                println!("impltors: {:?}", impltors);
                                                                // TODO get type of first arg
                                                                // (receiver)
                                                            },
                                                            None => {
                                                                panic!("missing implementors");
                                                            }
                                                        }
                                                    },
                                                    None => {
                                                        // FIXME are panics the only possible thing here?
                                                        let mut constraints = HashSet::default();
                                                        constraints.insert(VerifoptRval::Undef());
                                                        res_vec.push(constraints);
                                                    }
                                                }
                                            } else {
                                                let mut cmap_clone = cmap.clone();
                                                println!("\n### SETTING UP FUNC CALL (RESOLVING ARGS) for func {:?}\n", funcval.def_id);
                                                self.resolve_args(&mut cmap_clone, prev_scope, cur_scope, funcval, args);


                                                // visit callee
                                                let callee_body = self.tcx.optimized_mir(*def_id);
                                                match self.visit_body(&mut cmap_clone, Some(cur_scope), *def_id, callee_body) {
                                                    Ok(maybe_constraints) => {
                                                        println!("\n##### DONE w func {:?}\n", func);
                                                        // TODO MIR is not generated for intrinsics...
                                                        // need to augment interpreter knowledge
                                                        // somehow
                                                        cmap_vec.push(cmap_clone);
                                                        if let Some(constraints) = maybe_constraints {
                                                            res_vec.push(constraints);
                                                        }
                                                    },
                                                    err @ Err(_) => return err,
                                                }
                                            }
                                        }

                                        // merge cmap / returned value states
                                        if cmap_vec.len() > 1 || res_vec.len() > 1 {
                                            todo!("TODO: impl cmap_vec/res_vec merge");
                                        }

                                        if cmap_vec.len() > 0 {
                                            *cmap = cmap_vec.pop().unwrap();
                                        }

                                        if res_vec.len() > 0 {
                                            let val = res_vec.pop().unwrap();
                                            println!("setting return val: {:?}", val);
                                            cmap.scoped_set(
                                                Some(cur_scope),
                                                MapKey::Place(*destination),
                                                Box::new(VarType::Values(val)),
                                            );
                                            println!("~~~CMAP: {:?}", cmap);
                                        }

                                        Ok(None)
                                    },
                                    None => {
                                        println!("no such function (might be a dynamic call): {:?}", def_id);
                                        println!("is mir available? {:?}", self.tcx.is_mir_available(def_id));
                                        // TODO dynamic dispatch
                                        // if first arg == self, use constraints to prune funcvals
                                        Ok(None)
                                    },
                                }
                            },
                            _ => panic!("not a FnDef"),
                        }
                    },
                    _ => panic!("not a Val Const"),
                }
            },
            // TODO also handle indirect invokes (via variable name, _not_ in funcs)
            _ => panic!("not a Const"),
        }
    }

    fn interp_return(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        cur_scope: DefId,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        // return constraints at place _0
        println!("RETURNING...");
        match cmap.scoped_get(
            Some(cur_scope),
            &MapKey::Place(Place {
                local: Local::from_usize(0),
                projection: List::empty(),
            }),
            false,
        ) {
            Some(vartype) => {
                match vartype {
                    VarType::Values(constraints) => {
                        Ok(Some(constraints))
                    },
                    _ => panic!("return scope?"),
                }
            },
            None => {
                // assuming our interpreter is correct, if the local _0 was not assigned to then
                // this must mean that there is nothing to return. so, return nothing
                println!("_0 local was not assigned to");
                Ok(None)
            }
        }
    }

    fn interp_switchint(
        &self,
        cmap: &mut ConstraintMap<'tcx>, 
        bb: &BasicBlock,
        bb_deps: &mut BBDeps<'tcx>,
        cur_scope: DefId,
        discr: &Operand<'tcx>,
        targets: &SwitchTargets,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        match discr {
            Operand::Copy(place) | Operand::Move(place) => {
                match cmap.scoped_get(Some(cur_scope), &MapKey::Place(*place), false) {
                    Some(vartype) => match vartype {
                        VarType::Values(constraints) => {
                            // FIXME zero may not be the only option, or may not be an option
                            // period (e.g., could be 1)
                            let mut num_zeros = 0;
                            let mut num_nonzeros = 0;
                            let val = targets.all_values()[0];

                            for constraint in constraints.iter() {
                                match constraint {
                                    VerifoptRval::Scalar(scalar) => match scalar {
                                        Scalar::Int(s_int) => {
                                            if s_int.to_bits_unchecked() == val.get() {
                                                num_zeros += 1;
                                            } else {
                                                num_nonzeros += 1;
                                            }
                                        },
                                        _ => todo!("scalar ptr"),
                                    },
                                    VerifoptRval::Ref(_boxed_rval) => {
                                        todo!("ref");
                                    },
                                    _ => {},
                                }
                            }

                            if num_zeros == 0 && num_nonzeros != 0 {
                                // only explore "otherwise" target (prune "0")
                                bb_deps.prune(bb, targets.all_targets()[0]);
                            } else if num_nonzeros == 0 && num_zeros != 0 {
                                // only explore "0" target (prune "otherwise")
                                bb_deps.prune(bb, targets.all_targets()[1]);
                            }
                        },
                        _ => todo!("scope not impl"),
                    },
                    None => {
                        println!("\ncmap: {:?}", cmap);
                        println!("scope to search in: {:?}", cur_scope);
                        println!("place to get: {:?}", place);
                        panic!("place does not exist: {:?}", place);
                    },
                }
            },
            Operand::Constant(boxed_co) => {
                println!("boxed_co: {:?}", boxed_co);
                todo!("constant operand");
            }
            _ => println!("runtime checks (ignoring)"),
        }

        Ok(None)
    }

    fn visit_terminator(
        &self,
        cmap: &mut ConstraintMap<'tcx>, 
        bb: &BasicBlock,
        bb_deps: &mut BBDeps<'tcx>,
        prev_scope: Option<DefId>,
        cur_scope: DefId,
        terminator: &Terminator<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        match &terminator.kind {
            TerminatorKind::Call { func, args, destination, .. } => {
                self.interp_func_call(cmap, prev_scope, cur_scope, func, args, destination)
            },
            TerminatorKind::Return => {
                self.interp_return(cmap, cur_scope)
            }
            TerminatorKind::SwitchInt { discr, targets } => {
                self.interp_switchint(cmap, bb, bb_deps, cur_scope, discr, targets)
            }
            // TODO Goto ?
            // TODO TailCall
            _ => Ok(None),
        }
    }

    fn resolve_args(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        prev_scope: Option<DefId>,
        cur_scope: DefId,
        funcval: &FuncVal<'tcx>,
        args: &Box<[Spanned<Operand<'tcx>>]>,
    ) {
        let mut func_cmap = ConstraintMap::new();
        let arg_vec: Vec<Operand<'tcx>> = args.into_iter().map(|x| x.clone().node).collect();

        // add arg values into func_cmap
        //for ((param_name, param_type), arg) in std::iter::zip(funcval.params.clone(), arg_vec) {
        for (param_name, arg) in std::iter::zip(funcval.params.clone(), arg_vec) {
            println!("param_name: {:?}", param_name);
            //println!("param_type: {:?}", param_type);
            println!("arg: {:?}", arg);

            match arg {
                // cmap.scoped_get + add result to func_cmap
                Operand::Copy(place) | Operand::Move(place) => {
                    match cmap.scoped_get(Some(cur_scope), &MapKey::Place(place), false) {
                        Some(vartype) => match vartype {
                            VarType::Values(constraints) => {
                                func_cmap.cmap.insert(
                                    MapKey::Place(param_name),
                                    Box::new(VarType::Values(
                                        // TODO add type?
                                        constraints.clone(),
                                    )),
                                );
                            }
                            _ => {},
                        }
                        None => {
                            panic!("place doesn't exist in cmap, maybe this is a func name");
                        }
                    }
                }
                Operand::Constant(box co) => {
                    println!("co.const_: {:?}", co.const_);
                    match co.const_ {
                        Const::Val(val, ty) => match val {
                            ConstValue::Scalar(scalar) => {
                                println!("scalar: {:?}", scalar);
                                let mut constraints = HashSet::default();
                                constraints.insert(VerifoptRval::Scalar(scalar));
                                func_cmap.cmap.insert(
                                    MapKey::Place(param_name),
                                    Box::new(VarType::Values(constraints)),
                                );
                            },
                            ConstValue::ZeroSized => {
                                println!("zero-sized");
                                let mut constraints = HashSet::default();
                                constraints.insert(VerifoptRval::IdkType(ty));
                                func_cmap.cmap.insert(
                                    MapKey::Place(param_name),
                                    Box::new(VarType::Values(constraints)),
                                );
                            },
                            ConstValue::Slice { alloc_id, meta } => {
                                todo!("slice");
                            },
                            ConstValue::Indirect { alloc_id, offset } => {
                                todo!("indirect");
                            },
                        },
                        Const::Ty(..) => {
                            todo!("ty");
                        },
                        Const::Unevaluated(uneval, ty) => {
                            // TODO do we care about evaluating constants?
                            println!("unevaluated const");
                            println!("uneval: {:?}", uneval);
                            println!("ty: {:?}", ty);
                            let mut constraints = HashSet::default();
                            constraints.insert(VerifoptRval::IdkType(ty));
                            func_cmap.cmap.insert(
                                MapKey::Place(param_name),
                                Box::new(VarType::Values(constraints)),
                            );
                        },
                    }
                },
                _ => todo!("TODO runtime checks?"),
            }
        }

        // add func_cmap to outer cmap
        // FIXME assuming funcname does not exist in cmap
        cmap.cmap.insert(
            MapKey::ScopeId(funcval.def_id),
            Box::new(VarType::SubScope(
                // FIXME not getting properly set?
                Some(cur_scope),
                vec![(
                    // if this item is a function (which i guess since we're using
                    // VarType::SubScope it must be), this first part of the tuple seems to store
                    // the function type, so that if this function is ever (indirectly?) invoked,
                    // the return _type_ of the function will correspond to the type of the
                    // invocation result, but I am not sure how useful this is since we have types
                    // in lots of places already...
                    Box::new("functype (tbd)"),
                    func_cmap,
                )],
            )),
        );

        println!("~~~CMAP: {:?}", cmap);
    }
}

