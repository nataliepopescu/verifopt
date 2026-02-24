use rustc_data_structures::fx::FxHashSet as HashSet;
use rustc_data_structures::packed::Pu128;
use rustc_hir::def_id::DefId;
use rustc_index::IndexSlice;
use rustc_middle::mir::interpret::Scalar;
use rustc_middle::mir::*;
use rustc_middle::ty::{GenericArg, GenericArgKind, List, TyCtxt, TyKind};
use rustc_span::source_map::Spanned;

use crate::constraints::{ConstraintMap, Constraints, MapKey, VarType};
use crate::core::is_box;
use crate::core::{FuncVal, Merge, VerifoptConverter, VerifoptRval};
use crate::error::Error;
use crate::func_collect::FuncMap;
use crate::wto::BBDeps;

pub struct InterpPass<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub funcs: &'a FuncMap<'tcx>,
    pub converter: VerifoptConverter<'a, 'tcx>,
    pub debug: bool,
}

impl<'a, 'tcx> InterpPass<'a, 'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, funcs: &'a FuncMap<'tcx>, debug: bool) -> InterpPass<'a, 'tcx> {
        Self {
            tcx,
            funcs,
            converter: VerifoptConverter::new(tcx, funcs, debug),
            debug,
        }
    }

    pub fn run(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        prev_scope: Option<DefId>,
        cur_scope: DefId,
        body: &'tcx Body<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        // set up main / entry-point scope
        // FIXME perhaps add some of this back if entry-point is not `main`
        //let locals = body.local_decls.as_slice();
        //let ty = locals[Local::from_usize(0)].ty;
        //let mut set = HashSet::default();
        //set.insert(VerifoptRval::IdkType(ty));
        let main_cmap = ConstraintMap::new();

        //main_cmap.cmap.insert(
        //    MapKey::Place(Place {
        //        local: Local::from_usize(0),
        //        projection: List::empty(),
        //    }),
        //    Box::new(VarType::Values(set)),
        //);

        cmap.cmap.insert(
            MapKey::ScopeId(cur_scope),
            Box::new(VarType::SubScope(
                prev_scope,
                vec![(Box::new("main functype"), main_cmap)],
            )),
        );

        // TODO perhaps also setup a general `global` scope (which would replace the
        // current semantics of the `None` scope) once we change the meaning of backptr)

        // track call stack for debugging
        let mut call_stack = vec![cur_scope];

        // start interpretation
        self.visit_body(cmap, &mut call_stack, prev_scope, cur_scope, body)
    }

    fn visit_body(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        call_stack: &mut Vec<DefId>,
        prev_scope: Option<DefId>,
        cur_scope: DefId,
        body: &'tcx Body<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        // TODO how do loops affect this order?

        if self.debug {
            println!("\n###### INTERP-ING NEW BODY for func {:?}\n", cur_scope);
            println!("cur_scope: {:?}", cur_scope);
            println!("prev_scope: {:?}", prev_scope);
            println!("call_stack: {:?}", call_stack);
        }

        // if there exists a memoized WTO, use it; otherwise, create and save it
        let mut bb_deps;
        if let Some(mem_bb_deps) = cmap.wtos.get(&cur_scope) {
            bb_deps = mem_bb_deps.clone();
        } else {
            bb_deps = BBDeps::new(body, self.debug);
            cmap.wtos.insert(cur_scope, bb_deps.clone());
        }

        let mut last_res = None;
        loop {
            if bb_deps.ordering.is_empty() {
                break;
            }
            let bb = bb_deps.ordering.remove(0);
            if self.debug {
                println!("\n--NEW BB: {:?}", bb);
            }
            let data = body.basic_blocks.get(bb).unwrap();
            last_res = self.visit_basic_block_data(
                cmap,
                call_stack,
                &mut bb_deps,
                cur_scope,
                body.local_decls.as_slice(),
                &bb,
                data,
            )?;
        }

        Ok(last_res)
    }

    fn visit_basic_block_data(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        call_stack: &mut Vec<DefId>,
        bb_deps: &mut BBDeps,
        cur_scope: DefId,
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        bb: &BasicBlock,
        data: &BasicBlockData<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        let mut last_res = None;

        if self.debug {
            println!("#############################");
            println!("# visiting BASICBLOCK for {:?}", cur_scope);
            println!("#############################");
        }

        for (_, stmt) in data.statements.iter().enumerate() {
            if self.debug {
                println!("\n# -------------------------------");
                println!("# visiting STATEMENT in {:?} of defid {:?}", bb, cur_scope);
                println!("# -------------------------------");
            }
            last_res = self.visit_statement(cmap, cur_scope, body_locals, stmt)?;
        }

        if let Some(term) = &data.terminator {
            if self.debug {
                println!("\n# -------------------------------");
                println!("# visiting TERM in {:?} of defid {:?}", bb, cur_scope);
                println!("# -------------------------------");
            }
            // FIXME return basic blocks are the only ones whose interp returns Some(_), so could
            // re-architect this loop to only save the result if it is a Some(_) (then wouldn't
            // have to worry about executing cleanup blocks afterwards)
            last_res = self.visit_terminator(
                cmap,
                call_stack,
                body_locals,
                bb,
                bb_deps,
                cur_scope,
                &term,
            )?;
        }

        bb_deps.mark_visited(bb, cur_scope);

        Ok(last_res)
    }

    fn visit_statement(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        cur_scope: DefId,
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        statement: &Statement<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        match statement.kind {
            StatementKind::Assign(box (place, ref rvalue)) => {
                if self.debug {
                    println!("start assignment!");
                    println!("cur_scope: {:?}", cur_scope);
                    println!("place: {:?}", place);
                    println!("rval to resolve: {:?}", rvalue);
                }

                let rval_constraints =
                    self.converter
                        .from_rvalue(cmap, cur_scope, body_locals, rvalue);

                cmap.scoped_add(
                    Some(cur_scope),
                    MapKey::Place(place),
                    Box::new(VarType::Values(rval_constraints.clone())),
                    //true,
                );

                if self.debug {
                    println!("\nend assignment!");
                    println!("cur_scope: {:?}", cur_scope);
                    println!("place: {:?}", place);
                    println!("rval: {:?}", rval_constraints);
                    println!("cmap @ cur_scope @ place: {:?}", cmap.scoped_get(Some(cur_scope), &MapKey::Place(place), false));
                    //println!(
                    //    "cur_scope cmap: {:?}",
                    //    cmap.cmap.get(&MapKey::ScopeId(cur_scope))
                    //);
                }
            }
            _ => {}
        }

        Ok(None)
    }

    fn visit_terminator(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        call_stack: &mut Vec<DefId>,
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        bb: &BasicBlock,
        bb_deps: &mut BBDeps,
        cur_scope: DefId,
        terminator: &Terminator<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        match &terminator.kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => self.interp_func_call(
                cmap,
                call_stack,
                cur_scope,
                body_locals,
                func,
                args,
                destination,
            ),
            TerminatorKind::Return => self.interp_return(cmap, call_stack, cur_scope),
            TerminatorKind::SwitchInt { discr, targets } => {
                self.interp_switchint(cmap, bb, bb_deps, cur_scope, discr, targets)
            }
            TerminatorKind::TailCall { .. } => todo!("impl tail calls"),
            _ => Ok(None),
        }
    }

    fn interp_return(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        // return constraints at place _0
        if self.debug {
            println!("RETURNING...");
        }

        call_stack.pop();

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
                    if self.debug {
                        println!("returning value w following constraints: {:?}", constraints);
                    }

                    let mut ret_constraints = HashSet::default();
                    for constraint in constraints.clone().drain() {
                        if let VerifoptRval::IdkType(ty) = constraint {
                            if self.debug {
                                println!("returning a type!: {:?}", ty);
                            }
                            if let TyKind::Param(param) = ty.kind() {
                                // resolve generic
                                if self.debug {
                                    println!("GENERIC RETTY: {:?}", param);
                                    println!(
                                        "resolve generic param: {:?}",
                                        cmap.scoped_get(
                                            Some(cur_scope),
                                            &MapKey::Generic(param.name),
                                            false
                                        )
                                    );
                                }
                                match cmap.scoped_get(
                                    Some(cur_scope),
                                    &MapKey::Generic(param.name),
                                    false,
                                ) {
                                    Some(VarType::Values(constraints)) => {
                                        if self.debug {
                                            println!("constraints: {:?}", constraints);
                                        }
                                        ret_constraints = ret_constraints
                                            .union(&constraints)
                                            .map(|x| x.clone())
                                            .collect();
                                    }
                                    _ => panic!("unexpected"),
                                }
                            } else {
                                // not generic add return constraints
                                if self.debug {
                                    println!("NONGENERIC RETTY: {:?}", ty);
                                }
                                ret_constraints.insert(constraint);
                            }
                        } else {
                            // not generic, add return constraints
                            if self.debug {
                                println!("NONGENERIC RETVAL: {:?}", constraint);
                            }
                            ret_constraints.insert(constraint);
                        }
                    }

                    if ret_constraints.len() == 0 {
                        panic!("something went wrong, should return at least one constraint");
                    }

                    if self.debug {
                        println!("\n# RESOLVED RETURN CONSTRAINTS: {:?}", ret_constraints);
                    }

                    Ok(Some(ret_constraints))
                }
                _ => panic!("return scope?"),
            },
            None => {
                if self.debug {
                    println!("_0 local was not assigned to");
                    println!("cur_scope: {:?}", cur_scope);
                }

                // double check that nothing was supposed to be returned
                if let Some(funcval_vec) = self.funcs.funcs.get(&cur_scope) {
                    if funcval_vec.len() != 1 {
                        panic!("unexpected len");
                    }
                    if let Some(retty) = funcval_vec[0].rettype
                        && !retty.is_unit()
                    {
                        panic!("should have returned something of type: {:?}", retty);
                    }
                }

                // assuming our interpreter is correct, if the local _0 was not assigned to then
                // this must mean that there is nothing to return. so, return nothing
                Ok(None)
            }
        }
    }

    fn resolve_switchint_constraints(
        &self,
        constraint: &VerifoptRval<'tcx>,
        vals: &[Pu128],
        discr_vals_slice: &mut [i32],
    ) {
        match constraint {
            VerifoptRval::Scalar(scalar) => {
                let num = scalar.to_bits_unchecked();
                for (i, val) in vals.iter().enumerate() {
                    if num == val.get() {
                        discr_vals_slice[i] += 1;
                    }
                }
            }
            VerifoptRval::IdkType(_) | VerifoptRval::IdkDefId(_) | VerifoptRval::Idk() => {}
            VerifoptRval::Ref(reffed) => {
                self.resolve_switchint_constraints(&*reffed, vals, discr_vals_slice)
            }
            _ => panic!("unexpected switchint discr: {:?}", constraint),
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
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        if self.debug {
            println!("in SWITCHINT");
        }

        match discr {
            Operand::Copy(place) | Operand::Move(place) => {
                if self.debug {
                    println!("place.local: {:?}", place.local);
                    println!("place.projection: {:?}", place.projection);
                    println!(
                        "cmap @ scope: {:?}",
                        cmap.cmap.get(&MapKey::ScopeId(cur_scope))
                    );
                }

                let mut newplace = *place;
                if place.projection.len() > 0 {
                    // ignore projections for now, kind of wrong but let's see how much it impacts things
                    newplace = Place {
                        local: Local::from_u32(place.local.as_u32()),
                        projection: List::empty(),
                    };
                    if self.debug {
                        println!("updating newplace: {:?}", newplace);
                    }
                }

                match cmap.scoped_get(Some(cur_scope), &MapKey::Place(newplace), false) {
                    Some(vartype) => match vartype {
                        VarType::Values(constraints) => {
                            let vals = targets.all_values();
                            if self.debug {
                                println!("values: {:?}", vals);
                            }
                            let len = vals.len();
                            if len > 2 {
                                todo!("more than 2 possible switchint vals ({:?})", len);
                            }
                            let mut discr_vals: [i32; 3] = [0; 3];
                            let discr_vals_slice: &mut [i32] = &mut discr_vals[0..len + 1];

                            for constraint in constraints.iter() {
                                self.resolve_switchint_constraints(
                                    constraint,
                                    vals,
                                    discr_vals_slice,
                                );
                            }

                            // FIXME improve
                            if discr_vals[0] != 0 && discr_vals[1] == 0 && discr_vals[2] == 0 {
                                // only explore first target
                                bb_deps.prune(bb, targets.all_targets()[1]);
                                if len > 1 {
                                    bb_deps.prune(bb, targets.all_targets()[2]);
                                }
                            } else if discr_vals[0] == 0 && discr_vals[1] != 0 && discr_vals[2] == 0
                            {
                                // only explore second target
                                bb_deps.prune(bb, targets.all_targets()[0]);
                                if len > 1 {
                                    bb_deps.prune(bb, targets.all_targets()[2]);
                                }
                            } else if discr_vals[0] == 0 && discr_vals[1] == 0 && discr_vals[2] != 0
                            {
                                // only explore third target (if it exists)
                                bb_deps.prune(bb, targets.all_targets()[0]);
                                bb_deps.prune(bb, targets.all_targets()[1]);
                            }
                        }
                        _ => todo!("scope not impl"),
                    },
                    None => panic!("place does not exist: {:?}", place),
                }
            }
            // currently not pruning anything
            Operand::Constant(box co) => {
                if self.debug {
                    println!("const: {:?}", co);
                }
                match co.const_ {
                    Const::Val(val, _) => {
                        if self.debug {
                            println!("const val @ switchint: {:?}", val);
                        }
                        match val {
                            ConstValue::Scalar(Scalar::Int(_s_int)) => {
                                todo!("constant operand");
                            }
                            _ => {}
                        }
                    }
                    Const::Ty(_, _) => {}
                    Const::Unevaluated(_, _) => {}
                }
            }
            _ => {
                if self.debug {
                    println!("runtime checks (ignoring)")
                }
            }
        }

        Ok(None)
    }

    /*
     * Get the verifopt constraints for the `self` type of this dynamic dispatch call
     */
    fn resolve_dyn_self_constraint(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        cur_scope: DefId,
        impltors: &Vec<DefId>,
        constraint: VerifoptRval<'tcx>,
        args: &Box<[Spanned<Operand<'tcx>>]>,
    ) -> Option<Vec<VerifoptRval<'tcx>>> {
        match constraint {
            VerifoptRval::Ref(inner) => {
                self.resolve_dyn_self_constraint(cmap, cur_scope, impltors, *inner, args)
            }
            ref idkstruct @ VerifoptRval::IdkStruct(struct_defid, ref genarg_vec) => {
                if is_box(struct_defid) {
                    // get first (and only, for now) genarg constraint (i.e. IdkType(Cat))
                    if let Some(genargs_outer) = genarg_vec {
                        if genargs_outer.len() != 1 {
                            panic!("handle diff genarg len");
                        }
                        let genarg_constraint_vec = &genargs_outer[0];

                        // FIXME does this need to be in a separate function?
                        // otherwise could save a clone()...
                        return Some(genarg_constraint_vec.to_vec());
                    } else {
                        return None;
                    }
                } else {
                    if self.debug {
                        println!("struct is not a box: {:?}", struct_defid);
                        println!("genargs: {:?}", genarg_vec);
                    }
                    return Some(vec![idkstruct.clone()]);
                }
            }
            idkdid @ VerifoptRval::IdkDefId(_) => Some(vec![idkdid]),
            idkty @ VerifoptRval::IdkType(_) => Some(vec![idkty]),
            _ => todo!("handle other types: {:?}", constraint),
        }
    }

    /*
     * Get all the relevant trait function implementations (given self_defid)
     * for this particular dynamic invocation
     */
    fn get_trait_fn_impls_from_defid(
        &self,
        self_defid: &DefId,
        impltors: &Vec<DefId>,
    ) -> Vec<DefId> {
        let mut to_dispatch = vec![];
        // what are all of this struct's impl blocks?
        if let Some(impl_blocks) = self.funcs.struct_impls.get(&self_defid) {
            if self.debug {
                println!("self_defid: {:?}", self_defid);
                println!("impl_blocks: {:?}", impl_blocks);
            }

            // for each such impl block, get the exact list of (associated/trait)
            // functions that it is implementing
            for impl_block in impl_blocks {
                match self.funcs.impl_blocks_to_impls.get(&impl_block) {
                    Some(assoc) => {
                        if self.debug {
                            println!("assoc: {:?}", assoc);
                        }
                        // if this list of assoc/trait functions
                        // contains any of the concrete implementations of the
                        // top-level assoc/trait functions (the thing to
                        // dispatch dynamically), add the matching concrete
                        // implementation to the list of things we want to
                        // continue interpreting/analyzing
                        for impltor in impltors {
                            if assoc.contains(impltor) {
                                to_dispatch.push(*impltor);
                            }
                        }
                    }
                    None => {}
                }
            }
            return to_dispatch;
        } else {
            panic!("cannot get struct impl_blocks: {:?}", self_defid);
        }
    }

    /*
     * Get all the relevant trait function implementations (given self_constraint)
     * for this particular dynamic invocation
     */
    fn get_trait_fn_impls(
        &self,
        self_constraint: &VerifoptRval<'tcx>,
        impltors: &Vec<DefId>,
    ) -> Vec<DefId> {
        match *self_constraint {
            VerifoptRval::IdkStruct(did, _) => self.get_trait_fn_impls_from_defid(&did, impltors),
            VerifoptRval::IdkDefId(did) => self.get_trait_fn_impls_from_defid(&did, impltors),
            // FIXME cannot get fn_impls from types, so we ignore dispatch and
            // (later) use the function return type as the retval's "constraint"
            VerifoptRval::IdkType(_) => {
                if self.debug {
                    println!(
                        "cannot get fn_impl from type, return empty vec to ignore dispatch and use backup function rettype for retval"
                    );
                }
                vec![]
            }
            _ => todo!("handle other types: {:?}", self_constraint),
        }
    }

    fn dyn_to_static_dispatch(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        self_constraint_vec: Vec<VerifoptRval<'tcx>>,
        impltors: &Vec<DefId>,
        func_genargs: &[GenericArg<'tcx>],
        args: &Box<[Spanned<Operand<'tcx>>]>,
        destination: &Place<'tcx>,
    ) -> Vec<Result<Option<Constraints<'tcx>>, Error>> {
        // assoc dyn obj w the correct trait fn impl(s)
        let mut to_dispatch = Vec::new();
        for self_constraint in self_constraint_vec.iter() {
            to_dispatch.append(&mut self.get_trait_fn_impls(self_constraint, impltors));
        }

        if self.debug {
            println!("to_dispatch: {:?}", to_dispatch);
        }

        // call each impl
        let mut res_vec = Vec::new();
        for func_defid in to_dispatch {
            if self.debug {
                println!("LOOPING static dispatches");
                println!("func_defid: {:?}", func_defid);
            }

            let funcvals = self.funcs.funcs.get(&func_defid);
            if funcvals.is_none() {
                panic!("func not found: {:?}", func_defid);
            }
            let funcval_vec = funcvals.unwrap();
            if self.debug {
                println!("funcval_vec: {:?}", funcval_vec);
            }
            if funcval_vec.len() != 1 {
                panic!("unexpected number of functions");
            }

            res_vec.push(self.handle_static_dispatch(
                cmap,
                call_stack,
                cur_scope,
                body_locals,
                &funcval_vec[0],
                func_genargs,
                args,
                destination,
            ));
        }

        res_vec
    }

    fn handle_dyn_dispatch(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        assoc_funcval: &FuncVal<'tcx>,
        func_genargs: &[GenericArg<'tcx>],
        trait_def_id: &DefId,
        args: &Box<[Spanned<Operand<'tcx>>]>,
        destination: &Place<'tcx>,
    ) -> Vec<Result<Option<Constraints<'tcx>>, Error>> {
        if self.debug {
            println!("found in trait: {:?}", trait_def_id);
        }

        if args.len() == 0 {
            if self.debug {
                println!("argments list empty: cannot determine self type");
            }
            return vec![Ok(None)];
        }

        if assoc_funcval.self_arg.is_none() {
            if self.debug {
                println!("no self type: {:?}", assoc_funcval);
            }
            return vec![Ok(None)];
        }

        // `impltors` are the concrete implementations of this assoc_fn
        if let Some(impltors) = self
            .funcs
            .trait_fn_impltors
            .lock()
            .unwrap()
            .get(&assoc_funcval.def_id)
        {
            // get the first (`self`) arg place
            match args[0].node {
                Operand::Copy(place) | Operand::Move(place) => {
                    if self.debug {
                        println!("first arg: {:?}", args[0]);
                        println!("place: {:?}", place);
                        println!(
                            "value: {:?}",
                            cmap.scoped_get(Some(cur_scope), &MapKey::Place(place), false)
                        );
                        println!("impltors: {:#?}", impltors);
                        println!("structs: {:?}", self.funcs.trait_impltors.get(trait_def_id));
                    }

                    // get the constraints for the first (`self`) arg from the current scope
                    if let Some(VarType::Values(constraints)) =
                        cmap.scoped_get(Some(cur_scope), &MapKey::Place(place), false)
                    {
                        // for each possible constraint...
                        let mut res_vec = vec![];
                        if self.debug {
                            println!("\n# IN HANDLE_DYN_DISPATCH");
                            println!("all constraints: {:?}", constraints);
                        }

                        for constraint in constraints.clone().drain() {
                            if self.debug {
                                println!("LOOPING in HDD");
                                println!("constraint: {:?}", constraint);
                            }

                            // unwrap any refs or boxed types
                            if let Some(self_constraint_vec) = self.resolve_dyn_self_constraint(
                                cmap, cur_scope, impltors, constraint, args,
                            ) {
                                if self.debug {
                                    println!("\n## BEFORE DYN_TO_STATIC");
                                    println!("self_constraint_vec: {:?}", self_constraint_vec);
                                }
                                // interp dyn dispatch as one or more static dispatches
                                res_vec.append(&mut self.dyn_to_static_dispatch(
                                    cmap,
                                    call_stack,
                                    cur_scope,
                                    body_locals,
                                    self_constraint_vec,
                                    impltors,
                                    func_genargs,
                                    args,
                                    destination,
                                ));
                            } else {
                                todo!("no self constraint");
                            }
                        }
                        return res_vec;
                    } else {
                        panic!("first argument place does not exist: {:?}", place);
                    }
                }
                _ => {}
            }
        } else {
            panic!("missing implementors");
        }

        vec![Ok(None)]
    }

    fn handle_static_dispatch(
        &self,
        mut cmap: &mut ConstraintMap<'tcx>,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        funcval: &FuncVal<'tcx>,
        func_genargs: &[GenericArg<'tcx>],
        args: &Box<[Spanned<Operand<'tcx>>]>,
        destination: &Place<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        if self.debug {
            println!(
                "\n### SETTING UP FUNC CALL (RESOLVING ARGS) for func {:?}\n",
                funcval.def_id
            );
            println!("args: {:?}\n", args);
        }

        self.resolve_args(&mut cmap, cur_scope, funcval, args);

        if funcval.param_generics.is_some() {
            if self.debug {
                println!("\n## RESOLVING GENERICS IN ARGTYS...\n");
            }
            self.resolve_generic_argtys(&mut cmap, cur_scope, body_locals, funcval, func_genargs);
        }

        if funcval.ret_generic.is_some() {
            if self.debug {
                println!("\n## RESOLVING GENERICS IN RETTY...\n");
            }
            self.resolve_generic_retty(
                &mut cmap,
                cur_scope,
                body_locals,
                funcval,
                func_genargs,
                destination,
            );
        }

        // visit callee
        let callee_body = self.tcx.optimized_mir(funcval.def_id);
        call_stack.push(funcval.def_id);
        self.visit_body(
            cmap,
            call_stack,
            Some(cur_scope),
            funcval.def_id,
            callee_body,
        )
    }

    fn interp_direct_func_call(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        co: &ConstOperand<'tcx>,
        args: &Box<[Spanned<Operand<'tcx>>]>,
        destination: &Place<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        let mut res_vec = vec![];
        let mut cmap_vec = vec![];

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

                                let mut constraints = HashSet::default();
                                if let Some(ret_did) = funcval.ret_did {
                                    constraints.insert(VerifoptRval::IdkDefId(ret_did));
                                } else if let Some(rettype) = funcval.rettype {
                                    constraints.insert(VerifoptRval::IdkType(rettype));
                                }
                                res_vec.push(constraints);
                            } else if !self.tcx.is_mir_available(def_id) {
                                if self.debug {
                                    println!("MIR NOT AVAILABLE for {:?}", def_id);
                                }

                                let mutex = self.funcs.assocfns_to_traits.lock().unwrap();
                                match mutex.get(def_id) {
                                    Some(trait_def_id) => {
                                        if self.debug {
                                            println!("\n### DYN DISPATCH TO {:?}\n", def_id);
                                        }
                                        let trait_def_id_clone = trait_def_id.clone();
                                        std::mem::drop(mutex);

                                        let dyn_results = self.handle_dyn_dispatch(
                                            cmap,
                                            call_stack,
                                            cur_scope,
                                            body_locals,
                                            funcval,
                                            func_genargs.as_slice(),
                                            &trait_def_id_clone,
                                            args,
                                            destination,
                                        );

                                        if self.debug {
                                            println!("dyn_results: {:?}", dyn_results);
                                        }

                                        for dyn_res in dyn_results.iter() {
                                            match dyn_res {
                                                Ok(Some(constraints)) => {
                                                    if self.debug {
                                                        println!(
                                                            "\n##### DONE w DYN func {:?}\n",
                                                            def_id
                                                        );
                                                        println!("constraints: {:?}", constraints);
                                                    }

                                                    if !res_vec.contains(&constraints) {
                                                        res_vec.push(constraints.clone());
                                                    } else {
                                                        if self.debug {
                                                            println!("dup constraint: {:?}", constraints);
                                                        }
                                                    }
                                                }
                                                Ok(None) => {
                                                    if self.debug {
                                                        println!(
                                                            "\n##### DONE w DYN func {:?}\n",
                                                            def_id
                                                        );
                                                        println!("no retval, check for widening");
                                                    }
                                                    // if funcval has a return type, use that as the
                                                    // "summary" constraint
                                                    let mut constraints = HashSet::default();
                                                    if let Some(ret_did) = funcval.ret_did {
                                                        constraints.insert(VerifoptRval::IdkDefId(
                                                            ret_did,
                                                        ));
                                                        if !res_vec.contains(&constraints) {
                                                            res_vec.push(constraints);
                                                        } else {
                                                            if self.debug {
                                                                println!("dup constraint: {:?}", constraints);
                                                            }
                                                        }
                                                    } else if let Some(rettype) = funcval.rettype {
                                                        if self.debug {
                                                            println!("rettype: {:?}", rettype);
                                                        }

                                                        // FIXME resolve generics
                                                        constraints
                                                            .insert(VerifoptRval::IdkType(rettype));
                                                        if !res_vec.contains(&constraints) {
                                                            res_vec.push(constraints);
                                                        } else {
                                                            if self.debug {
                                                                println!("dup constraint: {:?}", constraints);
                                                            }
                                                        }
                                                    }
                                                }
                                                e @ Err(_) => return e.clone(),
                                            }
                                        }
                                    }
                                    None => {
                                        if self.debug {
                                            println!("\n### UNDEF FN / RES {:?}\n", def_id);
                                        }
                                        std::mem::drop(mutex);

                                        // if funcval has a return type, use that as the
                                        // "summary" constraint
                                        let mut constraints = HashSet::default();
                                        if let Some(ret_did) = funcval.ret_did {
                                            constraints.insert(VerifoptRval::IdkDefId(ret_did));
                                        } else if let Some(rettype) = funcval.rettype {
                                            constraints.insert(VerifoptRval::IdkType(rettype));
                                        } else {
                                            constraints.insert(VerifoptRval::Undef());
                                        }
                                        res_vec.push(constraints);
                                    }
                                }
                            } else {
                                if self.debug {
                                    println!("\n### STATIC DISPATCH TO: {:?}", def_id);
                                }

                                let mut cmap_clone = cmap.clone();
                                match self.handle_static_dispatch(
                                    &mut cmap_clone,
                                    call_stack,
                                    cur_scope,
                                    body_locals,
                                    funcval,
                                    func_genargs.as_slice(),
                                    args,
                                    destination,
                                ) {
                                    Ok(maybe_constraints) => {
                                        if self.debug {
                                            println!("\n##### DONE w func {:?}\n", def_id);
                                            println!(
                                                "maybe_constraints from func: {:?}",
                                                maybe_constraints
                                            );
                                        }

                                        cmap_vec.push(cmap_clone);
                                        if let Some(constraints) = maybe_constraints {
                                            if !res_vec.contains(&constraints) {
                                                if self.debug {
                                                    println!("res_vec {:?}", res_vec);
                                                    println!("adding constraint: {:?}", constraints);
                                                }
                                                res_vec.push(constraints);
                                            } else {
                                                if self.debug {
                                                    println!("dup constraint: {:?}", constraints);
                                                }
                                            }
                                        } else if let Some(rettype) = funcval.rettype {
                                            // is `None` actually a None, or was the
                                            // analysis just unable to interpret something?
                                            // i.e. if the called function is expected to
                                            // return something, this is the latter case, and
                                            // we can conservatively approx the constraints
                                            // to be the function return type
                                            if self.debug {
                                                println!(
                                                    "analysis failed, but the return type for func {:?} is: {:?}",
                                                    funcval.def_id, rettype
                                                );
                                            }
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

        if cmap_vec.len() > 1 {
            if self.debug {
                println!("cmap_vec len: {:?}", cmap_vec.len());
            }
            todo!("impl cmap_vec merge");
        }

        if cmap_vec.len() > 0 {
            if self.debug {
                println!("setting new cmap");
            }
            *cmap = cmap_vec.pop().unwrap();
        } else {
            if self.debug {
                println!("cmap_vec is empty");
            }
        }

        if self.debug {
            println!("res_vec len: {:?}", res_vec.len());
            println!("res_vec: {:?}", res_vec);
        }

        match res_vec.merge() {
            Ok(Some(merged)) => {
                cmap.scoped_add(
                    Some(cur_scope),
                    MapKey::Place(*destination),
                    Box::new(VarType::Values(merged)),
                );

                if self.debug {
                    println!("setting result in cmap");
                    println!(
                        "cmap @ dest ({:?}): {:?}",
                        destination,
                        cmap.scoped_get(Some(cur_scope), &MapKey::Place(*destination), false)
                    );
                }
            }
            Ok(None) => {
                if self.debug {
                    println!("res_vec empty");
                }
            }
            _ => panic!("unexpected result"),
        }

        Ok(None)
    }

    fn interp_func_call(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        call_stack: &mut Vec<DefId>,
        cur_scope: DefId,
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        func: &Operand<'tcx>,
        args: &Box<[Spanned<Operand<'tcx>>]>,
        destination: &Place<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        if self.debug {
            println!("\n-----------");
            println!("call!");
            println!("cur scope: {:?}", cur_scope);
            println!("func: {:?}", func);
            println!("args: {:?}", args);
            println!("destination place: {:?}", destination);
            println!(
                "cmap @ scope: {:?}",
                cmap.cmap.get(&MapKey::ScopeId(cur_scope))
            );
        }

        match func {
            Operand::Constant(box co) => self.interp_direct_func_call(
                cmap,
                call_stack,
                cur_scope,
                body_locals,
                co,
                args,
                destination,
            ),
            _ => todo!("handle indirect invocations"),
        }
    }

    fn resolve_arg(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        cur_scope: DefId,
        arg: Operand<'tcx>,
    ) -> Constraints<'tcx> {
        let mut constraints = HashSet::default();
        // get arg constraints from cmap
        match arg {
            Operand::Copy(place) | Operand::Move(place) => {
                match cmap.scoped_get(Some(cur_scope), &MapKey::Place(place), false) {
                    Some(vartype) => match vartype {
                        VarType::Values(constraints) => {
                            if self.debug {
                                println!("cur_scope: {:?}", cur_scope);
                                println!("place: {:?}", place);
                                println!("constraints: {:?}", constraints);
                            }
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
                        ConstValue::Scalar(Scalar::Int(scalar)) => {
                            if self.debug {
                                println!("scalar: {:?}", scalar);
                            }
                            constraints.insert(VerifoptRval::Scalar(scalar));
                        }
                        ConstValue::Scalar(Scalar::Ptr(ptr, _)) => {
                            todo!("ptr: {:?}", ptr);
                        }
                        ConstValue::ZeroSized => {
                            if self.debug {
                                println!("zero-sized");
                                println!("ty: {:?}", ty);
                            }
                            match ty.kind() {
                                TyKind::Adt(def, _) => {
                                    constraints.insert(VerifoptRval::IdkStruct(def.did(), None))
                                }
                                _ => constraints.insert(VerifoptRval::IdkType(ty)),
                            };
                        }
                        ConstValue::Slice { .. } => {
                            constraints.insert(VerifoptRval::IdkStr());
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
                        if self.debug {
                            println!("unevaluated const");
                            println!("defid: {:?}", uneval.def);
                            println!("args: {:?}", uneval.args);
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

    fn resolve_args(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        cur_scope: DefId,
        funcval: &FuncVal<'tcx>,
        args: &Box<[Spanned<Operand<'tcx>>]>,
    ) {
        let mut func_cmap = ConstraintMap::new();
        let arg_vec: Vec<Operand<'tcx>> = args.into_iter().map(|x| x.clone().node).collect();

        // add arg values into func_cmap
        for ((param_name, param_type), arg) in std::iter::zip(funcval.params.clone(), arg_vec) {
            if self.debug {
                println!("param_name: {:?}", param_name);
                println!("param_type: {:?}", param_type);
                println!("arg: {:?}", arg);
            }

            let constraints = self.resolve_arg(cmap, cur_scope, arg);
            if self.debug {
                println!("resolved constraints: {:?}", constraints);
            }
            func_cmap.cmap.insert(
                MapKey::Place(param_name),
                Box::new(VarType::Values(constraints.clone())),
            );

            match param_type.kind() {
                TyKind::Param(param) => {
                    if self.debug {
                        println!("SETTING GENERIC");
                        println!("param: {:?}", param);
                        println!("constraints: {:?}", constraints);
                    }
                    // e.g. map T -> Cat
                    func_cmap.cmap.insert(
                        MapKey::Generic(param.name),
                        Box::new(VarType::Values(constraints)),
                    );
                }
                _ => {}
            }
        }

        let key = MapKey::ScopeId(funcval.def_id);
        let mut new_cmap = func_cmap.clone();
        match cmap.cmap.get(&key) {
            Some(boxed_vartype) => match *boxed_vartype.clone() {
                VarType::SubScope(_, cmap_vec) => {
                    let old_cmap = &cmap_vec[0].1;
                    let new_cmaps = vec![old_cmap.clone(), func_cmap.clone()];
                    if self.debug {
                        println!("funcname already exists in cmap: {:?}", funcval.def_id);
                        println!("old_cmap: {:?}", old_cmap);
                        println!("new_cmap: {:?}", func_cmap);
                    }
                    if let Ok(Some(merged_cmap)) = new_cmaps.merge() {
                        new_cmap = merged_cmap;
                    }
                }
                _ => panic!("got Vartype::Values"),
            },
            _ => {}
        }

        // add func_cmap to outer cmap
        cmap.cmap.insert(
            key,
            Box::new(VarType::SubScope(
                Some(cur_scope),
                vec![(
                    // if this item is a function (which i guess since we're using
                    // VarType::SubScope it must be), this first part of the tuple seems to store
                    // the function type, so that if this function is ever (indirectly?) invoked,
                    // the return _type_ of the function will correspond to the type of the
                    // invocation result, but I am not sure how useful this is since we have types
                    // in lots of places already... not to mention we are not currently using it
                    // TODO however, maybe, analogously to our cmap, this functype may
                    // communicate more fine-grained information about the function... TBD
                    Box::new("functype (tbd)"),
                    new_cmap,
                )],
            )),
        );
    }

    fn resolve_generic_argtys(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        cur_scope: DefId,
        _body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        funcval: &FuncVal<'tcx>,
        func_genargs: &[GenericArg<'tcx>],
    ) {
        if self.debug {
            println!("funcval.params: {:?}", funcval.params);
            println!("funcval.param_generics: {:?}", funcval.param_generics);
            println!("func genargs: {:?}", func_genargs);
        }

        // filter lifetimes out of genargs
        let genargs = func_genargs.iter().filter(|genarg| match genarg.kind() {
            GenericArgKind::Lifetime(_) => false,
            _ => true,
        });

        if self.debug {
            println!("filtered genargs: {:?}", genargs);
        }

        // add argtype generic param to callee scope
        for (param_generic, gen_arg) in
            std::iter::zip(funcval.param_generics.clone().unwrap(), genargs)
        {
            if self.debug {
                println!(
                    "IN LOOP; arg resolution: ({:?}, {:?})",
                    param_generic, gen_arg
                );
            }

            if cmap
                .scoped_get(
                    Some(funcval.def_id),
                    &MapKey::Generic(param_generic.name),
                    false,
                )
                .is_none()
            {
                if self.debug {
                    println!("\tproceeding with adding to cmap");
                }
                let mut constraints = HashSet::default();
                let vartype = match gen_arg.kind() {
                    GenericArgKind::Type(ty) => match ty.kind() {
                        TyKind::Param(param) => {
                            if self.debug {
                                println!("ty param!! use outer scope ty val: {:?}", param);
                            }
                            match cmap.scoped_get(
                                Some(cur_scope),
                                &MapKey::Generic(param_generic.name),
                                false,
                            ) {
                                // propagate outer_scope value
                                Some(val) => val,
                                None => {
                                    // if no outer_scope value, try to resolve gen_arg itself
                                    match cmap.scoped_get(
                                        Some(cur_scope),
                                        &MapKey::Generic(param.name),
                                        false,
                                    ) {
                                        Some(val) => val,
                                        None => panic!(
                                            "cannot resolve {:?} in defid {:?}",
                                            param_generic.name, cur_scope
                                        ),
                                    }
                                }
                            }
                        }
                        _ => {
                            constraints.insert(VerifoptRval::IdkType(gen_arg.as_type().unwrap()));
                            VarType::Values(constraints)
                        }
                    },
                    _ => {
                        constraints.insert(VerifoptRval::IdkType(gen_arg.as_type().unwrap()));
                        VarType::Values(constraints)
                    }
                };

                if self.debug {
                    println!(
                        "outer-scope val @ {:?}: {:?}",
                        param_generic.name,
                        cmap.scoped_get(
                            Some(cur_scope),
                            &MapKey::Generic(param_generic.name),
                            false
                        )
                    );
                    println!(
                        "old val @ {:?}: {:?}",
                        param_generic.name,
                        cmap.scoped_get(
                            Some(funcval.def_id),
                            &MapKey::Generic(param_generic.name),
                            false
                        )
                    );
                }

                cmap.scoped_add(
                    Some(funcval.def_id),
                    MapKey::Generic(param_generic.name),
                    Box::new(vartype),
                );
            } else {
                if self.debug {
                    println!("\tmapping already exists; skipping");
                }
            }

            if self.debug {
                println!(
                    "val @ {:?}: {:?}",
                    param_generic.name,
                    cmap.scoped_get(
                        Some(funcval.def_id),
                        &MapKey::Generic(param_generic.name),
                        false
                    )
                );
            }
        }
    }

    fn resolve_generic_retty(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        _cur_scope: DefId,
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        funcval: &FuncVal<'tcx>,
        func_genargs: &[GenericArg<'tcx>],
        destination: &Place<'tcx>,
    ) {
        if self.debug {
            println!("funcval.ret_generic: {:?}", funcval.ret_generic);
            println!("func genargs: {:?}", func_genargs);
            println!("destination: {:?}", destination);
            println!("body_locals[dest]: {:?}", body_locals[destination.local]);
        }

        // add rettype generic param to callee scope
        let mut name_opt = None;
        let mut constraints = HashSet::default();
        if let Some(rettype) = funcval.rettype {
            match rettype.kind() {
                TyKind::Adt(adt_def, adt_genargs) => {
                    if self.debug {
                        println!("RETTYPE IS AN ADT: {:?}", adt_def.did());
                        println!("adt_genargs: {:?}", adt_genargs);
                    }

                    if adt_genargs.len() > 0 {
                        match adt_genargs[0].kind() {
                            GenericArgKind::Type(ty) => match ty.kind() {
                                TyKind::Param(param) => {
                                    if func_genargs.len() > 0 {
                                        if let Some(genarg_ty) = func_genargs[0].as_type() {
                                            name_opt = Some(param.name);
                                            constraints.insert(VerifoptRval::IdkType(genarg_ty));
                                        }
                                    }
                                }
                                _ => {}
                            },
                            _ => todo!("first genarg is not a ty: {:?}", adt_genargs[0].kind()),
                        }
                    }
                }
                TyKind::Param(param) => {
                    if self.debug {
                        println!("RETTYPE IS A PARAM: {:?}", param);
                    }

                    name_opt = Some(param.name);
                    constraints.insert(VerifoptRval::IdkType(body_locals[destination.local].ty));
                }
                _ => todo!("handle rettype kind: {:?}", rettype.kind()),
            }
        }

        if let Some(name) = name_opt {
            match cmap.scoped_get(Some(funcval.def_id), &MapKey::Generic(name), false) {
                Some(_) => {
                    if self.debug {
                        println!("\tmapping already exists");
                    }
                }
                None => {
                    if self.debug {
                        println!("\tadding to cmap");
                    }
                    cmap.scoped_add(
                        Some(funcval.def_id),
                        MapKey::Generic(name),
                        Box::new(VarType::Values(constraints)),
                    );
                }
            }

            if self.debug {
                println!(
                    "val @ {:?}: {:?}",
                    name,
                    cmap.scoped_get(Some(funcval.def_id), &MapKey::Generic(name), false,)
                );
            }
        }
    }
}
