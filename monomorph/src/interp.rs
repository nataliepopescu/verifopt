use rustc_public::DefId;
use rustc_public::mir::mono::{Instance, InstanceKind};
use rustc_public::mir::{
    BasicBlock, BasicBlockIdx, Body, ConstOperand, Local, LocalDecl, Operand, Place, Statement,
    StatementKind, Successors, SwitchTargets, Terminator, TerminatorKind,
};
use rustc_public::ty::{BoundVariableKind, FnDef, GenericArgs, PolyFnSig, RigidTy, TyKind};

use log::debug;

use crate::constraints::{Constraints, InterpStore, MapKey, MapValue, Merge, VORval};
use crate::convert::RvalConverter;
use crate::error::Error;
use crate::trait_collect::TraitStore;
use crate::wto::BBDeps;

pub struct InterpPass<'a> {
    pub tstore: &'a TraitStore,
    pub converter: RvalConverter<'a>,
}

/// Using `Instance` as unique ID (internal objects are interned so this is apparently cheap)

impl<'a> InterpPass<'a> {
    pub fn new(tstore: &'a TraitStore) -> InterpPass<'a> {
        Self {
            tstore,
            converter: RvalConverter::new(tstore),
        }
    }

    pub fn run(
        &self,
        istore: &mut InterpStore,
        start_scope: Instance,
    ) -> Result<Option<Constraints>, Error> {
        //let start_scope = instance;
        let mut call_stack = vec![start_scope];

        // Initialize InterpStore with entry_fn's substore
        let entry_fn_istore = InterpStore::new();
        istore.cmap.insert(
            MapKey::ScopeId(start_scope),
            Box::new(MapValue::Store(entry_fn_istore)),
        );

        self.visit_instance(istore, &mut call_stack, start_scope)
    }

    fn print_mir(&self, body: &Body) {
        debug!("arg count: {:?}", body.arg_locals().len());
        debug!("----START BODY----");

        let locals = body.locals();
        let blocks = &body.blocks;

        debug!("num LocalDecls: {:?}", locals.len());
        debug!("{{");
        for i in 0..locals.len() {
            debug!("-local{:?}", i);
            debug!("{:?}", locals[i]);
        }
        debug!("}}");

        debug!("num BasicBlocks: {:?}", blocks.len());
        debug!("{{");
        for i in 0..blocks.len() {
            debug!("-bb{:?}", i);
            debug!("{:?}", blocks[i]);
        }
        debug!("}}");

        debug!("----END BODY----");
    }

    fn visit_instance(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<Instance>,
        cur_scope: Instance,
    ) -> Result<Option<Constraints>, Error> {
        let body = cur_scope.body().unwrap();
        debug!("\n\n\n#############################");
        debug!("###### INTERP-ING NEW BODY for func {:?}", cur_scope);
        debug!("call_stack: {:#?}", call_stack);
        self.print_mir(&body);
        debug!("#############################\n\n");

        self.visit_body(istore, call_stack, cur_scope, &body)
    }

    fn visit_body(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<Instance>,
        cur_scope: Instance,
        body: &Body,
    ) -> Result<Option<Constraints>, Error> {
        // If there exists a memoized WTO, use it; otherwise, create it
        let mut bb_deps;
        if let Some(mem_bb_deps) = istore.wtos.get(&cur_scope) {
            bb_deps = mem_bb_deps.clone();
            debug!("OLD ordering: {:?}", bb_deps.ordering);
        } else {
            bb_deps = BBDeps::new(body);
            istore.wtos.insert(cur_scope, bb_deps.clone());
            debug!("NEW ordering: {:?}", bb_deps.ordering);
        }

        // Loop through basic blocks in WTO
        let mut last_res = None;
        loop {
            if bb_deps.ordering.is_empty() {
                break;
            }
            let bb = bb_deps.ordering.remove(0);
            debug!("--NEW BB: {:?}", bb);
            let data = body.blocks.get(bb).unwrap();
            last_res = self.visit_basic_block(
                istore,
                call_stack,
                cur_scope,
                body.locals(),
                &mut bb_deps,
                bb,
                data,
            )?;
        }

        Ok(last_res)
    }

    fn visit_basic_block(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<Instance>,
        cur_scope: Instance,
        local_decls: &[LocalDecl],
        bb_deps: &mut BBDeps,
        bb: usize,
        data: &BasicBlock,
    ) -> Result<Option<Constraints>, Error> {
        debug!("\n\n#############");
        debug!("# visiting BASICBLOCK {:?} for {:?}", bb, cur_scope);
        debug!("#############");

        for (i, stmt) in data.statements.iter().enumerate() {
            debug!(
                "\n\n# visiting STATEMENT {:?} in BB{:?} for {:?}",
                i, bb, cur_scope
            );
            self.visit_statement(istore, cur_scope, local_decls, stmt);
        }

        debug!(
            "\n\n# visiting TERMINATOR in BB{:?} for {:?}",
            bb, cur_scope
        );
        let res = self.visit_terminator(
            istore,
            call_stack,
            cur_scope,
            local_decls,
            bb_deps,
            bb,
            &data.terminator,
        )?;

        bb_deps.mark_visited(bb, cur_scope);

        Ok(res)
    }

    fn visit_statement(
        &self,
        istore: &mut InterpStore,
        cur_scope: Instance,
        local_decls: &[LocalDecl],
        stmt: &Statement,
    ) {
        match &stmt.kind {
            // Only interp assignments to track type constraint changes
            StatementKind::Assign(place, rvalue) => {
                debug!("start assignment!");
                debug!("cur_scope: {:?}", cur_scope);
                debug!("stmt: {:?}", &stmt.kind);
                debug!("place: {:?}", place);
                debug!("dest ty: {:?}", &local_decls[place.local].ty);
                debug!("rval: {:?}", rvalue);

                // convert Rvalue to VORval
                let constraints = self
                    .converter
                    .convert(istore, cur_scope, local_decls, rvalue);
                debug!("CONVERTED CONSTRAINTS: {:?}", constraints);

                // add resolved constraints to istore
                istore.scoped_update(
                    cur_scope,
                    MapKey::Local(place.local),
                    Box::new(MapValue::Constraints(constraints)),
                );
            }
            _ => {}
        }
    }

    fn visit_terminator(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<Instance>,
        cur_scope: Instance,
        local_decls: &[LocalDecl],
        bb_deps: &mut BBDeps,
        bb: usize,
        term: &Terminator,
    ) -> Result<Option<Constraints>, Error> {
        match &term.kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => match func {
                Operand::Constant(co) => self.interp_direct_call(
                    istore,
                    call_stack,
                    cur_scope,
                    local_decls,
                    co,
                    args,
                    destination,
                ),
                _ => todo!("handle indirect function invocations"),
            },
            TerminatorKind::Return => self.interp_return(istore, call_stack, cur_scope),
            TerminatorKind::SwitchInt { discr, targets } => {
                self.interp_switchint(istore, cur_scope, local_decls, bb, bb_deps, discr, targets)
            }
            _ => Ok(None),
        }
    }

    fn interp_direct_call(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<Instance>,
        cur_scope: Instance,
        local_decls: &[LocalDecl],
        co: &ConstOperand,
        args: &Vec<Operand>,
        destination: &Place,
    ) -> Result<Option<Constraints>, Error> {
        let ret_constraints = match co.const_.ty().kind() {
            TyKind::RigidTy(rigid_ty) => match rigid_ty {
                RigidTy::FnDef(fndef, genargs) => {
                    let instance = Instance::resolve(fndef, &genargs).unwrap();
                    debug!("instance def: {:?}", instance.def);
                    debug!("--- CALLING {:?}", fndef);
                    //debug!("call_stack: {:?}", call_stack);
                    match instance.kind {
                        InstanceKind::Item => {
                            debug!("regular static funccall");
                            self.interp_static_call(
                                istore,
                                call_stack,
                                cur_scope,
                                local_decls,
                                instance,
                                fndef,
                                args,
                                &genargs,
                            )
                        }
                        InstanceKind::Virtual { .. } => {
                            debug!("virtual funccall");
                            self.interp_virtual_call(
                                istore,
                                call_stack,
                                cur_scope,
                                local_decls,
                                &fndef,
                                &genargs,
                                args,
                            )
                        }
                        InstanceKind::Intrinsic => {
                            debug!("intrinsic funccall");
                            self.retty_fallback(fndef)
                        }
                        InstanceKind::Shim => todo!("shim funccall"),
                    }
                }
                other @ _ => todo!("different RigidTy: {:?}", other),
            },
            kind @ _ => todo!("funccall const is another kind: {:?}", kind),
        };

        // Set destination local to value in cmap
        debug!("RET FROM FUNC CALL ret_constraints: {:?}", ret_constraints);
        debug!("cur_scope: {:?}", cur_scope);
        debug!("destination: {:?}", destination);
        match ret_constraints {
            Ok(Some(constraints)) => {
                istore.scoped_update(
                    cur_scope,
                    MapKey::Local(destination.local),
                    Box::new(MapValue::Constraints(constraints)),
                );
            }
            Ok(None) => {}
            _ => panic!("error in constraints"),
        }

        Ok(None)
    }

    fn interp_static_call(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<Instance>,
        caller_scope: Instance,
        local_decls: &[LocalDecl],
        instance: Instance,
        fndef: FnDef,
        args: &Vec<Operand>,
        genargs: &GenericArgs,
    ) -> Result<Option<Constraints>, Error> {
        if instance.has_body() {
            let mut istore_clone = istore.clone();
            //let callee_scope = instance; //(fndef.0, instance);
            call_stack.push(instance);
            self.resolve_args(
                &mut istore_clone,
                caller_scope,
                local_decls,
                instance,
                fndef,
                args,
                genargs,
            );
            self.visit_instance(&mut istore_clone, call_stack, instance)
        } else {
            debug!("no body");
            self.retty_fallback(fndef)
        }
    }

    fn resolve_args(
        &self,
        istore: &mut InterpStore,
        caller_scope: Instance,
        local_decls: &[LocalDecl],
        callee_scope: Instance,
        fndef: FnDef,
        args: &Vec<Operand>,
        genargs: &GenericArgs,
    ) {
        debug!("RESOLVING ARGS");
        let mut new_substore = InterpStore::new();
        let key = MapKey::ScopeId(callee_scope);

        let sig = fndef.fn_sig();
        debug!("fn_sig: {:?}", sig);
        self.check_sig_boundvars(&sig);

        // Resolve generics + add arg values into new substore
        self.resolve_args_helper(
            istore,
            &mut new_substore,
            caller_scope,
            local_decls,
            args,
            genargs,
        );

        // Merge new substore into existing substore(s) at this scopeId
        match istore.cmap.get(&key) {
            Some(_) => todo!("merge substores"),
            None => {}
        }

        // Add new substore in top-level store
        istore
            .cmap
            .insert(key, Box::new(MapValue::Store(new_substore)));
    }

    fn resolve_args_helper(
        &self,
        istore: &InterpStore,
        new_substore: &mut InterpStore,
        caller_scope: Instance,
        local_decls: &[LocalDecl],
        args: &Vec<Operand>,
        genargs: &GenericArgs,
    ) {
        for (i, arg) in args.into_iter().enumerate() {
            debug!("arg position: {:?}", i);
            debug!("arg: {:?}", arg);
            let place = Place {
                local: i + 1,
                projection: vec![],
            };
            let arg_constraints = self.resolve_arg(istore, caller_scope, local_decls, arg);
            debug!("arg constraints: {:?}", arg_constraints);

            new_substore.cmap.insert(
                MapKey::Local(place.local),
                Box::new(MapValue::Constraints(arg_constraints)),
            );
        }
        debug!("generic args: {:?}", genargs);
    }

    fn resolve_arg(
        &self,
        istore: &InterpStore,
        caller_scope: Instance,
        _local_decls: &[LocalDecl],
        arg: &Operand,
    ) -> Constraints {
        match arg {
            Operand::Copy(place) | Operand::Move(place) => {
                match istore.scoped_get(caller_scope, &MapKey::Local(place.local)) {
                    Some(val) => match val {
                        MapValue::Constraints(constraints) => constraints,
                        _ => panic!("arg is a scope"),
                    },
                    None => panic!("place {:?} DNE in cmap (use backup ty?)", place),
                }
            }
            // TODO can maybe get a more precise VORval depending on kind
            Operand::Constant(const_op) => vec![VORval::IdkType(const_op.const_.ty())],
            _ => todo!("runtime check arg"),
        }
    }

    fn check_sig_boundvars(&self, sig: &PolyFnSig) {
        if !sig.bound_vars.is_empty() {
            // Might not be safe to just skip binder
            debug!("Bound vars - cannot just skip binder in call resolution");
            for bound_var in sig.bound_vars.iter() {
                match bound_var {
                    BoundVariableKind::Ty(_) => todo!("ty"),
                    BoundVariableKind::Const => todo!("const"),
                    BoundVariableKind::Region(_) => {}
                }
            }
        }
    }

    fn retty_fallback(&self, fndef: FnDef) -> Result<Option<Constraints>, Error> {
        let sig = fndef.fn_sig();
        debug!("fn_sig: {:?}", sig);
        self.check_sig_boundvars(&sig);
        debug!("output: {:?}", sig.value.output());

        // Return output type as VORval (widening)
        let ret_constraints = vec![VORval::IdkType(sig.value.output())];
        Ok(Some(ret_constraints))
    }

    fn interp_virtual_call(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<Instance>,
        cur_scope: Instance,
        local_decls: &[LocalDecl],
        fndef: &FnDef,
        genargs: &GenericArgs,
        args: &Vec<Operand>,
    ) -> Result<Option<Constraints>, Error> {
        // Steps:
        // - Get trait that this function is associated with
        //   - tstore.assoc_fn_traits (Map<AssocFn, Trait>)
        // - Get concrete type constraints for trait object
        //   - istore (FSA) / tstore (CHA / RTA)
        // - Get every concrete type constraint's impl of this function
        //   - tstore.struct_assoc_fns (Map<(Struct, Trait), FnImpls>)

        debug!("DYNAMIC DISPATCH");
        debug!("args: {:?}", args);

        let trait_defid = self.get_trait_defid(&fndef.0);
        debug!("trait_defid: {:?}", trait_defid);

        let assoc_fn_impls_cha = self.get_impls_cha(&fndef.0, &trait_defid);
        debug!("------- assoc_fn_impls_cha: {:?}", assoc_fn_impls_cha);

        let assoc_fn_impls_fsa = self.get_impls_fsa(istore, cur_scope, &fndef.0, args);
        debug!("------- assoc_fn_impls_fsa: {:?}", assoc_fn_impls_fsa);

        if assoc_fn_impls_cha != assoc_fn_impls_fsa {
            debug!("SET OF IMPLS DIFFER");
        } else {
            debug!("SET OF IMPLS SAME");
        }

        self.simulate_static_calls(
            istore,
            call_stack,
            cur_scope,
            local_decls,
            //assoc_fn_impls_cha,
            assoc_fn_impls_fsa,
            genargs,
            args,
        )
    }

    fn get_trait_defid(&self, assoc_fn_defid: &DefId) -> DefId {
        // Get trait that this function is associated with
        match self.tstore.assoc_fn_traits.get(assoc_fn_defid) {
            Some(trait_defid) => *trait_defid,
            None => panic!("assoc fn {:?} does not point to trait", assoc_fn_defid),
        }
    }

    fn get_impls_from_defids(
        &self,
        assoc_fn_defid: &DefId,
        constraint_defids: &Vec<DefId>,
    ) -> Vec<DefId> {
        // Get every concrete type constraint's impl of this function
        let mut assoc_fn_impls = Vec::new();
        for defid in constraint_defids {
            match self.tstore.struct_assoc_fns.get(&(*defid, *assoc_fn_defid)) {
                Some(assoc_fn_impl) => assoc_fn_impls.append(&mut assoc_fn_impl.clone()),
                None => panic!(
                    "struct/container ({:?}, {:?}) pair does not point to assoc fn",
                    defid, assoc_fn_defid
                ),
            }
        }
        assoc_fn_impls
    }

    fn get_impls_cha(&self, assoc_fn_defid: &DefId, trait_defid: &DefId) -> Vec<DefId> {
        debug!("GETTING CHA IMPLS");
        let constraint_defids = self.get_cha_tyconstraint_defids(&trait_defid);
        self.get_impls_from_defids(assoc_fn_defid, &constraint_defids)
    }

    fn get_cha_tyconstraint_defids(&self, trait_defid: &DefId) -> Vec<DefId> {
        // Get concrete type constraints for trait object
        match self.tstore.trait_structs.get(trait_defid) {
            Some(tyconstraints) => tyconstraints.to_vec(),
            None => panic!("trait {:?} does not point to any structs", trait_defid),
        }
    }

    fn get_impls_fsa(
        &self,
        istore: &InterpStore,
        cur_scope: Instance,
        assoc_fn_defid: &DefId,
        args: &Vec<Operand>,
    ) -> Vec<DefId> {
        debug!("GETTING FSA IMPLS");
        let local = self.get_traitobj_local(args);
        let tyconstraints = self.get_fsa_tyconstraints(istore, cur_scope, local);
        let constraint_defids = self.get_fsa_constraint_defids(&tyconstraints);
        self.get_impls_from_defids(assoc_fn_defid, &constraint_defids)
    }

    fn get_traitobj_local(&self, args: &Vec<Operand>) -> Local {
        match &args[0] {
            Operand::Copy(place) | Operand::Move(place) => {
                if !place.projection.is_empty() {
                    panic!("traitobj place has projections");
                }

                place.local
            }
            _ => panic!("unexpected operand: {:?}", args[0]),
        }
    }

    fn get_fsa_tyconstraints(
        &self,
        istore: &InterpStore,
        cur_scope: Instance,
        local: Local,
    ) -> Constraints {
        // Get concrete type constraints for trait object
        match istore.scoped_get(cur_scope, &MapKey::Local(local)) {
            Some(val) => match val {
                MapValue::Constraints(tyconstraints) => tyconstraints,
                MapValue::Store(_) => panic!("local {:?} refers to a scope", local),
            },
            None => panic!("local {:?} has no constraints", local),
        }
    }

    fn get_fsa_constraint_defids(&self, tyconstraints: &Constraints) -> Vec<DefId> {
        let mut defids = Vec::new();
        for vorval in tyconstraints {
            defids.push(self.resolve_defid(vorval));
        }
        defids
    }

    fn resolve_defid(&self, vorval: &VORval) -> DefId {
        match vorval {
            VORval::IdkAdt(adtdef, _) => adtdef.0,
            VORval::RawPtr(inner) | VORval::Ref(inner) => self.resolve_defid(inner),
            _ => panic!("other vorval: {:?}", vorval),
        }
    }

    fn simulate_static_calls(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<Instance>,
        cur_scope: Instance,
        local_decls: &[LocalDecl],
        assoc_fn_impls: Vec<DefId>,
        genargs: &GenericArgs,
        args: &Vec<Operand>,
    ) -> Result<Option<Constraints>, Error> {
        let mut results = Vec::<Option<Constraints>>::new();
        let mut istore_vec = Vec::new();

        for assoc_fn_impl in assoc_fn_impls {
            let fndef = FnDef(assoc_fn_impl);
            let instance = Instance::resolve(fndef, &genargs).unwrap();
            debug!("instance def: {:?}", instance.def);
            debug!("a converted static instance: {:?}", instance);
            let mut call_stack_clone = call_stack.clone();
            //let callee_scope = instance;
            call_stack_clone.push(instance);

            let mut istore_clone = istore.clone();
            self.resolve_args(
                &mut istore_clone,
                cur_scope,
                local_decls,
                instance,
                fndef,
                args,
                &genargs,
            );
            results.push(self.visit_instance(
                &mut istore_clone,
                &mut call_stack_clone,
                instance,
            )?);

            istore_vec.push(istore_clone);
        }

        self.merge_istores_and_set(istore, &mut istore_vec);
        self.merge_results_and_ret(&mut results)
    }

    fn merge_istores_and_set(&self, istore: &mut InterpStore, istore_vec: &mut Vec<InterpStore>) {
        match istore_vec.merge() {
            Ok(Some(merged_istore)) => {
                debug!("merged istore: {:?}", merged_istore);
                *istore = merged_istore;
            }
            Ok(None) => panic!("istores empty?"),
        }
    }

    fn merge_results_and_ret(
        &self,
        results: &mut Vec<Option<Constraints>>,
    ) -> Result<Option<Constraints>, Error> {
        // Filter out None constraints and unwrap all Some options
        debug!("results PRE filter: {:?}", results);
        let filtered_results: Vec<Constraints> = results
            .into_iter()
            .filter(|option| option.is_none())
            .map(|x| x.clone().unwrap())
            .collect();
        debug!("results POST filter: {:?}", filtered_results);

        match filtered_results.merge() {
            Ok(Some(merged_constraints)) => {
                debug!("merged constraints: {:?}", merged_constraints);
                return Ok(Some(merged_constraints));
            }
            Ok(None) => Ok(None),
        }
    }

    fn interp_switchint(
        &self,
        istore: &mut InterpStore,
        cur_scope: Instance,
        local_decls: &[LocalDecl],
        bb: usize,
        bb_deps: &mut BBDeps,
        discr: &Operand,
        targets: &SwitchTargets,
    ) -> Result<Option<Constraints>, Error> {
        debug!("SWITCHINT");
        debug!("discr: {:?}", discr);
        debug!("targets: {:?}", targets);

        match discr {
            Operand::Copy(place) | Operand::Move(place) => {
                match istore.scoped_get(cur_scope, &MapKey::Local(place.local)) {
                    Some(val) => match val {
                        MapValue::Constraints(constraints) => {
                            // Create a byte-map for finding statically-impossible successors
                            let mut discr_vals_uninit =
                                Box::<[u8]>::new_zeroed_slice(targets.len());
                            let discr_vals = discr_vals_uninit.write_filled(0);

                            // Check expected type of discriminant
                            debug!("expected ty: {:?}", place.ty(local_decls));

                            // Populate byte-map with possible branch values, based on constraints
                            for constraint in constraints {
                                self.set_bytemap(constraint, targets.branches(), discr_vals);
                            }

                            self.prune_switchint_targets(
                                bb,
                                bb_deps,
                                &targets.all_targets(),
                                discr_vals,
                            );
                        }
                        _ => panic!("cannot switch on scope"),
                    },
                    None => panic!("local DNE"),
                }
            }
            Operand::Constant(_co) => {}
            _ => debug!("runtime checks op (ignoring)"),
        }

        Ok(None)
    }

    fn set_bytemap(
        &self,
        constraint: VORval,
        branches: impl Iterator<Item = (u128, BasicBlockIdx)>,
        discr_vals: &mut [u8],
    ) {
        match constraint {
            VORval::Scalar(num) => {
                // Increment matching branch counters
                for (i, branch) in branches {
                    if num == branch.try_into().unwrap() {
                        discr_vals[usize::try_from(i).unwrap()] += 1;
                    }
                }
            }
            VORval::Bool => {
                // Increment all branch counters (since no static bool value)
                for (i, _branch) in branches {
                    discr_vals[usize::try_from(i).unwrap()] += 1;
                }
            }
            VORval::IdkType(_) | VORval::Idk => {
                // Increment "otherwise" branch counter
                discr_vals[discr_vals.len() - 1] += 1;
            }
            VORval::Ref(inner) => self.set_bytemap(*inner, branches, discr_vals),
            _ => panic!("unexpected switchint discr: {:?}", constraint),
        }
    }

    fn prune_switchint_targets(
        &self,
        bb: usize,
        bb_deps: &mut BBDeps,
        targets: &Successors,
        discr_vals: &mut [u8],
    ) {
        let prunable = self.get_prunable_targets(discr_vals);

        debug!("all targets: {:?}", targets);
        debug!("prunable: {:?}", prunable);

        if let Some(prune_idxs) = prunable {
            for prune_idx in prune_idxs {
                bb_deps.prune(bb, targets[prune_idx]);
            }
        }
    }

    fn get_prunable_targets(&self, discr_vals: &mut [u8]) -> Option<Vec<usize>> {
        // If the last value (otherwise branch) is > 0, cannot prune anything
        if discr_vals[discr_vals.len() - 1] > 0 {
            return None;
        }

        let mut poss_idxs = Vec::new();
        let mut imposs_idxs = Vec::new();
        for i in 0..discr_vals.len() {
            if discr_vals[i] > 0 {
                poss_idxs.push(i);
            } else {
                imposs_idxs.push(i);
            }
        }
        debug!("possible indices: {:?}", poss_idxs);
        debug!("impossible indices: {:?}", imposs_idxs);

        // If no possible indices, error?
        if poss_idxs.is_empty() {
            panic!("no possible branches");
        }

        // If some impossible indices, prune
        if !imposs_idxs.is_empty() {
            return Some(imposs_idxs);
        }

        // Some possible branches without any impossible branches -> cannot prune
        None
    }

    fn interp_return(
        &self,
        istore: &mut InterpStore,
        call_stack: &mut Vec<Instance>,
        cur_scope: Instance,
    ) -> Result<Option<Constraints>, Error> {
        debug!("RETURNING from scope {:?}...", cur_scope);
        //debug!("callstack PRE POP: {:?}", call_stack);
        let popped = call_stack.pop();
        if popped.unwrap() != cur_scope {
            panic!("call stack out of sorts");
        }
        //debug!("callstack POST POP: {:?}", call_stack);

        // Get and "return" the constraints at Place(0)
        match istore.scoped_get(cur_scope, &MapKey::Local(0)) {
            Some(retval) => match retval {
                MapValue::Constraints(retval_constraints) => {
                    debug!("returning constraints: {:?}", retval_constraints);
                    Ok(Some(retval_constraints))
                }
                _ => panic!("should not be returning a scope"),
            },
            None => {
                // TODO Double check that nothing _needs_ to be returned (for interp correctness)

                Ok(None)
            }
        }
    }
}
