use std::collections::HashMap;

use log::debug;

use crate::constraints::VOID;
use crate::interp::InterpPass;

pub(crate) struct SelfTimeGuard<'a, 'b> {
    pub(crate) pass: &'a InterpPass<'b>,
    pub(crate) scope: VOID,
    pub(crate) start: std::time::Instant,
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
    TermFinishFrameGetKey,
    TermFinishFrameDequeue,
    TermFinishFrameEarlyRet,
    TermFinishFramePrepareQueue,
    TermFinishFramePreserveVoidConflicts,
    TermFinishFrameInsertScope,
    TermFinishFrameCheckDepth,
    TermFinishFrameFirstPrepareReturn,
    TermFinishFrameSecondPrepareReturn,
    StmtNewConstraintsRef,
    StmtNewConstraintsStatic,
    StmtNewConstraintsFromConvert,
    TermCollectResolvedArgs,
    TermResolveArgs,
    TermInterpStaticCall,
    TermInterpStaticCallPost,
    TermInterpStaticCallPost1,
    TermInterpStaticCallPost2,
    TermInterpStaticCallPost3,
    TermInterpStaticCallPost4,
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
    ConvertAggAdtOpaqueAppend,
    ConvertAggAdtFields,
    ConvertAggFieldsInsert,
    ConvertAggConstructRes,
    ConvertAggConstraintsFrom,
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
    GetConstraintsFilterVariantPush1,
    GetConstraintsFilterVariantPush2,
    GetConstraintsStepField,
    StepFieldAppend,
    // lift_traitobjtys parts
    LiftTraitobjtysHashVal,
    LiftTraitobjtysUncached,
    LiftTraitobjtysUncachedGetTraitobj,
    LiftTraitobjtysUncachedPush1,
    LiftTraitobjtysUncachedPush2,
    LiftTraitobjtysUncachedPush3,
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
    // merge_stores_fallback parts - this path used to be permanently
    // un-instrumented (no InterpPass available at its callers); now that
    // scoped_update and the Merge trait carry timing through, it gets its
    // own categories, separate from merge_stores_timed's, so the two paths'
    // costs stay distinguishable.
    MergeStoresFallbackEs,
    MergeStoresFallbackCmapLoop,
    MergeStoresFallbackRefsUnion,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct TimingStats {
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
    pub(crate) fn dump_self_time_report(&self, label: &str, map: &HashMap<VOID, (std::time::Duration, u64)>) {
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

    pub(crate) fn dump_timing_report(&self, label: &str, map: &HashMap<TimingCat, TimingStats>) {
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

    pub(crate) fn dump_timing_by_scope_report(
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
    pub(crate) fn dump_timing_by_scope_exclusive_report(
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

}
