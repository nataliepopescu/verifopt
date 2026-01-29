use rustc_data_structures::fx::FxHashSet as HashSet;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::interpret::Scalar;
use rustc_middle::mir::*;
use rustc_middle::ty::{List, Ty, TyCtxt, TyKind};
//use rustc_data_structures::packed::Pu128;
use rustc_index::IndexSlice;
use rustc_span::source_map::Spanned;

use crate::constraints::{ConstraintMap, Constraints, MapKey, VarType};
use crate::core::{FuncVal, VerifoptRval};
use crate::error::Error;
use crate::func_collect::FuncMap;
use crate::wto::BBDeps;

pub struct InterpPass<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub funcs: &'a FuncMap<'tcx>,
}

impl<'a, 'tcx> InterpPass<'a, 'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, funcs: &'a FuncMap<'tcx>) -> InterpPass<'a, 'tcx> {
        Self { tcx, funcs }
    }

    pub fn run(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        prev_scope: Option<DefId>,
        cur_scope: DefId,
        body: &'tcx Body<'tcx>,
        debug: bool,
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
                vec![(Box::new("main functype"), main_cmap)],
            )),
        );

        // start interpretation
        self.visit_body(cmap, prev_scope, cur_scope, body, debug)
    }

    fn visit_body(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        prev_scope: Option<DefId>,
        cur_scope: DefId,
        body: &'tcx Body<'tcx>,
        debug: bool,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        // TODO how do loops affect this order?

        if debug {
            println!("\n###### INTERP-ING NEW BODY for func {:?}\n", cur_scope);
            println!("prev_scope: {:?}", prev_scope);
            println!("cur_scope: {:?}", cur_scope);
        }

        // get Weak Topological Ordering of function body
        // TODO memoize
        let mut bb_deps = BBDeps::new(body);
        let mut last_res = None;
        loop {
            if bb_deps.ordering.is_empty() {
                break;
            }
            let bb = bb_deps.ordering.remove(0);
            if debug {
                println!("\n--NEW BB: {:?}", bb);
            }
            let data = body.basic_blocks.get(bb).unwrap();
            last_res = self.visit_basic_block_data(
                cmap,
                &mut bb_deps,
                body.local_decls.as_slice(),
                //prev_scope,
                cur_scope,
                &bb,
                data,
                debug,
            )?;
        }

        Ok(last_res)
    }

    fn visit_basic_block_data(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        bb_deps: &mut BBDeps,
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        //prev_scope: Option<DefId>,
        cur_scope: DefId,
        bb: &BasicBlock,
        data: &BasicBlockData<'tcx>,
        debug: bool,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        let mut last_res = None;

        if debug {
            println!("#############################");
            println!("# visiting BASICBLOCK for {:?}", cur_scope);
            println!("#############################");
        }

        for (_, stmt) in data.statements.iter().enumerate() {
            if debug {
                println!("\n# -------------------------------");
                println!("# visiting STATEMENT");
                println!("# -------------------------------");
            }
            last_res = self.visit_statement(cmap, body_locals, cur_scope, stmt, debug)?;
        }

        if let Some(term) = &data.terminator {
            if debug {
                println!("\n# -------------------------------");
                println!("# visiting TERM");
                println!("# -------------------------------");
            }
            // FIXME return basic blocks are the only ones whose interp returns Some(_), so could
            // re-architect this loop to only save the result if it is a Some(_) (then wouldn't
            // have to worry about executing cleanup blocks afterwards)
            last_res = self.visit_terminator(cmap, bb, bb_deps, cur_scope, &term, debug)?;
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
        debug: bool,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        match statement.kind {
            StatementKind::Assign(box (place, ref rvalue)) => {
                if debug {
                    println!("assignment!");
                    println!("place: {:?}", place);
                    println!("rval: {:?}", rvalue);

                    /*
                    match rvalue {
                        Rvalue::Use(_op) => {
                            println!("use");
                        }
                        Rvalue::Cast(_, op, _) => {
                            println!("cast");
                            match op {
                                Operand::Copy(place) | Operand::Move(place) => {
                                    println!("place: {:?}", place);
                                    println!(
                                        "value @ place: {:?}",
                                        cmap.scoped_get(
                                            Some(cur_scope),
                                            &MapKey::Place(*place),
                                            false
                                        )
                                    );
                                }
                                _ => println!("other op"),
                            }
                        }
                        _ => println!("other rvalue kind"),
                    }
                    */
                }

                let mut set = HashSet::default();
                set.insert(VerifoptRval::from_rvalue(
                    self.tcx,
                    cmap,
                    cur_scope,
                    body_locals,
                    rvalue,
                    debug,
                ));

                cmap.scoped_add(
                    Some(cur_scope),
                    MapKey::Place(place),
                    Box::new(VarType::Values(set)),
                );
            }
            _ => {}
        }

        Ok(None)
    }

    fn visit_terminator(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        bb: &BasicBlock,
        bb_deps: &mut BBDeps,
        //prev_scope: Option<DefId>,
        cur_scope: DefId,
        terminator: &Terminator<'tcx>,
        debug: bool,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        match &terminator.kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => self.interp_func_call(cmap, cur_scope, func, args, destination, debug),
            TerminatorKind::Return => self.interp_return(cmap, cur_scope, debug),
            TerminatorKind::SwitchInt { discr, targets } => {
                self.interp_switchint(cmap, bb, bb_deps, cur_scope, discr, targets, debug)
            }
            // TODO TailCall
            _ => Ok(None),
        }
    }

    fn interp_return(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        cur_scope: DefId,
        debug: bool,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        // return constraints at place _0
        if debug {
            println!("RETURNING...");
        }
        match cmap.scoped_get(
            Some(cur_scope),
            &MapKey::Place(Place {
                local: Local::from_usize(0),
                projection: List::empty(),
            }),
            false,
        ) {
            Some(vartype) => match vartype {
                VarType::Values(constraints) => {
                    if debug {
                        println!("returning value w following constraints: {:?}", constraints);
                    }
                    Ok(Some(constraints))
                }
                _ => panic!("return scope?"),
            },
            None => {
                // assuming our interpreter is correct, if the local _0 was not assigned to then
                // this must mean that there is nothing to return. so, return nothing
                if debug {
                    println!("_0 local was not assigned to");
                }
                Ok(None)
            }
        }
    }

    fn interp_switchint(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        bb: &BasicBlock,
        bb_deps: &mut BBDeps,
        cur_scope: DefId,
        discr: &Operand<'tcx>,
        targets: &SwitchTargets,
        debug: bool,
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
                                        }
                                        _ => todo!("scalar ptr"),
                                    },
                                    VerifoptRval::IdkType(_)
                                    | VerifoptRval::IdkDefId(_)
                                    | VerifoptRval::Idk() => {}
                                    _ => panic!("unexpected switchint discr: {:?}", constraint),
                                }
                            }

                            if num_zeros == 0 && num_nonzeros != 0 {
                                // only explore "otherwise" target (prune "0")
                                bb_deps.prune(bb, targets.all_targets()[0]);
                            } else if num_nonzeros == 0 && num_zeros != 0 {
                                // only explore "0" target (prune "otherwise")
                                bb_deps.prune(bb, targets.all_targets()[1]);
                            }
                        }
                        _ => todo!("scope not impl"),
                    },
                    None => {
                        if debug {
                            println!("scope to search in: {:?}", cur_scope);
                            println!("place to get: {:?}", place);
                        }
                        panic!("place does not exist: {:?}", place);
                    }
                }
            }
            Operand::Constant(_boxed_co) => {
                todo!("constant operand");
            }
            _ => {
                if debug {
                    println!("runtime checks (ignoring)")
                }
            }
        }

        Ok(None)
    }

    fn handle_dyn_dispatch(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        cur_scope: DefId,
        args: &Box<[Spanned<Operand<'tcx>>]>,
        //destination: &Place<'tcx>,
        //assocfn_def_id: &DefId,
        assoc_funcval: &FuncVal<'tcx>,
        trait_def_id: &DefId,
        debug: bool,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        if debug {
            println!("found in trait: {:?}", trait_def_id);
        }
        match self
            .funcs
            .trait_fn_impltors
            .lock()
            .unwrap()
            .get(&assoc_funcval.def_id)
        {
            Some(impltors) => {
                if args.len() > 0 && assoc_funcval.is_method {
                    match args[0].node {
                        Operand::Copy(place) | Operand::Move(place) => {
                            if debug {
                                println!("first arg: {:?}", args[0]);
                                println!("place: {:?}", place);
                                //println!("\n~~~CMAP @ scope {:?}: {:?}\n", cur_scope, cmap.cmap.get(&MapKey::ScopeId(cur_scope)));
                                println!(
                                    "value: {:?}",
                                    cmap.scoped_get(Some(cur_scope), &MapKey::Place(place), false)
                                );
                                println!("impltors: {:?}", impltors);
                                println!(
                                    "structs: {:?}",
                                    self.funcs.trait_impltors.get(trait_def_id)
                                );
                            }

                            match cmap.scoped_get(Some(cur_scope), &MapKey::Place(place), false) {
                                Some(VarType::Values(constraints)) => {
                                    if constraints.len() != 1 {
                                        todo!("halp");
                                    }

                                    for constraint in constraints.clone().drain() {
                                        match constraint {
                                            VerifoptRval::Ref(inner) => match *inner {
                                                VerifoptRval::IdkStruct(
                                                    struct_defid,
                                                    genarg_vec,
                                                ) => {
                                                    // is this a Box?
                                                    if struct_defid.index.as_usize() == 662
                                                        && struct_defid.krate.as_usize() == 3
                                                    {
                                                        // get first (only, for now) genarg constraint (i.e. IdkType(Cat))
                                                        if let Some(genargs_outer) = genarg_vec {
                                                            if genargs_outer.len() != 1 {
                                                                panic!("handle diff genarg len");
                                                            }
                                                            let genarg_constraint_vec =
                                                                &genargs_outer[0];
                                                            if genarg_constraint_vec.len() != 1 {
                                                                panic!(
                                                                    "handle different genarg constraint len"
                                                                );
                                                            }
                                                            let constraint =
                                                                &genarg_constraint_vec[0];
                                                            println!(
                                                                "constraint: {:?}",
                                                                constraint
                                                            );

                                                            // TODO assoc Cat w the correct speak() impl
                                                        }
                                                    }
                                                }
                                                _ => {}
                                            },
                                            _ => {}
                                        }
                                    }
                                }
                                _ => panic!("why"),
                            }
                        }
                        _ => {}
                    }
                }

                // TODO get type of first arg
                // (receiver)
            }
            None => {
                panic!("missing implementors");
            }
        }

        // FIXME
        Ok(None)
    }

    fn handle_static_dispatch(
        &self,
        mut cmap: &mut ConstraintMap<'tcx>,
        cur_scope: DefId,
        funcval: &FuncVal<'tcx>,
        args: &Box<[Spanned<Operand<'tcx>>]>,
        debug: bool,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        if debug {
            println!(
                "\n### SETTING UP FUNC CALL (RESOLVING ARGS) for func {:?}\n",
                funcval.def_id
            );
        }

        self.resolve_args(&mut cmap, cur_scope, funcval, args, debug);

        // visit callee
        let callee_body = self.tcx.optimized_mir(funcval.def_id);
        self.visit_body(cmap, Some(cur_scope), funcval.def_id, callee_body, debug)
    }

    fn interp_direct_func_call(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        cur_scope: DefId,
        co: &ConstOperand<'tcx>,
        args: &Box<[Spanned<Operand<'tcx>>]>,
        destination: &Place<'tcx>,
        debug: bool,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        let mut res_vec = vec![];
        let mut cmap_vec = vec![];

        match co.const_ {
            Const::Val(_, ty) => match ty.kind() {
                TyKind::FnDef(def_id, _) => match self.funcs.funcs.get(def_id) {
                    Some(funcval_vec) => {
                        for funcval in funcval_vec.iter() {
                            if funcval.is_intrinsic {
                                if debug {
                                    println!("\n### FUNC IS INTRINSIC {:?}\n", def_id);
                                }
                                if let Some(rettype) = funcval.rettype {
                                    let mut constraints = HashSet::default();
                                    constraints.insert(VerifoptRval::IdkType(rettype));
                                    res_vec.push(constraints);
                                }
                            } else if !self.tcx.is_mir_available(def_id) {
                                if debug {
                                    println!("MIR NOT AVAILABLE for {:?}", def_id);
                                }
                                match self.funcs.assocfns_to_traits.lock().unwrap().get(def_id) {
                                    Some(trait_def_id) => {
                                        if debug {
                                            println!("\n### DYN DISPATCH TO {:?}\n", def_id);
                                        }
                                        match self.handle_dyn_dispatch(
                                            cmap,
                                            cur_scope,
                                            args,
                                            funcval, //def_id,
                                            trait_def_id,
                                            debug,
                                        ) {
                                            Ok(Some(constraints)) => {
                                                if debug {
                                                    println!(
                                                        "\n##### DONE w DYN func {:?}\n",
                                                        def_id
                                                    );
                                                    println!("constraints: {:?}", constraints);
                                                }
                                                res_vec.push(constraints);
                                            }
                                            Ok(None) => {
                                                if debug {
                                                    println!(
                                                        "\n##### DONE w DYN func {:?}\n",
                                                        def_id
                                                    );
                                                    println!("no retval");
                                                }
                                            }
                                            e @ Err(_) => return e,
                                        }
                                    }
                                    None => {
                                        if debug {
                                            println!("\n### UNDEF FN / RES {:?}\n", def_id);
                                        }
                                        // if funcval has a return type, use that as the
                                        // "summary" constraint
                                        let mut constraints = HashSet::default();
                                        match funcval.rettype {
                                            Some(ty) => {
                                                constraints.insert(VerifoptRval::IdkType(ty))
                                            }
                                            None => constraints.insert(VerifoptRval::Undef()),
                                        };
                                        res_vec.push(constraints);
                                    }
                                }
                            } else {
                                if debug {
                                    println!("\n### STATIC DISPATCH TO: {:?}", def_id);
                                }
                                let mut cmap_clone = cmap.clone();
                                match self.handle_static_dispatch(
                                    &mut cmap_clone,
                                    cur_scope,
                                    funcval,
                                    args,
                                    debug,
                                ) {
                                    Ok(maybe_constraints) => {
                                        if debug {
                                            println!("\n##### DONE w func {:?}\n", def_id);
                                            println!(
                                                "maybe_constraints from func: {:?}",
                                                maybe_constraints
                                            );
                                        }
                                        cmap_vec.push(cmap_clone);
                                        if let Some(constraints) = maybe_constraints {
                                            res_vec.push(constraints);
                                        }
                                    }
                                    err @ Err(_) => return err,
                                }
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

        if cmap_vec.len() > 1 || res_vec.len() > 1 {
            todo!("impl cmap_vec/res_vec merge");
        }

        if cmap_vec.len() > 0 {
            if debug {
                println!("setting new cmap");
            }
            *cmap = cmap_vec.pop().unwrap();
        } else {
            if debug {
                println!("cmap_vec is empty");
            }
        }

        if res_vec.len() > 0 {
            let val = res_vec.pop().unwrap();
            if debug {
                println!("setting return val: {:?}", val);
                println!("destination: {:?}", destination);
            }
            cmap.scoped_add(
                Some(cur_scope),
                MapKey::Place(*destination),
                Box::new(VarType::Values(val)),
            );
            //println!("cur_scope cmap: {:?}", cmap.cmap.get(&MapKey::ScopeId(cur_scope)));
        } else {
            if debug {
                println!("res_vec is empty");
            }
        }

        Ok(None)
    }

    fn interp_func_call(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        //prev_scope: Option<DefId>,
        cur_scope: DefId,
        func: &Operand<'tcx>,
        args: &Box<[Spanned<Operand<'tcx>>]>,
        destination: &Place<'tcx>,
        debug: bool,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        if debug {
            println!("\n-----------");
            println!("call!");
            println!("func: {:?}", func);
            println!("args: {:?}", args);
            println!("destination place: {:?}", destination);
        }

        match func {
            Operand::Constant(box co) => {
                self.interp_direct_func_call(cmap, cur_scope, co, args, destination, debug)
            }
            _ => todo!("handle indirect invocations"),
        }
    }

    fn resolve_arg(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        cur_scope: DefId,
        arg: Operand<'tcx>,
        debug: bool,
    ) -> Constraints<'tcx> {
        let mut constraints = HashSet::default();
        // get arg constraints from cmap
        match arg {
            Operand::Copy(place) | Operand::Move(place) => {
                match cmap.scoped_get(Some(cur_scope), &MapKey::Place(place), false) {
                    Some(vartype) => match vartype {
                        VarType::Values(constraints) => {
                            return constraints;
                        }
                        _ => {}
                    },
                    None => {
                        panic!("place doesn't exist in cmap, maybe this is a func name");
                    }
                }
            }
            Operand::Constant(box co) => {
                match co.const_ {
                    Const::Val(val, ty) => match val {
                        ConstValue::Scalar(scalar) => {
                            if debug {
                                println!("scalar: {:?}", scalar);
                            }
                            constraints.insert(VerifoptRval::Scalar(scalar));
                        }
                        ConstValue::ZeroSized => {
                            if debug {
                                println!("zero-sized");
                                println!("ty: {:?}", ty);
                                self.help_know_type(&ty);
                            }
                            constraints.insert(VerifoptRval::IdkType(ty));
                        }
                        ConstValue::Slice { .. } => {
                            todo!("slice");
                        }
                        ConstValue::Indirect { .. } => {
                            todo!("indirect");
                        }
                    },
                    Const::Ty(..) => {
                        todo!("ty");
                    }
                    Const::Unevaluated(uneval, ty) => {
                        // TODO do we care about evaluating constants?
                        if debug {
                            println!("unevaluated const");
                            println!("defid: {:?}", uneval.def);
                            println!("args: {:?}", uneval.args);
                            //println!("args typelist: {:?}", uneval.args.into_type_list(self.tcx));
                            println!("ty: {:?}", ty);
                        }
                        constraints.insert(VerifoptRval::IdkType(ty));
                    }
                }
            }
            _ => todo!("TODO runtime checks?"),
        }

        constraints
    }

    fn help_know_type(&self, ty: &Ty<'tcx>) {
        match ty.kind() {
            TyKind::Adt(_, _) => println!("adt"),
            TyKind::Param(param) => {
                println!("param");
                println!("index: {:?}", param.index);
                println!("name: {:?}", param.name);
            }
            _ => println!("other"),
        }
    }

    fn resolve_args(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        cur_scope: DefId,
        funcval: &FuncVal<'tcx>,
        args: &Box<[Spanned<Operand<'tcx>>]>,
        debug: bool,
    ) {
        let mut func_cmap = ConstraintMap::new();
        let arg_vec: Vec<Operand<'tcx>> = args.into_iter().map(|x| x.clone().node).collect();

        // add arg values into func_cmap
        for ((param_name, param_type), arg) in std::iter::zip(funcval.params.clone(), arg_vec) {
            if debug {
                println!("param_name: {:?}", param_name);
                println!("param_type: {:?}", param_type);
                //self.help_know_type(&param_type);
                println!("arg: {:?}", arg);
            }

            let constraints = self.resolve_arg(cmap, cur_scope, arg, debug);
            println!("resolved constraints: {:?}", constraints);
            func_cmap.cmap.insert(
                MapKey::Place(param_name),
                Box::new(VarType::Values(constraints.clone())),
            );

            match param_type.kind() {
                TyKind::Param(param) => {
                    // e.g. map T -> Cat
                    func_cmap.cmap.insert(
                        MapKey::Generic(param.name),
                        Box::new(VarType::Values(constraints)),
                    );
                }
                _ => {}
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
                    // in lots of places already... not to mention we are not currently using it
                    Box::new("functype (tbd)"),
                    func_cmap,
                )],
            )),
        );
    }
}
