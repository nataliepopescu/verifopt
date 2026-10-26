use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use im::HashMap as ImHashMap;
use im::HashSet as ImHashSet;
use im::hashmap::Entry as ImEntry;

use rustc_public::DefId;
use rustc_public::mir::mono::{Instance, InstanceKind, StaticDef};
use rustc_public::mir::{
    BasicBlock, Body, BorrowKind, ConstOperand, CopyNonOverlapping, LocalDecl, Mutability,
    NonDivergingIntrinsic, Operand, Place, ProjectionElem, Rvalue, Statement, StatementKind,
    Successors, SwitchTargets, Terminator, TerminatorKind,
};

use rustc_public::mir::alloc::GlobalAlloc;
use rustc_public::ty::{
    AdtDef, BoundVariableKind, ClosureDef, ClosureKind, ConstantKind, FnDef, GenericArgKind,
    GenericArgs, IntTy, PolyFnSig, Prov, RigidTy, Span, Ty, TyKind,
};
use rustc_public::{CrateDef, CrateDefType};

use log::{debug, error, warn};

use crate::Context;
use crate::common::{log_call_stack, log_scope};
use crate::constraints::{
    ADTFields, ArgSet, BaseScopeId, Constraint, ConstraintStore, Constraints, EnclosingScopes,
    Location, MapKey, MapValue, MemoKey, RunningConstraint, SummaryKey, TagProv,
    TraitObjConstraint, TraitObjTy, VOID, base_scope_id, hash_val, memoize_by_rc,
    push_caller_context, summary_key,
};
use crate::constraints::{unique_append, unique_push};
use crate::convert::RvalConverter;
use crate::error::Error;
use crate::merge::Merge;
use crate::sig_collect::{SigStore, SigVal};
use crate::timing::{SelfTimeGuard, TimingCat, TimingStats};
use crate::trait_collect::TraitStore;
use crate::wto::BBDeps;
use crate::write_log::SharedLog;
use indexmap::IndexSet;
use std::rc::{Rc, Weak};

const MAX_DEPTH: u32 = 50;

/// Cache key for `virtual_call_memo` - the call site (caller function's
/// DefId + basic block, same pair already used for `dispatch_cha`) plus
/// an `ArgSet` fingerprint of the call's operands as seen from the
/// caller's side. See `virtual_call_memo`'s field doc for the rationale.
type VirtualCallKey = ((DefId, usize, GenericArgs), ArgSet);

#[derive(Debug, Clone)]
pub enum ParamSummary {
    Built(Option<Constraints>),
    Unavailable,
}

/// Opt-in, name-filtered scope tracing for investigating specific
/// dispatch/recursion behavior without flooding the log for the whole
/// run - set VERIFOPT_TRACE_SCOPE to a substring (e.g. "sum" or
/// "recursive_dyn3") to enable. Mirrors the ad hoc
/// `if scope_name.contains("RawVecInner")`-style filtering already used
/// elsewhere in this file, just as a small, reusable, non-recursion-
/// depth-affecting env-var-gated helper instead of a one-off inline
/// check, since this investigation spans three different call sites.
fn trace_scope_filter() -> &'static Option<String> {
    static FILTER: OnceLock<Option<String>> = OnceLock::new();
    FILTER.get_or_init(|| std::env::var("VERIFOPT_TRACE_SCOPE").ok())
}

fn scope_matches_trace_filter(scope: &VOID) -> bool {
    match trace_scope_filter() {
        Some(filter) if !filter.is_empty() => {
            format!("{:?}", scope.0.name()).contains(filter.as_str())
        }
        _ => false,
    }
}

/// Checks whether (instance, genargs) - the base identity of a
/// prospective new scope, before any caller-tag would be applied - is
/// already active somewhere on call_stack. If so, this is genuine,
/// live recursion (the same function calling itself, directly or
/// through intermediate frames, while still on the stack) - not two
/// unrelated call sites reaching the same callee at different times,
/// which never overlap on the stack (e.g. diff_sites's
/// first_speak/second_speak, where first_speak's call has already
/// returned before second_speak's begins). Returns a clone of the
/// innermost (most recent) matching ancestor's own full scope,
/// including its own tag, so the new call reuses it verbatim -
/// collapsing the whole currently-active recursive chain back onto one
/// shared identity, restoring the property finish_frame's queue/revisit
/// cascade actually depends on (see the recursive_dyn2 regression this
/// addresses), without reverting to full context-insensitivity for
/// genuinely distinct, non-overlapping call sites.
fn find_active_recursive_scope(
    call_stack: &[VOID],
    instance: &Instance,
    genargs: &GenericArgs,
) -> Option<VOID> {
    let result = call_stack
        .iter()
        .rev()
        .find(|scope| &scope.0 == instance && &scope.1 == genargs)
        .cloned();

    if let Some(filter) = trace_scope_filter() {
        if !filter.is_empty() && format!("{:?}", instance.name()).contains(filter.as_str()) {
            debug!(
                "TRACE find_active_recursive_scope for {:?}: genargs={:?} call_stack_len={} call_stack={:?} MATCH={:?}",
                instance.name(),
                genargs,
                call_stack.len(),
                call_stack,
                result
            );
        }
    }

    result
}

pub struct InterpPass<'a> {
    pub sigstore: &'a SigStore,
    pub tstore: &'a TraitStore,
    pub converter: RvalConverter<'a>,

    // How many levels of immediate-caller context to fold into a new
    // scope's own identity when one is constructed (see
    // constraints::push_caller_context) - 0 reproduces the previous,
    // context-insensitive behavior. See --context-depth's own docs in
    // util/options.rs for the full rationale.
    pub call_context_k: usize,

    pub dispatch_targets:
        RefCell<ImHashMap<(DefId, usize, GenericArgs), (Span, Vec<(DefId, Option<GenericArgs>)>)>>,
    pub dispatch_cha:
        RefCell<ImHashMap<(DefId, usize, GenericArgs), (Span, Vec<(DefId, Option<GenericArgs>)>)>>,
    pub dispatch_tags: RefCell<ImHashMap<(DefId, usize, GenericArgs), TagPlan>>,

    /// Recursion summaries: the result, and the writes the summarized call
    /// made through its pointer parameters (replayed on a hit, see write_log).
    pub summaries: RefCell<HashMap<MemoKey, (Constraints, SharedLog)>>,
    pub in_queue: RefCell<HashSet<SummaryKey>>,
    pub key_stack: RefCell<Vec<SummaryKey>>,
    pub wq: RefCell<HashMap<SummaryKey, Vec<(VOID, Vec<Constraints>, Vec<VOID>)>>>,
    pub rec_depth: RefCell<u32>,
    //pub call_count: RefCell<u64>,
    pub bb_visit_count: RefCell<u64>,
    pub run_start: std::time::Instant,
    pub main_ctxt_ptr: RefCell<Option<usize>>,
    pub self_time_child_accum: RefCell<Vec<std::time::Duration>>,
    pub scope_self_time: RefCell<HashMap<VOID, (std::time::Duration, u64)>>,
    pub(crate) scope_self_time_global: RefCell<HashMap<VOID, (std::time::Duration, u64)>>,
    pub(crate) timing_global: RefCell<HashMap<TimingCat, TimingStats>>,
    pub(crate) timing_scope: RefCell<HashMap<(VOID, TimingCat), TimingStats>>,
    pub(crate) timing_scope_global: RefCell<HashMap<(VOID, TimingCat), TimingStats>>,
    pub(crate) timing_window: RefCell<HashMap<TimingCat, TimingStats>>,
    pub(crate) timing_child_stack: RefCell<Vec<std::time::Duration>>,
    pub(crate) timing_global_exclusive: RefCell<HashMap<TimingCat, TimingStats>>,
    pub(crate) timing_scope_exclusive: RefCell<HashMap<(VOID, TimingCat), TimingStats>>,
    pub(crate) timing_scope_exclusive_global: RefCell<HashMap<(VOID, TimingCat), TimingStats>>,
    pub(crate) timing_window_exclusive: RefCell<HashMap<TimingCat, TimingStats>>,
    pub dependencies: RefCell<ImHashMap<Span, HashSet<VOID>>>,
    pub incomplete: RefCell<ImHashSet<VOID>>,
    /// Per returning scope: the places the alias walk from its `_0` visited
    /// (interp_return), to find which operands its return value aliases.
    /// (visited places, static places reached).
    pub return_walks: RefCell<HashMap<VOID, (ImHashSet<(Place, VOID)>, ImHashSet<(Place, VOID)>)>>,
    /// The operands (by index) the call being returned from aliases through
    /// its return value - set by interp_static_call / interp_virtual_call,
    /// consumed by interp_direct_call when it assigns the destination.
    /// A stack: one frame per direct/indirect call being dispatched, so
    /// nested calls (and a virtual call's other candidates) don't mix theirs.
    pub pending_return_aliases: RefCell<Vec<Vec<crate::write_log::ReturnAlias>>>,
    pub wtos_merge_conflicts: RefCell<ImHashSet<VOID>>,
    pub refs_merge_conflicts: RefCell<ImHashSet<VOID>>,

    /// Result, the callee's epoch, and what the call wrote into memory its
    /// caller can see (replayed on a hit - see write_log).
    /// A hit also needs the statics the call read unchanged (the log's
    /// `reads`, write_log::reads_fresh).
    pub exact_memo: RefCell<HashMap<MemoKey, (Option<Constraints>, u64, SharedLog)>>,

    pub virtual_call_memo:
        RefCell<HashMap<VirtualCallKey, (Option<Constraints>, Vec<(VOID, u64)>, SharedLog)>>,

    pub scope_epoch: RefCell<HashMap<BaseScopeId, u64>>,
    pub scope_exact_memo_count: RefCell<HashMap<BaseScopeId, u32>>,
    pub scope_summaries_count: RefCell<HashMap<BaseScopeId, u32>>,
    body_cache: RefCell<HashMap<Instance, Rc<Body>>>,
    /// Memory diagnostics (VERIFOPT_CACHE_SIZES): one entry per active
    /// candidate simulation (simulate_static_calls), outermost first - the
    /// dispatching function, candidate index, candidate count, and the
    /// merged context's (`acc`'s) written scopes and their size in nodes.
    sim_levels: RefCell<Vec<(String, usize, usize, usize, usize)>>,
    wto_templates: RefCell<HashMap<Instance, BBDeps>>,
    //pub param_summaries: RefCell<HashMap<VOID, ParamSummary>>,
    pub summary_build_taint_stack: RefCell<Vec<bool>>,
    pub building_summaries: RefCell<HashSet<VOID>>,
}

thread_local! {
    static LIFT_TRAITOBJTYS_CACHE: RefCell<HashMap<(usize, u64), (Weak<IndexSet<Constraint>>, Constraints)>> = RefCell::new(HashMap::new());
}

#[derive(Clone, PartialEq)]
pub enum TagPlan {
    Poisoned,
    Tagged(
        Vec<(
            usize,               /* bb */
            usize,               /* stmt */
            DefId,               /* impl */
            Option<GenericArgs>, /* concrete Self, when resolvable */
        )>,
    ),
}

impl TagPlan {
    fn join(&mut self, o: &TagPlan) {
        match (&*self, o) {
            (TagPlan::Poisoned, _) | (_, TagPlan::Poisoned) => *self = TagPlan::Poisoned,

            (TagPlan::Tagged(a), TagPlan::Tagged(b)) => {
                let mut by_site: HashMap<(usize, usize), (DefId, Option<GenericArgs>)> =
                    HashMap::new();

                for (bb, stmt, did, genargs) in a.iter().chain(b.iter()) {
                    match by_site.entry((*bb, *stmt)) {
                        Entry::Vacant(e) => {
                            e.insert((*did, genargs.clone()));
                        }
                        Entry::Occupied(e) => {
                            // Poison if either the target impl or its
                            // resolved Self type disagrees across what's
                            // being joined - a single tag site only makes
                            // sense if it always resolves to the exact
                            // same concrete candidate.
                            if e.get().0 != *did || e.get().1 != *genargs {
                                *self = TagPlan::Poisoned;
                                return;
                            }
                        }
                    }
                }

                let mut out: Vec<(usize, usize, DefId, Option<GenericArgs>)> = by_site
                    .into_iter()
                    .map(|((bb, stmt), (impl_did, genargs))| (bb, stmt, impl_did, genargs))
                    .collect();
                out.sort_by_key(|(bb, stmt, _, _)| (*bb, *stmt));

                *self = TagPlan::Tagged(out);
            }
        }
    }
}

impl<'a> InterpPass<'a> {
    pub fn new(
        sigstore: &'a SigStore,
        tstore: &'a TraitStore,
        call_context_k: usize,
    ) -> InterpPass<'a> {
        Self {
            sigstore,
            tstore,
            converter: RvalConverter::new(tstore),
            call_context_k,
            dispatch_targets: ImHashMap::new().into(),
            dispatch_cha: ImHashMap::new().into(),
            dispatch_tags: ImHashMap::new().into(),
            wq: HashMap::new().into(),
            summaries: HashMap::new().into(),
            in_queue: HashSet::new().into(),
            key_stack: Vec::new().into(),
            rec_depth: 0.into(),
            //call_count: 0.into(),
            bb_visit_count: 0.into(),
            run_start: std::time::Instant::now(),
            main_ctxt_ptr: None.into(),
            self_time_child_accum: Vec::new().into(),
            scope_self_time: HashMap::new().into(),
            scope_self_time_global: HashMap::new().into(),
            timing_global: HashMap::new().into(),
            timing_scope: HashMap::new().into(),
            timing_scope_global: HashMap::new().into(),
            timing_window: HashMap::new().into(),
            timing_child_stack: Vec::new().into(),
            timing_global_exclusive: HashMap::new().into(),
            timing_scope_exclusive: HashMap::new().into(),
            timing_scope_exclusive_global: HashMap::new().into(),
            timing_window_exclusive: HashMap::new().into(),
            dependencies: ImHashMap::new().into(),
            incomplete: ImHashSet::new().into(),
            return_walks: HashMap::new().into(),
            pending_return_aliases: Vec::new().into(),
            wtos_merge_conflicts: ImHashSet::new().into(),
            refs_merge_conflicts: ImHashSet::new().into(),
            exact_memo: HashMap::new().into(),
            virtual_call_memo: HashMap::new().into(),
            scope_epoch: HashMap::new().into(),
            scope_exact_memo_count: HashMap::new().into(),
            scope_summaries_count: HashMap::new().into(),
            body_cache: HashMap::new().into(),
            sim_levels: RefCell::new(Vec::new()),
            wto_templates: HashMap::new().into(),
            //param_summaries: HashMap::new().into(),
            summary_build_taint_stack: Vec::new().into(),
            building_summaries: HashSet::new().into(),
        }
    }

    fn check_call_stack(&self, call_stack: &Vec<VOID>, scope: &VOID) {
        let last_item = call_stack[call_stack.len() - 1].clone();
        if *scope != last_item {
            log_call_stack(call_stack);
            panic!("call stack out of sorts (scope does not match last call_stack elem)");
        }
    }

    fn assert_stacks_synced(&self, call_stack: &[VOID], where_: &str) {
        let key_len = self.key_stack.borrow().len();
        if call_stack.len() != key_len {
            let names: Vec<String> = call_stack
                .iter()
                .map(|v| format!("{:?}", v.0.name()))
                .collect();
            let keys: Vec<String> =
                self.key_stack.borrow().iter().map(|k| format!("{:?}", k.0.0.name())).collect();
            panic!(
                "STACK DESYNC at {}: call_stack.len()={} key_stack.len()={}\ncall_stack contents:\n{:#?}\nkey_stack contents:\n{:#?}\nlast stack operations (op, scope, call_stack.len, key_stack.len):\n{}",
                where_,
                call_stack.len(),
                key_len,
                names,
                keys,
                recent_stack_ops()
            );
        }
    }

    /// A memo hit's side effects: the writes the memoized call made through
    /// its operands (and to statics), made again - through this call's
    /// operands, so the usual alias resolution routes them to this call's
    /// places (see write_log). Always merges.
    fn replay_writes(
        &self,
        ctxt: &mut Context,
        caller_scope: &VOID,
        args: &[Operand],
        writes: &crate::write_log::WriteLog,
    ) {
        crate::write_log::replay_reads(writes);
        for (idx, values) in &writes.ports {
            if let Some(Operand::Copy(place) | Operand::Move(place)) = args.get(*idx) {
                for value in values {
                    ctxt.cstore.scoped_update(
                        caller_scope,
                        MapKey::Var(place.clone()),
                        Box::new(MapValue::Constraints(value.clone())),
                        Some(self),
                    );
                }
            }
        }
        for (def, value) in writes.statics.iter().flat_map(|(d, vs)| vs.iter().map(move |v| (d, v))) {
            let (place, scope) = self.static_place(ctxt, *def);
            ctxt.cstore.scoped_update_local(
                &scope,
                MapKey::Var(place),
                Box::new(MapValue::Constraints(value.clone())),
                Some(self),
            );
        }
    }

    /// Adds to the innermost dispatching call's pending return aliases.
    fn add_pending_return_aliases(&self, operands: impl IntoIterator<Item = crate::write_log::ReturnAlias>) {
        if let Some(top) = self.pending_return_aliases.borrow_mut().last_mut() {
            top.extend(operands);
        }
    }

    /// The operands (by index) whose caller-side places the alias walk from
    /// `callee`'s return value passed through (interp_return): the return
    /// value points into what they point to.
    fn return_alias_operands(&self, callee: &VOID, caller: &VOID, args: &[Operand]) -> Vec<crate::write_log::ReturnAlias> {
        use crate::write_log::ReturnAlias;
        let Some((walk, statics)) = self.return_walks.borrow_mut().remove(callee) else { return Vec::new() };
        operand_places(args)
            .into_iter()
            .filter(|(_, place)| walk.contains(&(place.clone(), caller.clone())))
            .map(|(i, _)| ReturnAlias::Operand(i))
            .chain(statics.into_iter().map(|(p, s)| ReturnAlias::Static(p, s)))
            .collect()
    }

    /// A call's destination aliases the operands its return value does
    /// (pending_return_aliases): e.g. `p = cell.get()` points where `cell`
    /// does, so a write through `p` reaches the Cell's owner.
    fn add_return_aliases(
        &self,
        ctxt: &mut Context,
        cur_scope: &VOID,
        args: &[Operand],
        destination: &Place,
        aliases: Vec<crate::write_log::ReturnAlias>,
    ) {
        use crate::write_log::ReturnAlias;
        for alias in aliases {
            let to = match alias {
                ReturnAlias::Operand(idx) => match args.get(idx) {
                    Some(Operand::Copy(place) | Operand::Move(place)) => (place.clone(), cur_scope.clone()),
                    _ => continue,
                },
                ReturnAlias::Static(place, scope) => (place, scope),
            };
            ctxt.cstore.add_ref((destination.clone(), cur_scope.clone()), to, Mutability::Mut);
        }
    }

    fn prepare_call(&self, call_stack: &mut Vec<VOID>, key: &SummaryKey) {
        call_stack.push(key.0.clone());
        self.key_stack.borrow_mut().push(key.clone());
        note_stack_op("call", &key.0, call_stack.len(), self.key_stack.borrow().len());
        self.assert_stacks_synced(call_stack, "prepare_call");
    }

    /// Error-path unwinding: truncates both stacks back to `len`, their
    /// length before the failed call's prepare_call. Not a single
    /// prepare_return: a callee may already have popped its own frame before
    /// failing (finish_frame pops first, then can still fail - RecurseLimit
    /// from a queued re-interpretation, or the re-visit), so popping again
    /// removed the *caller's* frame - from a cloned call_stack in candidate
    /// simulation, but always from the shared key_stack (STACK DESYNC).
    fn unwind_stacks_to(&self, call_stack: &mut Vec<VOID>, len: usize, where_: &str) {
        call_stack.truncate(len);
        self.key_stack.borrow_mut().truncate(len);
        self.assert_stacks_synced(call_stack, where_);
    }

    fn prepare_return(&self, call_stack: &mut Vec<VOID>) -> Option<VOID> {
        self.key_stack.borrow_mut().pop();
        let popped = call_stack.pop();
        if let Some(p) = &popped {
            note_stack_op("return", p, call_stack.len(), self.key_stack.borrow().len());
        }
        self.assert_stacks_synced(call_stack, "prepare_return");
        popped
    }

    pub fn run(
        &self,
        ctxt: &mut Context,
        start_instance: Instance,
    ) -> Result<Option<Constraints>, Error> {
        *self.main_ctxt_ptr.borrow_mut() = Some(ctxt as *const Context as usize);

        let start_scope = (start_instance, GenericArgs(vec![]), Vec::new());
        let mut call_stack = vec![start_scope.clone()];

        self.key_stack
            .borrow_mut()
            .push((start_scope.clone(), ArgSet::new(&[])));

        let entry_fn_cstore = ConstraintStore::new();
        ctxt.set_cstore_scope(&start_scope, entry_fn_cstore, None);

        let result = self.visit_body(
            ctxt,
            &mut call_stack,
            &start_scope,
            &self.get_body(&start_scope),
        );

        debug!(
            "TOTAL WALL CLOCK bb_visit={} elapsed_ms={:.3}",
            *self.bb_visit_count.borrow(),
            self.run_start.elapsed().as_secs_f64() * 1000.0
        );
        //crate::constraints::dump_flatten_all_cache_stats("FINAL");
        self.dump_self_time_report("FINAL", &self.scope_self_time_global.borrow());
        self.dump_timing_report("FINAL GLOBAL", &self.timing_global.borrow());
        self.dump_timing_by_scope_report("FINAL", &self.timing_scope_global.borrow());
        self.dump_timing_report(
            "FINAL GLOBAL EXCLUSIVE",
            &self.timing_global_exclusive.borrow(),
        );
        self.dump_timing_by_scope_exclusive_report(
            "FINAL",
            &self.timing_scope_exclusive_global.borrow(),
        );

        result
    }

    /// The scope's MIR body, converted once per instance: `Instance::body()`
    /// builds a fresh `Body` from rustc's MIR on every call (millions of
    /// times in a ripgrep run), and a function's body doesn't depend on the
    /// call context.
    fn get_body(&self, cur_scope: &VOID) -> Rc<Body> {
        self.body_cache
            .borrow_mut()
            .entry(cur_scope.0)
            .or_insert_with(|| Rc::new(cur_scope.0.body().unwrap()))
            .clone()
    }

    /// A fresh block ordering for `cur_scope`: per-instance template (see
    /// BBDeps::new), cloned so each scope consumes its own `ordering`; the
    /// template's blocks are shared through their Rc instead of being copied
    /// again for every call context.
    fn new_bb_deps(&self, cur_scope: &VOID, body: &Body) -> BBDeps {
        self.wto_templates
            .borrow_mut()
            .entry(cur_scope.0)
            .or_insert_with(|| BBDeps::new(body))
            .clone()
    }

    fn visit_body(
        &self,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        body: &Body,
    ) -> Result<Option<Constraints>, Error> {
        self.self_time_child_accum
            .borrow_mut()
            .push(std::time::Duration::ZERO);
        let _self_time_guard = SelfTimeGuard {
            pass: self,
            scope: cur_scope.clone(),
            start: std::time::Instant::now(),
        };

        debug!(
            "###### INTERP-ING NEW BODY for func {:?}",
            cur_scope.0.name()
        );
        if scope_matches_trace_filter(cur_scope) {
            log::warn!("[verifopt trace] enter {:?} (called from {:?})", cur_scope.0.name(), call_stack.last().map(|s| s.0.name()));
        }
        log_call_stack(call_stack);

        self.check_call_stack(call_stack, cur_scope);

        let mut bb_deps;
        if let Some(mem_bb_deps) = ctxt.get_wto(cur_scope) {
            bb_deps = mem_bb_deps.clone();
            if bb_deps.has_ret && bb_deps.ordering.is_empty() {
                if self.wtos_merge_conflicts.borrow().contains(cur_scope) {
                    debug!(
                        "POTENTIAL SOUNDNESS GAP: re-entering scope {:?} with an already-empty \
                         wtos ordering, but this scope's wtos entry survived a union() collision \
                         (see wtos_merge_conflicts) - the discarded side's own body-level \
                         derivation may never get re-run against the now-merged cmap",
                        cur_scope.0.name()
                    );
                }
            }
            // Gate on whether there's anything left to walk, not on
            // whether the function has a Return terminator. Block-level
            // filtering already excludes Unreachable/Resume/Abort blocks
            // (see BBDeps::new) — a function whose *entire* body is
            // panic/abort paths correctly ends up with an empty ordering
            // here. But a function that does substantial real work and
            // only diverges at the very end (e.g. an embedded kernel's
            // `main`, which never returns because it ends in an infinite
            // scheduler loop or a `-> !` call) still has real blocks left
            // in `ordering` and should still be walked — gating on
            // `has_ret` alone was skipping those entirely.
            if bb_deps.ordering.is_empty() {
                return self.finish_frame(ctxt, call_stack, cur_scope, None);
            }
        } else {
            bb_deps = self.new_bb_deps(cur_scope, body);
            if bb_deps.ordering.is_empty() {
                return self.finish_frame(ctxt, call_stack, cur_scope, None);
            }
            ctxt.set_wto(cur_scope, &bb_deps);
        }

        let mut last_res = None;
        let num_bbs = bb_deps.ordering.len();
        let mut saw_return = false;

        loop {
            if bb_deps.ordering.is_empty() {
                break;
            }

            {
                let mut n = self.bb_visit_count.borrow_mut();
                *n += 1;
                //if *n % 200 == 0 {
                //    debug!(
                //        "\nRSS at bb visit {} rss_kb={}\n",
                //        *n,
                //        Self::current_rss_kb().unwrap_or(0)
                //    );
                //}
                /*
                if *n % 10 == 0 {
                    let is_main =
                        Some(ctxt as *const Context as usize) == *self.main_ctxt_ptr.borrow();
                    debug!(
                        "\nCMAP SIZE at bb visit {} is_main={} cmap_len={}\n",
                        *n,
                        is_main,
                        ctxt.cstore.cmap.len()
                    );
                    debug!(
                        "\nMEMO SIZES at bb visit {} exact_memo={} summaries={} param_summaries={}\n",
                        *n,
                        self.exact_memo.borrow().len(),
                        self.summaries.borrow().len(),
                        self.param_summaries.borrow().len()
                    );
                    let mut max_vars = 0usize;
                    let mut max_vars_scope = String::new();
                    let mut sum_vars = 0usize;
                    let mut num_scopes_with_vars = 0usize;
                    for (key, val) in ctxt.cstore.cmap.iter() {
                        if let (MapKey::ScopeId(scope), MapValue::Store(inner, _)) =
                            (key, val.as_ref())
                        {
                            let vc = inner.cmap.len();
                            sum_vars += vc;
                            num_scopes_with_vars += 1;
                            if vc > max_vars {
                                max_vars = vc;
                                max_vars_scope = format!("{:?}", scope.0.name());
                            }
                        }
                    }
                    debug!(
                        "\nVAR COUNTS at bb visit {} is_main={} num_scopes={} sum_vars={} max_vars={} max_vars_scope={}\n",
                        *n, is_main, num_scopes_with_vars, sum_vars, max_vars, max_vars_scope
                    );
                }
                */
                if *n % 200 == 0 {
                    debug!(
                        "TOTAL WALL CLOCK bb_visit={} elapsed_ms={:.3}",
                        *n,
                        self.run_start.elapsed().as_secs_f64() * 1000.0
                    );
                    //crate::constraints::dump_flatten_all_cache_stats(&format!("bb visit {}", *n));
                    //self.log_bb_cache_sizes(*n, cur_scope, ctxt, bb_deps.ordering.len());
                }
                // Memory diagnostics for long runs: every N bb visits (env
                // VERIFOPT_CACHE_SIZES=N), the caches' sizes at warn level.
                if let Some(every) = cache_sizes_every() && *n % every == 0
                {
                    self.log_memory_diag(*n, ctxt);
                }
                if *n % 200 == 0 {
                    self.dump_self_time_report(
                        &format!("bb visit {}", *n),
                        &self.scope_self_time.borrow(),
                    );
                    self.scope_self_time.borrow_mut().clear();
                    self.dump_timing_by_scope_report(
                        &format!("bb visit {}", *n),
                        &self.timing_scope.borrow(),
                    );
                    self.timing_scope.borrow_mut().clear();
                    self.dump_timing_report(
                        &format!("window ending bb visit {}", *n),
                        &self.timing_window.borrow(),
                    );
                    self.timing_window.borrow_mut().clear();
                    self.dump_timing_by_scope_exclusive_report(
                        &format!("bb visit {}", *n),
                        &self.timing_scope_exclusive.borrow(),
                    );
                    self.timing_scope_exclusive.borrow_mut().clear();
                    self.dump_timing_report(
                        &format!("window ending bb visit {} EXCLUSIVE", *n),
                        &self.timing_window_exclusive.borrow(),
                    );
                    self.timing_window_exclusive.borrow_mut().clear();
                }
            }

            let bb = bb_deps.ordering.pop_front().unwrap();

            let data = body.blocks.get(bb).unwrap();
            if matches!(data.terminator.kind, TerminatorKind::Return) {
                saw_return = true;
            }

            last_res = self.visit_basic_block(
                ctxt,
                call_stack,
                cur_scope,
                body.locals(),
                &mut bb_deps,
                num_bbs,
                bb,
                data,
            )?;
        }

        if !saw_return {
            self.finish_frame(ctxt, call_stack, cur_scope, None)
        } else {
            Ok(last_res)
        }
    }

    fn visit_basic_block(
        &self,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        bb_deps: &mut BBDeps,
        num_bbs: usize,
        bb: usize,
        data: &BasicBlock,
    ) -> Result<Option<Constraints>, Error> {
        debug!(
            "# visiting BASICBLOCK {:?} ({:?}/{:?}) for {:?}",
            bb,
            bb + 1,
            num_bbs,
            cur_scope.0.name()
        );

        ctxt.bb_written_places.remove(cur_scope);

        self.check_call_stack(call_stack, cur_scope);

        let num_stmts = data.statements.len();
        let _timing_guard = self.timing_span(TimingCat::BbStatements, cur_scope);
        for (i, stmt) in data.statements.iter().enumerate() {
            debug!(
                "\n\n# visiting STATEMENT {:?} ({:?}/{:?}) in BB{:?} for {:?}\n\nSPAN: {:?}",
                i,
                i + 1,
                num_stmts,
                bb,
                cur_scope.0.name(),
                stmt.span,
            );
            self.visit_statement(ctxt, cur_scope, local_decls, stmt, bb, i);
        }
        drop(_timing_guard);

        debug!(
            "\n\n# visiting TERMINATOR in BB{:?} for {:?}\n\nSPAN: {:?}",
            bb,
            cur_scope.0.name(),
            &data.terminator.span,
        );

        let _timing_guard = self.timing_span(TimingCat::BbTerminator, cur_scope);
        let res = self.visit_terminator(
            ctxt,
            call_stack,
            cur_scope,
            local_decls,
            bb_deps,
            bb,
            &data.terminator,
        )?;
        drop(_timing_guard);

        bb_deps.mark_visited(bb, cur_scope);

        Ok(res)
    }

    fn contains_dyn(&self, ty: &Ty) -> Option<Vec<TraitObjTy>> {
        match ty.kind() {
            TyKind::RigidTy(rigidty) => match rigidty {
                RigidTy::Dynamic(trait_vec, _) => {
                    let mut desttys = Vec::new();
                    for trait_ in trait_vec {
                        if let Some(toty) = TraitObjTy::new_from_bound_existential(&trait_) {
                            unique_push(&mut desttys, toty);
                        }
                    }
                    return Some(desttys);
                }
                RigidTy::Adt(_def, genargs) => {
                    for genarg in genargs.0 {
                        match genarg {
                            GenericArgKind::Type(ty) => {
                                let maybe_dyn = self.contains_dyn(&ty);
                                if maybe_dyn.is_some() {
                                    return maybe_dyn;
                                }
                            }
                            _ => continue,
                        }
                    }
                }
                RigidTy::Tuple(ty_vec) => {
                    for ty in ty_vec {
                        let maybe_dyn = self.contains_dyn(&ty);
                        if maybe_dyn.is_some() {
                            return maybe_dyn;
                        }
                    }
                }
                RigidTy::Array(ty, _)
                | RigidTy::Slice(ty)
                | RigidTy::Pat(ty, _)
                | RigidTy::RawPtr(ty, _)
                | RigidTy::Ref(_, ty, _) => {
                    let maybe_dyn = self.contains_dyn(&ty);
                    if maybe_dyn.is_some() {
                        return maybe_dyn;
                    }
                }
                _ => {}
            },
            _ => {}
        }

        None
    }

    fn static_rvalue(&self, rval: &Rvalue) -> Option<DefId> {
        let op = match rval {
            Rvalue::Use(op) | Rvalue::Cast(_, op, _) => op,
            _ => return None,
        };

        if let Operand::Constant(co) = op
            && let ConstantKind::Allocated(alloc) = co.const_.kind()
        {
            for (_off, Prov(aid)) in alloc.provenance.ptrs.iter() {
                if let GlobalAlloc::Static(StaticDef(defid)) = GlobalAlloc::from(*aid) {
                    return Some(defid);
                }
            }
        }

        None
    }

    /// Static `def`'s place (`_0` of its synthetic scope, see
    /// constraints::static_scope), holding its initial value if nothing has
    /// been stored there yet.
    fn static_place(&self, ctxt: &mut Context, def: DefId) -> (Place, VOID) {
        let scope = crate::constraints::static_scope(def);
        let place = Place { local: 0, projection: vec![] };
        let key = MapKey::Var(place.clone());
        if !ctxt.cstore.has_own_entry(&scope, &key) {
            // Its initializer, lifted to its type's trait objects like any
            // assigned value (`static mut S: &dyn T = &X` holds a `dyn T`).
            let init = self.static_get_constraints(ctxt, def);
            let ty = StaticDef(def).ty();
            let init = self.lift_traitobjtys(&self.contains_dyn(&ty), init, &scope);
            ctxt.cstore.scoped_update_local(&scope, key, Box::new(MapValue::Constraints(init)), Some(self));
        }
        (place, scope)
    }

    fn static_get_constraints(&self, ctxt: &mut Context, defid: DefId) -> Constraints {
        if let Some(cs) = ctxt.get_static(defid) {
            return cs;
        }

        let sdef = StaticDef(defid);
        let ty = sdef.ty();

        // `extern { static X: T; }` has no initializer: its value comes from
        // the linker (e.g. tock's `_sstorage`/`_estack` linker-script symbols)
        // or from foreign code, which may also write to it. Asking rustc to
        // evaluate it doesn't return Err - it ICEs (`can't type-check body
        // of ...`, via mir_for_ctfe), so the Err fallback below never gets a
        // chance. Its contents are unknown to Rust anyway: use the type-based
        // fallback, and cache it like any other static.
        if rustc_public::CrateItem(defid).is_foreign_item() {
            let (_, c) = self
                .converter
                .convert_ty(&Location::unknown(), &ty, None, Some(self));
            let constraints = Constraints::from(c);
            ctxt.set_static(defid, constraints.clone());
            return constraints;
        }

        let alloc = match sdef.eval_initializer() {
            Ok(a) => a,
            Err(_) => {
                let (_, c) = self
                    .converter
                    .convert_ty(&Location::unknown(), &ty, None, Some(self));
                return Constraints::from(c);
            }
        };

        let mut seen = Vec::new();
        // Its initializer is its value only if it can't change. A `static
        // mut` (or an interior-mutable static) starts as any value of its
        // type, and its place collects every write the analysis sees
        // (static_place): starting from the initializer instead is only
        // sound if no write is missed - and some still are (through pointers
        // kept in fields), which left e.g. tock's process table `[None; N]`
        // forever, so the closures `process_until` calls on its processes
        // were never interpreted.
        let frozen =
            matches!(alloc.mutability, Mutability::Not) && self.converter.is_frozen(&ty, &mut seen);

        let constraints = if frozen {
            self.converter
                .convert_static_const(&Location::unknown(), &ty, &alloc, None, Some(self))
        } else {
            let (_, c) = self
                .converter
                .convert_ty(&Location::unknown(), &ty, None, Some(self));
            Constraints::from(c)
        };

        ctxt.set_static(defid, constraints.clone());
        constraints
    }

    /// If any of the constraints contain a type that implements one of the Traits listed in
    /// `maybe_trait_destty`, copy those types and put them into the TraitObjConstraint field
    /// of the constraint, leaving the RunningConstraint field unchanged
    fn lift_traitobjtys(
        &self,
        maybe_trait_destty: &Option<Vec<TraitObjTy>>,
        old_constraints: Constraints,
        cur_scope: &VOID,
    ) -> Constraints {
        let extra_key = {
            let _g = self.timing_span(TimingCat::LiftTraitobjtysHashVal, cur_scope);
            hash_val(maybe_trait_destty)
        };
        LIFT_TRAITOBJTYS_CACHE.with(|cache| {
            memoize_by_rc(cache, &old_constraints, extra_key, || {
                let _g = self.timing_span(TimingCat::LiftTraitobjtysUncached, cur_scope);
                self.lift_traitobjtys_uncached(maybe_trait_destty, &old_constraints, cur_scope)
            })
        })
    }

    fn lift_traitobjtys_uncached(
        &self,
        maybe_trait_destty: &Option<Vec<TraitObjTy>>,
        old_constraints: &Constraints,
        cur_scope: &VOID,
    ) -> Constraints {
        let mut constraints = Constraints::new();
        for constraint in old_constraints.inner.iter() {
            let constraint = constraint.clone();
            let _g = self.timing_span(TimingCat::LiftTraitobjtysUncachedGetTraitobj, cur_scope);
            let to = self.converter.get_traitobj(maybe_trait_destty, &constraint);
            drop(_g);

            match to {
                toc @ Some(_) => match constraint {
                    Constraint {
                        toc: None,
                        cfc,
                        prov,
                    } => {
                        let _g =
                            self.timing_span(TimingCat::LiftTraitobjtysUncachedPush1, cur_scope);
                        constraints.push(Constraint::new(toc, cfc).with_prov(prov));
                    }
                    Constraint {
                        toc: Some(ref existing_toc),
                        cfc: ref _cfc,
                        prov: _,
                    } => {
                        if *existing_toc != toc.unwrap() {
                            todo!("update existing TOC");
                        } else {
                            let _g = self
                                .timing_span(TimingCat::LiftTraitobjtysUncachedPush2, cur_scope);
                            constraints.push(constraint);
                        }
                    }
                },
                None => {
                    let _g = self.timing_span(TimingCat::LiftTraitobjtysUncachedPush3, cur_scope);
                    constraints.push(constraint);
                }
            }
        }

        constraints
    }

    fn visit_statement(
        &self,
        ctxt: &mut Context,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        stmt: &Statement,
        bb: usize,
        stmt_idx: usize,
    ) {
        match &stmt.kind {
            StatementKind::Assign(place, rvalue) => {
                debug!("start assignment!\nplace: {:?}\nrval: {:?}", place, rvalue);
                log_scope(cur_scope);

                let dest_ty = place.ty(local_decls).unwrap();
                let _timing_guard = self.timing_span(TimingCat::StmtContainsDyn, cur_scope);
                let maybe_trait_destty = self.contains_dyn(&dest_ty);
                drop(_timing_guard);

                let _timing_guard = self.timing_span(TimingCat::StmtNewConstraints, cur_scope);
                let constraints = if let Rvalue::Ref(_region, bk, to) = rvalue.clone() {
                    let _timing_guard =
                        self.timing_span(TimingCat::StmtNewConstraintsRef, cur_scope);
                    let to = match to.projection.as_slice() {
                        [ProjectionElem::Deref] => Place {
                            local: to.local,
                            projection: vec![],
                        },
                        _ => to.clone(),
                    };
                    ctxt.cstore.add_ref(
                        (place.clone(), cur_scope.clone()),
                        (to, cur_scope.clone()),
                        if matches!(bk, BorrowKind::Mut { .. }) {
                            Mutability::Mut
                        } else {
                            Mutability::Not
                        },
                    );
                    debug!("stmt: FROM REF (empty)");
                    Constraints::new()
                } else if place.projection.is_empty()
                    && let Some((to, mt)) = pointer_alias_source(rvalue, &dest_ty, local_decls)
                {
                    // A raw pointer to a place, or a pointer copied / cast
                    // from a pointer local: an alias edge, like a borrow, so
                    // writes through it reach the pointee (and reads see its
                    // current value) instead of a copy of it.
                    ctxt.cstore.add_ref(
                        (place.clone(), cur_scope.clone()),
                        (to, cur_scope.clone()),
                        mt,
                    );
                    debug!("stmt: FROM POINTER ALIAS (empty)");
                    Constraints::new()
                } else if place.projection.is_empty()
                    && let Some(def) = static_pointer_rvalue(rvalue, &dest_ty)
                {
                    // A pointer to a static: an alias edge to its place.
                    let to = self.static_place(ctxt, def);
                    ctxt.cstore.add_ref((place.clone(), cur_scope.clone()), to, Mutability::Mut);
                    debug!("stmt: FROM STATIC POINTER (empty)");
                    Constraints::new()
                } else if let Some(defid) = self.static_rvalue(rvalue) {
                    let _timing_guard =
                        self.timing_span(TimingCat::StmtNewConstraintsStatic, cur_scope);
                    let c = self.static_get_constraints(ctxt, defid);
                    debug!(
                        "stmt: FROM STATIC, scope={:?} local={} converted_disjuncts={}",
                        cur_scope.0.name(),
                        place.local,
                        crate::constraints::constraints_size(&c)
                    );
                    c
                } else {
                    let _timing_guard =
                        self.timing_span(TimingCat::StmtNewConstraintsFromConvert, cur_scope);
                    let c = self.converter.convert(
                        ctxt,
                        &Location::new_at(cur_scope.0.def.def_id(), bb, stmt_idx),
                        local_decls,
                        cur_scope,
                        &dest_ty,
                        rvalue,
                        Some(self),
                    );
                    debug!(
                        "stmt: FROM CONVERTER, scope={:?} local={} converted_disjuncts={}",
                        cur_scope.0.name(),
                        place.local,
                        crate::constraints::constraints_size(&c)
                    );
                    c
                };
                drop(_timing_guard);
                debug!(
                    "stmt: GENERAL scope={:?} local={} disjuncts={}",
                    cur_scope.0.name(),
                    place.local,
                    crate::constraints::constraints_size(&constraints)
                );

                let _timing_guard = self.timing_span(TimingCat::StmtLiftTraitobj, cur_scope);
                let final_constraints =
                    self.lift_traitobjtys(&maybe_trait_destty, constraints.clone(), cur_scope);
                drop(_timing_guard);
                debug!(
                    "stmt: GENERAL post-lift scope={:?} local={} disjuncts={}",
                    cur_scope.0.name(),
                    place.local,
                    crate::constraints::constraints_size(&final_constraints)
                );
                //debug!("FINAL CONSTRAINTS: {:?}", final_constraints);

                let mut write_proj = place.projection.as_slice();
                while let [ProjectionElem::Deref, rest @ ..] = write_proj {
                    write_proj = rest;
                }

                if write_proj.is_empty() {
                    let _timing_guard = self.timing_span(TimingCat::StmtSetScoped, cur_scope);
                    ctxt.assign_constraints(cur_scope, place, place, final_constraints, Some(self));
                    drop(_timing_guard);
                } else {
                    let base = Place {
                        local: place.local,
                        projection: vec![],
                    };
                    let _timing_guard = self.timing_span(TimingCat::StmtCurConstraints, cur_scope);
                    let base_lookup =
                        ctxt.get_constraints(cur_scope, local_decls, &base, false, Some(self));
                    drop(_timing_guard);
                    match base_lookup {
                        Some(mut base_constraints) => {
                            debug!(
                                "stmt: OLD BASE scope={:?} local={} old_disjuncts={}",
                                cur_scope.0.name(),
                                base.local,
                                crate::constraints::constraints_size(&base_constraints)
                            );
                            let _timing_guard =
                                self.timing_span(TimingCat::StmtWriteFields, cur_scope);
                            base_constraints
                                .write_field(place.projection.clone(), final_constraints);
                            drop(_timing_guard);
                            debug!(
                                "stmt: OLD BASE post-field-write scope={:?} local={} old_disjuncts={}",
                                cur_scope.0.name(),
                                base.local,
                                crate::constraints::constraints_size(&base_constraints)
                            );
                            let _timing_guard =
                                self.timing_span(TimingCat::StmtSetScoped, cur_scope);
                            ctxt.assign_constraints(
                                cur_scope,
                                place,
                                &base,
                                base_constraints,
                                Some(self),
                            );
                            drop(_timing_guard);
                        }
                        None => {
                            debug!(
                                "stmt: FRESH BASE scope={:?} local={}",
                                cur_scope.0.name(),
                                base.local
                            );
                            let mut base_constraints = Constraints::new();
                            let _timing_guard =
                                self.timing_span(TimingCat::StmtWriteFields, cur_scope);
                            base_constraints
                                .write_field(place.projection.clone(), final_constraints);
                            drop(_timing_guard);
                            debug!(
                                "stmt: FRESH BASE post-field-write scope={:?} local={} fresh_disjuncts={}",
                                cur_scope.0.name(),
                                base.local,
                                crate::constraints::constraints_size(&base_constraints)
                            );
                            let _timing_guard =
                                self.timing_span(TimingCat::StmtSetScoped, cur_scope);
                            ctxt.assign_constraints(
                                cur_scope,
                                place,
                                &base,
                                base_constraints,
                                Some(self),
                            );
                            drop(_timing_guard);
                        }
                    }
                }
            }
            StatementKind::FakeRead(_, _)
            | StatementKind::StorageLive(_)
            | StatementKind::StorageDead(_) => {}
            StatementKind::Intrinsic(ndi) => match ndi {
                NonDivergingIntrinsic::Assume(_) => {}
                NonDivergingIntrinsic::CopyNonOverlapping(cno) => {
                    self.handle_copy_nonoverlapping(ctxt, cur_scope, local_decls, cno)
                }
            },
            _ => todo!("new statement kind: {:?}", &stmt.kind),
        }
    }

    fn handle_copy_nonoverlapping(
        &self,
        ctxt: &mut Context,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        cno: &CopyNonOverlapping,
    ) {
        let count =
            match self.get_operand_constraints(ctxt, cur_scope, local_decls, &cno.count, false) {
                Some(constraints) => self.get_usize(&constraints),
                None => None,
            };
        let src = self.get_operand_constraints(ctxt, cur_scope, local_decls, &cno.src, false);

        match &cno.dst {
            Operand::Copy(place) | Operand::Move(place) => {
                assert!(place.projection.is_empty());
                let dst_ty = place.ty(local_decls).unwrap();
                let maybe_trait_destty = self.contains_dyn(&dst_ty);

                if let Some(count) = count
                    && let Some(ref src) = src
                    && !src.inner.is_empty()
                {
                    let always_one = count.iter().all(|&n| n == 1);

                    let dst_disjuncts: Vec<Constraint> = src
                        .inner
                        .iter()
                        .filter_map(|c| match &c.cfc {
                            Some(RunningConstraint::Ptr(inner)) => {
                                let pointee = if always_one {
                                    (**inner).clone()
                                } else {
                                    Constraint::new(
                                        inner.toc.clone(),
                                        Some(RunningConstraint::List(inner.clone())),
                                    )
                                };
                                Some(Constraint::new(
                                    c.toc.clone(),
                                    Some(RunningConstraint::Ptr(Box::new(pointee))),
                                ))
                            }
                            _ => None,
                        })
                        .collect();

                    let dst_constraints = self.lift_traitobjtys(
                        &maybe_trait_destty,
                        Constraints::from_vec(dst_disjuncts),
                        cur_scope,
                    );
                    ctxt.set_scoped_constraints(cur_scope, &place, dst_constraints, Some(self));
                } else if let Some(src) = src
                    && !src.inner.is_empty()
                {
                    // no usable count -> always model as a sequence (List), never "exactly one"
                    let dst_disjuncts: Vec<Constraint> = src
                        .inner
                        .iter()
                        .filter_map(|c| match &c.cfc {
                            Some(RunningConstraint::Ptr(inner)) => {
                                let pointee = Constraint::new(
                                    inner.toc.clone(),
                                    Some(RunningConstraint::List(inner.clone())),
                                );
                                Some(Constraint::new(
                                    c.toc.clone(),
                                    Some(RunningConstraint::Ptr(Box::new(pointee))),
                                ))
                            }
                            _ => None,
                        })
                        .collect();

                    let dst_constraints = self.lift_traitobjtys(
                        &maybe_trait_destty,
                        Constraints::from_vec(dst_disjuncts),
                        cur_scope,
                    );
                    ctxt.set_scoped_constraints(cur_scope, &place, dst_constraints, Some(self));
                } else {
                    let (_, dst) = self.converter.convert_ty(
                        &Location::unknown(),
                        &place.ty(local_decls).unwrap(),
                        Some(cur_scope),
                        Some(self),
                    );
                    let dst_constraints = self.lift_traitobjtys(
                        &maybe_trait_destty,
                        Constraints::from(dst),
                        cur_scope,
                    );
                    ctxt.set_scoped_constraints(cur_scope, &place, dst_constraints, Some(self));
                }
            }
            _ => panic!("dst is not a place"),
        }
    }

    fn get_operand_constraints(
        &self,
        ctxt: &mut Context,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        op: &Operand,
        is_closure: bool,
    ) -> Option<Constraints> {
        match op {
            Operand::Copy(place) | Operand::Move(place) => {
                match ctxt.get_constraints(cur_scope, local_decls, &place, is_closure, Some(self)) {
                    Some(constraints) => Some(constraints),
                    None => None,
                }
            }
            Operand::Constant(const_op) => Some(self.converter.convert_const(
                &Location::unknown(),
                &const_op,
                Some(cur_scope),
                Some(self),
            )),
            _ => panic!("got runtime checks"),
        }
    }

    fn get_usize(&self, constraints: &Constraints) -> Option<Vec<usize>> {
        let mut nums = Vec::new();
        for constraint in constraints.inner.iter() {
            match constraint.cfc {
                Some(RunningConstraint::Scalar(Some(num))) => nums.push(num.try_into().unwrap()),
                _ => {}
            }
        }

        if nums.is_empty() { None } else { Some(nums) }
    }

    fn visit_terminator(
        &self,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        bb_deps: &mut BBDeps,
        bb: usize,
        term: &Terminator,
    ) -> Result<Option<Constraints>, Error> {
        debug!("TERM KIND: {:?}", &term.kind);
        match &term.kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => match func {
                Operand::Constant(co) => {
                    let _timing_guard = self.timing_span(TimingCat::TermDirectCall, cur_scope);
                    let r = self.interp_direct_call(
                        &term.span,
                        ctxt,
                        call_stack,
                        cur_scope,
                        local_decls,
                        bb,
                        co,
                        args,
                        destination,
                    );
                    drop(_timing_guard);
                    r
                }
                Operand::Copy(place) | Operand::Move(place) => {
                    let _timing_guard = self.timing_span(TimingCat::TermIndirectCall, cur_scope);
                    let r = self.interp_indirect_call(
                        &term.span,
                        ctxt,
                        call_stack,
                        cur_scope,
                        local_decls,
                        bb,
                        place,
                        args,
                        destination,
                    );
                    drop(_timing_guard);
                    r
                }
                _ => todo!("calling runtime check operand?"),
            },
            TerminatorKind::Return => {
                let _timing_guard = self.timing_span(TimingCat::TermReturn, cur_scope);
                let r = self.interp_return(ctxt, call_stack, cur_scope);
                drop(_timing_guard);
                r
            }
            TerminatorKind::SwitchInt { discr, targets } => {
                let _timing_guard = self.timing_span(TimingCat::TermSwitch, cur_scope);
                let r =
                    self.interp_switchint(ctxt, cur_scope, local_decls, bb, bb_deps, discr, targets);
                drop(_timing_guard);
                r
            }
            TerminatorKind::Assert { .. }
            | TerminatorKind::InlineAsm { .. }
            | TerminatorKind::Drop { .. }
            | TerminatorKind::Goto { .. } => Ok(None),
            _ => todo!("other term kind: {:?}", &term.kind),
        }
    }

    /// An indirect call has its own pending-return-aliases frame (so its
    /// callees' don't land on an enclosing call's destination); its own
    /// return aliases aren't used yet.
    fn interp_indirect_call(
        &self,
        term_span: &Span,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        bb: usize,
        place: &Place,
        args: &Vec<Operand>,
        destination: &Place,
    ) -> Result<Option<Constraints>, Error> {
        let depth = {
            let mut p = self.pending_return_aliases.borrow_mut();
            p.push(Vec::new());
            p.len() - 1
        };
        let result = self.interp_indirect_call_inner(
            term_span, ctxt, call_stack, cur_scope, local_decls, bb, place, args, destination,
        );
        self.pending_return_aliases.borrow_mut().truncate(depth);
        result
    }

    fn interp_indirect_call_inner(
        &self,
        term_span: &Span,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        bb: usize,
        place: &Place,
        args: &Vec<Operand>,
        destination: &Place,
    ) -> Result<Option<Constraints>, Error> {
        let dest_ty = place.ty(local_decls).unwrap();
        let maybe_trait_destty = self.contains_dyn(&dest_ty);

        let mut ret_constraints = Constraints::new();
        match ctxt.get_constraints(cur_scope, local_decls, place, false, Some(self)) {
            Some(constraints) => {
                // Disjuncts without a cfc - or no disjuncts at all, e.g. a fn
                // pointer lost through pointer arithmetic - call nothing below,
                // silently; the ones interp_constraint_as_fn sees record
                // themselves.
                let unknown = if constraints.inner.is_empty() {
                    Some("callee value empty")
                } else if constraints.inner.iter().any(|c| c.cfc.is_none()) {
                    Some("callee value partly unknown")
                } else {
                    None
                };
                if let Some(what) = unknown {
                    crate::assumptions::record(
                        crate::assumptions::Reason::FnPtrCall,
                        format!("indirect call in {} ({what})", cur_scope.0.name()),
                        Vec::new,
                    );
                }
                for constraint in constraints.inner.iter() {
                    let constraint = constraint.clone();
                    match constraint {
                        Constraint {
                            toc: _,
                            cfc: Some(cf),
                            prov: _,
                        } => match self.interp_constraint_as_fn(
                            term_span,
                            ctxt,
                            call_stack,
                            cur_scope,
                            local_decls,
                            bb,
                            &cf,
                            args,
                        ) {
                            Ok(Some(new_constraints)) => {
                                ret_constraints.append(new_constraints);
                            }
                            Ok(None) => {}
                            // e.g. RecurseLimit from a callee reached through
                            // a fn pointer (now interpreted, not just answered
                            // from its signature): propagated like a direct
                            // call's, for the frames above to unwind.
                            Err(e) => return Err(e),
                        },
                        _ => {}
                    }
                }
            }
            None => panic!("fnptr value not found in cmap"),
        }

        log_scope(cur_scope);
        let constraints = self.lift_traitobjtys(&maybe_trait_destty, ret_constraints, cur_scope);
        debug!("destination: {:?}", destination);
        debug!(
            "indirect call: DESTINATION scope={:?} local={} disjuncts={}",
            cur_scope.0.name(),
            destination.local,
            crate::constraints::constraints_size(&constraints)
        );
        //debug!("\n\n####### RETURNED VAL (CONSTRAINTS): {:?}", constraints);
        ctxt.assign_constraints(cur_scope, destination, destination, constraints, Some(self));

        Ok(None)
    }

    fn interp_constraint_as_fn(
        &self,
        term_span: &Span,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        bb: usize,
        constraint: &RunningConstraint,
        args: &Vec<Operand>,
    ) -> Result<Option<Constraints>, Error> {
        match constraint {
            RunningConstraint::FnDef(fndef, genargs) => self.interp_fn_def(
                term_span,
                ctxt,
                call_stack,
                cur_scope,
                local_decls,
                bb,
                *fndef,
                &genargs,
                args,
            ),
            RunningConstraint::FnPtr(sigval) => self.interp_fn_ptr(
                term_span,
                ctxt,
                call_stack,
                cur_scope,
                local_decls,
                sigval,
                args,
            ),
            RunningConstraint::Closure(cdef, genargs, _) => self.interp_closure(
                term_span,
                ctxt,
                call_stack,
                cur_scope,
                local_decls,
                *cdef,
                &genargs,
                args,
            ),
            RunningConstraint::Param(..) => {
                if let Some(tainted) = self.summary_build_taint_stack.borrow_mut().last_mut() {
                    *tainted = true;
                }
                Ok(None)
            }
            _ => {
                debug!(
                    "interp_constraint_as_fn: constraint isn't fn-like, falling back to imprecise (Ok(None)): {:?}",
                    constraint
                );
                if scope_matches_trace_filter(cur_scope) {
                    log::warn!(
                        "[verifopt trace] {:?}: indirect call on a non-fn value {}",
                        cur_scope.0.name(),
                        format!("{:?}", constraint).chars().take(400).collect::<String>()
                    );
                }
                crate::assumptions::record(
                    crate::assumptions::Reason::FnPtrCall,
                    format!("indirect call in {} (callee value not fn-like)", cur_scope.0.name()),
                    Vec::new,
                );
                if let Some(tainted) = self.summary_build_taint_stack.borrow_mut().last_mut() {
                    *tainted = true;
                }
                Ok(None)
            }
        }
    }

    fn interp_fn_ptr(
        &self,
        _term_span: &Span,
        _ctxt: &mut Context,
        _call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        _local_decls: &[LocalDecl],
        sigval: &SigVal,
        _args: &Vec<Operand>,
    ) -> Result<Option<Constraints>, Error> {
        // The target is unknown, so nothing it does - writes through its
        // arguments, dispatches on trait objects it's given - is modelled.
        crate::assumptions::record(
            crate::assumptions::Reason::FnPtrCall,
            format!("fn pointer called in {}", cur_scope.0.name()),
            || crate::assumptions::input_risks(&sigval.inputs),
        );
        //debug!(
        //    "interp_fn_ptr: falling back for sigval with output {:?}",
        //    sigval.output
        //);

        self.retty_fallback_from_sigval(sigval)
    }

    fn interp_closure(
        &self,
        term_span: &Span,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        cdef: ClosureDef,
        genargs: &GenericArgs,
        args: &Vec<Operand>,
    ) -> Result<Option<Constraints>, Error> {
        let closure_kind = self.get_closure_kind(&genargs);
        if let Some(_body) = cdef.body() {
            let instance = Instance::resolve_closure(cdef, &genargs, closure_kind).unwrap();
            let new_scope = find_active_recursive_scope(call_stack, &instance, genargs)
                .unwrap_or_else(|| {
                    (
                        instance,
                        genargs.clone(),
                        push_caller_context(cur_scope, term_span.clone(), self.call_context_k),
                    )
                });
            let body = self.get_body(&new_scope);

            let key = summary_key(
                self,
                new_scope.clone(),
                ctxt,
                term_span,
                cur_scope,
                &body,
                local_decls,
                args,
                true,
            );

            self.resolve_args(
                ctxt,
                term_span,
                cur_scope,
                &body,
                &new_scope,
                local_decls,
                args,
                &genargs,
                true,
            );
            let stack_len = call_stack.len();
            self.prepare_call(call_stack, &key);
            match self.visit_body(ctxt, call_stack, &new_scope, &body) {
                Ok(r) => Ok(r),
                Err(e) => {
                    self.unwind_stacks_to(call_stack, stack_len, "interp_closure error");
                    Err(e)
                }
            }
        } else {
            todo!("closure has no body");
        }
    }

    fn get_closure_kind(&self, genargs: &GenericArgs) -> ClosureKind {
        if genargs.0.is_empty() {
            panic!("no closure kind in genargs (empty)");
        }

        match genargs.0[0].expect_ty().kind() {
            // Rustc encodings for closure kinds
            TyKind::RigidTy(RigidTy::Int(IntTy::I8)) => ClosureKind::Fn,
            TyKind::RigidTy(RigidTy::Int(IntTy::I16)) => ClosureKind::FnMut,
            TyKind::RigidTy(RigidTy::Int(IntTy::I32)) => ClosureKind::FnOnce,
            other @ _ => panic!("first genarg is unexpected: {:?}", other),
        }
    }

    fn interp_direct_call(
        &self,
        term_span: &Span,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        bb: usize,
        co: &ConstOperand,
        args: &Vec<Operand>,
        destination: &Place,
    ) -> Result<Option<Constraints>, Error> {
        let dest_ty = destination.ty(local_decls).unwrap();
        let maybe_trait_destty = self.contains_dyn(&dest_ty);
        self.pending_return_aliases.borrow_mut().push(Vec::new());
        let ret_constraints = match co.const_.ty().kind() {
            TyKind::RigidTy(rigid_ty) => match rigid_ty {
                RigidTy::FnDef(fndef, genargs) => self.interp_fn_def(
                    term_span,
                    ctxt,
                    call_stack,
                    cur_scope,
                    local_decls,
                    bb,
                    fndef,
                    &genargs,
                    args,
                ),
                RigidTy::FnPtr(poly_sig) => {
                    let sigval = SigVal::new_from_poly(&poly_sig);
                    self.interp_fn_ptr(
                        term_span,
                        ctxt,
                        call_stack,
                        cur_scope,
                        local_decls,
                        &sigval,
                        args,
                    )
                }
                other @ _ => todo!("different RigidTy: {:?}", other),
            },
            kind @ _ => todo!("funccall const is another kind: {:?}", kind),
        };

        let return_aliases = self.pending_return_aliases.borrow_mut().pop().unwrap_or_default();

        log_scope(cur_scope);
        debug!("destination: {:?}", destination);

        match ret_constraints {
            Ok(Some(constraints_)) => {
                let constraints =
                    self.lift_traitobjtys(&maybe_trait_destty, constraints_.clone(), cur_scope);
                debug!(
                    "direct call: DESTINATION scope={:?} local={} disjuncts={}",
                    cur_scope.0.name(),
                    destination.local,
                    crate::constraints::constraints_size(&constraints)
                );

                ctxt.assign_constraints(
                    cur_scope,
                    destination,
                    destination,
                    constraints.clone(),
                    Some(self),
                );
                self.add_return_aliases(ctxt, cur_scope, args, destination, return_aliases);

                //debug!("\n\n####### RETURNED VAL (CONSTRAINTS): {:?}", constraints);

                return Ok(Some(constraints));
            }
            Ok(None) => Ok(None),
            err @ Err(_) => return err,
        }
    }

    fn interp_fn_def(
        &self,
        term_span: &Span,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        bb: usize,
        fndef: FnDef,
        genargs: &GenericArgs,
        args: &Vec<Operand>,
    ) -> Result<Option<Constraints>, Error> {
        // Only reachable from a body that was itself still generic; resolving
        // with free params would hand rustc an unnormalizable type (see
        // has_unresolved_param). The declared signature is the safe,
        // widened answer.
        if has_unresolved_param(genargs) {
            debug!(
                "interp_fn_def: {:?} called with non-monomorphic genargs {:?}, falling back to poly sig",
                fndef, genargs
            );
            crate::assumptions::record_call(crate::assumptions::Reason::NonMonomorphicArgs, &fndef);
            return self.retty_fallback_from_poly(fndef.fn_sig());
        }
        let instance = match Instance::resolve(fndef, genargs) {
            Ok(instance_) => instance_,
            Err(_) => {
                // Support instances we can't resolve without more info
                // (i.e. this is a declaration, not an implementation).
                // We likely got here b/c we interpreted a trait func w a default implementation
                // that calls a trait func without a default implementation
                //
                // The "more info" being:
                // - is this a trait method without an implementation?
                // - if so, who implements it? execute those implementations
                // This tends to be stuff that dynamic dispatch does for us anyway
                let _g_fallback =
                    self.timing_span(TimingCat::InterpFnDefVirtualFallback, cur_scope);
                let _timing_guard = self.timing_span(TimingCat::TermInterpVirtualCall, cur_scope);
                let r = self.interp_virtual_call(
                    term_span,
                    ctxt,
                    call_stack,
                    cur_scope,
                    local_decls,
                    bb,
                    &fndef,
                    &genargs,
                    args,
                );
                drop(_timing_guard);
                return r;
            }
        };

        let new_scope = find_active_recursive_scope(call_stack, &instance, genargs)
            .unwrap_or_else(|| {
                (
                    instance.clone(),
                    genargs.clone(),
                    push_caller_context(cur_scope, term_span.clone(), self.call_context_k),
                )
            });
        debug!(
            "--- CALLING {:?} -> resolved instance: kind={:?} name={:?}",
            fndef,
            instance.kind,
            instance.name()
        );
        log_scope(cur_scope);

        // Nested re-interpretation of queued recursive calls deeper than
        // MAX_DEPTH: answered from the signature, like a recursion cut. It
        // used to return Err(RecurseLimit), which nothing turns back into a
        // value - so it unwound to InterpPass::run, whose result is ignored,
        // and silently ended the whole analysis with partial results (e.g.
        // ripgrep through std's addr2line, reached from Backtrace's Display).
        if *self.rec_depth.borrow() > MAX_DEPTH {
            crate::assumptions::record_call(crate::assumptions::Reason::DepthLimit, &fndef);
            self.incomplete.borrow_mut().insert(cur_scope.clone());
            return self.retty_fallback_from_poly(fndef.fn_sig());
        }

        let stdlib_result = {
            let _g = self.timing_span(TimingCat::InterpFnDefStdlibStub, cur_scope);
            self.stdlib_stub(
                ctxt,
                cur_scope,
                term_span,
                local_decls,
                &fndef,
                genargs,
                args,
            )
        };
        if let Some(result) = stdlib_result {
            return result;
        }

        let fetchable_body = matches!(instance.kind, InstanceKind::Item | InstanceKind::Shim)
            && new_scope.0.has_body();

        if !fetchable_body {
            let _g = self.timing_span(TimingCat::InterpFnDefFetchableBody, cur_scope);
            return self.dispatch_call(
                term_span,
                ctxt,
                call_stack,
                cur_scope,
                &new_scope,
                local_decls,
                bb,
                fndef,
                args,
                genargs,
                instance,
                Vec::new(),
            );
        }

        let _timing_guard = self.timing_span(TimingCat::TermCollectResolvedArgs, cur_scope);
        let new_cs: Vec<Constraints> = self.collect_resolved_args(
            ctxt,
            term_span,
            cur_scope,
            &self.get_body(&new_scope),
            local_decls,
            args,
            false,
        );
        drop(_timing_guard);

        if call_stack.contains(&new_scope) {
            if scope_matches_trace_filter(&new_scope) {
                debug!(
                    "TRACE recursive_hit ENTER for {:?}: new_scope={:?}",
                    new_scope.0.name(),
                    new_scope
                );
            }
            let _g = self.timing_span(TimingCat::InterpFnDefCallStackChecks, cur_scope);
            let precise_count = *self
                .scope_summaries_count
                .borrow()
                .get(&base_scope_id(&new_scope))
                .unwrap_or(&0);
            let new_key: MemoKey = if precise_count >= 50 {
                let widened: Vec<Constraints> = new_cs
                    .iter()
                    .map(crate::constraints::widen_constraints)
                    .collect();
                (base_scope_id(&new_scope), ArgSet::new(&widened))
            } else {
                (base_scope_id(&new_scope), ArgSet::new(&new_cs))
            };

            let summary = self.summaries.borrow().get(&new_key).cloned();
            if let Some((cs, writes)) = summary {
                if precise_count >= 50 {
                    crate::assumptions::record(
                        crate::assumptions::Reason::MemoWidenedReuse,
                        new_scope.0.name(),
                        Vec::new,
                    );
                }
                if scope_matches_trace_filter(&new_scope) {
                    debug!(
                        "TRACE recursive_hit MEMOIZED HIT for {:?}: new_scope={:?} cached={:?}",
                        new_scope.0.name(),
                        new_scope,
                        cs
                    );
                }
                record_memo_hit(&new_scope, precise_count >= 50, &writes);
                self.replay_writes(ctxt, cur_scope, args, &writes);
                return Ok(Some(cs));
            }

            crate::assumptions::record_call(crate::assumptions::Reason::RecursionCut, &fndef);
            let retty = self
                .retty_fallback_from_poly(fndef.fn_sig())?
                .unwrap_or_default();
            if scope_matches_trace_filter(&new_scope) {
                debug!(
                    "TRACE recursive_hit FALLBACK COMPUTED for {:?}: new_scope={:?} retty={:?}",
                    new_scope.0.name(),
                    new_scope,
                    retty
                );
            }
            self.summaries
                .borrow_mut()
                .insert(new_key.clone(), (retty.clone(), SharedLog::default()));
            if precise_count < 50 {
                *self
                    .scope_summaries_count
                    .borrow_mut()
                    .entry(base_scope_id(&new_scope))
                    .or_insert(0) += 1;
            }

            let cur_key = self.key_stack.borrow().last().cloned().unwrap();

            if scope_matches_trace_filter(&new_scope) {
                debug!(
                    "TRACE recursive_hit QUEUEING for {:?}: new_scope={:?} new_cs={:?}",
                    new_scope.0.name(),
                    new_scope,
                    new_cs
                );
            }
            self.wq.borrow_mut().entry(cur_key).or_default().push((
                new_scope.clone(),
                new_cs,
                call_stack.clone(),
            ));

            return Ok(Some(retty));
        }

        let _g = self.timing_span(TimingCat::InterpFnDefFinalDispatch, cur_scope);
        self.dispatch_call(
            term_span,
            ctxt,
            call_stack,
            cur_scope,
            &new_scope,
            local_decls,
            bb,
            fndef,
            args,
            genargs,
            instance,
            new_cs,
        )
    }

    fn dispatch_call(
        &self,
        term_span: &Span,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        new_scope: &VOID,
        local_decls: &[LocalDecl],
        bb: usize,
        fndef: FnDef,
        args: &Vec<Operand>,
        genargs: &GenericArgs,
        instance: Instance,
        new_cs: Vec<Constraints>,
    ) -> Result<Option<Constraints>, Error> {
        match instance.kind {
            InstanceKind::Item | InstanceKind::Shim => {
                let _timing_guard = self.timing_span(TimingCat::TermInterpStaticCall, cur_scope);
                let r = self.interp_static_call(
                    term_span,
                    ctxt,
                    call_stack,
                    cur_scope,
                    &new_scope,
                    local_decls,
                    fndef,
                    args,
                    &genargs,
                    &new_cs,
                    false,
                );
                drop(_timing_guard);
                r
            }
            InstanceKind::Virtual { .. } => {
                debug!(
                    "[verifopt debug][dispatch_call Virtual] fndef={:?} cur_scope_def={:?} cur_scope_genargs={:?} new_scope_def={:?} new_scope_genargs={:?}",
                    fndef,
                    cur_scope.0.def.def_id(),
                    cur_scope.1,
                    new_scope.0.def.def_id(),
                    new_scope.1,
                );
                let _timing_guard = self.timing_span(TimingCat::TermInterpVirtualCall, cur_scope);
                let r = self.interp_virtual_call(
                    term_span,
                    ctxt,
                    call_stack,
                    cur_scope,
                    local_decls,
                    bb,
                    &fndef,
                    &genargs,
                    args,
                );
                drop(_timing_guard);
                r
            }
            InstanceKind::Intrinsic => {
                crate::assumptions::record_call(crate::assumptions::Reason::Intrinsic, &fndef);
                self.retty_fallback_from_poly(fndef.fn_sig())
            }
        }
    }

    /*
    fn build_param_summary(
        &self,
        scope: &VOID,
        is_closure: bool,
    ) -> Result<Option<Constraints>, Error> {
        self.building_summaries.borrow_mut().insert(scope.clone());

        let body = self.get_body(scope);
        let n_args = body.arg_locals().len();

        let param_cs: Vec<Constraints> = (0..n_args)
            .map(|i| {
                Constraints::from(Constraint::new(
                    None,
                    Some(RunningConstraint::Param(i, vec![])),
                ))
            })
            .collect();

        let mut substore = ConstraintStore::new();
        for (i, cs) in param_cs.iter().enumerate() {
            let local = if is_closure { i + 2 } else { i + 1 };
            let place = Place {
                local,
                projection: vec![],
            };
            substore.cmap.insert(
                MapKey::Var(place),
                Box::new(MapValue::Constraints(cs.clone())),
            );
        }

        let mut summary_ctxt = Context::empty();
        summary_ctxt.cstore.cmap.insert(
            MapKey::ScopeId(scope.clone()),
            Box::new(MapValue::Store(substore, None)),
        );

        let mut summary_stack = vec![scope.clone()];
        let saved_key_stack = self
            .key_stack
            .replace(vec![(scope.clone(), ArgSet::new(&param_cs))]);

        let dispatch_cha_snapshot = self.dispatch_cha.borrow().clone();
        let dispatch_targets_snapshot = self.dispatch_targets.borrow().clone();
        let dispatch_tags_snapshot = self.dispatch_tags.borrow().clone();
        let dependencies_snapshot = self.dependencies.borrow().clone();
        let incomplete_snapshot = self.incomplete.borrow().clone();

        self.summary_build_taint_stack.borrow_mut().push(false);
        let result = self.visit_body(&mut summary_ctxt, &mut summary_stack, scope, &body);
        self.key_stack.replace(saved_key_stack);
        let tainted = self
            .summary_build_taint_stack
            .borrow_mut()
            .pop()
            .expect("summary_build_taint_stack: push/pop imbalance");

        if tainted || result.is_err() {
            *self.dispatch_cha.borrow_mut() = dispatch_cha_snapshot;
            *self.dispatch_targets.borrow_mut() = dispatch_targets_snapshot;
            *self.dispatch_tags.borrow_mut() = dispatch_tags_snapshot;
            *self.dependencies.borrow_mut() = dependencies_snapshot;
            *self.incomplete.borrow_mut() = incomplete_snapshot;
        }

        self.building_summaries.borrow_mut().remove(scope);

        match result {
            Ok(_) if tainted => Err(Error::SummaryImprecise),
            other => other,
        }
    }
    */

    /*
    fn log_cache_sizes(&self, n: u64) {
        debug!(
            "\nCACHE SIZES at call {}: exact_memo={} summaries={} wq={} in_queue={} param_summaries={} building_summaries={} dispatch_targets={} dispatch_cha={} dispatch_tags={} dependencies={} incomplete={} scope_epoch={} scope_exact_memo_count={} scope_summaries_count={} key_stack={}\n",
            n,
            self.exact_memo.borrow().len(),
            self.summaries.borrow().len(),
            self.wq.borrow().len(),
            self.in_queue.borrow().len(),
            self.param_summaries.borrow().len(),
            self.building_summaries.borrow().len(),
            self.dispatch_targets.borrow().len(),
            self.dispatch_cha.borrow().len(),
            self.dispatch_tags.borrow().len(),
            self.dependencies.borrow().len(),
            self.incomplete.borrow().len(),
            self.scope_epoch.borrow().len(),
            self.scope_exact_memo_count.borrow().len(),
            self.scope_summaries_count.borrow().len(),
            self.key_stack.borrow().len(),
        );
    }

    fn current_rss_kb() -> Option<u64> {
        let status = std::fs::read_to_string("/proc/self/status").ok()?;
        for line in status.lines() {
            if let Some(rest) = line.strip_prefix("VmRSS:") {
                return rest.trim().split_whitespace().next()?.parse().ok();
            }
        }
        None
    }

    fn log_bb_cache_sizes(&self, n: u64, cur_scope: &VOID, ctxt: &Context, ordering_len: usize) {
        let (ps_max, ps_sum, ps_max_scope) = {
            let ps = self.param_summaries.borrow();
            let mut max = 0usize;
            let mut sum = 0usize;
            let mut max_scope = String::new();
            for (scope, v) in ps.iter() {
                if let ParamSummary::Built(Some(cs)) = v {
                    let s = crate::constraints::constraints_size(cs);
                    sum += s;
                    if s > max {
                        max = s;
                        max_scope = format!("{:?}", scope.0.name());
                    }
                }
            }
            (max, sum, max_scope)
        };
        let (em_max, em_sum, em_max_scope) = {
            let em = self.exact_memo.borrow();
            let mut max = 0usize;
            let mut sum = 0usize;
            let mut max_scope = String::new();
            for (key, (cs, _, _)) in em.iter() {
                if let Some(c) = cs.as_ref() {
                    let s = crate::constraints::constraints_size(c);
                    sum += s;
                    if s > max {
                        max = s;
                        max_scope = format!("{:?}", key.0.0.name());
                    }
                }
            }
            (max, sum, max_scope)
        };
        let (sm_max, sm_sum, sm_max_scope) = {
            let sm = self.summaries.borrow();
            let mut max = 0usize;
            let mut sum = 0usize;
            let mut max_scope = String::new();
            for (key, (cs, _)) in sm.iter() {
                let s = crate::constraints::constraints_size(cs);
                sum += s;
                if s > max {
                    max = s;
                    max_scope = format!("{:?}", key.0.0.name());
                }
            }
            (max, sum, max_scope)
        };
        debug!(
            "\nBB CACHE SIZES at bb visit {} rss_kb={} for {:?}: cstore_cmap={} ordering_remaining={} exact_memo={} (max_disjuncts={} max_scope={} sum_disjuncts={}) summaries={} (max_disjuncts={} max_scope={} sum_disjuncts={}) wq={} param_summaries={} (max_disjuncts={} max_scope={} sum_disjuncts={}) dispatch_targets={} dispatch_cha={} dispatch_tags={} dependencies={}\n",
            n,
            Self::current_rss_kb().unwrap_or(0),
            cur_scope.0.name(),
            ctxt.cstore.cmap.len(),
            ordering_len,
            self.exact_memo.borrow().len(),
            em_max,
            em_max_scope,
            em_sum,
            self.summaries.borrow().len(),
            sm_max,
            sm_max_scope,
            sm_sum,
            self.wq.borrow().len(),
            self.param_summaries.borrow().len(),
            ps_max,
            ps_max_scope,
            ps_sum,
            self.dispatch_targets.borrow().len(),
            self.dispatch_cha.borrow().len(),
            self.dispatch_tags.borrow().len(),
            self.dependencies.borrow().len(),
        );
    }
    */

    /// Memory diagnostics (see VERIFOPT_CACHE_SIZES): RSS, then each cache's
    /// entry count and, for the value-holding ones, the total node count of
    /// its values (constraints_size, a tree count).
    fn log_memory_diag(&self, n: u64, ctxt: &Context) {
        let rss_mb = std::fs::read_to_string("/proc/self/status")
            .ok()
            .and_then(|s| {
                s.lines()
                    .find_map(|l| l.strip_prefix("VmRSS:"))
                    .and_then(|r| r.trim().split_whitespace().next()?.parse::<u64>().ok())
            })
            .unwrap_or(0)
            / 1024;
        let size = crate::constraints::constraints_size;
        let (em_n, em_nodes, em_key_nodes) = {
            let em = self.exact_memo.borrow();
            (
                em.len(),
                em.values().map(|(c, _, _)| c.as_ref().map(size).unwrap_or(0)).sum::<usize>(),
                em.keys().map(|(_, a)| crate::constraints::argset_size(a)).sum::<usize>(),
            )
        };
        let sm_key_nodes =
            self.summaries.borrow().keys().map(|(_, a)| crate::constraints::argset_size(a)).sum::<usize>();
        // Replay logs (write_log) kept by the memos: entries with one, logged
        // values, and the distinct ones (by pointer) with their node count.
        let (wl_entries, wl_values, wl_distinct, wl_nodes) = {
            let mut seen: HashSet<*const IndexSet<Constraint>> = HashSet::new();
            let (mut entries, mut values, mut nodes) = (0usize, 0usize, 0usize);
            let em = self.exact_memo.borrow();
            let vm = self.virtual_call_memo.borrow();
            let sm = self.summaries.borrow();
            let logs = em
                .values()
                .map(|(_, _, l)| l)
                .chain(vm.values().map(|(_, _, l)| l))
                .chain(sm.values().map(|(_, l)| l));
            for log in logs.filter(|l| !l.is_empty()) {
                entries += 1;
                for v in log.ports.values().chain(log.statics.values()).flatten() {
                    values += 1;
                    if seen.insert(Rc::as_ptr(&v.inner)) {
                        nodes += size(v);
                    }
                }
            }
            (entries, values, seen.len(), nodes)
        };
        let (store_vars, store_nodes) = crate::constraints::store_size(&ctxt.cstore);
        let (sm_n, sm_nodes) = {
            let sm = self.summaries.borrow();
            (sm.len(), sm.values().map(|(c, _)| size(c)).sum::<usize>())
        };
        let (wq_keys, wq_items, wq_nodes, wq_stack) = {
            let wq = self.wq.borrow();
            let items = wq.values().map(|v| v.len()).sum::<usize>();
            let nodes = wq.values().flatten().map(|(_, cs, _)| cs.iter().map(size).sum::<usize>()).sum::<usize>();
            let stack = wq.values().flatten().map(|(_, _, st)| st.len()).sum::<usize>();
            (wq.len(), items, nodes, stack)
        };
        // Bytes the allocator has handed out and not got back (all arenas,
        // plus mmapped chunks), vs RSS: tells live growth from memory freed
        // but kept by the allocator (RSS rarely shrinks).
        let malloc_mb = {
            #[repr(C)]
            struct Mallinfo2 {
                arena: usize,
                ordblks: usize,
                smblks: usize,
                hblks: usize,
                hblkhd: usize,
                usmblks: usize,
                fsmblks: usize,
                uordblks: usize,
                fordblks: usize,
                keepcost: usize,
            }
            unsafe extern "C" {
                fn mallinfo2() -> Mallinfo2;
            }
            let m = unsafe { mallinfo2() };
            let _ = (m.arena, m.ordblks, m.smblks, m.hblks, m.usmblks, m.fsmblks, m.fordblks, m.keepcost);
            (m.uordblks + m.hblkhd) / (1024 * 1024)
        };
        // The functions whose exact_memo keys are largest (entries, key
        // nodes): memo keys are the calls' argument values.
        let memo_top = {
            let mut by: HashMap<String, (usize, usize)> = HashMap::new();
            for (k, a) in self.exact_memo.borrow().keys() {
                let e = by.entry(k.0.name()).or_default();
                e.0 += 1;
                e.1 += crate::constraints::argset_size(a);
            }
            let mut v: Vec<_> = by.into_iter().collect();
            v.sort_by(|a, b| b.1.1.cmp(&a.1.1));
            v.iter()
                .take(10)
                .map(|(name, (n, nodes))| format!("{name}:{n}/{}M", nodes / 1_000_000))
                .collect::<Vec<_>>()
                .join(" | ")
        };
        let (vm_n, vm_key_nodes) = {
            let vm = self.virtual_call_memo.borrow();
            (vm.len(), vm.keys().map(|(_, a)| crate::constraints::argset_size(a)).sum::<usize>())
        };
        let (refs_scopes, refs_places, refs_edges) = {
            let refs = &ctxt.cstore.refs;
            let places = refs.values().map(|m| m.len()).sum::<usize>();
            let edges = refs.values().flat_map(|m| m.values()).map(|s| s.len()).sum::<usize>();
            (refs.len(), places, edges)
        };
        let (dt_n, dt_targets) = {
            let dt = self.dispatch_targets.borrow();
            (dt.len(), dt.values().map(|(_, t)| t.len()).sum::<usize>())
        };
        {
            let levels = self.sim_levels.borrow();
            let held: usize = levels.iter().map(|l| l.4).sum();
            let shown = levels
                .iter()
                .map(|(name, i, len, d, nodes)| format!("{name} {i}/{len} dirty={d} {}M", nodes / 1_000_000))
                .collect::<Vec<_>>();
            let shown = if shown.len() > 24 {
                let mut s = shown[..12].to_vec();
                s.push(format!("... {} more ...", shown.len() - 24));
                s.extend_from_slice(&shown[shown.len() - 12..]);
                s
            } else {
                shown
            };
            log::warn!(
                "[verifopt memory sim] bb_visits={n} depth={} held_nodes={held} levels=[{}]",
                levels.len(),
                shown.join(" > ")
            );
        }
        log::warn!(
            "[verifopt memory extra] bb_visits={n} malloc_in_use={malloc_mb}MB rss={rss_mb}MB virtual_memo={vm_n} ({vm_key_nodes} key nodes) refs={refs_scopes} scopes/{refs_places} places/{refs_edges} edges return_walks={} body_cache={} wto_templates={} dispatch_targets={dt_n} ({dt_targets} targets) dispatch_cha={} assumptions={} memo_top=[{memo_top}]",
            self.return_walks.borrow().len(),
            self.body_cache.borrow().len(),
            self.wto_templates.borrow().len(),
            self.dispatch_cha.borrow().len(),
            crate::assumptions::entries(),
        );
        log::warn!(
            "[verifopt memory] bb_visits={n} rss={rss_mb}MB elapsed={}s exact_memo={em_n} ({em_nodes} value nodes, {em_key_nodes} key nodes) summaries={sm_n} ({sm_nodes} value nodes, {sm_key_nodes} key nodes) stores={store_vars} vars ({store_nodes} nodes) replay_logs={wl_entries} entries ({wl_values} values, {wl_distinct} distinct, {wl_nodes} nodes) rc_caches=[{}lift={}/{} cast={}/{}] wq={wq_keys} keys/{wq_items} items ({wq_nodes} arg nodes, {wq_stack} saved stack frames) in_queue={} dependencies={} incomplete={} dispatch_tags={} scope_epoch={} cstore_cmap={} key_stack={} stack=[{}]",
            self.run_start.elapsed().as_secs(),
            crate::constraints::rc_cache_sizes(),
            LIFT_TRAITOBJTYS_CACHE.with(crate::constraints::rc_cache_size).0,
            LIFT_TRAITOBJTYS_CACHE.with(crate::constraints::rc_cache_size).1,
            crate::convert::cast_cache_size().0,
            crate::convert::cast_cache_size().1,
            self.in_queue.borrow().len(),
            self.dependencies.borrow().len(),
            self.incomplete.borrow().len(),
            self.dispatch_tags.borrow().len(),
            self.scope_epoch.borrow().len(),
            ctxt.cstore.cmap.len(),
            self.key_stack.borrow().len(),
            self.key_stack.borrow().iter().rev().take(6).map(|k| k.0.0.name()).collect::<Vec<_>>().join(" < "),
        );
    }

    fn interp_static_call(
        &self,
        term_span: &Span,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        caller_scope: &VOID,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        fndef: FnDef,
        args: &Vec<Operand>,
        genargs: &GenericArgs,
        cur_cs: &Vec<Constraints>,
        is_closure: bool,
    ) -> Result<Option<Constraints>, Error> {
        //{
        //    let mut n = self.call_count.borrow_mut();
        //    *n += 1;
        //    if *n % 10_000 == 0 {
        //        self.log_cache_sizes(*n);
        //    }
        //}

        if cur_scope.0.has_body() {
            let body = self.get_body(cur_scope);
            let key = (cur_scope.clone(), ArgSet::new(cur_cs));

            /*
            let summary_start = std::time::Instant::now();
            let cached_summary = self.param_summaries.borrow().get(cur_scope).cloned();
            match cached_summary {
                Some(ParamSummary::Built(summary)) => {
                    let summary_size = summary
                        .as_ref()
                        .map(crate::constraints::constraints_size)
                        .unwrap_or(0);
                    debug!(
                        "param_summary hit for {:?} (summary_disjuncts={}) - about to substitute",
                        cur_scope.0.name(),
                        summary_size
                    );
                    let substituted = summary.as_ref().map(|s| substitute_params(s, cur_cs));
                    debug!("substitute_params returned for {:?}", cur_scope.0.name());
                    return Ok(substituted);
                }
                Some(ParamSummary::Unavailable) => {
                    // fall through to Step 1 below
                }
                None => {
                    if self.building_summaries.borrow().contains(cur_scope) {
                        debug!(
                            "param_summary unavailable for {:?}: already building (recursive)",
                            cur_scope.0.name()
                        );
                        self.param_summaries
                            .borrow_mut()
                            .insert(cur_scope.clone(), ParamSummary::Unavailable);
                    } else {
                        let rust_call =
                            rust_call_arity(&self.get_body(cur_scope), args, local_decls).is_some();
                        let build_result =
                            self.build_param_summary(cur_scope, is_closure && !rust_call);
                        match build_result {
                            Ok(built) => {
                                debug!("built param_summary for {:?}", cur_scope.0.name());
                                self.param_summaries
                                    .borrow_mut()
                                    .insert(cur_scope.clone(), ParamSummary::Built(built.clone()));
                                let substituted = built.map(|s| substitute_params(&s, cur_cs));
                                return Ok(substituted);
                            }
                            Err(e) => {
                                debug!(
                                    "param_summary unavailable for {:?}: {:?}",
                                    cur_scope.0.name(),
                                    e
                                );
                                self.param_summaries
                                    .borrow_mut()
                                    .insert(cur_scope.clone(), ParamSummary::Unavailable);
                                // fall through to Step 1 below
                            }
                        }
                    }
                }
            }
            self.record_timing(
                TimingCat::TermParamSummary,
                cur_scope,
                summary_start.elapsed(),
            );
            */

            let _timing_guard = self.timing_span(TimingCat::TermMemo, cur_scope);
            let precise_count = *self
                .scope_exact_memo_count
                .borrow()
                .get(&base_scope_id(cur_scope))
                .unwrap_or(&0);
            let memo_key: MemoKey = if precise_count >= 50 {
                let widened: Vec<Constraints> = cur_cs
                    .iter()
                    .map(crate::constraints::widen_constraints)
                    .collect();
                (base_scope_id(cur_scope), ArgSet::new(&widened))
            } else {
                (base_scope_id(cur_scope), key.1.clone())
            };

            //let scope_name = format!("{:?}", cur_scope.0.name());
            //if scope_name.contains("RawVecInner") {
            //    let will_hit = self
            //        .exact_memo
            //        .borrow()
            //        .get(&memo_key)
            //        .map(|(_, e)| *e == *self.scope_epoch.borrow().get(cur_scope).unwrap_or(&0))
            //        .unwrap_or(false);
            //    debug!(
            //        "\nRAWVEC TRACE for {}: precise_count={} widened={} exact_memo_hit={} exact_memo_key_hash={:?}\n",
            //        scope_name,
            //        precise_count,
            //        precise_count >= 50,
            //        will_hit,
            //        memo_key.1,
            //    );
            //}

            let epoch_before = *self.scope_epoch.borrow().get(&base_scope_id(cur_scope)).unwrap_or(&0);
            let hit = self.exact_memo.borrow().get(&memo_key).cloned();
            if let Some((cached, cached_epoch, writes)) = hit {
                if cached_epoch == epoch_before && crate::write_log::reads_fresh(&writes) {
                    if precise_count >= 50 {
                        crate::assumptions::record(
                            crate::assumptions::Reason::MemoWidenedReuse,
                            cur_scope.0.name(),
                            Vec::new,
                        );
                    }
                    record_memo_hit(cur_scope, precise_count >= 50, &writes);
                    self.replay_writes(ctxt, caller_scope, args, &writes);
                    self.add_pending_return_aliases(writes.returns.iter().cloned());
                    debug!(
                        "exact_memo hit for {:?} at epoch {:?}",
                        cur_scope.0.name(),
                        epoch_before
                    );
                    return Ok(cached.clone());
                }
            }
            drop(_timing_guard);

            let _timing_guard = self.timing_span(TimingCat::TermResolveArgs, cur_scope);
            self.resolve_args(
                ctxt,
                term_span,
                caller_scope,
                &body,
                cur_scope,
                local_decls,
                args,
                genargs,
                is_closure,
            );
            drop(_timing_guard);
            let stack_len = call_stack.len();
            let frame = crate::write_log::push(operand_places(args), caller_scope);
            self.prepare_call(call_stack, &key);
            let result = self.visit_body(ctxt, call_stack, cur_scope, &body);
            let mut log = frame.finish();
            log.returns = self.return_alias_operands(cur_scope, caller_scope, args);
            self.add_pending_return_aliases(log.returns.iter().cloned());
            let writes: SharedLog = Rc::new(log);

            let _timing_guard = self.timing_span(TimingCat::TermInterpStaticCallPost, cur_scope);
            if let Ok(ref cs) = result {
                let _g1 = self.timing_span(TimingCat::TermInterpStaticCallPost1, cur_scope);
                let epoch_after = *self.scope_epoch.borrow().get(&base_scope_id(cur_scope)).unwrap_or(&0);
                drop(_g1);

                let _g2 = self.timing_span(TimingCat::TermInterpStaticCallPost2, cur_scope);
                let is_new = !self.exact_memo.borrow().contains_key(&memo_key);
                drop(_g2);

                let _g3 = self.timing_span(TimingCat::TermInterpStaticCallPost3, cur_scope);
                self.exact_memo
                    .borrow_mut()
                    .insert(memo_key.clone(), (cs.clone(), epoch_after, writes));
                drop(_g3);

                let _g4 = self.timing_span(TimingCat::TermInterpStaticCallPost4, cur_scope);
                if is_new && precise_count < 50 {
                    *self
                        .scope_exact_memo_count
                        .borrow_mut()
                        .entry(base_scope_id(cur_scope))
                        .or_insert(0) += 1;
                }
            }
            drop(_timing_guard);

            match result {
                Ok(r) => Ok(r),
                Err(e) => {
                    self.unwind_stacks_to(call_stack, stack_len, "interp_static_call error");
                    Err(e)
                }
            }
        } else {
            crate::assumptions::record_call(crate::assumptions::Reason::NoMir, &fndef);
            self.retty_fallback_from_poly(fndef.fn_sig())
        }
    }

    fn resolve_args(
        &self,
        ctxt: &mut Context,
        term_span: &Span,
        caller_scope: &VOID,
        body: &Body,
        callee_scope: &VOID,
        local_decls: &[LocalDecl],
        args: &Vec<Operand>,
        _genargs: &GenericArgs,
        is_closure: bool,
    ) {
        let mut new_ctxt = Context::empty();
        self.resolve_args_helper(
            ctxt,
            term_span,
            &mut new_ctxt,
            caller_scope,
            callee_scope,
            body,
            local_decls,
            args,
            is_closure,
        );

        // Merge new substore into existing substore at this scopeId
        let store;
        let mut widened = false;
        match ctxt.get_cstore_scope(callee_scope) {
            Some(box MapValue::Store(old_substore, old_es)) => {
                store = self.merge_stores_timed(
                    callee_scope,
                    &old_substore,
                    &old_es,
                    &new_ctxt.cstore,
                    &Some(Rc::new(vec![caller_scope.clone()])),
                );
                widened = !store.0.cmap.ptr_eq(&old_substore.cmap);
            }
            Some(_) => panic!("got constraint, expected store"),
            None => {
                store = (new_ctxt.cstore, Some(Rc::new(vec![caller_scope.clone()])));
            }
        }

        if widened {
            let mut epochs = self.scope_epoch.borrow_mut();
            let e = epochs.entry(base_scope_id(callee_scope)).or_insert(0);
            *e += 1;
        }

        ctxt.set_cstore_scope(callee_scope, store.0, store.1);
    }

    pub fn collect_resolved_args(
        &self,
        ctxt: &Context,
        term_span: &Span,
        caller_scope: &VOID,
        body: &Body,
        local_decls: &[LocalDecl],
        args: &Vec<Operand>,
        is_closure: bool,
    ) -> Vec<Constraints> {
        if let Some(arity) = rust_call_arity(body, args, local_decls) {
            return self.resolve_rust_call_args(
                ctxt,
                term_span,
                caller_scope,
                body,
                local_decls,
                args,
                arity,
                is_closure,
            );
        }
        let arg_count = body.arg_locals().len();
        let mut resolved = Vec::new();

        for (i, arg) in args.into_iter().enumerate() {
            let local = if is_closure { i + 2 } else { i + 1 };
            let place = Place {
                local,
                projection: vec![],
            };

            let maybe_trait_argty = if i > arg_count - 1 {
                None
            } else {
                let arg_ty = place.ty(body.locals()).unwrap();
                self.contains_dyn(&arg_ty)
            };

            resolved.push(self.resolve_arg(
                ctxt,
                term_span,
                caller_scope,
                &maybe_trait_argty,
                local_decls,
                arg,
                is_closure,
            ));
        }

        return resolved;
    }

    /// For each argument:
    /// - get the local to put constraints into
    /// - check if any arg types are traitobjects that we should translate existing concrete
    /// constraints into
    /// - resolve the argument into constraints (given constraints in our store)
    /// - update the substore for the callee
    fn resolve_args_helper(
        &self,
        ctxt: &mut Context,
        term_span: &Span,
        new_ctxt: &mut Context,
        caller_scope: &VOID,
        callee_scope: &VOID,
        body: &Body,
        local_decls: &[LocalDecl],
        args: &Vec<Operand>,
        is_closure: bool,
    ) {
        let resolved = self.collect_resolved_args(
            ctxt,
            term_span,
            caller_scope,
            body,
            local_decls,
            args,
            is_closure,
        );

        // An untupled rust-call (see rust_call_arity) is aligned with the
        // callee's argument locals from `_1`; only `self` is a caller operand.
        let rust_call = rust_call_arity(body, args, local_decls).is_some();
        for (i, constraints) in resolved.into_iter().enumerate() {
            debug!("\narg position: {:?}", i);
            let local = if is_closure && !rust_call { i + 2 } else { i + 1 };
            let place = Place {
                local,
                projection: vec![],
            };

            if local < body.locals().len() && (!rust_call || i == 0) {
                let arg_ty = place.ty(body.locals()).unwrap();
                if let TyKind::RigidTy(RigidTy::Ref(_, _, mt) | RigidTy::RawPtr(_, mt)) = arg_ty.kind() {
                    if let Operand::Copy(caller_place) | Operand::Move(caller_place) = &args[i] {
                        ctxt.cstore.add_ref(
                            (place.clone(), callee_scope.clone()),
                            (caller_place.clone(), caller_scope.clone()),
                            mt,
                        );
                    } else if let Some(def) = static_pointer_operand(&args[i], &arg_ty) {
                        let to = self.static_place(ctxt, def);
                        ctxt.cstore.add_ref((place.clone(), callee_scope.clone()), to, mt);
                    }
                }
            }

            //debug!("arg constraints: {:?}", constraints);
            debug!("arg place in new scope: {:?}\n", place);

            new_ctxt.cstore.cmap.insert(
                MapKey::Var(place.clone()),
                Box::new(MapValue::Constraints(constraints)),
            );
        }
    }

    /// If constraints exist for the argument local, return those (potentially transforming/pulling
    /// out traitobj constraints if this arg contains a traitobj).
    /// If the arg is a constant, return the constraints gotten by converting the type into
    /// VerifOpt constraints.
    pub fn resolve_arg(
        &self,
        ctxt: &Context,
        _term_span: &Span,
        caller_scope: &VOID,
        maybe_trait_argty: &Option<Vec<TraitObjTy>>,
        local_decls: &[LocalDecl],
        arg: &Operand,
        is_closure: bool,
    ) -> Constraints {
        // FIXME implementation is similar to convert::convert_place()
        match arg {
            Operand::Copy(place) | Operand::Move(place) => {
                match ctxt.get_constraints(caller_scope, local_decls, place, is_closure, Some(self))
                {
                    Some(constraints) => {
                        self.lift_traitobjtys(maybe_trait_argty, constraints, caller_scope)
                    }
                    None => {
                        let (_maybe_traitobjty, constraint) = self.converter.convert_ty(
                            &Location::unknown(),
                            &place.ty(local_decls).unwrap(),
                            Some(caller_scope),
                            Some(self),
                        );
                        Constraints::from(constraint)
                    }
                }
            }
            Operand::Constant(const_op) => self.converter.convert_const(
                &Location::unknown(),
                &const_op,
                Some(caller_scope),
                Some(self),
            ),
            _ => todo!("runtime check arg"),
        }
    }

    /// The arguments of a rust-call closure invocation (see rust_call_arity),
    /// as the callee's argument locals see them: `self` for `_1`, then the
    /// tuple's elements for `_2..` (each from the tuple's value, or "a value
    /// of the parameter's type" if that's unknown).
    fn resolve_rust_call_args(
        &self,
        ctxt: &Context,
        term_span: &Span,
        caller_scope: &VOID,
        body: &Body,
        local_decls: &[LocalDecl],
        args: &Vec<Operand>,
        arity: usize,
        is_closure: bool,
    ) -> Vec<Constraints> {
        let locals = body.locals();
        let mut out = Vec::with_capacity(1 + arity);
        out.push(self.resolve_arg(
            ctxt,
            term_span,
            caller_scope,
            &self.contains_dyn(&locals[1].ty),
            local_decls,
            &args[0],
            is_closure,
        ));
        let tuple =
            self.resolve_arg(ctxt, term_span, caller_scope, &None, local_decls, &args[1], is_closure);
        for j in 0..arity {
            let param_ty = &locals[j + 2].ty;
            let mut elem = Constraints::new();
            for c in tuple.inner.iter() {
                if let Some(RunningConstraint::Tuple(inner)) = &c.cfc
                    && let Some(e) = inner.get(j)
                {
                    elem.append(e.clone());
                }
            }
            if elem.inner.is_empty() {
                let (_, constraint) = self.converter.convert_ty(
                    &Location::unknown(),
                    param_ty,
                    Some(caller_scope),
                    Some(self),
                );
                elem = Constraints::from(constraint);
            }
            out.push(self.lift_traitobjtys(&self.contains_dyn(param_ty), elem, caller_scope));
        }
        out
    }

    fn check_sig_boundvars(&self, sig: &PolyFnSig) {
        if !sig.bound_vars.is_empty() {
            // Might not be safe to just skip binder
            //debug!("Bound vars - cannot just skip binder in call resolution");
            for bound_var in sig.bound_vars.iter() {
                match bound_var {
                    BoundVariableKind::Ty(_) => todo!("ty"),
                    BoundVariableKind::Const => todo!("const"),
                    BoundVariableKind::Region(_) => {}
                }
            }
        }
    }

    pub fn retty_fallback_from_poly(&self, sig: PolyFnSig) -> Result<Option<Constraints>, Error> {
        //debug!("fn_sig: {:?}", sig);
        self.check_sig_boundvars(&sig);
        //debug!("output: {:?}", sig.value.output());

        // Return output type that matches type info (widening)
        let (_, constraint) =
            self.converter
                .convert_ty(&Location::unknown(), &sig.value.output(), None, Some(self));
        Ok(Some(Constraints::from(constraint)))
    }

    fn retty_fallback_from_sigval(&self, sigval: &SigVal) -> Result<Option<Constraints>, Error> {
        //debug!("sigval: {:?}", sigval);
        if !sigval.bound_tys.is_empty() {
            todo!(
                "SigVal fallback with bound type-vars in signature: {:?}",
                sigval.bound_tys
            );
        }

        let (_, constraint) =
            self.converter
                .convert_ty(&Location::unknown(), &sigval.output, None, Some(self));
        Ok(Some(Constraints::from(constraint)))
    }

    /// Interpret dynamic dispatch.
    ///
    /// First determine the set of calls as determined by CHA.
    ///
    /// Then determine the set of calls as determined by FSA.
    ///
    /// Comparing these sets will help us determine where FSA might win over CHA (or other
    /// baselines TBD)
    ///
    /// Then continue interpretation given the FSA candidate function set.
    fn interp_virtual_call(
        &self,
        term_span: &Span,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        caller_scope: &VOID,
        local_decls: &[LocalDecl],
        bb: usize,
        fndef: &FnDef,
        genargs: &GenericArgs,
        args: &Vec<Operand>,
    ) -> Result<Option<Constraints>, Error> {
        debug!("\nDYNAMIC CALL - fndef: {:?}\n", fndef);
        log_scope(caller_scope);

        // Get trait that this function is associated with
        // - tstore.assoc_fn_traits (Map<AssocFn, Trait>)
        let trait_defid = self.get_trait_defid(&fndef.0);
        debug!("trait_defid: {:?}", trait_defid);

        // Keyed on the *resolved* instance's args (what codegen sees as
        // `instance.args`), not the args this scope was reached with
        // (caller_scope.1) - those differ whenever the caller was reached
        // through a trait method, e.g. `[Self = Formatter<'_, E>]` vs the
        // impl's own `['_, E]`, and the rewrite side could never match them.
        let key = (caller_scope.0.def.def_id(), bb, caller_scope.0.args());
        debug!(
            "[verifopt debug][interp_virtual_call key] fndef={:?} caller_def={:?} bb={:?} caller_genargs={:?}",
            fndef,
            key.0,
            key.1,
            key.2,
        );

        let _timing_guard = self.timing_span(TimingCat::TermVirtualMemo, caller_scope);
        let resolved_args: Vec<Constraints> = args
            .iter()
            .map(|op| {
                self.resolve_arg(ctxt, term_span, caller_scope, &None, local_decls, op, false)
            })
            .collect();
        let virtual_key: VirtualCallKey = (key.clone(), ArgSet::new(&resolved_args));
        let hit = self.virtual_call_memo.borrow().get(&virtual_key).cloned();
        match hit {
            Some((cached, recorded_scopes, writes)) if crate::write_log::reads_fresh(&writes) => {
                let stale: Vec<(DefId, u64, u64)> = recorded_scopes
                    .iter()
                    .filter_map(|(scope, recorded_epoch)| {
                        let current_epoch =
                            *self.scope_epoch.borrow().get(&base_scope_id(scope)).unwrap_or(&0);
                        (current_epoch != *recorded_epoch)
                            .then(|| (scope.0.def.def_id(), *recorded_epoch, current_epoch))
                    })
                    .collect();
                if stale.is_empty() {
                    debug!(
                        "virtual_call_memo hit for call site {:?} ({} dependent scopes, all fresh)",
                        key,
                        recorded_scopes.len()
                    );
                    // `_timing_guard` (TermVirtualMemo) drops here automatically,
                    // correctly closing this span even on this early-return
                    // exit - same reasoning as `TermMemo`'s cache-hit path.
                    record_memo_hit(caller_scope, false, &writes);
                    self.replay_writes(ctxt, caller_scope, args, &writes);
                    self.add_pending_return_aliases(writes.returns.iter().cloned());
                    return Ok(cached);
                } else {
                    debug!(
                        "virtual_call_memo MISS (stale) for call site {:?}: {} of {} dependent scopes changed: {:?}",
                        key,
                        stale.len(),
                        recorded_scopes.len(),
                        stale
                    );
                }
            }
            _ => {
                debug!("virtual_call_memo MISS (no entry) for call site {:?}", key);
            }
        }
        drop(_timing_guard);

        // Get concrete type constraints for trait object
        // - ctxt (FSA) / tstore (CHA / RTA)
        // Get every concrete type constraint's impl of this function
        // - tstore.struct_assoc_fns (Map<(Struct, Trait), FnImpls>)
        let assoc_fn_impls_cha;
        let _timing_guard = self.timing_span(TimingCat::TermGetImplsCha, caller_scope);
        if let Some(cha_impls) = self.dispatch_cha.borrow().get(&key) {
            assoc_fn_impls_cha = cha_impls.clone().1;
        } else {
            assoc_fn_impls_cha = self.get_impls_cha(&fndef.0, &trait_defid, genargs);
        }
        drop(_timing_guard);

        for (cha_impl, _) in &assoc_fn_impls_cha {
            if *cha_impl == fndef.0 {
                //debug!("CHA set has the virtual dyn function");
            }
        }

        let _timing_guard = self.timing_span(TimingCat::TermGetImplsFsa, caller_scope);
        let (is_closure, receiver_is_param, mut assoc_fn_impls_fsa) = self.get_impls_fsa(
            ctxt,
            term_span,
            caller_scope,
            local_decls,
            &trait_defid,
            &fndef.0,
            args,
        );
        drop(_timing_guard);

        let _timing_guard = self.timing_span(TimingCat::TermVirtualCallPrep, caller_scope);
        let fsa_empty = assoc_fn_impls_fsa.is_empty();

        // The receiver's constraint is a `Param` placeholder, not a real (even if
        // unresolvable) value - we're inside `build_param_summary`, interpreting this
        // function generically before any caller's concrete argument is known.
        if fsa_empty && receiver_is_param {
            debug!(
                "FSA empty because receiver is an unresolved summary param (not yet known) - \
                 deferring to real call site instead of falling back to CHA"
            );
            // `_timing_guard` (TermVirtualCallPrep) drops here automatically,
            // correctly closing this span even on this early-return exit.
            return Err(Error::SummaryImprecise);
        }

        // Cross-check: CHA is the fallback whenever FSA comes up empty, so it
        // must be a superset of anything FSA can find - an impl FSA sees but
        // CHA lacks means that fallback would drop a real target.
        //
        // Compare by impl DefId only. CHA candidates never carry generic args
        // (get_cha_tyconstraint_defids maps every one to `None`), so comparing
        // whole (DefId, args) pairs reported every generic impl FSA found as
        // "missing" - e.g. all of `FilterMap::next`, `vec::IntoIter::next`.
        //
        // Skip Fn-family dispatch: closures only implement the builtin Fn
        // traits (see closure_can_implement), and CHA's trait_structs only
        // records nominal types with impl blocks, so CHA can never contain a
        // closure candidate by construction.
        if !crate::constraints::is_fn_family_method(&fndef.0) {
            for (fsa_did, fsa_args) in &assoc_fn_impls_fsa {
                if !assoc_fn_impls_cha.iter().any(|(cha_did, _)| cha_did == fsa_did) {
                    error!("CHA missing impl: {:?} (FSA genargs {:?})", fsa_did, fsa_args);
                }
            }
        }

        if fsa_empty {
            debug!("nothing to call, FSA set is empty, falling back to CHA");
            assoc_fn_impls_fsa = assoc_fn_impls_cha.clone();
        }

        // Log CHA vs FSA diffs
        if assoc_fn_impls_cha != assoc_fn_impls_fsa {
            debug!(
                "\n\nDYNAMIC DISPATCH - SET OF IMPLS DIFFER [Trait {:?}]: (CHA:FSA) = ({:?}:{:?})\tFNDEF = {:?}\tterm={:?}\n",
                trait_defid,
                assoc_fn_impls_cha.len(),
                assoc_fn_impls_fsa.len(),
                fndef,
                term_span,
            );
        } else {
            debug!(
                "\n\nDYNAMIC DISPATCH - SET OF IMPLS SAME [Trait {:?}]: (CHA:FSA) = ({:?}:{:?})\tFNDEF = {:?}\tterm={:?}\n",
                trait_defid,
                assoc_fn_impls_cha.len(),
                assoc_fn_impls_fsa.len(),
                fndef,
                term_span,
            );
        }

        self.dispatch_cha
            .borrow_mut()
            .entry(key.clone())
            .or_insert((*term_span, assoc_fn_impls_cha));

        {
            // collect possible calls (mostly for recursion)
            let mut dt = self.dispatch_targets.borrow_mut();
            let entry = dt.entry(key.clone()).or_insert((*term_span, Vec::new()));
            for f in &assoc_fn_impls_fsa {
                if !entry.1.contains(f) {
                    entry.1.push(f.clone());
                }
            }
        }

        {
            let ds = &mut self.dependencies.borrow_mut();
            let entry = ds.entry(*term_span).or_default();
            for c in call_stack.iter() {
                entry.insert(c.clone());
            }
        }

        let plan = self.compute_tag_plan(
            ctxt,
            term_span,
            caller_scope,
            local_decls,
            &trait_defid,
            &fndef.0,
            args,
            fsa_empty,
        );
        {
            let mut dt = self.dispatch_tags.borrow_mut();
            match dt.entry(key) {
                ImEntry::Occupied(mut e) => {
                    e.get_mut().join(&plan);
                }
                ImEntry::Vacant(e) => {
                    e.insert(plan);
                }
            }
        }
        drop(_timing_guard);

        let mut touched_scopes: Vec<(VOID, u64)> = Vec::new();
        let frame = crate::write_log::push(operand_places(args), caller_scope);
        let result = self.simulate_static_calls(
            term_span,
            ctxt,
            call_stack,
            caller_scope,
            local_decls,
            assoc_fn_impls_fsa,
            genargs,
            args,
            is_closure,
            &fndef.0,
            &mut touched_scopes,
        );

        // Only cache on success - an `Err` (e.g. a caught-and-skipped
        // candidate panic bubbling up, or `SummaryImprecise`) shouldn't be
        // treated as a stable, reusable outcome for this call site.
        let mut log = frame.finish();
        // The candidates' return aliases (simulate_static_calls passes them
        // this call's operands), for the memo.
        log.returns = {
            let mut r: Vec<crate::write_log::ReturnAlias> = Vec::new();
            for a in self.pending_return_aliases.borrow().last().cloned().unwrap_or_default() {
                if !r.contains(&a) {
                    r.push(a);
                }
            }
            r
        };
        let writes: SharedLog = Rc::new(log);
        if let Ok(ref cs) = result {
            self.virtual_call_memo
                .borrow_mut()
                .insert(virtual_key, (cs.clone(), touched_scopes, writes));
        }

        result
    }

    fn compute_tag_plan(
        &self,
        ctxt: &Context,
        term_span: &Span,
        caller_scope: &VOID,
        local_decls: &[LocalDecl],
        trait_defid: &DefId,
        assoc_fn_defid: &DefId,
        args: &Vec<Operand>,
        fsa_empty: bool,
    ) -> TagPlan {
        if fsa_empty {
            return TagPlan::Poisoned;
        }

        let place = self.get_traitobj_place(args);
        let cs = match ctxt.get_constraints(caller_scope, local_decls, &place, false, Some(self)) {
            Some(cs) => cs,
            None => return TagPlan::Poisoned,
        };
        if cs.is_empty() {
            return TagPlan::Poisoned;
        }

        let caller_did = caller_scope.0.def.def_id();

        let mut by_site: HashMap<(DefId, usize, usize), (DefId, Option<GenericArgs>)> =
            HashMap::new();

        for c in cs.inner.iter() {
            let c = c.clone();
            let tags = match &c.prov {
                TagProv::Tags(t) if !t.is_empty() => t,
                _ => return TagPlan::Poisoned,
            };

            let (_is_closure, defids) = self.resolve_defid(term_span, trait_defid, &c);
            let impls = self.get_impls_from_defids(assoc_fn_defid, &defids, true);
            if impls.len() != 1 {
                return TagPlan::Poisoned;
            }

            let (target, target_genargs) = (impls[0].0, impls[0].1.clone());

            for site in tags {
                if site.0 != caller_did {
                    return TagPlan::Poisoned;
                }

                match by_site.entry(*site) {
                    Entry::Occupied(e) => {
                        // same site claimed by two
                        if e.get().0 != target || e.get().1 != target_genargs {
                            return TagPlan::Poisoned;
                        }
                    }
                    Entry::Vacant(e) => {
                        e.insert((target, target_genargs.clone()));
                    }
                }
            }
        }

        let mut out: Vec<(usize, usize, DefId, Option<GenericArgs>)> = by_site
            .into_iter()
            .map(|((_fn_did, bb, stmt), (impl_did, genargs))| (bb, stmt, impl_did, genargs))
            .collect();
        out.sort_by_key(|(bb, stmt, _, _)| (*bb, *stmt));

        TagPlan::Tagged(out)
    }

    fn get_trait_defid(&self, assoc_fn_defid: &DefId) -> DefId {
        // Get trait that this function is associated with
        match self.tstore.assoc_fn_traits.get(assoc_fn_defid) {
            Some(trait_defid) => *trait_defid,
            None => panic!("assoc fn {:?} does not point to trait", assoc_fn_defid),
        }
    }

    /// Returns a set of candidate functions (implementation DefIds) given an input set of types
    /// (constraint DefIds)
    ///
    /// If there are no candidates based on input constraints, and this is on the FSA path, add the default
    /// implementation to the returned candidate function set, if there exists one.
    /// For CHA, add the default implementation (if it exists) no matter what.
    // True if ty still contains a free type/const parameter anywhere
    // (previously only an Adt's own top-level Type args were checked, which
    // missed e.g. `Option<<T as Iterator>::Item>` or `Vec<Vec<T>>`).
    fn ty_has_unresolved_param(ty: &Ty) -> bool {
        has_unresolved_param(ty)
    }

    fn get_impls_from_defids(
        &self,
        assoc_fn_defid: &DefId,
        constraint_defids: &Vec<(DefId, Option<GenericArgs>)>,
        _fsa: bool,
    ) -> Vec<(DefId, Option<GenericArgs>)> {
        // CHA-collected defid genargs are None, while FSA-collected defid genargs might be Some().
        // Get every concrete type constraint's impl of this function
        let mut assoc_fn_impls = Vec::new();
        for (_i, (defid, genargs)) in constraint_defids.iter().enumerate() {
            match self.tstore.struct_assoc_fns.get(&(*defid, *assoc_fn_defid)) {
                Some(assoc_fn_impl) => {
                    unique_append(
                        &mut assoc_fn_impls,
                        assoc_fn_impl
                            .clone()
                            .into_iter()
                            .filter_map(|x| {
                                if x == *assoc_fn_defid {
                                    // Inherited default method - x is the trait's own
                                    // decl defid, generic over Self.
                                    let self_ty = match genargs {
                                        Some(g) => AdtDef(*defid).ty_with_args(g),
                                        None => AdtDef(*defid).ty(),
                                    };
                                    if Self::ty_has_unresolved_param(&self_ty) {
                                        // defid itself generic, no concrete
                                        // instantiation available - skip.
                                        None
                                    } else {
                                        Some((
                                            x,
                                            Some(GenericArgs(vec![GenericArgKind::Type(self_ty)])),
                                        ))
                                    }
                                } else {
                                    Some((x, genargs.clone()))
                                }
                            })
                            .collect(),
                    );
                }
                None => {
                    // No impl entry: the only way a bare callable DefId (a
                    // closure or fn item) implements the dispatched method
                    // is through the builtin Fn-family impls. For any other
                    // trait, accepting it here would interpret the closure's
                    // own body as if it were that trait's method.
                    if crate::constraints::is_fn_family_method(assoc_fn_defid)
                        && FnDef(*defid).body().is_some()
                    {
                        // This is a callable item, push the impl defid
                        unique_push(&mut assoc_fn_impls, (*defid, genargs.clone()));
                    }
                }
            }
        }

        assoc_fn_impls
    }

    /// The CHAmax-only mode (`VERIFOPT_CHA_ONLY`): instead of interpreting
    /// from the entry point, every dynamic dispatch in `instances`' bodies
    /// (chamax::compiled_instances - all compiled code, reached by FSA or
    /// not) with its CHA candidates, keyed like interp_virtual_call's sites.
    /// The caller uses these as the "FSA" sets too, so each site gets exactly
    /// what CHAmax allows.
    pub fn enumerate_sites(
        &self,
        instances: &[Instance],
    ) -> HashMap<(DefId, usize, GenericArgs), (Span, Vec<(DefId, Option<GenericArgs>)>)> {
        let mut out = HashMap::new();
        for instance in instances {
            let Ok(Some(body)) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| instance.body())) else {
                continue;
            };
            for (bb, block) in body.blocks.iter().enumerate() {
                let TerminatorKind::Call { func: Operand::Constant(co), .. } = &block.terminator.kind else {
                    continue;
                };
                let TyKind::RigidTy(RigidTy::FnDef(fndef, genargs)) = co.const_.ty().kind() else {
                    continue;
                };
                if has_unresolved_param(&genargs) {
                    continue;
                }
                let is_virtual = matches!(
                    Instance::resolve(fndef, &genargs),
                    Ok(Instance { kind: InstanceKind::Virtual { .. }, .. })
                );
                if !is_virtual {
                    continue;
                }
                let Some(trait_defid) = self.tstore.assoc_fn_traits.get(&fndef.0).copied() else {
                    warn!("[verifopt cha-only] {:?}: no trait for {:?}, site skipped", instance.def.name(), fndef);
                    continue;
                };
                let cha = self.get_impls_cha(&fndef.0, &trait_defid, &genargs);
                out.insert((instance.def.def_id(), bb, instance.args()), (block.terminator.span, cha));
            }
        }
        out
    }

    fn get_impls_cha(
        &self,
        //callee_scope: &VOID,
        assoc_fn_defid: &DefId,
        trait_defid: &DefId,
        call_site_genargs: &GenericArgs,
    ) -> Vec<(DefId, Option<GenericArgs>)> {
        debug!("\n\nGETTING CHA IMPLS");
        let constraint_defids = self.get_cha_tyconstraint_defids(&trait_defid, call_site_genargs);
        //debug!(
        //    "constraint defids ({:?} total): {:?}",
        //    constraint_defids.len(),
        //    constraint_defids
        //);
        self.get_impls_from_defids(assoc_fn_defid, &constraint_defids, false)
    }

    /// get_cha_tyconstraint_defids for a trait object's own args (the
    /// existential trait ref's: no `Self`), compared with each impl's
    /// trait args after `Self`.
    fn get_cha_defids_for_object_args(
        &self,
        trait_defid: &DefId,
        object_args: &GenericArgs,
    ) -> Vec<(DefId, Option<GenericArgs>)> {
        match self.tstore.trait_structs.get(trait_defid) {
            Some(tyconstraints) => tyconstraints
                .iter()
                .filter(|(_, impl_genargs)| {
                    let rest = impl_genargs.0.get(1..).unwrap_or(&[]);
                    rest.len() == object_args.0.len()
                        && object_args.0.iter().zip(rest).all(|(a, b)| generic_arg_may_unify(a, b))
                })
                .map(|(defid, _)| (defid.clone(), None))
                .collect(),
            None => Vec::new(),
        }
    }

    fn get_cha_tyconstraint_defids(
        &self,
        trait_defid: &DefId,
        call_site_genargs: &GenericArgs,
    ) -> Vec<(DefId, Option<GenericArgs>)> {
        // Get concrete type constraints for trait object, filtered to only
        // those whose own trait parameterization (e.g. `Fn<Args>`'s `Args`)
        // matches this call site's - a `Fn<(&u8,)>` impl is never a valid
        // candidate for a `Fn<()>` call site, even though both implement
        // "the same trait" by DefId.
        match self.tstore.trait_structs.get(trait_defid) {
            Some(tyconstraints) => tyconstraints
                .iter()
                .filter(|(struct_defid, impl_genargs)| {
                    Self::trait_params_compatible(call_site_genargs, struct_defid, impl_genargs)
                })
                .map(|(defid, _)| (defid.clone(), None))
                .collect(),
            None => Vec::new(), //panic!("trait {:?} does not point to any structs", trait_defid),
        }
    }

    /// Compares a call site's own generic args (e.g. `std::ops::Fn::call`'s
    /// `[Self, Args]`, where `Self` here is the erased `dyn Trait` object) -
    /// against one specific impl's own `TraitRef` args.
    ///
    /// Must never reject an impl that could be selected (CHA has to stay a
    /// superset), so args are compared with `generic_arg_may_unify`, not `==`:
    /// the call site's lifetimes are erased while the impl's are `'a`/`'static`,
    /// and an impl's args may be generic (`impl<F> Client<F> for X`). Plain
    /// equality rejected every impl of e.g. tock's `uart::Transmit<'a>`.
    fn trait_params_compatible(
        call_site_genargs: &GenericArgs,
        impl_struct_defid: &DefId,
        impl_genargs: &GenericArgs,
    ) -> bool {
        if call_site_genargs.0.len() != impl_genargs.0.len() {
            // Different arities entirely - can't be the same trait
            // instantiation regardless of which position is Self.
            return false;
        }

        for (call_arg, impl_arg) in call_site_genargs.0.iter().zip(impl_genargs.0.iter()) {
            let is_self_position = matches!(
                impl_arg,
                GenericArgKind::Type(ty) if matches!(
                    ty.kind(),
                    TyKind::RigidTy(RigidTy::Adt(adtdef, _)) if adtdef.0 == *impl_struct_defid
                )
            );
            if is_self_position {
                continue;
            }
            if !generic_arg_may_unify(call_arg, impl_arg) {
                return false;
            }
        }
        true
    }

    /// Returns `(is_closure, receiver_is_unresolved_param, impls)`. See
    /// `get_fsa_tyconstraints` for what `receiver_is_unresolved_param` means and why callers must
    /// not conflate it with a genuinely-empty FSA result.
    fn get_impls_fsa(
        &self,
        ctxt: &Context,
        term_span: &Span,
        caller_scope: &VOID,
        local_decls: &[LocalDecl],
        trait_defid: &DefId,
        assoc_fn_defid: &DefId,
        args: &Vec<Operand>,
    ) -> (bool, bool, Vec<(DefId, Option<GenericArgs>)>) {
        debug!("\n\nGETTING FSA IMPLS");
        let place = self.get_traitobj_place(args);
        debug!("traitobj place: {:?}", place);
        let (receiver_is_param, tyconstraints) =
            self.get_fsa_tyconstraints(ctxt, caller_scope, local_decls, place);
        for (i, c) in tyconstraints.inner.iter().enumerate() {
            debug!(
                "[fsa receiver] {:?} disjunct {}: {}",
                caller_scope.0.name(),
                i,
                describe_receiver_constraint(&self.converter, c)
            );
        }
        //debug!("tyconstraints: {:?}", tyconstraints);
        let (is_closure, constraint_defids) =
            self.get_fsa_constraint_defids(term_span, trait_defid, &tyconstraints);
        //debug!(
        //    "constraint defids ({:?} total): {:?}",
        //    constraint_defids.len(),
        //    constraint_defids
        //);
        (
            is_closure,
            receiver_is_param,
            self.get_impls_from_defids(assoc_fn_defid, &constraint_defids, true),
        )
    }

    fn get_traitobj_place(&self, args: &Vec<Operand>) -> Place {
        match &args[0] {
            Operand::Copy(place) | Operand::Move(place) => {
                if !place.projection.is_empty() {
                    panic!("traitobj place has projections");
                }

                place.clone()
            }
            _ => panic!("unexpected operand: {:?}", args[0]),
        }
    }

    fn get_fsa_tyconstraints(
        &self,
        ctxt: &Context,
        caller_scope: &VOID,
        local_decls: &[LocalDecl],
        place: Place,
    ) -> (bool, Constraints) {
        // Get concrete type constraints for trait object
        match ctxt.get_constraints(caller_scope, local_decls, &place, false, Some(self)) {
            Some(constraints) => {
                let is_param = crate::constraints::contains_param(&constraints);
                if is_param {
                    if let Some(tainted) = self.summary_build_taint_stack.borrow_mut().last_mut() {
                        *tainted = true;
                    }
                }
                (is_param, constraints)
            }
            None => {
                debug!(
                    "place {:?} has no constraints - returning empty (FSA will fall back to CHA)",
                    place
                );
                (false, Constraints::new())
            }
        }
    }

    /// For each concrete type constraint, if it contains a type that implements the trait of the
    /// traitobject we are dispatching on, return that type's DefId
    ///
    /// This will later be used to get that type's implementation of the function-to-dispatch
    fn get_fsa_constraint_defids(
        &self,
        term_span: &Span,
        trait_defid: &DefId,
        tyconstraints: &Constraints,
    ) -> (bool, Vec<(DefId, Option<GenericArgs>)>) {
        let mut defids = Vec::new();
        let mut is_closure = false;
        for constraint in tyconstraints.inner.iter() {
            let (is_closure_, res) = self.resolve_defid(term_span, trait_defid, &constraint);
            is_closure = is_closure || is_closure_;
            unique_append(&mut defids, res);
        }
        (is_closure, defids)
    }

    /// If a concrete type constraint contains a type that implements the trait of the
    /// traitobject we are dispatching on, return that type's DefId
    ///
    /// This will later be used to get that type's implementation of the function-to-dispatch
    fn resolve_defid(
        &self,
        term_span: &Span,
        trait_defid: &DefId,
        constraint: &Constraint,
    ) -> (bool, Vec<(DefId, Option<GenericArgs>)>) {
        //debug!("RESOLVE DEFID");

        match constraint {
            Constraint {
                toc: Some(toc_),
                cfc: _,
                prov: _,
            } => {
                if *trait_defid != toc_.0.def.0 {
                    return (false, vec![]);
                }

                match toc_ {
                    (_, TraitObjConstraint::Adt(adtdef, genargs, _, fields)) => {
                        self.resolve_adt_helper(term_span, trait_defid, adtdef, genargs, fields)
                    }
                    (_, TraitObjConstraint::Closure(cdef, genargs, _)) => {
                        if !crate::constraints::closure_can_implement(trait_defid) {
                            return (false, vec![]);
                        }
                        if genargs.0.is_empty() {
                            (true, vec![(cdef.0, None)])
                        } else {
                            (true, vec![(cdef.0, Some(genargs.clone()))])
                        }
                    }
                }
            }
            Constraint {
                toc: None,
                cfc: Some(cfc),
                prov: _,
            } => match cfc {
                RunningConstraint::Adt(adtdef, genargs, _, fields) => {
                    self.resolve_adt_helper(term_span, trait_defid, adtdef, genargs, fields)
                }
                RunningConstraint::Closure(cdef, genargs, _) => {
                    // Reached e.g. via resolve_adt_helper's recursion into an
                    // ADT's fields/genargs: a closure field of an iterator
                    // adapter must not become an `Iterator` candidate.
                    if !crate::constraints::closure_can_implement(trait_defid) {
                        return (false, vec![]);
                    }
                    if genargs.0.is_empty() {
                        (true, vec![(cdef.0, None)])
                    } else {
                        (true, vec![(cdef.0, Some(genargs.clone()))])
                    }
                }
                RunningConstraint::Scalar(_) | RunningConstraint::Float => (false, vec![]),
                // An unknown trait object: any implementor (CHA). Its args are
                // the trait object's - without `Self` - so they're compared
                // against the impls' args after `Self` (with the call-site
                // comparison, whose args start with `Self`, the arities never
                // matched and every impl was dropped: an unknown object
                // contributed no targets at all).
                RunningConstraint::Dynamic(tys) => {
                    match tys.iter().find(|ty| ty.def.0 == *trait_defid) {
                        Some(matching_ty) => (
                            false,
                            self.get_cha_defids_for_object_args(trait_defid, &matching_ty.genargs),
                        ),
                        None => (false, vec![]),
                    }
                }
                RunningConstraint::Ptr(box c) => self.resolve_defid(term_span, trait_defid, c),
                RunningConstraint::Idk(box cs) => {
                    let mut defids = Vec::new();
                    for c in cs.inner.iter() {
                        let (_, res_defids) = self.resolve_defid(term_span, trait_defid, c);
                        unique_append(&mut defids, res_defids);
                    }
                    (false, defids)
                }
                RunningConstraint::Tuple(_)
                | RunningConstraint::List(_)
                | RunningConstraint::Param(..)
                | RunningConstraint::FnDef(..)
                | RunningConstraint::FnPtr(_) => (false, vec![]),
            },
            _ => (false, vec![]),
        }
    }

    fn resolve_adt_helper(
        &self,
        term_span: &Span,
        trait_defid: &DefId,
        adtdef: &AdtDef,
        genargs: &GenericArgs,
        fields: &ADTFields,
    ) -> (bool, Vec<(DefId, Option<GenericArgs>)>) {
        //debug!("\nRESOLVE ADT HELPER");

        let mut resvec = Vec::new();
        match self.tstore.struct_traits.get(&adtdef.0) {
            // Does this ADT implement the desired trait? If so, add to vec
            Some(traits) => {
                if traits.contains(trait_defid) {
                    if genargs.0.is_empty() {
                        unique_push(&mut resvec, (adtdef.0, None));
                    } else {
                        unique_push(&mut resvec, (adtdef.0, Some(genargs.clone())));
                    }
                }
            }
            None => {}
        }

        // If this ADT implements the trait, it *is* the object's concrete
        // type: the vtable is its own, so the dispatch goes to its method and
        // never to a method of something stored inside it. Searching its
        // fields/genargs then only adds unreachable targets - e.g. for a
        // `Map<vec::IntoIter<..>, F>` object (ripgrep's `sort`), the `iter`
        // field's `<vec::IntoIter<..> as Iterator>::next`; for a
        // `Box<Countdown>` object (`box_box_dyn_iter`), `Countdown::next`,
        // which Box's forwarding `next` calls statically, never via this
        // dispatch. Those extras cost precision (a one-target site can't be
        // an Edit::Single), and analysis time (each one gets simulated).
        //
        // The search below stays as the fallback for when this ADT can't be
        // the object: a receiver constraint that is some container of the
        // object, from an imprecise read (e.g. a field read that fell back
        // to the whole enclosing value). Caveat: struct_traits is per ADT,
        // not per instantiation (it records `Box: Iterator` regardless of
        // `impl<I: Iterator> .. for Box<I>`'s bound), so "implements" here
        // is judged per ADT - the same test that already decided whether to
        // push this ADT's own candidate above.
        if !resvec.is_empty() {
            return (false, resvec);
        }

        // Search in fields (in addition to genargs) b/c constraints are already there + don't need
        // to reconstruct them; however, this might pose a termination problem
        for (_key, field_constraints) in fields {
            for fc in field_constraints.inner.iter() {
                let (_is_closure, inner_resvec) = self.resolve_defid(term_span, trait_defid, fc);
                unique_append(&mut resvec, inner_resvec);
            }
        }

        // Also search in genargs for an implementing type
        for genarg in &genargs.0 {
            match self
                .converter
                .convert_genarg(&Location::unknown(), &genarg, None, Some(self))
            {
                Some(genarg_constraint) => {
                    let (_is_closure, inner_resvec) =
                        self.resolve_defid(term_span, trait_defid, &genarg_constraint);
                    unique_append(&mut resvec, inner_resvec);
                }
                _ => {}
            }
        }

        (false, resvec)
    }

    /// For each of the FSA candidate functions to call, resolve into a monomorphic
    /// instance and interpret as if it were a static call
    fn simulate_static_calls(
        &self,
        term_span: &Span,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        assoc_fn_impls: Vec<(DefId, Option<GenericArgs>)>,
        method_genargs: &GenericArgs,
        args: &Vec<Operand>,
        is_closure: bool,
        assoc_fn_defid: &DefId,
        touched_scopes: &mut Vec<(VOID, u64)>,
    ) -> Result<Option<Constraints>, Error> {
        let mut results = Vec::<Option<Constraints>>::new();

        debug!("\nSIMULATING STATIC CALL(S)");
        let len = assoc_fn_impls.len();
        let mut acc: Option<Context> = None;
        // Whether this simulation runs inside an outer one (its context
        // already records writes): if not, the result stops recording when
        // it replaces ctxt below, so the record spans one outermost
        // simulation instead of growing for the rest of the run.
        let origin_tracks_writes = ctxt.cstore.cmap.dirty().is_some();
        // Diagnostics: this simulation's level (sim_levels), popped on exit.
        struct LevelGuard<'a>(&'a RefCell<Vec<(String, usize, usize, usize, usize)>>);
        impl Drop for LevelGuard<'_> {
            fn drop(&mut self) {
                self.0.borrow_mut().pop();
            }
        }
        let diag = cache_sizes_every().is_some();
        let _level = diag.then(|| {
            self.sim_levels.borrow_mut().push((cur_scope.0.name(), 0, len, 0, 0));
            LevelGuard(&self.sim_levels)
        });
        for (i, (assoc_fn_impl, adt_genargs)) in assoc_fn_impls.iter().enumerate() {
            if diag && let Some(l) = self.sim_levels.borrow_mut().last_mut() {
                l.1 = i + 1;
            }
            debug!(
                "\n---ITER {:?} out of {:?} ({:?}/{:?})",
                i,
                len - 1,
                i + 1,
                len
            );
            // Snapshotted so a caught panic (below) can roll `call_stack`/
            // `key_stack` back to exactly this point
            let pre_candidate_call_stack_len = call_stack.len();
            let pre_candidate_key_stack_len = self.key_stack.borrow().len();
            let candidate_result: std::thread::Result<Result<(), Error>> = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| -> Result<(), Error> {
                    let _timing_guard =
                        self.timing_span(TimingCat::TermSimulateCallPrep, cur_scope);
                    let genargs = if *assoc_fn_impl == *assoc_fn_defid {
                        let self_ty = adt_genargs
                            .as_ref()
                            .and_then(|g| g.0.first())
                            .and_then(|k| k.ty())
                            .copied()
                            .expect("get_impls_from_defids should set Self for a default method");
                        // Self isn't necessarily at index 0 - the trait/method
                        // may have leading lifetime params ahead of it, which
                        // must be preserved unchanged (rustc's own type
                        // instantiation panics if a Type arg lands where it
                        // expects a Region). Replace the first Type arg found,
                        // wherever it is, instead of assuming position 0.
                        let mut replaced = false;
                        let mut new_args: Vec<GenericArgKind> = method_genargs
                            .0
                            .iter()
                            .cloned()
                            .map(|arg| {
                                if !replaced && matches!(arg, GenericArgKind::Type(_)) {
                                    replaced = true;
                                    GenericArgKind::Type(self_ty)
                                } else {
                                    arg
                                }
                            })
                            .collect();
                        if !replaced {
                            // No existing Type arg to replace (e.g.
                            // method_genargs was empty) - append instead.
                            new_args.push(GenericArgKind::Type(self_ty));
                        }
                        GenericArgs(new_args)
                    } else if is_closure && adt_genargs.is_some() {
                        GenericArgs(
                            adt_genargs
                                .clone()
                                .unwrap()
                                .0
                                .iter()
                                .chain(method_genargs.0.iter())
                                .cloned()
                                .collect(),
                        )
                    } else if !is_closure {
                        adt_genargs.clone().unwrap_or_else(|| GenericArgs(Vec::new()))
                    } else {
                        method_genargs.clone()
                    };

                    // TODO different resolves for fn_ptr / closure
                    let fndef = FnDef(*assoc_fn_impl);
                    let identity_args = match fndef.ty().kind() {
                        TyKind::RigidTy(RigidTy::FnDef(_, identity_args)) => Some(identity_args),
                        _ => None,
                    };
                    let expected_count = identity_args.as_ref().map(|a| a.0.len()).unwrap_or(0);

                    if genargs.0.len() < expected_count {
                        debug!(
                            "skipping {:?}: only {:?} genargs available but {:?} required, falling back to poly sig",
                            assoc_fn_impl,
                            genargs.0.len(),
                            expected_count
                        );
                        crate::assumptions::record_call(crate::assumptions::Reason::ArgsMismatch, &fndef);
                        results.push(self.retty_fallback_from_poly(fndef.fn_sig()).unwrap());
                        return Ok(());
                    }

                    // Beyond length, each position's *kind* (Type vs Lifetime
                    // vs Const) must match what the callee's real signature
                    // expects. A mismatch here - e.g. a Type landing in a
                    // slot the signature says is a Lifetime - is exactly the
                    // invariant rustc's own type instantiation panics on, so
                    // treat it the same as a length mismatch: skip, don't
                    // hand rustc a malformed args list.
                    let kind_mismatch =
                        identity_args.as_ref().is_some_and(|template| {
                            genargs.0.iter().zip(template.0.iter()).any(|(a, b)| {
                                std::mem::discriminant(a) != std::mem::discriminant(b)
                            })
                        });
                    if kind_mismatch {
                        debug!(
                            "skipping {:?}: genargs kind shape doesn't match callee signature, falling back to poly sig",
                            assoc_fn_impl,
                        );
                        crate::assumptions::record_call(crate::assumptions::Reason::ArgsMismatch, &fndef);
                        results.push(self.retty_fallback_from_poly(fndef.fn_sig()).unwrap());
                        return Ok(());
                    }

                    if has_unresolved_param(&genargs) {
                        debug!(
                            "skipping {:?}: genargs {:?} are not fully monomorphic, falling back to poly sig",
                            assoc_fn_impl, genargs
                        );
                        crate::assumptions::record_call(crate::assumptions::Reason::NonMonomorphicArgs, &fndef);
                        results.push(self.retty_fallback_from_poly(fndef.fn_sig()).unwrap());
                        return Ok(());
                    }

                    let instance_ = match Instance::resolve(fndef, &genargs) {
                        Ok(i) => i,
                        Err(_) => {
                            crate::assumptions::record_call(crate::assumptions::Reason::UnresolvedInstance, &fndef);
                            results.push(self.retty_fallback_from_poly(fndef.fn_sig()).unwrap());
                            return Ok(());
                        }
                    };
                    let (is_virtual, instance) = match instance_.kind {
                        // Likely a default trait method implementation, convert to a concrete InstanceKind
                        // so we can interpret it
                        InstanceKind::Virtual { .. } => (
                            true,
                            Instance {
                                kind: InstanceKind::Item,
                                def: instance_.def,
                            },
                        ),
                        _ => (false, instance_),
                    };
                    let callee_scope = find_active_recursive_scope(call_stack, &instance, &genargs)
                        .unwrap_or_else(|| {
                            (
                                instance.clone(),
                                genargs.clone(),
                                push_caller_context(cur_scope, term_span.clone(), self.call_context_k),
                            )
                        });
                    debug!(
                        "[verifopt debug][simulate_static_calls] assoc_fn_impl={:?} is_virtual={:?} genargs={:?} callee_scope_def={:?} callee_scope_genargs={:?}",
                        assoc_fn_impl,
                        is_virtual,
                        genargs,
                        callee_scope.0.def.def_id(),
                        callee_scope.1,
                    );
                    drop(_timing_guard);

                    // the `if` and `else if` blocks might be creating a soundness error...
                    // if we're not actually stepping into new code + updating our cmap,
                    // we could be omitting actually-used concrete type variants in our
                    // eventual rewrite FIXME
                    let recursive_hit = call_stack.contains(&callee_scope);
                    if recursive_hit {
                        let _timing_guard =
                            self.timing_span(TimingCat::TermSimulateRecursiveFallback, cur_scope);
                        crate::assumptions::record_call(crate::assumptions::Reason::RecursionCut, &fndef);
                        results.push(self.retty_fallback_from_poly(fndef.fn_sig()).unwrap());
                        drop(_timing_guard);
                        return Ok(());
                    }

                    let _timing_guard =
                        self.timing_span(TimingCat::TermSimulateStdlibStub, cur_scope);
                    let stub_attempt = self.stdlib_stub(
                        ctxt,
                        cur_scope,
                        term_span,
                        local_decls,
                        &fndef,
                        &genargs,
                        args,
                    );
                    match stub_attempt {
                        Some(stub_result) => {
                            drop(_timing_guard);
                            results.push(stub_result?);
                        }
                        None => {
                            drop(_timing_guard);

                            if !instance.has_body() {
                                let _timing_guard =
                                    self.timing_span(TimingCat::TermSigFallback, cur_scope);
                                crate::assumptions::record_call(crate::assumptions::Reason::NoMir, &fndef);
                                results
                                    .push(self.retty_fallback_from_poly(fndef.fn_sig()).unwrap());
                                drop(_timing_guard);
                                return Ok(());
                            }

                            // Resolution came back Virtual, i.e. Self is still
                            // `dyn Trait`: the only body available is the
                            // trait's generic, unmonomorphized one
                            // (fndef.body()), which previously got interpreted
                            // here anyway ("FIXME not monomorphized") - and any
                            // layout/resolve/body call on its generic locals
                            // can ICE. Widen to the declared signature instead.
                            if is_virtual {
                                let _timing_guard =
                                    self.timing_span(TimingCat::TermSigFallback, cur_scope);
                                debug!(
                                    "{:?} resolved to a Virtual instance; not interpreting its generic body, falling back to poly sig",
                                    assoc_fn_impl
                                );
                                crate::assumptions::record_call(crate::assumptions::Reason::VirtualInstance, &fndef);
                                results
                                    .push(self.retty_fallback_from_poly(fndef.fn_sig()).unwrap());
                                drop(_timing_guard);
                                return Ok(());
                            }

                            let _timing_guard =
                                self.timing_span(TimingCat::TermSimulateRealCall, cur_scope);
                            let base_wtos_len = ctxt.wtos.len();
                            let base_refs_len = ctxt.cstore.refs.len();
                            // O(1) clone; snapshot for the ptr_eq check below.
                            let wtos_snapshot = ctxt.wtos.clone();
                            let refs_snapshot = ctxt.cstore.refs.clone();
                            let mut ctxt_clone = ctxt.clone();
                            // Record which scopes this candidate writes, so
                            // merging it back visits only those (see
                            // merge_cstores_timed).
                            ctxt_clone.cstore.cmap.track_writes();
                            let mut call_stack_clone = call_stack.clone();

                            let body = self.get_body(&callee_scope);

                            let cs = self.collect_resolved_args(
                                ctxt,
                                term_span,
                                cur_scope,
                                &body,
                                local_decls,
                                args,
                                is_closure,
                            );

                            results.push(self.interp_static_call(
                                term_span,
                                &mut ctxt_clone,
                                &mut call_stack_clone,
                                cur_scope,
                                &callee_scope,
                                local_decls,
                                fndef,
                                args,
                                &genargs,
                                &cs,
                                is_closure,
                            )?);
                            // ptr_eq, not ==: set_wto/add_ref are the only
                            // mutation sites for wtos/refs, and both only
                            // fire on genuinely new info, so an untouched
                            // map stays the same allocation - O(1), sound,
                            // false negatives just fall back to union().
                            let wtos_unchanged = ctxt_clone.wtos.ptr_eq(&wtos_snapshot);
                            let refs_unchanged = ctxt_clone.cstore.refs.ptr_eq(&refs_snapshot);
                            touched_scopes.push((
                                callee_scope.clone(),
                                *self.scope_epoch.borrow().get(&base_scope_id(&callee_scope)).unwrap_or(&0),
                            ));
                            drop(_timing_guard);

                            let _timing_guard = self
                                .timing_span(TimingCat::TermSimulateLoopMergeResults, cur_scope);

                            let _tg = self.timing_span(TimingCat::Take, cur_scope);
                            let taken = acc.take();
                            drop(_tg);

                            acc = Some(match taken {
                                None => ctxt_clone,
                                Some(a) => {
                                    let _tg =
                                        self.timing_span(TimingCat::VecConstruction, cur_scope);
                                    let vec = Vec::from([a, ctxt_clone]);
                                    drop(_tg);

                                    self.merge_contexts_timed(
                                        cur_scope,
                                        vec,
                                        Some(base_wtos_len),
                                        Some(base_refs_len),
                                        Some(wtos_unchanged),
                                        Some(refs_unchanged),
                                    )?
                                    .unwrap()
                                }
                            });
                            drop(_timing_guard);
                            if diag
                                && let Some(a) = &acc
                                && let Some(dirty) = a.cstore.cmap.dirty()
                            {
                                // Sizing the written scopes walks them: sampled.
                                thread_local!(static MERGES: std::cell::Cell<u64> = const { std::cell::Cell::new(0) });
                                let sample = MERGES.with(|m| {
                                    m.set(m.get() + 1);
                                    m.get() % 32 == 0
                                });
                                let nodes = if !sample {
                                    self.sim_levels.borrow().last().map_or(0, |l| l.4)
                                } else { dirty
                                    .iter()
                                    .filter_map(|k| a.cstore.cmap.get(k))
                                    .map(|v| match &**v {
                                        MapValue::Constraints(cs) => crate::constraints::constraints_size(cs),
                                        MapValue::Store(inner, _) => crate::constraints::store_size(inner).1,
                                    })
                                    .sum::<usize>() };
                                if let Some(l) = self.sim_levels.borrow_mut().last_mut() {
                                    l.3 = dirty.len();
                                    l.4 = nodes;
                                }
                            }
                        }
                    }
                    Ok(())
                }),
            );

            match candidate_result {
                Ok(Ok(())) => {}
                Ok(Err(e)) => return Err(e),
                Err(panic_payload) => {
                    let msg = panic_payload
                        .downcast_ref::<&str>()
                        .map(|s| s.to_string())
                        .or_else(|| panic_payload.downcast_ref::<String>().cloned())
                        .unwrap_or_else(|| "<non-string panic payload>".to_string());
                    warn!(
                        "candidate {:?} panicked during simulation, widening its result to the \
                         dispatched method's declared return type: {}",
                        assoc_fn_impl, msg
                    );

                    // Previously nothing was pushed here, so a genuine target
                    // that panicked silently contributed *no* return value to
                    // the merge - dropping whatever trait objects it returns
                    // from every downstream dispatch. Push the dispatched
                    // (trait) method's own declared return type instead: that
                    // is always a real fn signature (unlike assoc_fn_impl,
                    // which can be a closure DefId), and converting it never
                    // calls back into rustc (convert_ty maps Param/Alias to an
                    // unconstrained value).
                    //
                    // NOTE this only restores the *return value*. Writes the
                    // candidate made through pointers into caller state lived
                    // in its ctxt_clone, which is discarded along with the
                    // panic, so recovery is still not fully sound - the warn!
                    // above says which candidate to investigate.
                    crate::assumptions::record_call(crate::assumptions::Reason::CandidatePanicked, &FnDef(*assoc_fn_defid));
                    let fallback = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        self.retty_fallback_from_poly(FnDef(*assoc_fn_defid).fn_sig())
                    }));
                    match fallback {
                        Ok(Ok(res)) => results.push(res),
                        _ => results.push(Some(Constraints::from(Constraint::new(None, None)))),
                    }

                    debug!(
                        "STACK STATE before catch_unwind-recovery truncate: call_stack.len()={} key_stack.len()={} target_len={}",
                        call_stack.len(),
                        self.key_stack.borrow().len(),
                        pre_candidate_call_stack_len
                    );
                    call_stack.truncate(pre_candidate_call_stack_len);
                    self.key_stack
                        .borrow_mut()
                        .truncate(pre_candidate_key_stack_len);
                    note_stack_op("panic-recovery truncate", cur_scope, call_stack.len(), self.key_stack.borrow().len());
                    self.assert_stacks_synced(
                        call_stack,
                        "simulate_static_calls catch_unwind recovery",
                    );
                }
            }
        }

        let _timing_guard = self.timing_span(TimingCat::TermSimulateMergeResults, cur_scope);
        match acc {
            Some(mut acc) => {
                if !origin_tracks_writes {
                    acc.cstore.cmap.stop_tracking();
                }
                *ctxt = acc
            }
            None => {}
        }
        let result = self.merge_results_and_ret(&mut results, cur_scope);
        drop(_timing_guard);
        result
    }

    /// Instrumented equivalent of `Vec<Context>::merge()`
    /// Counts how many keys `a` and `b` have in common, iterating whichever
    /// is smaller. Used purely for diagnostics (see the `MERGE_OVERLAP_STATS`
    /// log lines below) - this is *not* free (O(min(len)) lookups into the
    /// other map), so it's a real, temporary addition to the run's cost
    /// while this specific investigation is ongoing, not something to
    /// leave on permanently.
    fn count_shared_keys<K, V>(a: &ImHashMap<K, V>, b: &ImHashMap<K, V>) -> usize
    where
        K: std::hash::Hash + Eq + Clone,
        V: Clone,
    {
        let (smaller, larger) = if a.len() <= b.len() { (a, b) } else { (b, a) };
        smaller.keys().filter(|k| larger.contains_key(k)).count()
    }

    /// Keys present in both `a` and `b` whose values genuinely differ
    /// (`PartialEq`, not just present). Used specifically to detect a
    /// real `union()` collision - not just "this key exists on both
    /// sides" (which `count_shared_keys` already covers and is the
    /// overwhelmingly common, harmless case per the earlier overlap
    /// investigation), but "the two sides disagreed about what this key
    /// maps to, and `union()` is about to arbitrarily keep one and
    /// discard the other." See `wtos_merge_conflicts`.
    fn find_conflicting_keys<K, V>(a: &ImHashMap<K, V>, b: &ImHashMap<K, V>) -> Vec<K>
    where
        K: std::hash::Hash + Eq + Clone,
        V: Clone + PartialEq,
    {
        let (smaller, larger) = if a.len() <= b.len() { (a, b) } else { (b, a) };
        smaller
            .iter()
            .filter_map(|(k, v)| match larger.get(k) {
                Some(other_v) if other_v != v => Some(k.clone()),
                _ => None,
            })
            .collect()
    }

    fn merge_contexts_timed(
        &self,
        scope: &VOID,
        contexts: Vec<Context>,
        // The shared-ancestor `wtos` size for the *last* context in
        // `contexts` (the only caller passes exactly 2, so this
        // corresponds 1:1 to the single wtos-union iteration below) - see
        // where this gets computed in `simulate_static_calls`, right
        // before that context's own clone is taken. `None` when the
        // caller has no meaningful ancestor to report (e.g. the final
        // top-level merge in `merge_results_and_ret`'s caller).
        rhs_base_wtos_len: Option<usize>,
        // Same idea, for `refs` (threaded through to `merge_cstores_timed`).
        rhs_base_refs_len: Option<usize>,
        // `Some(true)` when a full equality check (not just a size
        // comparison) confirmed this candidate's `wtos` is identical to
        // what it started from before its own simulation ran - see
        // `simulate_static_calls`. When true, the union() below is
        // skipped entirely: the accumulator is already a strict superset
        // (union() only ever adds keys), so there's nothing this
        // candidate could contribute that isn't already reflected.
        rhs_wtos_unchanged: Option<bool>,
        // Same idea, for `refs` (threaded through to `merge_cstores_timed`).
        rhs_refs_unchanged: Option<bool>,
    ) -> Result<Option<Context>, Error> {
        let _timing_guard = self.timing_span(TimingCat::TermMergeContextsSetup, scope);
        if contexts.is_empty() {
            return Ok(None);
        }
        if contexts.len() == 1 {
            // Ownership means this is a move, not a clone - `contexts[0].clone()`
            // used to be unavoidable here since `contexts` was only borrowed.
            return Ok(Some(contexts.into_iter().next().unwrap()));
        }
        drop(_timing_guard);

        // Splitting each `Context` into its two fields, by move, instead of
        // cloning `.cstore`/`.wtos` off a borrowed slice - see
        // `merge_cstores_timed` and the `wtos` union loop below for why this
        // matters: whichever side arrives here uniquely owned (refcount 1)
        // lets `im::HashMap`'s copy-on-write `insert`/`entry` skip the
        // forced-copy path entirely, rather than just deferring that same
        // cost from this "clone" step into the very next "union" step.
        let mut cstores = Vec::with_capacity(contexts.len());
        let mut wtos_list = Vec::with_capacity(contexts.len());
        for ctxt in contexts.into_iter() {
            cstores.push(ctxt.cstore);
            wtos_list.push(ctxt.wtos);
        }

        let _timing_guard = self.timing_span(TimingCat::TermMergeCstoresMerge, scope);
        let m_cstores =
            self.merge_cstores_timed(scope, cstores, rhs_base_refs_len, rhs_refs_unchanged);
        drop(_timing_guard);

        let mut wtos_iter = wtos_list.into_iter();
        // Moved, not cloned - this used to be `contexts[0].wtos.clone()`.
        let mut m_wtos = wtos_iter.next().unwrap();

        let _timing_guard = self.timing_span(TimingCat::TermMergeWtosUnion, scope);
        for wtos in wtos_iter {
            if rhs_wtos_unchanged == Some(true) {
                // Verified unchanged (full equality, not just size) at
                // the point this candidate's simulation finished - skip
                // the union entirely, `m_wtos` already reflects
                // everything `wtos` has to contribute. Nothing to log:
                // there's no overlap/conflict question to ask about a
                // side that's provably identical to a subset of the
                // accumulator.
                continue;
            }

            let shared = Self::count_shared_keys(&m_wtos, &wtos);
            let ptr_identical = m_wtos.ptr_eq(&wtos);
            let rhs_len = wtos.len();
            let new_entries = rhs_base_wtos_len.map(|base| rhs_len.saturating_sub(base));
            debug!(
                "MERGE_OVERLAP_STATS kind=wtos bb_visit={} lhs_len={} rhs_len={} shared={} ptr_identical={} rhs_base_len={} rhs_new_entries={}",
                *self.bb_visit_count.borrow(),
                m_wtos.len(),
                rhs_len,
                shared,
                ptr_identical,
                rhs_base_wtos_len
                    .map(|b| b.to_string())
                    .unwrap_or_else(|| "n/a".to_string()),
                new_entries
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| "n/a".to_string()),
            );

            // Diagnostic-only: same O(min(len)) cost caveat as
            // `count_shared_keys` above. Records which scopes' `wtos`
            // entries are *actually* about to have one side's progress
            // discarded by the `union()` below (not just "present on both
            // sides," which per the earlier overlap investigation is the
            // overwhelmingly common, harmless case) - see `visit_body`
            // for where this gets checked against the specific pattern
            // that could plausibly cause silent under-processing.
            //
            // Mirrors `REFS MERGE CONFLICT` below exactly - MERGE_OVERLAP_STATS
            // above only reports *shared* keys, which per that same overlap
            // investigation are overwhelmingly harmless (identical values on
            // both sides); this reports specifically the subset where the
            // values actually disagree, which is what `union()` silently
            // discards one side of.
            let wtos_conflicts = Self::find_conflicting_keys(&m_wtos, &wtos);
            if !wtos_conflicts.is_empty() {
                let detailed: Vec<_> = wtos_conflicts
                    .iter()
                    .map(|k| (k, m_wtos.get(k), wtos.get(k)))
                    .collect();
                panic!(
                    "WTOS MERGE CONFLICT: bb_visit={} {} scope(s) had disagreeing BBDeps - \
                     union() would silently discard one side's traversal state.\n\
                     (scope, existing_BBDeps_kept, incoming_BBDeps_discarded): {:#?}",
                    *self.bb_visit_count.borrow(),
                    wtos_conflicts.len(),
                    detailed
                );
            }
            for conflicting_scope in wtos_conflicts {
                self.wtos_merge_conflicts
                    .borrow_mut()
                    .insert(conflicting_scope);
            }

            m_wtos = m_wtos.union(wtos);
        }
        drop(_timing_guard);

        let _timing_guard = self.timing_span(TimingCat::TermMergeNewContext, scope);
        let c = Context::new(m_cstores, m_wtos);
        drop(_timing_guard);

        Ok(Some(c))
    }

    /// Replaces the free function `merge_stores` (in merge.rs, now removed)
    /// + its private `merge_stores_helper`, which built a 2-element `Vec`
    /// and dispatched to the generic `impl Merge<ConstraintStore> for
    /// Vec<ConstraintStore>` trait impl - a *second*, completely separate
    /// place `refs` got unioned, with none of the instrumentation
    /// `merge_cstores_timed` has (including the `REFS MERGE CONFLICT`
    /// detection). Pulling this in as a method means both callers -
    /// `simulate_static_calls`'s candidate merging, and this function's
    /// caller `resolve_args`, invoked on every function call - now go
    /// through the exact same instrumented path, rather than maintaining
    /// two implementations of the same merge logic with only one of them
    /// visible to diagnostics.
    pub(crate) fn merge_stores_timed(
        &self,
        scope: &VOID,
        cur_store: &ConstraintStore,
        cur_es: &EnclosingScopes,
        new_store: &ConstraintStore,
        new_es: &EnclosingScopes,
    ) -> (ConstraintStore, EnclosingScopes) {
        let _tg = self.timing_span(TimingCat::TermMergeStoresEs, scope);
        let merged_es = crate::constraints::merge_enclosing_scopes(cur_es, new_es);
        drop(_tg);

        let merged_store = self.merge_cstores_timed(
            scope,
            vec![cur_store.clone(), new_store.clone()],
            None,
            None,
        );

        (merged_store, merged_es)
    }

    /// Instrumented equivalent of `Vec<ConstraintStore>::merge()` (see
    /// `merge.rs`), called from `merge_contexts_timed` above and from
    /// `merge_stores_timed` below. Takes `stores` by value (not
    /// `&[ConstraintStore]`) for the same reason `merge_contexts_timed`
    /// now takes `contexts` by value - see the comments there.
    fn merge_cstores_timed(
        &self,
        scope: &VOID,
        stores: Vec<ConstraintStore>,
        _rhs_base_refs_len: Option<usize>,
        // `Some(true)` when a full equality check confirmed this
        // candidate's `refs` is identical to what it started from before
        // its own simulation ran - see `simulate_static_calls` and the
        // matching `rhs_wtos_unchanged` parameter on
        // `merge_contexts_timed`. When true, the `refs` union below is
        // skipped (the per-key `cmap` merge still happens - that's a
        // separate concern from `refs`).
        rhs_refs_unchanged: Option<bool>,
    ) -> ConstraintStore {
        let mut stores_iter = stores.into_iter();
        // Moved, not cloned - this used to be `stores[0].clone()`.
        let mut merged = stores_iter.next().unwrap();

        for store in stores_iter {
            let _timing_guard = self.timing_span(TimingCat::TermMergeIdentityCheck, scope);
            let identical = merged.cmap.ptr_eq(&store.cmap)
                && (merged.refs.ptr_eq(&store.refs) || merged.refs == store.refs);
            drop(_timing_guard);
            if identical {
                continue;
            }

            // HERE
            let _timing_guard = self.timing_span(TimingCat::TermMergePerKeyMapvals, scope);
            // When both sides record their writes, they come from the same
            // context (candidate clones of it, or merges of those), so an
            // entry neither side wrote is still that context's on both and
            // needs nothing: visit only the union of the written keys. Both
            // sides, not just `store`'s: `merged` may hold an earlier
            // candidate's strong update that must be joined with the
            // original value `store` still has.
            let written: Option<Vec<MapKey>> = match (merged.cmap.dirty(), store.cmap.dirty()) {
                (Some(a), Some(b)) => Some(a.clone().union(b.clone()).into_iter().collect()),
                _ => None,
            };
            let entries: Box<dyn Iterator<Item = (MapKey, &Box<MapValue>)>> = match &written {
                Some(keys) => Box::new(
                    keys.iter().filter_map(|k| store.cmap.get(k).map(|v| (k.clone(), v))),
                ),
                None => Box::new(store.cmap.iter().map(|(k, v)| (k.clone(), v))),
            };
            for (key, val) in entries {
                let key = &key;
                // `get`, not `get_mut`: the value is only read, and get_mut
                // copies the map's nodes on the way down (copy-on-write).
                match merged.cmap.get(key) {
                    // A candidate's context is a persistent clone of the
                    // base, so most entries are still the very same value on
                    // both sides; merging a value with itself gives it back,
                    // so skip those instead of re-merging and re-inserting
                    // every scope's every variable at every merge.
                    Some(merged_val) if crate::merge::mapvals_identical(merged_val, val) => {}
                    Some(merged_val) => {
                        let _tg = self.timing_span(TimingCat::TermMergeMapsvalsMerge, scope);
                        let new_merged_val =
                            crate::merge::merge_mapvals(merged_val, val, Some((self, scope)));
                        drop(_tg);
                        merged.cmap.insert(key.clone(), Box::new(new_merged_val));
                    }
                    None => {
                        merged.cmap.insert(key.clone(), val.clone());
                    }
                }
            }
            drop(_timing_guard);

            // HERE - `store.refs` moved directly into `union`, not cloned
            // first (used to be `store.refs.clone()` under
            // `TermMergeRefsClone`, now gone - nothing left to clone).
            let _timing_guard = self.timing_span(TimingCat::TermMergeRefsUnion, scope);
            if rhs_refs_unchanged == Some(true) {
                // Verified unchanged (full equality, not just size) at
                // the point this candidate's simulation finished - skip
                // the union entirely, `merged.refs` already reflects
                // everything `store.refs` has to contribute. Nothing to
                // log or check for conflicts: a side that's provably
                // identical to a subset of the accumulator can't
                // introduce a genuine collision.
                drop(_timing_guard);
                continue;
            }
            //let shared = Self::count_shared_keys(&merged.refs, &store.refs);
            //let ptr_identical = merged.refs.ptr_eq(&store.refs);
            //let rhs_len = store.refs.len();
            //let new_entries = rhs_base_refs_len.map(|base| rhs_len.saturating_sub(base));
            //debug!(
            //    "MERGE_OVERLAP_STATS kind=refs bb_visit={} lhs_len={} rhs_len={} shared={} ptr_identical={} rhs_base_len={} rhs_new_entries={}",
            //    *self.bb_visit_count.borrow(),
            //    merged.refs.len(),
            //    rhs_len,
            //    shared,
            //    ptr_identical,
            //    rhs_base_refs_len.map(|b| b.to_string()).unwrap_or_else(|| "n/a".to_string()),
            //    new_entries.map(|n| n.to_string()).unwrap_or_else(|| "n/a".to_string()),
            //);

            // Same idea as the `wtos` conflict tracking above, but also
            // logged immediately here (not just recorded for a later
            // read-side check) since `resolve_ref`/`resolve_mut_ref` can't
            // conveniently check `refs_merge_conflicts` themselves - see
            // the field doc. Unlike before, this is no longer a
            // panic-worthy situation: `refs`'s own value is now a *set*
            // of possible alias targets (see its field doc), so two
            // disagreeing values from different merge sources just get
            // unioned together below, retaining both rather than being
            // forced to discard one. This debug log is purely
            // diagnostic now - how often genuinely different
            // alias-target sets get combined - not a correctness signal.
            // Diagnostics only, so only with debug logging on: the scan
            // walks the smaller alias map on every context merge (once per
            // simulated dispatch candidate) - over a quarter of the CPU in
            // a ripgrep run - and refs_merge_conflicts is never read.
            if log::log_enabled!(log::Level::Debug) {
                let refs_conflicts = Self::find_conflicting_keys(&merged.refs, &store.refs);
                if !refs_conflicts.is_empty() {
                    debug!(
                        "REFS MERGE: bb_visit={} {} key(s) had disagreeing alias-target sets \
                         (both retained via union, not discarded)",
                        *self.bb_visit_count.borrow(),
                        refs_conflicts.len(),
                    );
                }
                for conflicting_key in refs_conflicts {
                    self.refs_merge_conflicts
                        .borrow_mut()
                        .insert(conflicting_key);
                }
            }

            merged.refs = crate::constraints::union_refs(merged.refs, store.refs);
            drop(_timing_guard);
        }

        merged
    }

    fn merge_results_and_ret(
        &self,
        results: &mut Vec<Option<Constraints>>,
        cur_scope: &VOID,
    ) -> Result<Option<Constraints>, Error> {
        let filtered_results: Vec<Constraints> = results
            .into_iter()
            .filter(|option| option.is_some())
            .map(|x| x.clone().unwrap())
            .collect();

        match filtered_results.merge(Some((self, cur_scope))) {
            Ok(Some(merged_constraints)) => {
                return Ok(Some(merged_constraints));
            }
            Ok(None) => Ok(None),
            Err(_) => panic!(),
        }
    }

    fn interp_switchint(
        &self,
        ctxt: &mut Context,
        cur_scope: &VOID,
        local_decls: &[LocalDecl],
        bb: usize,
        bb_deps: &mut BBDeps,
        discr: &Operand,
        targets: &SwitchTargets,
    ) -> Result<Option<Constraints>, Error> {
        match discr {
            Operand::Copy(place) | Operand::Move(place) => {
                match ctxt.get_constraints(cur_scope, local_decls, place, false, Some(self)) {
                    // No value at all (a read that found nothing): unknown, so
                    // every branch is possible - pruning them all made the
                    // code after the switch unreachable, and everything it
                    // calls unanalyzed (tock's IPC `command` closures).
                    Some(constraints) if constraints.inner.is_empty() => {
                        if scope_matches_trace_filter(cur_scope) {
                            log::warn!(
                                "[verifopt trace] {:?} bb{bb}: switch on {:?} with no value - all branches kept",
                                cur_scope.0.name(),
                                place
                            );
                        }
                    }
                    Some(constraints) => {
                        if crate::constraints::contains_param(&constraints) {
                            if let Some(tainted) =
                                self.summary_build_taint_stack.borrow_mut().last_mut()
                            {
                                *tainted = true;
                            }
                        }

                        // Create a byte-map for finding statically-impossible successors
                        let mut discr_vals_uninit = Box::<[u8]>::new_zeroed_slice(targets.len());
                        let discr_vals = discr_vals_uninit.write_filled(0);

                        // Populate byte-map with possible branch values, based on constraints
                        self.set_bytemap(&constraints, targets, discr_vals);
                        if scope_matches_trace_filter(cur_scope) {
                            log::warn!(
                                "[verifopt trace] {:?} bb{bb}: switch on {:?} = {} -> branch counts {:?}",
                                cur_scope.0.name(),
                                place,
                                format!("{:?}", constraints).chars().take(300).collect::<String>(),
                                &discr_vals[..]
                            );
                        }

                        self.prune_switchint_targets(
                            bb,
                            bb_deps,
                            &targets.all_targets(),
                            discr_vals,
                        );
                    }
                    None => {}
                }
            }
            Operand::Constant(_co) => {}
            _ => {}
        }

        Ok(None)
    }

    fn set_bytemap(
        &self,
        constraints: &Constraints,
        targets: &SwitchTargets,
        discr_vals: &mut [u8],
    ) {
        if constraints.is_empty() {
            // Increment all branch counters (since no statically-known discr value)
            for (i, _) in targets.branches().enumerate() {
                discr_vals[usize::try_from(i).unwrap()] += 1;
            }
            discr_vals[discr_vals.len() - 1] += 1;
            return;
        }

        for constraint in constraints.inner.iter() {
            match constraint {
                Constraint {
                    toc: _,
                    cfc: Some(RunningConstraint::Scalar(num_opt)),
                    prov: _,
                } => {
                    if let Some(num) = num_opt {
                        // Increment matching branch counters
                        let mut set = false;
                        for (i, (val, _bb)) in targets.branches().enumerate() {
                            if *num == <u128 as TryInto<i128>>::try_into(val).unwrap() {
                                discr_vals[usize::try_from(i).unwrap()] += 1;
                                set = true;
                            }
                        }
                        if !set {
                            discr_vals[discr_vals.len() - 1] += 1;
                        }
                    } else {
                        // Increment all branch counters (since no statically-known discr value)
                        for (i, _) in targets.branches().enumerate() {
                            discr_vals[usize::try_from(i).unwrap()] += 1;
                        }
                        discr_vals[discr_vals.len() - 1] += 1;
                    }
                }
                _ => {
                    // Increment all branch counters (since no statically-known discr value)
                    for (i, _) in targets.branches().enumerate() {
                        discr_vals[usize::try_from(i).unwrap()] += 1;
                    }
                    discr_vals[discr_vals.len() - 1] += 1;
                }
            }
        }
    }

    fn prune_switchint_targets(
        &self,
        bb: usize,
        bb_deps: &mut BBDeps,
        targets: &Successors,
        discr_vals: &mut [u8],
    ) {
        let prunable_indices_opt = self.get_prunable_indices(discr_vals);

        if let Some(prunable_indices) = prunable_indices_opt {
            // Some prunable items point to bbs that are also pointed to by non-prunable items,
            // so need to check that prunable bbs are ONLY pointed to by prunable items
            let mut keep_targets = Vec::new();
            let mut prunable_targets = Vec::new();

            // First collect all targets to keep
            for (i, target) in targets.iter().enumerate() {
                if !prunable_indices.contains(&i) {
                    keep_targets.push(target.clone());
                }
            }

            // Then collect all targets we might be able to prune
            for prune_idx in prunable_indices {
                prunable_targets.push(targets[prune_idx]);
            }

            // Finally, prune all targets that are _not_ in the list of targets to keep
            for prunable_target in prunable_targets {
                if keep_targets.contains(&prunable_target) {
                    continue;
                }

                bb_deps.prune(bb, prunable_target);
            }
        }
    }

    fn get_prunable_indices(&self, discr_vals: &mut [u8]) -> Option<Vec<usize>> {
        let mut poss_idxs = Vec::new();
        let mut imposs_idxs = Vec::new();
        for i in 0..discr_vals.len() {
            if discr_vals[i] > 0 {
                poss_idxs.push(i);
            } else {
                imposs_idxs.push(i);
            }
        }

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

    fn reinterp_recursive(
        &self,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        callee_scope: &VOID,
        callee_cs: &[Constraints],
    ) -> Result<Option<Constraints>, Error> {
        if scope_matches_trace_filter(callee_scope) {
            debug!(
                "TRACE reinterp_recursive ENTER for {:?}: callee_scope={:?} callee_cs={:?}",
                callee_scope.0.name(),
                callee_scope,
                callee_cs
            );
        }
        let mut substore = ConstraintStore::new();
        for (i, cs) in callee_cs.iter().enumerate() {
            let place = Place {
                local: i + 1,
                projection: vec![],
            };
            substore.cmap.insert(
                MapKey::Var(place.clone()),
                Box::new(MapValue::Constraints(cs.clone())),
            );
        }
        ctxt.cstore.cmap.insert(
            MapKey::ScopeId(base_scope_id(callee_scope)),
            Box::new(MapValue::Store(substore, None)),
        );

        let key = (callee_scope.clone(), ArgSet::new(&callee_cs));
        let saved_key_stack = self.key_stack.replace(
            std::iter::repeat_with(|| key.clone())
                .take(call_stack.len())
                .collect(),
        );
        note_stack_op("reinterp swap", callee_scope, call_stack.len(), self.key_stack.borrow().len());

        // Re-interpreted on its own, with no caller: writes through its
        // pointer parameters end at its own parameter locals, so those are
        // this frame's ports (operand i <-> local i + 1, as in substore).
        let frame = crate::write_log::push(
            (0..callee_cs.len()).map(|i| (i, Place { local: i + 1, projection: vec![] })),
            callee_scope,
        );
        self.prepare_call(call_stack, &key);
        let body = self.get_body(callee_scope);
        let refined = match self.visit_body(ctxt, call_stack, callee_scope, &body) {
            Ok(r) => r,
            Err(e) => {
                self.key_stack.replace(saved_key_stack);
                note_stack_op("reinterp restore (err)", callee_scope, call_stack.len(), self.key_stack.borrow().len());
                return Err(e);
            }
        };
        self.key_stack.replace(saved_key_stack);
        note_stack_op("reinterp restore", callee_scope, call_stack.len(), self.key_stack.borrow().len());

        if scope_matches_trace_filter(callee_scope) {
            debug!(
                "TRACE reinterp_recursive REFINED for {:?}: callee_scope={:?} refined={:?}",
                callee_scope.0.name(),
                callee_scope,
                refined
            );
        }

        // publish refined summary for widened constraints
        self.summaries.borrow_mut().insert(
            (base_scope_id(&key.0), key.1.clone()),
            (refined.clone().unwrap_or_default(), Rc::new(frame.finish())),
        );

        Ok(refined)
    }

    fn interp_return(
        &self,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
    ) -> Result<Option<Constraints>, Error> {
        debug!("RETURNING from scope {:?}...", cur_scope.0.name());
        log_call_stack(call_stack);

        let ret_place = Place {
            local: 0,
            projection: vec![],
        };

        // Get and "return" the constraints at Place(0)
        let _timing_guard = self.timing_span(TimingCat::TermReturnScopedGet, cur_scope);
        let scoped_get_result =
            ctxt.cstore
                .scoped_get(cur_scope, &MapKey::Var(ret_place.clone()), false);
        drop(_timing_guard);

        let retval = match scoped_get_result {
            Some(retval) => match retval {
                MapValue::Constraints(retval_constraints) => {
                    //debug!(
                    //    "\n###### RETURNING constraints:\n\t{:?}\n\n",
                    //    retval_constraints
                    //);
                    debug!(
                        "RETURNING scope={:?} local={} disjuncts={}",
                        cur_scope.0.name(),
                        ret_place.local,
                        crate::constraints::constraints_size(&retval_constraints)
                    );
                    Some(retval_constraints)
                }
                _ => panic!("should not be returning a scope"),
            },
            None => {
                // TODO Double check that nothing _needs_ to be returned (for interp correctness)
                None
            }
        };

        // Before finish_frame drops this scope's alias edges.
        let (visited, targets) = ctxt.cstore.alias_walk(ret_place, cur_scope.clone());
        if visited.len() > 1 {
            // The visited places (for the operands) and the static places
            // reached (statics are the same place for the caller).
            let statics: ImHashSet<(Place, VOID)> = targets
                .into_iter()
                .filter(|(_, s)| crate::constraints::static_of_scope(s).is_some())
                .collect();
            let mut walks = self.return_walks.borrow_mut();
            let entry = walks.entry(cur_scope.clone()).or_default();
            entry.0.extend(visited);
            entry.1.extend(statics);
        }

        let _timing_guard = self.timing_span(TimingCat::TermReturnFinishFrame, cur_scope);
        let result = self.finish_frame(ctxt, call_stack, cur_scope, retval);
        drop(_timing_guard);
        result
    }

    /// Returns from `cur_scope` (finish_frame_inner: queued recursive calls
    /// re-interpreted, the scope re-visited if needed), then drops its alias
    /// edges if it's no longer on the call stack - otherwise they pile up for
    /// every scope and call context ever entered. Safe for the fixpoint: a
    /// queued recursive call is only ever re-interpreted while the scope it
    /// recursed into is still on the stack (it was queued because that scope
    /// was active, and is processed when a frame below it finishes), so its
    /// aliases are still there; a later call registers them afresh.
    fn finish_frame(
        &self,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        retval: Option<Constraints>,
    ) -> Result<Option<Constraints>, Error> {
        let result = self.finish_frame_inner(ctxt, call_stack, cur_scope, retval);
        if !call_stack.contains(cur_scope) {
            ctxt.cstore.drop_scope_refs(cur_scope);
        }
        result
    }

    fn finish_frame_inner(
        &self,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        retval: Option<Constraints>,
    ) -> Result<Option<Constraints>, Error> {
        let _timing_guard = self.timing_span(TimingCat::TermFinishFrameGetKey, cur_scope);
        let key = self.key_stack.borrow().last().cloned().unwrap();
        drop(_timing_guard);

        let _timing_guard =
            self.timing_span(TimingCat::TermFinishFrameFirstPrepareReturn, cur_scope);
        let old_scope = self.prepare_return(call_stack);
        if old_scope.clone().unwrap() != *cur_scope {
            log_call_stack(call_stack);
            panic!("call stack out of sorts");
        }
        drop(_timing_guard);

        let _timing_guard = self.timing_span(TimingCat::TermFinishFrameDequeue, cur_scope);
        let queued = self.wq.borrow_mut().remove(&key).unwrap_or_default();
        drop(_timing_guard);

        let _timing_guard = self.timing_span(TimingCat::TermFinishFrameEarlyRet, cur_scope);
        if self.in_queue.borrow().contains(&key) || queued.is_empty() {
            // use summary version OR
            // no recursive calls, no recursed interp needed
            if scope_matches_trace_filter(cur_scope) {
                debug!(
                    "TRACE finish_frame EARLY RETURN (no reprocessing) for {:?}: cur_scope={:?} retval={:?}",
                    cur_scope.0.name(),
                    cur_scope,
                    retval
                );
            }
            return Ok(retval);
        }
        drop(_timing_guard);

        // about to be queueing, set to prevent infinite recursion
        let _timing_guard = self.timing_span(TimingCat::TermFinishFramePrepareQueue, cur_scope);
        self.in_queue.borrow_mut().insert(key.clone());
        drop(_timing_guard);

        // janky method to preserve stores conflicting on voids but not keys
        let _timing_guard =
            self.timing_span(TimingCat::TermFinishFramePreserveVoidConflicts, cur_scope);
        let saved: Vec<(VOID, Option<Box<MapValue>>)> = queued
            .iter()
            .map(|(scope, _, _)| {
                (
                    scope.clone(),
                    ctxt.cstore
                        .cmap
                        .get(&MapKey::ScopeId(base_scope_id(scope)))
                        .cloned(),
                )
            })
            .collect();
        drop(_timing_guard);

        *self.rec_depth.borrow_mut() += 1;
        let _timing_guard = self.timing_span(TimingCat::TermFinishFrameReinterp, cur_scope);
        for (scope, constraints, stack) in queued {
            let depth = call_stack.len();

            // reevaluate recursive calls
            let restored = stack;
            let res = self.reinterp_recursive(ctxt, &mut restored.clone(), &scope, &constraints);

            if matches!(res, Err(Error::RecurseLimit(_))) {
                debug!(
                    "STACK STATE before RecurseLimit truncate: call_stack.len()={} key_stack.len()={} target_depth={}",
                    call_stack.len(),
                    self.key_stack.borrow().len(),
                    depth
                );
                // truncate to before call on error
                call_stack.truncate(depth);
                self.key_stack.borrow_mut().truncate(depth);
                note_stack_op("RecurseLimit truncate", cur_scope, call_stack.len(), self.key_stack.borrow().len());
                self.assert_stacks_synced(call_stack, "finish_frame RecurseLimit truncate");

                self.incomplete.borrow_mut().insert(cur_scope.clone());

                *self.rec_depth.borrow_mut() -= 1;
                // `_timing_guard` (TermFinishFrameReinterp) drops here
                // automatically, correctly closing this span even on this
                // early-return exit.
                return res;
            }
        }
        drop(_timing_guard);

        let _timing_guard = self.timing_span(TimingCat::TermFinishFrameInsertScope, cur_scope);
        for (scope, old) in saved {
            match old {
                Some(v) => {
                    ctxt.cstore.cmap.insert(MapKey::ScopeId(base_scope_id(&scope)), v);
                }
                None => {
                    ctxt.cstore.cmap.remove(&MapKey::ScopeId(base_scope_id(&scope)));
                }
            }
        }
        drop(_timing_guard);

        let _timing_guard = self.timing_span(TimingCat::TermFinishFrameCheckDepth, cur_scope);
        if *self.rec_depth.borrow() > MAX_DEPTH {
            *self.rec_depth.borrow_mut() -= 1;
            self.incomplete.borrow_mut().insert(cur_scope.clone());
            if scope_matches_trace_filter(cur_scope) {
                debug!(
                    "TRACE finish_frame MAX_DEPTH RETURN for {:?}: cur_scope={:?} retval={:?}",
                    cur_scope.0.name(),
                    cur_scope,
                    retval
                );
            }
            return Ok(retval);
        }
        drop(_timing_guard);

        // final traverse after recursive wq drained
        let _timing_guard = self.timing_span(TimingCat::TermFinishFrameRevisit, cur_scope);
        let body = self.get_body(cur_scope);

        // constrain arguments
        let mut substore = ConstraintStore::new();
        for (i, arg_set) in key.1.args.iter().enumerate() {
            let place = Place {
                local: i + 1,
                projection: vec![],
            };
            let cs: Constraints = Constraints::from_vec(arg_set.iter().cloned().collect());
            substore
                .cmap
                .insert(MapKey::Var(place), Box::new(MapValue::Constraints(cs)));
        }
        ctxt.cstore.cmap.insert(
            MapKey::ScopeId(base_scope_id(cur_scope)),
            Box::new(MapValue::Store(substore, None)),
        );

        let stack_len = call_stack.len();
        self.prepare_call(call_stack, &key);
        let result = self.visit_body(ctxt, call_stack, cur_scope, &body);
        drop(_timing_guard);
        *self.rec_depth.borrow_mut() -= 1;
        if scope_matches_trace_filter(cur_scope) {
            debug!(
                "TRACE finish_frame POST-REVISIT RESULT (after queued reprocessing drained) for {:?}: cur_scope={:?} original_retval={:?} result={:?}",
                cur_scope.0.name(),
                cur_scope,
                retval,
                result
            );
        }
        match result {
            Ok(r) => Ok(r),
            Err(e) => {
                let _timing_guard =
                    self.timing_span(TimingCat::TermFinishFrameSecondPrepareReturn, cur_scope);
                self.unwind_stacks_to(call_stack, stack_len, "finish_frame re-visit error");
                drop(_timing_guard);
                Err(e)
            }
        }
    }
}

/// True if `x` still mentions a free generic parameter (type or const)
/// anywhere, at any depth - i.e. it is not fully monomorphic.
///
/// rustc_public's Instance::resolve / Instance::body / layout paths
/// normalize with `normalize_erasing_regions` in an empty ParamEnv, which
/// ICEs (via `bug!`, not a recoverable error) on anything generic - e.g.
/// `Failed to normalize Option<<T as Iterator>::Item>`. An ICE is emitted
/// as a diagnostic *before* it unwinds, so catching the panic afterwards
/// doesn't undo it; these calls have to be guarded up front instead.
pub(crate) fn has_unresolved_param<T: rustc_public::visitor::Visitable>(x: &T) -> bool {
    use rustc_public::visitor::{Visitable, Visitor};
    use std::ops::ControlFlow;

    struct FindParam;
    impl Visitor for FindParam {
        type Break = ();
        fn visit_ty(&mut self, ty: &Ty) -> ControlFlow<()> {
            match ty.kind() {
                TyKind::Param(_) | TyKind::Bound(..) => ControlFlow::Break(()),
                _ => ty.super_visit(self),
            }
        }
        fn visit_const(&mut self, c: &rustc_public::ty::TyConst) -> ControlFlow<()> {
            match c.kind() {
                rustc_public::ty::TyConstKind::Param(_)
                | rustc_public::ty::TyConstKind::Bound(..) => ControlFlow::Break(()),
                _ => c.super_visit(self),
            }
        }
    }
    x.visit(&mut FindParam).is_break()
}

/// Whether a call site's trait arg (`call`, concrete but with erased regions)
/// could be an instantiation of an impl's trait arg (`imp`, which may name the
/// impl's own lifetimes and type/const params). Conservative: `false` only
/// when the two certainly differ - see `Interpreter::trait_params_compatible`.
pub(crate) fn generic_arg_may_unify(call: &GenericArgKind, imp: &GenericArgKind) -> bool {
    match (call, imp) {
        // Regions never decide which impl a dyn call selects.
        (GenericArgKind::Lifetime(_), GenericArgKind::Lifetime(_)) => true,
        (GenericArgKind::Type(a), GenericArgKind::Type(b)) => ty_may_unify(a, b),
        (GenericArgKind::Const(a), GenericArgKind::Const(b)) => const_may_unify(a, b),
        _ => false,
    }
}

fn generic_args_may_unify(call: &GenericArgs, imp: &GenericArgs) -> bool {
    call.0.len() == imp.0.len()
        && call.0.iter().zip(imp.0.iter()).all(|(a, b)| generic_arg_may_unify(a, b))
}

fn const_may_unify(a: &rustc_public::ty::TyConst, b: &rustc_public::ty::TyConst) -> bool {
    use rustc_public::ty::TyConstKind;
    match (a.kind(), b.kind()) {
        (TyConstKind::Value(_, x), TyConstKind::Value(_, y)) => x == y,
        _ => true,
    }
}

/// Structural comparison ignoring regions, with params (on either side) and
/// anything not modelled here treated as matching.
fn ty_may_unify(a: &Ty, b: &Ty) -> bool {
    let (TyKind::RigidTy(ra), TyKind::RigidTy(rb)) = (a.kind(), b.kind()) else {
        return true; // Param / Bound / Alias
    };
    match (&ra, &rb) {
        (RigidTy::Adt(da, ga), RigidTy::Adt(db, gb)) => da == db && generic_args_may_unify(ga, gb),
        (RigidTy::Ref(_, ta, ma), RigidTy::Ref(_, tb, mb))
        | (RigidTy::RawPtr(ta, ma), RigidTy::RawPtr(tb, mb)) => ma == mb && ty_may_unify(ta, tb),
        (RigidTy::Slice(ta), RigidTy::Slice(tb)) => ty_may_unify(ta, tb),
        (RigidTy::Array(ta, ca), RigidTy::Array(tb, cb)) => {
            ty_may_unify(ta, tb) && const_may_unify(ca, cb)
        }
        (RigidTy::Tuple(ta), RigidTy::Tuple(tb)) => {
            ta.len() == tb.len() && ta.iter().zip(tb.iter()).all(|(x, y)| ty_may_unify(x, y))
        }
        (RigidTy::Dynamic(pa, _), RigidTy::Dynamic(pb, _)) => {
            let principal = |preds: &Vec<rustc_public::ty::Binder<rustc_public::ty::ExistentialPredicate>>| {
                preds.iter().find_map(|p| match &p.value {
                    rustc_public::ty::ExistentialPredicate::Trait(t) => Some(t.def_id),
                    _ => None,
                })
            };
            principal(pa) == principal(pb)
        }
        (
            RigidTy::Bool
            | RigidTy::Char
            | RigidTy::Int(_)
            | RigidTy::Uint(_)
            | RigidTy::Float(_)
            | RigidTy::Str
            | RigidTy::Never,
            _,
        )
        | (
            _,
            RigidTy::Bool
            | RigidTy::Char
            | RigidTy::Int(_)
            | RigidTy::Uint(_)
            | RigidTy::Float(_)
            | RigidTy::Str
            | RigidTy::Never,
        ) => ra == rb,
        // Same constructor we don't model (fn ptrs, closures, ...): assume a match.
        _ if std::mem::discriminant(&ra) == std::mem::discriminant(&rb) => true,
        _ => false,
    }
}

/// One-line shape of a dispatch receiver's constraint, for debugging which
/// resolve_defid path it takes: whether a trait-object constraint (toc) is
/// attached, the running constraint's kind, and for ADTs their field keys -
/// recursing through pointers and one level of ADT fields.
fn describe_receiver_constraint(conv: &RvalConverter, c: &Constraint) -> String {
    fn cfc_shape(conv: &RvalConverter, rc: &RunningConstraint, depth: usize) -> String {
        match rc {
            RunningConstraint::Adt(adtdef, genargs, variant, fields) => {
                let mut s = format!(
                    "Adt({}{}, variant={:?}, n_genargs={}, fields=[",
                    adtdef.0.name(),
                    if conv.wrapper_kind(adtdef).is_some() { " [wrapper]" } else { "" },
                    variant,
                    genargs.0.len()
                );
                for (k, v) in fields {
                    s.push_str(&format!("{}:", k));
                    if depth < 2 {
                        let inner: Vec<String> =
                            v.inner.iter().map(|c| constraint_shape(conv, c, depth + 1)).collect();
                        s.push_str(&format!("{{{}}} ", inner.join(" | ")));
                    } else {
                        s.push_str(&format!("{{{} disjuncts}} ", v.inner.len()));
                    }
                }
                s.push_str("])");
                s
            }
            RunningConstraint::Ptr(inner) => format!("Ptr({})", constraint_shape(conv, inner, depth + 1)),
            RunningConstraint::Closure(cdef, ..) => format!("Closure({})", cdef.0.name()),
            RunningConstraint::Idk(cs) => format!("Idk({} disjuncts)", cs.inner.len()),
            other => {
                let dbg = format!("{:?}", other);
                dbg.chars().take(60).collect()
            }
        }
    }
    fn constraint_shape(conv: &RvalConverter, c: &Constraint, depth: usize) -> String {
        let toc = match &c.toc {
            Some((ty, TraitObjConstraint::Adt(adtdef, ..))) => {
                format!("toc=({} -> Adt {}) ", ty.def.0.name(), adtdef.0.name())
            }
            Some((ty, TraitObjConstraint::Closure(cdef, ..))) => {
                format!("toc=({} -> Closure {}) ", ty.def.0.name(), cdef.0.name())
            }
            None => "toc=None ".to_string(),
        };
        let cfc = match &c.cfc {
            Some(rc) => cfc_shape(conv, rc, depth),
            None => "cfc=None".to_string(),
        };
        format!("{}{}", toc, cfc)
    }
    constraint_shape(conv, c, 0)
}

/// The argument tuple's arity if `args` call the closure body `body` in the
/// "rust-call" form `Fn*::call*(self, (a, b, ..))`, which the body declares
/// as `_1` (the closure, by value or reference) and `_2..` (the tuple's
/// elements, untupled). Not for a closure called through a fn pointer, whose
/// arguments are the parameters themselves.
fn rust_call_arity(body: &Body, args: &[Operand], local_decls: &[LocalDecl]) -> Option<usize> {
    fn closure_def(ty: &Ty) -> Option<ClosureDef> {
        match ty.kind() {
            TyKind::RigidTy(RigidTy::Ref(_, inner, _)) => closure_def(&inner),
            TyKind::RigidTy(RigidTy::Closure(def, _)) => Some(def),
            _ => None,
        }
    }
    let n_params = body.arg_locals().len();
    if args.len() != 2 || n_params == 0 {
        return None;
    }
    fn is_dyn(ty: &Ty) -> bool {
        match ty.kind() {
            TyKind::RigidTy(RigidTy::Ref(_, inner, _)) => is_dyn(&inner),
            TyKind::RigidTy(RigidTy::Dynamic(..)) => true,
            _ => false,
        }
    }
    let env = closure_def(&body.locals()[1].ty)?;
    // The receiver is the closure itself (static dispatch) or a `dyn Fn*`
    // (virtual dispatch to this closure).
    let receiver = args[0].ty(local_decls).ok()?;
    if closure_def(&receiver) != Some(env) && !is_dyn(&receiver) {
        return None;
    }
    match args[1].ty(local_decls).ok()?.kind() {
        TyKind::RigidTy(RigidTy::Tuple(tys)) if tys.len() == n_params - 1 => Some(tys.len()),
        _ => None,
    }
}

thread_local! {
    /// The last stack operations (see note_stack_op), for STACK DESYNC reports.
    static STACK_OPS: RefCell<std::collections::VecDeque<(&'static str, Instance, usize, usize)>> =
        RefCell::new(std::collections::VecDeque::with_capacity(64));
}

/// Records a call_stack / key_stack operation and both lengths after it.
fn note_stack_op(op: &'static str, scope: &VOID, call_len: usize, key_len: usize) {
    STACK_OPS.with(|ops| {
        let mut ops = ops.borrow_mut();
        if ops.len() == 64 {
            ops.pop_front();
        }
        ops.push_back((op, scope.0, call_len, key_len));
    });
}

fn recent_stack_ops() -> String {
    STACK_OPS.with(|ops| {
        ops.borrow()
            .iter()
            // (reinterp_recursive runs on a cloned call_stack, so its
            // entries compare that clone with the shared key_stack: only a
            // call/return mismatch is a real desync)
            .map(|(op, inst, c, k)| {
                let desync = c != k && matches!(*op, "call" | "return");
                format!("  {op:<24} c={c:<3} k={k:<3} {}{}", inst.name(), if desync { "   <-- DESYNC" } else { "" })
            })
            .collect::<Vec<_>>()
            .join("\n")
    })
}

/// Memory diagnostics every N bb visits (env VERIFOPT_CACHE_SIZES=N).
fn cache_sizes_every() -> Option<u64> {
    static EVERY: OnceLock<Option<u64>> = OnceLock::new();
    *EVERY.get_or_init(|| std::env::var("VERIFOPT_CACHE_SIZES").ok().and_then(|v| v.parse().ok()).filter(|n| *n > 0))
}

/// Records a memo hit (exact_memo, or virtual_call_memo for `scope` = the
/// caller), and whether it replayed writes (see write_log).
fn record_memo_hit(scope: &VOID, widened: bool, writes: &crate::write_log::WriteLog) {
    use crate::assumptions::{Reason, record};
    record(Reason::MemoHit, if widened { "(widened)" } else { "(precise)" }.to_string(), Vec::new);
    if !writes.is_empty() {
        record(Reason::MemoReplay, scope.0.name(), Vec::new);
    }
}

/// A call's place operands by index: its ports in write_log.
fn operand_places(args: &[Operand]) -> Vec<(usize, Place)> {
    args.iter()
        .enumerate()
        .filter_map(|(i, op)| match op {
            Operand::Copy(p) | Operand::Move(p) => Some((i, p.clone())),
            Operand::Constant(_) => None,
            _ => None,
        })
        .collect()
}

/// For an assignment of `rvalue` to a local (not a store through a pointer
/// or into a field - that writes the pointer's value) of type `dest_ty`: the place the
/// result aliases, if it's a pointer to a place - `&raw mut x` / `&raw const
/// x` (a leading Deref stripped, as for `&`) - or a pointer-typed copy, move
/// or pointer-to-pointer cast of a pointer local. Unsizing casts are left out:
/// they turn the value into a trait-object value (lift_traitobjtys), which an
/// alias edge would skip. So are copies of places with projections (a pointer
/// read out of a field): their pointer is the field's value.
/// Whether `ty` is a pointer to a slice or `str` - the target of an array
/// unsizing (`&[T; N]` -> `&[T]`), which keeps pointing at the same place
/// (unlike an unsizing to `dyn Trait`, which makes the value a trait-object
/// value).
fn is_slice_pointer(ty: &Ty) -> bool {
    match ty.kind() {
        TyKind::RigidTy(RigidTy::Ref(_, t, _)) | TyKind::RigidTy(RigidTy::RawPtr(t, _)) => {
            matches!(t.kind(), TyKind::RigidTy(RigidTy::Slice(_) | RigidTy::Str))
        }
        _ => false,
    }
}

fn pointer_alias_source(rvalue: &Rvalue, dest_ty: &Ty, local_decls: &[LocalDecl]) -> Option<(Place, Mutability)> {
    let dest_mt = match dest_ty.kind() {
        TyKind::RigidTy(RigidTy::Ref(_, _, mt)) | TyKind::RigidTy(RigidTy::RawPtr(_, mt)) => mt,
        _ => return None,
    };
    match rvalue {
        Rvalue::AddressOf(kind, to) => {
            let to = match to.projection.as_slice() {
                [ProjectionElem::Deref] => Place { local: to.local, projection: vec![] },
                _ => to.clone(),
            };
            let mt = match kind {
                rustc_public::mir::RawPtrKind::Mut => Mutability::Mut,
                _ => Mutability::Not,
            };
            Some((to, mt))
        }
        Rvalue::Use(Operand::Copy(src) | Operand::Move(src))
        | Rvalue::Cast(
            rustc_public::mir::CastKind::PtrToPtr
            | rustc_public::mir::CastKind::Transmute
            | rustc_public::mir::CastKind::PointerCoercion(
                rustc_public::mir::PointerCoercion::MutToConstPointer
                | rustc_public::mir::PointerCoercion::ArrayToPointer,
            ),
            Operand::Copy(src) | Operand::Move(src),
            _,
        ) if src.projection.is_empty() => pointer_cast_alias(src, dest_ty, dest_mt, local_decls),
        // An array-to-slice unsizing: the slice points at the same array
        // (lost as a value copy, the callee's writes through a `&mut [T]`
        // made from `&mut ARRAY` never reached the array).
        Rvalue::Cast(
            rustc_public::mir::CastKind::PointerCoercion(rustc_public::mir::PointerCoercion::Unsize),
            Operand::Copy(src) | Operand::Move(src),
            _,
        ) if src.projection.is_empty() && is_slice_pointer(dest_ty) => Some((src.clone(), dest_mt)),
        _ => None,
    }
}

/// The alias of a pointer-to-pointer cast of local `src` (see
/// pointer_alias_source).
fn pointer_cast_alias(src: &Place, dest_ty: &Ty, dest_mt: Mutability, local_decls: &[LocalDecl]) -> Option<(Place, Mutability)> {
    // A cast from a pointer to a wrapper to a pointer to what it wraps
    // (`*const UnsafeCell<T>` -> `*mut T`, MaybeUninit's `as_mut_ptr`,
    // Cell/ManuallyDrop/#[repr(transparent)]): the pointer is to the field at
    // offset 0, so it aliases that field - not the wrapper, which would mix
    // the wrapper's value and the field's (cell_client_wiring).
    let pointee = |ty: &Ty| match ty.kind() {
        TyKind::RigidTy(RigidTy::Ref(_, t, _)) | TyKind::RigidTy(RigidTy::RawPtr(t, _)) => Some(t),
        _ => None,
    };
    let path = src
        .ty(local_decls)
        .ok()
        .as_ref()
        .and_then(pointee)
        .zip(pointee(dest_ty))
        .and_then(|(from, to)| field_path_at_offset_zero(from, to, 0))
        .unwrap_or_default();
    if path.is_empty() {
        Some((src.clone(), dest_mt))
    } else {
        let mut projection = vec![ProjectionElem::Deref];
        projection.extend(path);
        Some((Place { local: src.local, projection }, dest_mt))
    }
}

/// The field path from type `from` to a field of type `to` at offset 0 -
/// what a pointer cast from `*from` to `*to` points at - through struct
/// fields at offset 0 (per the layout) and union fields. Empty if
/// `from == to`, None if there's no such field.
fn field_path_at_offset_zero(from: Ty, to: Ty, depth: usize) -> Option<Vec<ProjectionElem>> {
    if from == to {
        return Some(Vec::new());
    }
    if depth >= 4 {
        return None;
    }
    let TyKind::RigidTy(RigidTy::Adt(def, args)) = from.kind() else { return None };
    let kind = def.kind();
    if !(kind.is_struct() || kind.is_union()) {
        return None;
    }
    let offsets = match from.layout().ok()?.shape().fields {
        rustc_public::abi::FieldsShape::Arbitrary { offsets } => Some(offsets),
        _ => None, // a union: every field at offset 0
    };
    let variant = def.variants_iter().next()?;
    for (i, field) in variant.fields().into_iter().enumerate() {
        let at_zero = match &offsets {
            Some(offsets) => offsets.get(i).is_some_and(|o| o.bytes() == 0),
            None => kind.is_union(),
        };
        if !at_zero {
            continue;
        }
        let fty = field.ty_with_args(&args);
        if let Some(rest) = field_path_at_offset_zero(fty, to, depth + 1) {
            let mut path = vec![ProjectionElem::Field(i, fty)];
            path.extend(rest);
            return Some(path);
        }
    }
    None
}

/// The static `rvalue` points to, if it's exactly a pointer to one: a use or
/// cast of a constant of pointer type (`dest_ty`) whose only provenance is a
/// static at offset 0 - not a constant that merely contains such pointers
/// (e.g. a promoted `[&A, &B]`), which keeps being a value.
fn static_pointer_rvalue(rvalue: &Rvalue, dest_ty: &Ty) -> Option<DefId> {
    use rustc_public::mir::{CastKind, PointerCoercion};
    match rvalue {
        // Not unsizing casts (`&STATIC as &dyn Trait`): those make the value
        // a trait-object value (lift_traitobjtys), which an alias edge would
        // skip - as for pointer_alias_source.
        Rvalue::Use(op)
        | Rvalue::Cast(
            CastKind::PtrToPtr
            | CastKind::Transmute
            | CastKind::PointerCoercion(PointerCoercion::MutToConstPointer | PointerCoercion::ArrayToPointer),
            op,
            _,
        ) => static_pointer_operand(op, dest_ty),
        // `&mut STATIC_ARRAY as &mut [T]` points at the static too.
        Rvalue::Cast(CastKind::PointerCoercion(PointerCoercion::Unsize), op, _) if is_slice_pointer(dest_ty) => {
            static_pointer_operand(op, dest_ty)
        }
        _ => None,
    }
}

/// See static_pointer_rvalue.
fn static_pointer_operand(op: &Operand, ty: &Ty) -> Option<DefId> {
    if !matches!(ty.kind(), TyKind::RigidTy(RigidTy::Ref(..) | RigidTy::RawPtr(..))) {
        return None;
    }
    let Operand::Constant(co) = op else { return None };
    let ConstantKind::Allocated(alloc) = co.const_.kind() else { return None };
    match alloc.provenance.ptrs.as_slice() {
        [(0, Prov(aid))] => match GlobalAlloc::from(*aid) {
            GlobalAlloc::Static(StaticDef(def)) => Some(def),
            _ => None,
        },
        _ => None,
    }
}
