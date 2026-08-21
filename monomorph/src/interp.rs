use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

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

use log::{debug, error};

use crate::Context;
use crate::common::{log_call_stack, log_scope};
use crate::constraints::{
    ADTFields, ArgSet, Constraint, ConstraintStore, Constraints, EnclosingScopes, Location, MapKey,
    MapValue, RunningConstraint, SummaryKey, TagProv, TraitObjConstraint, TraitObjTy, VOID,
    hash_val, memoize_by_rc, summary_key,
};
use crate::constraints::{unique_append, unique_push};
use crate::convert::RvalConverter;
use crate::error::Error;
use crate::merge::Merge;
use crate::sig_collect::{SigStore, SigVal};
use crate::trait_collect::TraitStore;
use crate::wto::BBDeps;
use indexmap::IndexSet;
use std::rc::Weak;

const MAX_DEPTH: u32 = 50;

/// Cache key for `virtual_call_memo` - the call site (caller function's
/// DefId + basic block, same pair already used for `dispatch_cha`) plus
/// an `ArgSet` fingerprint of the call's operands as seen from the
/// caller's side. See `virtual_call_memo`'s field doc for the rationale.
type VirtualCallKey = ((DefId, usize), ArgSet);

#[derive(Debug, Clone)]
pub enum ParamSummary {
    Built(Option<Constraints>),
    Unavailable,
}

pub struct InterpPass<'a> {
    pub sigstore: &'a SigStore,
    pub tstore: &'a TraitStore,
    pub converter: RvalConverter<'a>,

    pub dispatch_targets:
        RefCell<ImHashMap<(DefId, usize), (Span, Vec<(DefId, Option<GenericArgs>)>)>>,
    pub dispatch_cha: RefCell<ImHashMap<(DefId, usize), (Span, Vec<(DefId, Option<GenericArgs>)>)>>,
    pub dispatch_tags: RefCell<ImHashMap<(DefId, usize), TagPlan>>,

    pub summaries: RefCell<HashMap<SummaryKey, Constraints>>,
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
    scope_self_time_global: RefCell<HashMap<VOID, (std::time::Duration, u64)>>,
    timing_global: RefCell<HashMap<TimingCat, TimingStats>>,
    timing_scope: RefCell<HashMap<(VOID, TimingCat), TimingStats>>,
    timing_scope_global: RefCell<HashMap<(VOID, TimingCat), TimingStats>>,
    timing_window: RefCell<HashMap<TimingCat, TimingStats>>,
    timing_child_stack: RefCell<Vec<std::time::Duration>>,
    timing_global_exclusive: RefCell<HashMap<TimingCat, TimingStats>>,
    timing_scope_exclusive: RefCell<HashMap<(VOID, TimingCat), TimingStats>>,
    timing_scope_exclusive_global: RefCell<HashMap<(VOID, TimingCat), TimingStats>>,
    timing_window_exclusive: RefCell<HashMap<TimingCat, TimingStats>>,
    pub dependencies: RefCell<ImHashMap<Span, HashSet<VOID>>>,
    pub incomplete: RefCell<ImHashSet<VOID>>,
    pub wtos_merge_conflicts: RefCell<ImHashSet<VOID>>,
    pub refs_merge_conflicts: RefCell<ImHashSet<(Place, VOID)>>,

    pub exact_memo: RefCell<HashMap<SummaryKey, (Option<Constraints>, u64)>>,

    pub virtual_call_memo:
        RefCell<HashMap<VirtualCallKey, (Option<Constraints>, Vec<(VOID, u64)>)>>,

    pub scope_epoch: RefCell<HashMap<VOID, u64>>,
    pub scope_exact_memo_count: RefCell<HashMap<VOID, u32>>,
    pub scope_summaries_count: RefCell<HashMap<VOID, u32>>,
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
            usize, /* bb */
            usize, /* stmt */
            DefId, /* impl */
        )>,
    ),
}

impl TagPlan {
    fn join(&mut self, o: &TagPlan) {
        match (&*self, o) {
            (TagPlan::Poisoned, _) | (_, TagPlan::Poisoned) => *self = TagPlan::Poisoned,

            (TagPlan::Tagged(a), TagPlan::Tagged(b)) => {
                let mut by_site = HashMap::new();

                for &(bb, stmt, did) in a.iter().chain(b.iter()) {
                    match by_site.entry((bb, stmt)) {
                        Entry::Vacant(e) => {
                            e.insert(did);
                        }
                        Entry::Occupied(e) => {
                            if *e.get() != did {
                                *self = TagPlan::Poisoned;
                                return;
                            }
                        }
                    }
                }

                let mut out: Vec<(usize, usize, DefId)> = by_site
                    .into_iter()
                    .map(|((bb, stmt), impl_did)| (bb, stmt, impl_did))
                    .collect();
                out.sort_by_key(|(bb, stmt, _)| (*bb, *stmt));

                *self = TagPlan::Tagged(out);
            }
        }
    }
}

struct SelfTimeGuard<'a, 'b> {
    pass: &'a InterpPass<'b>,
    scope: VOID,
    start: std::time::Instant,
}

impl<'a, 'b> Drop for SelfTimeGuard<'a, 'b> {
    fn drop(&mut self) {
        let total_elapsed = self.start.elapsed();
        let child_time = self
            .pass
            .self_time_child_accum
            .borrow_mut()
            .pop()
            .unwrap_or(std::time::Duration::ZERO);
        let self_elapsed = total_elapsed.saturating_sub(child_time);

        let scope_name =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.scope.0.name()))
                .unwrap_or_else(|_| "<unprintable scope: name() panicked>".to_string());

        debug!(
            "CALL SELF TIME: bb_visit_count={} scope={:?} self_time_ms={:.3}",
            *self.pass.bb_visit_count.borrow(),
            scope_name,
            self_elapsed.as_secs_f64() * 1000.0
        );

        {
            let mut map = self.pass.scope_self_time.borrow_mut();
            let entry = map
                .entry(self.scope.clone())
                .or_insert((std::time::Duration::ZERO, 0));
            entry.0 += self_elapsed;
            entry.1 += 1;
        }
        {
            let mut map = self.pass.scope_self_time_global.borrow_mut();
            let entry = map
                .entry(self.scope.clone())
                .or_insert((std::time::Duration::ZERO, 0));
            entry.0 += self_elapsed;
            entry.1 += 1;
        }

        if let Some(parent_accum) = self.pass.self_time_child_accum.borrow_mut().last_mut() {
            *parent_accum += total_elapsed;
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum TimingCat {
    BbStatements,
    BbTerminator,
    StmtContainsDyn,
    StmtNewConstraints,
    StmtLiftTraitobj,
    StmtCurConstraints,
    StmtWriteFields,
    StmtSetScoped,
    TermDirectCall,
    TermIndirectCall,
    TermReturn,
    TermSwitch,
    TermReturnScopedGet,
    TermReturnFinishFrame,
    TermFinishFrameReinterp,
    TermFinishFrameRevisit,
    StmtNewConstraintsRef,
    StmtNewConstraintsStatic,
    StmtNewConstraintsFromConvert,
    TermCollectResolvedArgs,
    TermResolveArgs,
    TermInterpStaticCall,
    TermInterpStaticCallPost,
    TermInterpVirtualCall,
    //TermParamSummary,
    TermMemo,
    TermMergeStoresEs,
    TermVirtualMemo,
    TermGetImplsCha,
    TermGetImplsFsa,
    TermVirtualCallPrep,
    TermSimulateCallPrep,
    TermSimulateRecursiveFallback,
    TermSimulateStdlibStub,
    TermSimulateRealCall,
    TermSigFallback,
    TermSimulateMergeResults,
    TermSimulateLoopMergeResults,
    Take,
    VecConstruction,
    TermMergeCstoresMerge,
    TermMergeIdentityCheck,
    TermMergePerKeyMapvals,
    TermMergeMapsvalsMerge,
    TermMergeConstraintsAppend,
    TermMergeConstraintsWiden,
    TermMergeRefsUnion,
    TermMergeWtosUnion,
    TermMergeContextsSetup,
    TermMergeNewContext,
    ConvertOp,
    ConvertPlace,
    ConvertPlaceGetConstraints,
    ConvertCast,
    ConvertAgg,
    ConvertUnop,
    ConvertBinop,
    ConvertCheckedBinop,
    ConvertType,
    // convert_agg branches
    ConvertAggAdtOpaque,
    ConvertAggAdtFields,
    ConvertAggTuple,
    ConvertAggRawPtr,
    ConvertAggArray,
    ConvertAggClosure,
    // constraints::get_constraints blocks
    GetConstraintsIfBlock,
    GetConstraintsElseBlock,
    GetConstraintsProjLoop,
    GetConstraintsIsOpaqueInternal,
    GetConstraintsFlattenAll,
    GetConstraintsFilterVariant,
    GetConstraintsStepField,
    // lift_traitobjtys parts
    LiftTraitobjtysHashVal,
    LiftTraitobjtysUncached,
    LiftTraitobjtysUncachedGetTraitobj,
    // set_scoped_constraints parts
    SetScopedConstraints,
    SetScopedConstraintsReplace,
    SetScopedConstraintsUpdate,
    ScopedUpdateResolve,
    ScopedUpdateGetScope,
    ScopedUpdateStorePre,
    ScopedUpdateStoreMerge,
    ScopedUpdateStorePost,
    ScopedUpdateInitScope,
    // interp_fn_def blocks
    InterpFnDefVirtualFallback,
    InterpFnDefStdlibStub,
    InterpFnDefFetchableBody,
    InterpFnDefCallStackChecks,
    InterpFnDefFinalDispatch,
    FlattenAll,
    FlattenOne,
    FlattenAllAppend,
    FlattenOneAppend,
    StepFieldOneAdt,
    StepFieldOneTuple,
    StepFieldOnePtr,
    StepFieldOneIdk,
    StepFieldOneParam,
}

#[derive(Clone, Copy, Default)]
struct TimingStats {
    total: std::time::Duration,
    count: u64,
    min: std::time::Duration,
    max: std::time::Duration,
}

impl TimingStats {
    fn record(&mut self, d: std::time::Duration) {
        if self.count == 0 || d < self.min {
            self.min = d;
        }
        if self.count == 0 || d > self.max {
            self.max = d;
        }
        self.total += d;
        self.count += 1;
    }

    fn avg(&self) -> std::time::Duration {
        if self.count == 0 {
            std::time::Duration::ZERO
        } else {
            self.total / self.count as u32
        }
    }
}

pub(crate) struct TimingSpanGuard<'a, 'b> {
    pass: &'a InterpPass<'b>,
    cat: TimingCat,
    scope: VOID,
    start: std::time::Instant,
}

impl<'a, 'b> Drop for TimingSpanGuard<'a, 'b> {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        self.pass.record_timing(self.cat, &self.scope, elapsed);

        let child_time = self
            .pass
            .timing_child_stack
            .borrow_mut()
            .pop()
            .unwrap_or(std::time::Duration::ZERO);
        let exclusive = elapsed.saturating_sub(child_time);

        self.pass
            .timing_global_exclusive
            .borrow_mut()
            .entry(self.cat)
            .or_default()
            .record(exclusive);
        self.pass
            .timing_scope_exclusive
            .borrow_mut()
            .entry((self.scope.clone(), self.cat))
            .or_default()
            .record(exclusive);
        self.pass
            .timing_scope_exclusive_global
            .borrow_mut()
            .entry((self.scope.clone(), self.cat))
            .or_default()
            .record(exclusive);
        self.pass
            .timing_window_exclusive
            .borrow_mut()
            .entry(self.cat)
            .or_default()
            .record(exclusive);

        if let Some(parent_accum) = self.pass.timing_child_stack.borrow_mut().last_mut() {
            *parent_accum += elapsed;
        }
    }
}

impl<'a> InterpPass<'a> {
    pub fn new(sigstore: &'a SigStore, tstore: &'a TraitStore) -> InterpPass<'a> {
        Self {
            sigstore,
            tstore,
            converter: RvalConverter::new(tstore),
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
            wtos_merge_conflicts: ImHashSet::new().into(),
            refs_merge_conflicts: ImHashSet::new().into(),
            exact_memo: HashMap::new().into(),
            virtual_call_memo: HashMap::new().into(),
            scope_epoch: HashMap::new().into(),
            scope_exact_memo_count: HashMap::new().into(),
            scope_summaries_count: HashMap::new().into(),
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
            panic!(
                "STACK DESYNC at {}: call_stack.len()={} key_stack.len()={}\ncall_stack contents:\n{:#?}",
                where_,
                call_stack.len(),
                key_len,
                names
            );
        }
    }

    fn prepare_call(&self, call_stack: &mut Vec<VOID>, key: &SummaryKey) {
        call_stack.push(key.0.clone());
        self.key_stack.borrow_mut().push(key.clone());
        self.assert_stacks_synced(call_stack, "prepare_call");
    }

    fn prepare_return(&self, call_stack: &mut Vec<VOID>) -> Option<VOID> {
        self.key_stack.borrow_mut().pop();
        let popped = call_stack.pop();
        self.assert_stacks_synced(call_stack, "prepare_return");
        popped
    }

    pub fn run(
        &self,
        ctxt: &mut Context,
        start_instance: Instance,
    ) -> Result<Option<Constraints>, Error> {
        *self.main_ctxt_ptr.borrow_mut() = Some(ctxt as *const Context as usize);

        let start_scope = (start_instance, GenericArgs(vec![]));
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

    fn dump_self_time_report(&self, label: &str, map: &HashMap<VOID, (std::time::Duration, u64)>) {
        let mut entries: Vec<(&VOID, &(std::time::Duration, u64))> = map.iter().collect();
        entries.sort_by(|a, b| b.1.0.cmp(&a.1.0));

        let mut lines = String::new();
        lines.push_str(&format!(
            "\n=== SELF-TIME REPORT [{}] (top 50 by accumulated self time) ===\n",
            label
        ));
        lines.push_str(&format!(
            "{:>12} {:>10} {:>14}  scope\n",
            "self_time", "calls", "avg_per_call"
        ));
        for entry in entries.iter().take(50) {
            let scope = entry.0;
            let total = entry.1.0;
            let count = entry.1.1;
            let avg = if count > 0 {
                total / (count as u32)
            } else {
                std::time::Duration::ZERO
            };
            lines.push_str(&format!(
                "{:>10.3}ms {:>10} {:>12.3}ms  {:?}\n",
                total.as_secs_f64() * 1000.0,
                count,
                avg.as_secs_f64() * 1000.0,
                scope.0.name()
            ));
        }
        lines.push_str(&format!("=== END SELF-TIME REPORT [{}] ===\n", label));
        debug!("{}", lines);
    }

    fn record_timing(&self, cat: TimingCat, scope: &VOID, d: std::time::Duration) {
        self.timing_global
            .borrow_mut()
            .entry(cat)
            .or_default()
            .record(d);
        self.timing_scope
            .borrow_mut()
            .entry((scope.clone(), cat))
            .or_default()
            .record(d);
        self.timing_scope_global
            .borrow_mut()
            .entry((scope.clone(), cat))
            .or_default()
            .record(d);
        self.timing_window
            .borrow_mut()
            .entry(cat)
            .or_default()
            .record(d);
    }

    pub(crate) fn timing_span<'b>(
        &'b self,
        cat: TimingCat,
        scope: &VOID,
    ) -> TimingSpanGuard<'b, 'a> {
        self.timing_child_stack
            .borrow_mut()
            .push(std::time::Duration::ZERO);
        TimingSpanGuard {
            pass: self,
            cat,
            scope: scope.clone(),
            start: std::time::Instant::now(),
        }
    }

    fn dump_timing_report(&self, label: &str, map: &HashMap<TimingCat, TimingStats>) {
        let mut cats: Vec<(&TimingCat, &TimingStats)> = map.iter().collect();
        cats.sort_by(|a, b| b.1.total.cmp(&a.1.total));

        let mut lines = String::new();
        lines.push_str(&format!("\n=== TIMING REPORT [{}] ===\n", label));
        lines.push_str(&format!(
            "{:>10} {:>10} {:>10} {:>10} {:>10}  category\n",
            "total_ms", "count", "avg_ms", "min_ms", "max_ms"
        ));
        for (cat, stats) in cats {
            lines.push_str(&format!(
                "{:>10.3} {:>10} {:>10.3} {:>10.3} {:>10.3}  {:?}\n",
                stats.total.as_secs_f64() * 1000.0,
                stats.count,
                stats.avg().as_secs_f64() * 1000.0,
                stats.min.as_secs_f64() * 1000.0,
                stats.max.as_secs_f64() * 1000.0,
                cat,
            ));
        }
        lines.push_str(&format!("=== END TIMING REPORT [{}] ===\n", label));
        debug!("{}", lines);
    }

    fn dump_timing_by_scope_report(
        &self,
        label: &str,
        map: &HashMap<(VOID, TimingCat), TimingStats>,
    ) {
        let mut entries: Vec<(&(VOID, TimingCat), &TimingStats)> = map.iter().collect();
        entries.sort_by(|a, b| b.1.total.cmp(&a.1.total));

        let mut lines = String::new();
        lines.push_str(&format!(
            "\n=== TIMING BY SCOPE [{}] (top 50 by total) ===\n",
            label
        ));
        for ((scope, cat), stats) in entries.iter().take(50) {
            lines.push_str(&format!(
                "{:>10.3}ms {:>10} {:>10.3}ms  {:?} {:?}\n",
                stats.total.as_secs_f64() * 1000.0,
                stats.count,
                stats.avg().as_secs_f64() * 1000.0,
                cat,
                scope.0.name(),
            ));
        }
        lines.push_str(&format!("=== END TIMING BY SCOPE [{}] ===\n", label));
        debug!("{}", lines);
    }

    /// Exclusive-time counterpart to `dump_timing_by_scope_report` - same
    /// format, reading from `timing_scope_exclusive` instead of
    /// `timing_scope`, so nested work isn't double-counted.
    fn dump_timing_by_scope_exclusive_report(
        &self,
        label: &str,
        map: &HashMap<(VOID, TimingCat), TimingStats>,
    ) {
        let mut entries: Vec<(&(VOID, TimingCat), &TimingStats)> = map.iter().collect();
        entries.sort_by(|a, b| b.1.total.cmp(&a.1.total));

        let mut lines = String::new();
        lines.push_str(&format!(
            "\n=== EXCLUSIVE TIMING BY SCOPE [{}] (top 50 by total) ===\n",
            label
        ));
        for ((scope, cat), stats) in entries.iter().take(50) {
            lines.push_str(&format!(
                "{:>10.3}ms {:>10} {:>10.3}ms  {:?} {:?}\n",
                stats.total.as_secs_f64() * 1000.0,
                stats.count,
                stats.avg().as_secs_f64() * 1000.0,
                cat,
                scope.0.name(),
            ));
        }
        lines.push_str(&format!(
            "=== END EXCLUSIVE TIMING BY SCOPE [{}] ===\n",
            label
        ));
        debug!("{}", lines);
    }

    fn get_body(&self, cur_scope: &VOID) -> Body {
        cur_scope.0.body().unwrap()
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
            if !bb_deps.has_ret {
                return self.finish_frame(ctxt, call_stack, cur_scope, None);
            }
        } else {
            bb_deps = BBDeps::new(body);
            if !bb_deps.has_ret {
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
                    //self.log_bb_cache_sizes(*n, cur_scope, ctxt, bb_deps.ordering.len());
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

    fn static_get_constraints(&self, ctxt: &mut Context, defid: DefId) -> Constraints {
        if let Some(cs) = ctxt.get_static(defid) {
            return cs;
        }

        let sdef = StaticDef(defid);
        let ty = sdef.ty();

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
        let frozen =
            matches!(alloc.mutability, Mutability::Not) && self.converter.is_frozen(&ty, &mut seen);

        let constraints = if frozen {
            self.converter.convert_static_const(
                &Location::unknown(),
                &ty,
                &alloc,
                None,
                Some(self),
            )
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
                            constraints.push(constraint);
                        }
                    }
                },
                None => {
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
                    ctxt.set_scoped_constraints(cur_scope, place, final_constraints, Some(self));
                    drop(_timing_guard);
                } else {
                    let base = Place {
                        local: place.local,
                        projection: vec![],
                    };
                    let _timing_guard = self.timing_span(TimingCat::StmtCurConstraints, cur_scope);
                    let base_lookup = ctxt.get_constraints(cur_scope, local_decls, &base, false, Some(self));
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
                            ctxt.set_scoped_constraints(cur_scope, &base, base_constraints, Some(self));
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
                            ctxt.set_scoped_constraints(cur_scope, &base, base_constraints, Some(self));
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
                    let dst_constraints =
                        self.lift_traitobjtys(&maybe_trait_destty, Constraints::from(dst), cur_scope);
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
            | TerminatorKind::Drop { .. }
            | TerminatorKind::Goto { .. } => Ok(None),
            _iasm @ TerminatorKind::InlineAsm {
                ..
                //template,
                //operands,
                //options,
                //line_spans,
                //destination,
                //unwind
            } => {
                //debug!("iasm: {:#?}", iasm);
                // TODO do not interp, try to get rettype
                todo!("inline asm");
            }
            _ => todo!("other term kind: {:?}", &term.kind),
        }
    }

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
        let dest_ty = place.ty(local_decls).unwrap();
        let maybe_trait_destty = self.contains_dyn(&dest_ty);

        let mut ret_constraints = Constraints::new();
        match ctxt.get_constraints(cur_scope, local_decls, place, false, Some(self)) {
            Some(constraints) => {
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
                            e @ Err(_) => {
                                panic!("interping constraint as fn, got error: {:?}", e)
                            }
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
        ctxt.set_scoped_constraints(cur_scope, destination, constraints, Some(self));

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
            RunningConstraint::Closure(cdef, genargs) => self.interp_closure(
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
        _cur_scope: &VOID,
        _local_decls: &[LocalDecl],
        sigval: &SigVal,
        _args: &Vec<Operand>,
    ) -> Result<Option<Constraints>, Error> {
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
            let new_scope = (instance, genargs.clone());
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
            self.prepare_call(call_stack, &key);
            match self.visit_body(ctxt, call_stack, &new_scope, &body) {
                Ok(r) => Ok(r),
                Err(e) => {
                    self.prepare_return(call_stack);
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

                ctxt.set_scoped_constraints(cur_scope, destination, constraints.clone(), Some(self));

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

        let new_scope = (instance, genargs.clone());
        debug!(
            "--- CALLING {:?} -> resolved instance: kind={:?} name={:?}",
            fndef,
            instance.kind,
            instance.name()
        );
        log_scope(cur_scope);

        // checking for recursive stack depths of > 50
        if *self.rec_depth.borrow() > MAX_DEPTH {
            return Err(Error::RecurseLimit(MAX_DEPTH));
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
            let _g = self.timing_span(TimingCat::InterpFnDefCallStackChecks, cur_scope);
            let precise_count = *self
                .scope_summaries_count
                .borrow()
                .get(&new_scope)
                .unwrap_or(&0);
            let new_key = if precise_count >= 50 {
                let widened: Vec<Constraints> = new_cs
                    .iter()
                    .map(crate::constraints::widen_constraints)
                    .collect();
                (new_scope.clone(), ArgSet::new(&widened))
            } else {
                (new_scope.clone(), ArgSet::new(&new_cs))
            };

            if let Some(cs) = self.summaries.borrow().get(&new_key).cloned() {
                return Ok(Some(cs));
            }

            let retty = self
                .retty_fallback_from_poly(fndef.fn_sig())?
                .unwrap_or_default();
            self.summaries
                .borrow_mut()
                .insert(new_key.clone(), retty.clone());
            if precise_count < 50 {
                *self
                    .scope_summaries_count
                    .borrow_mut()
                    .entry(new_scope.clone())
                    .or_insert(0) += 1;
            }

            let cur_key = self.key_stack.borrow().last().cloned().unwrap();

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
            InstanceKind::Intrinsic => self.retty_fallback_from_poly(fndef.fn_sig()),
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
            for (key, (cs, _)) in em.iter() {
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
            for (key, cs) in sm.iter() {
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
                        let build_result = self.build_param_summary(cur_scope, is_closure);
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
                .get(cur_scope)
                .unwrap_or(&0);
            let memo_key: SummaryKey = if precise_count >= 50 {
                let widened: Vec<Constraints> = cur_cs
                    .iter()
                    .map(crate::constraints::widen_constraints)
                    .collect();
                (cur_scope.clone(), ArgSet::new(&widened))
            } else {
                key.clone()
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

            let epoch_before = *self.scope_epoch.borrow().get(cur_scope).unwrap_or(&0);
            if let Some((cached, cached_epoch)) = self.exact_memo.borrow().get(&memo_key) {
                if *cached_epoch == epoch_before {
                    debug!(
                        "exact_memo hit for {:?} at epoch {}",
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
            self.prepare_call(call_stack, &key);
            let result = self.visit_body(ctxt, call_stack, cur_scope, &body);

            let _timing_guard = self.timing_span(TimingCat::TermInterpStaticCallPost, cur_scope);
            if let Ok(ref cs) = result {
                let epoch_after = *self.scope_epoch.borrow().get(cur_scope).unwrap_or(&0);
                let is_new = !self.exact_memo.borrow().contains_key(&memo_key);
                self.exact_memo
                    .borrow_mut()
                    .insert(memo_key.clone(), (cs.clone(), epoch_after));
                if is_new && precise_count < 50 {
                    *self
                        .scope_exact_memo_count
                        .borrow_mut()
                        .entry(cur_scope.clone())
                        .or_insert(0) += 1;
                }
            }
            drop(_timing_guard);

            match result {
                Ok(r) => Ok(r),
                Err(e) => {
                    self.prepare_return(call_stack);
                    Err(e)
                }
            }
        } else {
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
                    &Some(vec![caller_scope.clone()]),
                );
                widened = !store.0.cmap.ptr_eq(&old_substore.cmap);
            }
            Some(_) => panic!("got constraint, expected store"),
            None => {
                store = (new_ctxt.cstore, Some(vec![caller_scope.clone()]));
            }
        }

        if widened {
            let mut epochs = self.scope_epoch.borrow_mut();
            let e = epochs.entry(callee_scope.clone()).or_insert(0);
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

        for (i, constraints) in resolved.into_iter().enumerate() {
            debug!("\narg position: {:?}", i);
            let local = if is_closure { i + 2 } else { i + 1 };
            let place = Place {
                local,
                projection: vec![],
            };

            if local < body.locals().len() {
                let arg_ty = place.ty(body.locals()).unwrap();
                if let TyKind::RigidTy(RigidTy::Ref(_, _, mt)) = arg_ty.kind() {
                    if let Operand::Copy(caller_place) | Operand::Move(caller_place) = &args[i] {
                        ctxt.cstore.add_ref(
                            (place.clone(), callee_scope.clone()),
                            (caller_place.clone(), caller_scope.clone()),
                            mt,
                        );
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
                match ctxt.get_constraints(caller_scope, local_decls, place, is_closure, Some(self)) {
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

        let key = (caller_scope.0.def.def_id(), bb);

        let _timing_guard = self.timing_span(TimingCat::TermVirtualMemo, caller_scope);
        let resolved_args: Vec<Constraints> = args
            .iter()
            .map(|op| {
                self.resolve_arg(ctxt, term_span, caller_scope, &None, local_decls, op, false)
            })
            .collect();
        let virtual_key: VirtualCallKey = (key, ArgSet::new(&resolved_args));
        match self.virtual_call_memo.borrow().get(&virtual_key) {
            Some((cached, recorded_scopes)) => {
                let stale: Vec<(DefId, u64, u64)> = recorded_scopes
                    .iter()
                    .filter_map(|(scope, recorded_epoch)| {
                        let current_epoch = *self.scope_epoch.borrow().get(scope).unwrap_or(&0);
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
                    return Ok(cached.clone());
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
            None => {
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

        for fsa_impl in &assoc_fn_impls_fsa {
            if !assoc_fn_impls_cha.contains(&fsa_impl) {
                error!("CHA missing impl: {:?}", fsa_impl);
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
            .entry(key)
            .or_insert((*term_span, assoc_fn_impls_cha));

        {
            // collect possible calls (mostly for recursion)
            let mut dt = self.dispatch_targets.borrow_mut();
            let entry = dt.entry(key).or_insert((*term_span, Vec::new()));
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
        if let Ok(ref cs) = result {
            self.virtual_call_memo
                .borrow_mut()
                .insert(virtual_key, (cs.clone(), touched_scopes));
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

        let mut by_site = HashMap::new();

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

            let target = impls[0].0;

            for site in tags {
                if site.0 != caller_did {
                    return TagPlan::Poisoned;
                }

                match by_site.entry(*site) {
                    Entry::Occupied(e) => {
                        // same site claimed by two
                        if *e.get() != target {
                            return TagPlan::Poisoned;
                        }
                    }
                    Entry::Vacant(e) => {
                        e.insert(target);
                    }
                }
            }
        }

        let mut out: Vec<(usize, usize, DefId)> = by_site
            .into_iter()
            .map(|((_fn_did, bb, stmt), impl_did)| (bb, stmt, impl_did))
            .collect();
        out.sort_by_key(|(bb, stmt, _)| (*bb, *stmt));

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
    // True if ty is an Adt with a free TyKind::Param in its own GenericArgs.
    fn ty_has_unresolved_param(ty: &Ty) -> bool {
        match ty.kind() {
            TyKind::RigidTy(RigidTy::Adt(_, args)) => args.0.iter().any(
                |k| matches!(k, GenericArgKind::Type(t) if matches!(t.kind(), TyKind::Param(_))),
            ),
            _ => false,
        }
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
                    if FnDef(*defid).body().is_some() {
                        // This is a callable item, push the impl defid
                        unique_push(&mut assoc_fn_impls, (*defid, genargs.clone()));
                    }
                }
            }
        }

        assoc_fn_impls
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
    /// against one specific impl's own `TraitRef` args
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
            if call_arg != impl_arg {
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
                    (_, TraitObjConstraint::Closure(cdef, genargs)) => {
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
                RunningConstraint::Closure(cdef, genargs) => {
                    if genargs.0.is_empty() {
                        (true, vec![(cdef.0, None)])
                    } else {
                        (true, vec![(cdef.0, Some(genargs.clone()))])
                    }
                }
                RunningConstraint::Scalar(_) | RunningConstraint::Float => (false, vec![]),
                RunningConstraint::Dynamic(tys) => {
                    match tys.iter().find(|ty| ty.def.0 == *trait_defid) {
                        Some(matching_ty) => (
                            false,
                            self.get_cha_tyconstraint_defids(trait_defid, &matching_ty.genargs),
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
        for (i, (assoc_fn_impl, adt_genargs)) in assoc_fn_impls.iter().enumerate() {
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
                    } else if !is_closure && adt_genargs.is_some() {
                        adt_genargs.clone().unwrap()
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
                    let kind_mismatch = identity_args.as_ref().is_some_and(|template| {
                        genargs
                            .0
                            .iter()
                            .zip(template.0.iter())
                            .any(|(a, b)| std::mem::discriminant(a) != std::mem::discriminant(b))
                    });
                    if kind_mismatch {
                        debug!(
                            "skipping {:?}: genargs kind shape doesn't match callee signature, falling back to poly sig",
                            assoc_fn_impl,
                        );
                        results.push(self.retty_fallback_from_poly(fndef.fn_sig()).unwrap());
                        return Ok(());
                    }

                    let instance_ = match Instance::resolve(fndef, &genargs) {
                        Ok(i) => i,
                        Err(_) => {
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
                    let callee_scope = (instance, genargs.clone());
                    drop(_timing_guard);

                    // the `if` and `else if` blocks might be creating a soundness error...
                    // if we're not actually stepping into new code + updating our cmap,
                    // we could be omitting actually-used concrete type variants in our
                    // eventual rewrite FIXME
                    let recursive_hit = call_stack.contains(&callee_scope);
                    if recursive_hit {
                        let _timing_guard =
                            self.timing_span(TimingCat::TermSimulateRecursiveFallback, cur_scope);
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
                            let mut call_stack_clone = call_stack.clone();

                            let body = if is_virtual {
                                // FIXME not monomorphized
                                fndef.body().unwrap()
                            } else {
                                self.get_body(&callee_scope)
                            };

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
                                *self.scope_epoch.borrow().get(&callee_scope).unwrap_or(&0),
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
                    debug!(
                        "candidate {:?} panicked during simulation, skipping (no fallback pushed): {}",
                        assoc_fn_impl, msg
                    );

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
                    self.assert_stacks_synced(
                        call_stack,
                        "simulate_static_calls catch_unwind recovery",
                    );
                }
            }
        }

        let _timing_guard = self.timing_span(TimingCat::TermSimulateMergeResults, cur_scope);
        match acc {
            Some(acc) => *ctxt = acc,
            None => {}
        }
        let result = self.merge_results_and_ret(&mut results);
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
            for conflicting_scope in Self::find_conflicting_keys(&m_wtos, &wtos) {
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
        let merged_es = match (cur_es, new_es) {
            (Some(cur_es_vec), Some(new_es_vec)) => {
                let mut merged_es_vec = cur_es_vec.clone();
                unique_append(&mut merged_es_vec, new_es_vec.to_vec());
                Some(merged_es_vec)
            }
            (Some(cur_es_vec), None) => Some(cur_es_vec.to_vec()),
            (None, Some(new_es_vec)) => Some(new_es_vec.to_vec()),
            (None, None) => None,
        };
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
            let identical = merged.cmap.ptr_eq(&store.cmap) && merged.refs == store.refs;
            drop(_timing_guard);
            if identical {
                continue;
            }

            // HERE
            let _timing_guard = self.timing_span(TimingCat::TermMergePerKeyMapvals, scope);
            for (key, val) in store.cmap.iter() {
                match merged.cmap.get_mut(key) {
                    Some(merged_val) => {
                        // HERE
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
            // the field doc. This alone answers "how often does a
            // genuine refs collision happen at all," which is the thing
            // to look at first before deciding whether the harder,
            // read-side-consequence check is worth building.
            let refs_conflicts = Self::find_conflicting_keys(&merged.refs, &store.refs);
            if !refs_conflicts.is_empty() {
                debug!(
                    "REFS MERGE CONFLICT: bb_visit={} {} key(s) had disagreeing alias targets, \
                     one side's will be silently discarded by union(): {:?}",
                    *self.bb_visit_count.borrow(),
                    refs_conflicts.len(),
                    refs_conflicts
                );
            }
            for conflicting_key in refs_conflicts {
                self.refs_merge_conflicts
                    .borrow_mut()
                    .insert(conflicting_key);
            }

            merged.refs = merged.refs.union(store.refs);
            drop(_timing_guard);
        }

        merged
    }

    fn merge_results_and_ret(
        &self,
        results: &mut Vec<Option<Constraints>>,
    ) -> Result<Option<Constraints>, Error> {
        let filtered_results: Vec<Constraints> = results
            .into_iter()
            .filter(|option| option.is_some())
            .map(|x| x.clone().unwrap())
            .collect();

        match filtered_results.merge() {
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
            MapKey::ScopeId(callee_scope.clone()),
            Box::new(MapValue::Store(substore, None)),
        );

        let key = (callee_scope.clone(), ArgSet::new(&callee_cs));
        let saved_key_stack = self.key_stack.replace(
            std::iter::repeat_with(|| key.clone())
                .take(call_stack.len())
                .collect(),
        );

        self.prepare_call(call_stack, &key);
        let body = self.get_body(callee_scope);
        let refined = match self.visit_body(ctxt, call_stack, callee_scope, &body) {
            Ok(r) => r,
            Err(e) => {
                self.key_stack.replace(saved_key_stack);
                return Err(e);
            }
        };
        self.key_stack.replace(saved_key_stack);

        // publish refined summary for widened constraints
        self.summaries
            .borrow_mut()
            .insert(key.clone(), refined.clone().unwrap_or_default());

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

        let _timing_guard = self.timing_span(TimingCat::TermReturnFinishFrame, cur_scope);
        let result = self.finish_frame(ctxt, call_stack, cur_scope, retval);
        drop(_timing_guard);
        result
    }

    fn finish_frame(
        &self,
        ctxt: &mut Context,
        call_stack: &mut Vec<VOID>,
        cur_scope: &VOID,
        retval: Option<Constraints>,
    ) -> Result<Option<Constraints>, Error> {
        let key = self.key_stack.borrow().last().cloned().unwrap();

        let old_scope = self.prepare_return(call_stack);
        if old_scope.clone().unwrap() != *cur_scope {
            log_call_stack(call_stack);
            panic!("call stack out of sorts");
        }

        let queued = self.wq.borrow_mut().remove(&key).unwrap_or_default();

        if self.in_queue.borrow().contains(&key) || queued.is_empty() {
            // use summary version OR
            // no recursive calls, no recursed interp needed
            return Ok(retval);
        }

        // about to be queueing, set to prevent infinite recursion
        self.in_queue.borrow_mut().insert(key.clone());

        // janky method to preserve stores conflicting on voids but not keys
        let saved: Vec<(VOID, Option<Box<MapValue>>)> = queued
            .iter()
            .map(|(scope, _, _)| {
                (
                    scope.clone(),
                    ctxt.cstore
                        .cmap
                        .get(&MapKey::ScopeId(scope.clone()))
                        .cloned(),
                )
            })
            .collect();

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

        for (scope, old) in saved {
            match old {
                Some(v) => {
                    ctxt.cstore.cmap.insert(MapKey::ScopeId(scope), v);
                }
                None => {
                    ctxt.cstore.cmap.remove(&MapKey::ScopeId(scope));
                }
            }
        }

        if *self.rec_depth.borrow() > MAX_DEPTH {
            *self.rec_depth.borrow_mut() -= 1;
            self.incomplete.borrow_mut().insert(cur_scope.clone());
            return Ok(retval);
        }

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
            MapKey::ScopeId(cur_scope.clone()),
            Box::new(MapValue::Store(substore, None)),
        );

        self.prepare_call(call_stack, &key);
        let result = self.visit_body(ctxt, call_stack, cur_scope, &body);
        drop(_timing_guard);
        *self.rec_depth.borrow_mut() -= 1;
        match result {
            Ok(r) => Ok(r),
            Err(e) => {
                self.prepare_return(call_stack);
                Err(e)
            }
        }
    }
}
