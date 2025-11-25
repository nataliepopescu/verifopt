use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::ty::{List, TyCtxt, TyKind};
use rustc_data_structures::fx::{FxHashSet as HashSet};
use rustc_index::IndexSlice;
use rustc_span::source_map::Spanned;

use crate::func_collect::FuncMap;
use crate::constraints::{Constraints, ConstraintMap, MapKey, VarType};
use crate::core::{FuncVal, VerifoptRval};
use crate::wto::BBDeps;
use crate::error::Error;

pub struct InterpPass<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub func_map: &'a FuncMap<'tcx>,
}

impl<'a, 'tcx> InterpPass<'a, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        func_map: &'a FuncMap<'tcx>,
    ) -> InterpPass<'a, 'tcx> {
        Self { tcx, func_map }
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
                    // FIXME forgot what this field is for
                    Box::new("main"),
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
        // 
        // is it correct that we don't _really_ need to worry about order of 
        // traversal (assuming NO loops) due to SSA?
        //
        // TODO instead of visitor, traverse one-by-one like in SimpleInterp
        // (easier for, e.g., conditionals state merging)

        // TODO get/calc callgraph for every function body we encounter

        println!("\n###### INTERP-ING NEW BODY\n");
        println!("~~~CMAP: {:?}", cmap);

        // get Weak Topological Ordering of function body
        //let bb_deps = BBDeps::new(body);

        let mut last_res = None;
        for (bb, data) in traversal::reverse_postorder(body) {
            println!("bb: {:?}", bb);
            last_res = self.visit_basic_block_data(cmap, body.local_decls.as_slice(), prev_scope, cur_scope, bb, data)?;
        }
        Ok(last_res)
    }

    fn visit_basic_block_data(
        &self,
        cmap: &mut ConstraintMap<'tcx>, 
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        prev_scope: Option<DefId>,
        cur_scope: DefId,
        _block: BasicBlock,
        data: &BasicBlockData<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        let mut last_res = None;

        for (_, stmt) in data.statements.iter().enumerate() {
            last_res = self.visit_statement(cmap, body_locals, cur_scope, stmt)?;
        }

        if let Some(term) = &data.terminator {
            last_res = self.visit_terminator(cmap, prev_scope, cur_scope, &term)?;
        }

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
        println!("-----------");
        println!("call!");
        println!("func: {:?}", func);
        println!("args: {:?}", args);
        println!("place: {:?}", destination);

        match func {
            Operand::Constant(box co) => {
                match co.const_ {
                    Const::Val(_, ty) => {
                        match ty.kind() {
                            TyKind::FnDef(def_id, _) => {
                                match self.func_map.funcs.get(def_id) {
                                    Some(funcval_vec) => {
                                        let mut res_vec = vec![];
                                        let mut cmap_vec = vec![];

                                        for funcval in funcval_vec.iter() {
                                            let mut cmap_clone = cmap.clone();
                                            self.resolve_args(&mut cmap_clone, prev_scope, funcval, args);

                                            // visit callee
                                            let callee_body = self.tcx.optimized_mir(*def_id);
                                            match self.visit_body(&mut cmap_clone, Some(cur_scope), *def_id, callee_body) {
                                                Ok(maybe_constraints) => {
                                                    println!("\n##### DONE\n");
                                                    cmap_vec.push(cmap_clone);
                                                    if let Some(constraints) = maybe_constraints {
                                                        println!("res constraints: {:?}", constraints);
                                                        res_vec.push(constraints);
                                                    }
                                                },
                                                err @ Err(_) => return err,
                                            }
                                        }

                                        // merge cmap / returned value states
                                        if cmap_vec.len() > 1 {
                                            panic!("TODO: impl cmap_vec/res_vec merge");
                                        }

                                        *cmap = cmap_vec.pop().unwrap();
                                        let val = res_vec.pop().unwrap();
                                        println!("val: {:?}", val);
                                        cmap.scoped_set(
                                            Some(cur_scope),
                                            MapKey::Place(*destination),
                                            Box::new(VarType::Values(val)),
                                        );
                                        println!("~~~CMAP: {:?}", cmap);

                                        Ok(None)
                                    },
                                    None => {
                                        println!("no such function (might be a dynamic call): {:?}", def_id);
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
            // TODO also handle indirect invokes (via variable name, _not_ in func_map)
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
                        println!("_0 constraints: {:?}", constraints);
                        Ok(Some(constraints))
                    },
                    _ => panic!("return scope?"),
                }
            },
            None => panic!("no _0 local"),
        }
    }

    fn visit_terminator(
        &self,
        cmap: &mut ConstraintMap<'tcx>, 
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
            // TODO SwitchInt
            // - note: MIR is _not_ SSA! (_2 is assigned twice in get_animal)
            // TODO Goto
            // TODO TailCall
            _ => Ok(None),
        }
    }

    fn resolve_args(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        prev_scope: Option<DefId>,
        funcval: &FuncVal<'tcx>,
        args: &Box<[Spanned<Operand<'tcx>>]>,
    ) {
        println!("RESOLVING ARGS");

        let mut func_cmap = ConstraintMap::new();
        let arg_vec: Vec<Operand<'tcx>> = args.into_iter().map(|x| x.clone().node).collect();

        // add arg values into func_cmap
        for ((param_name, param_type), arg) in std::iter::zip(funcval.params.clone(), arg_vec) {
            println!("param_name: {:?}", param_name);
            println!("param_type: {:?}", param_type);
            println!("arg: {:?}", arg);

            // FIXME how do outer-scope arg names/places interact w current scope (should
            // disambiguate when bring into scope)
            match arg {
                // TODO cmap.scoped_get + add result to func_cmap
                Operand::Copy(place)
                | Operand::Move(place) => {
                    match cmap.scoped_get(prev_scope, &MapKey::Place(place), false) {
                        Some(vartype) => {
                            println!("vartype: {:?}", vartype);
                            match vartype {
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
                        }
                        None => {
                            println!("place doesn't exist in cmap, maybe this is a func name");
                        },
                    }
                }
                Operand::Constant(box co) => {
                    println!("co.const_: {:?}", co.const_);
                    match co.const_ {
                        Const::Val(val, _) => match val {
                            ConstValue::Scalar(scalar) => {
                                println!("scalar: {:?}", scalar);
                                let mut constraints = HashSet::default();
                                constraints.insert(VerifoptRval::Scalar(scalar));
                                func_cmap.cmap.insert(
                                    MapKey::Place(param_name),
                                    Box::new(VarType::Values(
                                            constraints,
                                    )),
                                );
                            },
                            _ => println!("non-scalar val"),
                        },
                        _ => println!("non-val const"),
                    }
                },
            }
        }

        // add func_cmap to outer cmap

        // FIXME assuming "name" (funcname I believe) does not exist in cmap
        cmap.cmap.insert(
            MapKey::ScopeId(funcval.def_id),
            Box::new(VarType::SubScope(
                prev_scope,
                vec![(
                    // FIXME forgot what this field is for
                    Box::new("func"),
                    func_cmap,
                )],
            )),
        );

        println!("~~~CMAP: {:?}", cmap);
    }
}

