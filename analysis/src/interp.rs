use rustc_data_structures::fx::FxHashSet as HashSet;
use rustc_hir::def_id::DefId;
use rustc_index::IndexSlice;
use rustc_middle::mir::interpret::Scalar;
use rustc_middle::mir::*;
use rustc_middle::ty::{List, TyCtxt, TyKind};
use rustc_span::source_map::Spanned;

use crate::constraints::{ConstraintMap, Constraints, MapKey, VarType};
use crate::core::is_box;
use crate::core::{FuncVal, Merge, VerifoptRval};
use crate::error::Error;
use crate::func_collect::FuncMap;
use crate::wto::BBDeps;

pub struct InterpPass<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub funcs: &'a FuncMap<'tcx>,
    pub debug: bool,
}

impl<'a, 'tcx> InterpPass<'a, 'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, funcs: &'a FuncMap<'tcx>, debug: bool) -> InterpPass<'a, 'tcx> {
        Self { tcx, funcs, debug }
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
        // TODO how do loops affect this order?

        if self.debug {
            println!("\n###### INTERP-ING NEW BODY for func {:?}\n", cur_scope);
            println!("prev_scope: {:?}", prev_scope);
            println!("cur_scope: {:?}", cur_scope);
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
                &mut bb_deps,
                //prev_scope,
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
        bb_deps: &mut BBDeps,
        //prev_scope: Option<DefId>,
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
                println!("# visiting STATEMENT");
                println!("# -------------------------------");
            }
            last_res = self.visit_statement(cmap, cur_scope, body_locals, stmt)?;
        }

        if let Some(term) = &data.terminator {
            if self.debug {
                println!("\n# -------------------------------");
                println!("# visiting TERM");
                println!("# -------------------------------");
            }
            // FIXME return basic blocks are the only ones whose interp returns Some(_), so could
            // re-architect this loop to only save the result if it is a Some(_) (then wouldn't
            // have to worry about executing cleanup blocks afterwards)
            last_res = self.visit_terminator(cmap, body_locals, bb, bb_deps, cur_scope, &term)?;
        }

        bb_deps.mark_visited(bb);

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
                    println!("rval: {:?}", rvalue);
                }

                let mut set = HashSet::default();
                let v_rval = VerifoptRval::from_rvalue(
                    self.tcx,
                    self.funcs,
                    cmap,
                    cur_scope,
                    body_locals,
                    rvalue,
                    self.debug,
                );
                set.insert(v_rval.clone());

                cmap.scoped_add(
                    Some(cur_scope),
                    MapKey::Place(place),
                    Box::new(VarType::Values(set)),
                );

                if self.debug {
                    println!("end assignment!");
                    println!("cur_scope: {:?}", cur_scope);
                    println!("place: {:?}", place);
                    println!("rval: {:?}", v_rval);
                    println!(
                        "cur_scope cmap: {:?}",
                        cmap.cmap.get(&MapKey::ScopeId(cur_scope))
                    );
                }
            }
            _ => {}
        }

        Ok(None)
    }

    fn visit_terminator(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        bb: &BasicBlock,
        bb_deps: &mut BBDeps,
        //prev_scope: Option<DefId>,
        cur_scope: DefId,
        terminator: &Terminator<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        match &terminator.kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => self.interp_func_call(cmap, cur_scope, body_locals, func, args, destination),
            TerminatorKind::Return => self.interp_return(cmap, cur_scope),
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
        cur_scope: DefId,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        // return constraints at place _0
        if self.debug {
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
                    if self.debug {
                        println!("returning value w following constraints: {:?}", constraints);
                    }

                    for constraint in constraints.clone().drain() {
                        if let VerifoptRval::IdkType(ty) = constraint {
                            if self.debug {
                                println!("returning a type!: {:?}", ty);
                            }
                            if let TyKind::Param(param) = ty.kind() {
                                if self.debug {
                                    panic!("GENERIC RETTY: {:?}", param);
                                }
                            }
                        }
                    }

                    Ok(Some(constraints))
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
                match cmap.scoped_get(Some(cur_scope), &MapKey::Place(*place), false) {
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
                                match constraint {
                                    VerifoptRval::Scalar(scalar) => match scalar {
                                        Scalar::Int(s_int) => {
                                            let num = s_int.to_bits_unchecked();
                                            for (i, val) in vals.iter().enumerate() {
                                                if num == val.get() {
                                                    discr_vals_slice[i] += 1;
                                                }
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
                    None => {
                        if self.debug {
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
    ) -> Option<VerifoptRval<'tcx>> {
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
                        if genarg_constraint_vec.len() != 1 {
                            panic!("handle different genarg constraint len");
                        }
                        let genarg_constraint = &genarg_constraint_vec[0];

                        // FIXME does this need to be in a separate function?
                        // otherwise could save a clone()
                        return Some(genarg_constraint.clone());
                    } else {
                        return None;
                    }
                } else {
                    if self.debug {
                        println!("struct is not a box: {:?}", struct_defid);
                        println!("genargs: {:?}", genarg_vec);
                    }
                    return Some(idkstruct.clone());
                }
            }
            idkdid @ VerifoptRval::IdkDefId(_) => Some(idkdid),
            idkty @ VerifoptRval::IdkType(_) => Some(idkty),
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
        self_constraint: VerifoptRval<'tcx>,
        impltors: &Vec<DefId>,
    ) -> Vec<DefId> {
        match self_constraint {
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
        cur_scope: DefId,
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        self_constraint: VerifoptRval<'tcx>,
        impltors: &Vec<DefId>,
        args: &Box<[Spanned<Operand<'tcx>>]>,
        destination: &Place<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        // assoc dyn obj w the correct trait fn impl(s)
        let to_dispatch = self.get_trait_fn_impls(self_constraint, impltors);

        // call each impl
        for func_defid in to_dispatch {
            let funcvals = self.funcs.funcs.get(&func_defid);
            if funcvals.is_none() {
                panic!("func not found: {:?}", func_defid);
            }
            let funcval_vec = funcvals.unwrap();
            if funcval_vec.len() != 1 {
                panic!("unexpected number of functions");
            }
            return self.handle_static_dispatch(
                cmap,
                cur_scope,
                body_locals,
                &funcval_vec[0],
                args,
                destination,
            );
        }

        Ok(None)
    }

    fn handle_dyn_dispatch(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        cur_scope: DefId,
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        assoc_funcval: &FuncVal<'tcx>,
        trait_def_id: &DefId,
        args: &Box<[Spanned<Operand<'tcx>>]>,
        destination: &Place<'tcx>,
    ) -> Result<Option<Constraints<'tcx>>, Error> {
        if self.debug {
            println!("found in trait: {:?}", trait_def_id);
        }

        if args.len() == 0 {
            if self.debug {
                println!("argments list empty: cannot determine self type");
            }
            return Ok(None);
        }

        if assoc_funcval.self_arg.is_none() {
            if self.debug {
                println!("no self type: {:?}", assoc_funcval);
            }
            return Ok(None);
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
                        if constraints.len() != 1 {
                            todo!("unexpected number of constraints");
                        }

                        // for each possible constraint...
                        for constraint in constraints.clone().drain() {
                            // unwrap any refs or boxed types
                            if let Some(self_constraint) = self.resolve_dyn_self_constraint(
                                cmap, cur_scope, impltors, constraint, args,
                            ) {
                                if self.debug {
                                    println!("self_constraint: {:?}", self_constraint);
                                }
                                // interp dyn dispatch as one or more static dispatches
                                return self.dyn_to_static_dispatch(
                                    cmap,
                                    cur_scope,
                                    body_locals,
                                    self_constraint,
                                    impltors,
                                    args,
                                    destination,
                                );
                            } else {
                                todo!("no self constraint");
                            }
                        }
                    } else {
                        panic!("first argument place does not exist: {:?}", place);
                    }
                }
                _ => {}
            }
        } else {
            panic!("missing implementors");
        }

        Ok(None)
    }

    fn handle_static_dispatch(
        &self,
        mut cmap: &mut ConstraintMap<'tcx>,
        cur_scope: DefId,
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        funcval: &FuncVal<'tcx>,
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

        if funcval.ret_generic.is_some() {
            self.resolve_generic_retty(&mut cmap, cur_scope, body_locals, funcval, destination);
        }

        // visit callee
        let callee_body = self.tcx.optimized_mir(funcval.def_id);
        self.visit_body(cmap, Some(cur_scope), funcval.def_id, callee_body)
    }

    fn interp_direct_func_call(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
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
                TyKind::FnDef(def_id, _) => match self.funcs.funcs.get(def_id) {
                    Some(funcval_vec) => {
                        for funcval in funcval_vec.iter() {
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

                                        match self.handle_dyn_dispatch(
                                            cmap,
                                            cur_scope,
                                            body_locals,
                                            funcval,
                                            &trait_def_id_clone,
                                            args,
                                            destination,
                                        ) {
                                            Ok(Some(constraints)) => {
                                                if self.debug {
                                                    println!(
                                                        "\n##### DONE w DYN func {:?}\n",
                                                        def_id
                                                    );
                                                    println!("constraints: {:?}", constraints);
                                                }

                                                res_vec.push(constraints);
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
                                                    constraints
                                                        .insert(VerifoptRval::IdkDefId(ret_did));
                                                    res_vec.push(constraints);
                                                } else if let Some(rettype) = funcval.rettype {
                                                    if self.debug {
                                                        println!("rettype: {:?}", rettype);
                                                    }

                                                    // FIXME resolve generics
                                                    constraints
                                                        .insert(VerifoptRval::IdkType(rettype));
                                                    res_vec.push(constraints);
                                                }
                                            }
                                            e @ Err(_) => return e,
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
                                    cur_scope,
                                    body_locals,
                                    funcval,
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
                                            res_vec.push(constraints);
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

        if cmap_vec.len() > 1 || res_vec.len() > 1 {
            todo!("impl cmap_vec/res_vec merge");
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

        if res_vec.len() > 0 {
            let val = res_vec.pop().unwrap();
            if self.debug {
                println!("setting return val: {:?}", val);
                println!("destination: {:?}", destination);
            }
            cmap.scoped_add(
                Some(cur_scope),
                MapKey::Place(*destination),
                Box::new(VarType::Values(val)),
            );
        } else {
            if self.debug {
                println!("res_vec is empty");
                println!("cur_scope: {:?}", cur_scope);
                println!("destination: {:?}", destination);
            }
        }

        Ok(None)
    }

    fn interp_func_call(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        //prev_scope: Option<DefId>,
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
        }

        match func {
            Operand::Constant(box co) => {
                self.interp_direct_func_call(cmap, cur_scope, body_locals, co, args, destination)
            }
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
                        ConstValue::Scalar(scalar) => {
                            if self.debug {
                                println!("scalar: {:?}", scalar);
                            }
                            constraints.insert(VerifoptRval::Scalar(scalar));
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
            Some(boxed_vartype) => {
                match *boxed_vartype.clone() {
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
                }
            },
            _ => {},
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

    fn resolve_generic_retty(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        _cur_scope: DefId,
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        funcval: &FuncVal<'tcx>,
        destination: &Place<'tcx>,
    ) {
        if self.debug {
            println!("HERE");
            println!("funcval.ret_generic: {:?}", funcval.ret_generic);
            println!("destination: {:?}", destination);
            println!("body_locals[dest]: {:?}", body_locals[destination.local]);
        }

        // add rettype generic param to callee scope
        if let Some(rettype) = funcval.rettype {
            match rettype.kind() {
                TyKind::Param(param) => {
                    if self.debug {
                        println!("RETTYPE IS A PARAM: {:?}", param);
                    }

                    let mut constraints = HashSet::default();
                    constraints.insert(VerifoptRval::IdkType(body_locals[destination.local].ty));

                    cmap.scoped_add(
                        Some(funcval.def_id),
                        MapKey::Generic(param.name),
                        Box::new(VarType::Values(constraints)),
                    );

                    if self.debug {
                        println!(
                            "new val @ {:?}: {:?}",
                            param.name,
                            cmap.scoped_get(
                                Some(funcval.def_id),
                                &MapKey::Generic(param.name),
                                false,
                            )
                        );
                    }
                }
                _ => {}
            }
        }
    }
}
