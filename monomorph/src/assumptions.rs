//! Where the analysis relied on assumptions instead of interpreting real code.
//!
//! The analysis is only as sound as its fallbacks. Wherever it answers a call
//! without interpreting the callee's MIR - no body, a compiler intrinsic, a
//! recursion cut, a stub - it models the result (usually "any value of the
//! return type") but not the call's *side effects*: writes through `&mut` or
//! raw-pointer arguments, or calls it makes to closures and trait objects it
//! was given. A trait object flowing only through such an effect is invisible
//! to FSA. This module records every such event per run, so the remaining
//! assumptions can be reviewed (they're listed at the end of `stats`); with
//! the standard library built from source (VERIFOPT_BUILD_STD) the list
//! should be short. It doesn't say which dispatch sites depend on which
//! assumption - that would need provenance tracked through every constraint.
//!
//! It also records the analysis's own *widenings* that can lose information.
//! In this analysis a missing ADT field reads as empty - "nothing", not
//! "unknown" - so `widen_constraints`, which drops ADT fields to bound the
//! analysis's size, forgets what those fields held (see the Widen* reasons),
//! and a memo lookup under a widened key can reuse a result computed for
//! different arguments (MemoWidenedReuse).
//!
//! The log is thread-local (the analysis runs on one thread) so code without
//! access to the interpreter - Constraints, merging - can record into it.

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::fmt::Write as _;

use rustc_public::mir::Mutability;
use rustc_public::ty::{FnDef, GenericArgKind, PolyFnSig, RigidTy, Ty, TyKind};

/// Why a call wasn't interpreted from its MIR (or what else was assumed).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Reason {
    /// No MIR body available (e.g. precompiled, or foreign).
    NoMir,
    /// A compiler intrinsic.
    Intrinsic,
    /// A recursive call answered from its signature.
    RecursionCut,
    /// A call answered from its signature because nested re-interpretation
    /// of recursive calls exceeded MAX_DEPTH.
    DepthLimit,
    /// Generic arguments still containing parameters.
    NonMonomorphicArgs,
    /// Generic arguments of the wrong count or kind for the callee.
    ArgsMismatch,
    /// `Instance::resolve` failed.
    UnresolvedInstance,
    /// A dispatch candidate that itself resolves to a vtable call.
    VirtualInstance,
    /// A dispatch candidate's simulation panicked and was recovered.
    CandidatePanicked,
    /// Answered by a hand-written summary (stdlib_stubs).
    Stub,
    /// A compiler-inserted precondition check (stubbed; can't affect
    /// trait-object flow - listed separately so it doesn't drown the rest).
    PreconditionCheck,
    /// A collection's contents widened to "any value of the element types"
    /// (stdlib_stubs::widen_collection) - sound: it *adds* possibilities.
    CollectionWidened,
    /// "Any value" of a type with a trait object nested in an ADT the model
    /// can't represent (stdlib_stubs::any_value) - reads of its fields come
    /// back empty, so that trait object is lost.
    NestedTraitObjectLost,
    /// A merge with more than MERGE_WIDEN_THRESHOLD disjuncts was widened,
    /// dropping the fields of this ADT (merge.rs).
    WidenMerge,
    /// A field write's value was larger than NEW_VALUE_WIDEN_THRESHOLD nodes
    /// and was widened, dropping the fields of this ADT (Constraints::write_field).
    WidenWriteValue,
    /// A field write into more than WRITE_FIELD_WIDEN_THRESHOLD disjuncts
    /// widened them first, dropping the fields of this ADT (Constraints::write_field).
    WidenWriteDisjuncts,
    /// A memoized result was reused under a *widened* key (after 50 distinct
    /// argument sets for this function): it may have been computed for
    /// arguments that differ from this call's inside ADT fields.
    MemoWidenedReuse,
    /// A closure's captured value was larger than NEW_VALUE_WIDEN_THRESHOLD
    /// nodes and was widened, dropping the fields of this ADT / closure
    /// (convert_agg, AggregateKind::Closure).
    WidenCapture,
    /// A call through a fn pointer whose target the analysis doesn't know
    /// (interp_fn_ptr): answered from the signature, the callee isn't
    /// interpreted - e.g. `core::fmt::rt::Argument::fmt` calling a
    /// `Display::fmt` impl.
    FnPtrCall,
}

impl Reason {
    pub fn label(self) -> &'static str {
        match self {
            Reason::NoMir => "no MIR",
            Reason::Intrinsic => "intrinsic",
            Reason::RecursionCut => "recursion cut",
            Reason::DepthLimit => "recursion depth limit",
            Reason::NonMonomorphicArgs => "non-monomorphic args",
            Reason::ArgsMismatch => "args mismatch",
            Reason::UnresolvedInstance => "unresolved instance",
            Reason::VirtualInstance => "virtual instance",
            Reason::CandidatePanicked => "candidate panicked",
            Reason::Stub => "stub",
            Reason::PreconditionCheck => "precondition check",
            Reason::CollectionWidened => "collection widened",
            Reason::NestedTraitObjectLost => "nested trait object lost",
            Reason::WidenMerge => "widened (merge size) - dropped fields of",
            Reason::WidenWriteValue => "widened (written value size) - dropped fields of",
            Reason::WidenWriteDisjuncts => "widened (written place's disjuncts) - dropped fields of",
            Reason::MemoWidenedReuse => "memo reuse under widened key",
            Reason::WidenCapture => "widened (captured value size) - dropped fields of",
            Reason::FnPtrCall => "fn-pointer call (not followed)",
        }
    }

    /// Calls answered without interpreting the callee (so its side effects
    /// through arguments aren't modelled) - the ones whose risk flags matter.
    fn skips_side_effects(self) -> bool {
        matches!(
            self,
            Reason::NoMir
                | Reason::Intrinsic
                | Reason::RecursionCut
                | Reason::DepthLimit
                | Reason::NonMonomorphicArgs
                | Reason::ArgsMismatch
                | Reason::UnresolvedInstance
                | Reason::VirtualInstance
                | Reason::CandidatePanicked
                | Reason::FnPtrCall
        )
    }
}

struct Entry {
    count: usize,
    risks: Vec<&'static str>,
}

#[derive(Default)]
pub struct Assumptions {
    entries: BTreeMap<(Reason, String), Entry>,
}

impl Assumptions {
    /// Records one event; `risks` is only computed the first time `name` is
    /// seen for `reason`.
    pub fn record(&mut self, reason: Reason, name: String, risks: impl FnOnce() -> Vec<&'static str>) {
        self.entries
            .entry((reason, name))
            .or_insert_with(|| Entry { count: 0, risks: risks() })
            .count += 1;
    }

    /// The `--ASSUMPTIONS--` section for `stats`: first the calls whose
    /// unmodelled side effects could hide trait-object flows (by count), then
    /// everything per reason. (No line starts with `Caller:`/`Span:`, so the
    /// test harness's stats parser ignores it.)
    pub fn report(&self) -> String {
        let mut out = String::from("--ASSUMPTIONS--\n");
        let mut by_reason: BTreeMap<Reason, (usize, usize)> = BTreeMap::new();
        for ((reason, _), e) in &self.entries {
            let r = by_reason.entry(*reason).or_default();
            r.0 += 1;
            r.1 += e.count;
        }
        let _ = writeln!(out, "Summary (distinct functions / calls):");
        for (reason, (distinct, calls)) in &by_reason {
            let _ = writeln!(out, "  {}: {distinct} / {calls}", reason.label());
        }

        let mut risky: Vec<(&(Reason, String), &Entry)> = self
            .entries
            .iter()
            .filter(|((reason, _), e)| reason.skips_side_effects() && !e.risks.is_empty())
            .collect();
        risky.sort_by(|a, b| b.1.count.cmp(&a.1.count).then(a.0.cmp(b.0)));
        let _ = writeln!(
            out,
            "Not interpreted, with arguments through which effects could be missed ({}):",
            risky.len()
        );
        for ((reason, name), e) in &risky {
            let _ = writeln!(out, "  [{}] {name} x{} ({})", reason.label(), e.count, e.risks.join(", "));
        }

        let _ = writeln!(out, "All, per reason:");
        for ((reason, name), e) in &self.entries {
            let risks = if e.risks.is_empty() { String::new() } else { format!(" ({})", e.risks.join(", ")) };
            let _ = writeln!(out, "  [{}] {name} x{}{risks}", reason.label(), e.count);
        }
        out
    }
}

/// What a callee's parameters would let it do that a fallback doesn't model.
pub fn signature_risks(sig: &PolyFnSig) -> Vec<&'static str> {
    input_risks(sig.value.inputs())
}

/// signature_risks, for a list of parameter types.
pub fn input_risks(inputs: &[Ty]) -> Vec<&'static str> {
    let mut risks = Vec::new();
    for input in inputs {
        collect_risks(input, 0, &mut risks);
    }
    risks.sort();
    risks.dedup();
    risks
}

fn collect_risks(ty: &Ty, depth: usize, risks: &mut Vec<&'static str>) {
    if depth > 4 {
        return;
    }
    match ty.kind() {
        TyKind::RigidTy(RigidTy::Ref(_, inner, mutability)) => {
            if matches!(mutability, Mutability::Mut) {
                risks.push("&mut arg");
            }
            collect_risks(&inner, depth + 1, risks);
        }
        TyKind::RigidTy(RigidTy::RawPtr(inner, _)) => {
            risks.push("raw-pointer arg");
            collect_risks(&inner, depth + 1, risks);
        }
        TyKind::RigidTy(RigidTy::Dynamic(..)) => risks.push("trait-object arg"),
        TyKind::RigidTy(
            RigidTy::Closure(..) | RigidTy::FnPtr(..) | RigidTy::FnDef(..) | RigidTy::Coroutine(..),
        ) => risks.push("closure/fn arg"),
        TyKind::RigidTy(RigidTy::Adt(_, args)) => {
            for arg in &args.0 {
                if let GenericArgKind::Type(t) = arg {
                    collect_risks(t, depth + 1, risks);
                }
            }
        }
        TyKind::RigidTy(RigidTy::Tuple(tys)) => {
            for t in &tys {
                collect_risks(t, depth + 1, risks);
            }
        }
        TyKind::RigidTy(RigidTy::Slice(inner) | RigidTy::Array(inner, _)) => {
            collect_risks(&inner, depth + 1, risks);
        }
        TyKind::Param(_) => risks.push("generic arg (may be a closure)"),
        _ => {}
    }
}

thread_local! {
    static LOG: RefCell<Assumptions> = RefCell::new(Assumptions::default());
    static WIDEN_CAUSE: Cell<Option<Reason>> = const { Cell::new(None) };
}

/// Records one event in this thread's log (see Assumptions::record).
pub fn record(reason: Reason, name: String, risks: impl FnOnce() -> Vec<&'static str>) {
    LOG.with(|log| log.borrow_mut().record(reason, name, risks));
}

/// A call to `fndef` answered for `reason`.
pub fn record_call(reason: Reason, fndef: &FnDef) {
    record(reason, fndef.0.name(), || signature_risks(&fndef.fn_sig()));
}

/// Runs `f` (a widening of *values*) with `cause` as the reason any dropped
/// ADT fields are recorded under. widen_constraints is also used for memo
/// *keys*, which lose nothing - those calls run without a cause, and record
/// nothing.
pub fn with_widen_cause<T>(cause: Reason, f: impl FnOnce() -> T) -> T {
    let previous = WIDEN_CAUSE.with(|c| c.replace(Some(cause)));
    let out = f();
    WIDEN_CAUSE.with(|c| c.set(previous));
    out
}

/// Called by widen_constraints when it drops a non-empty field map of ADT
/// `adt` (recorded only under a cause - see with_widen_cause).
pub fn note_dropped_fields(adt: &str) {
    if let Some(cause) = WIDEN_CAUSE.with(|c| c.get()) {
        record(cause, adt.to_string(), Vec::new);
    }
}

/// Starts a fresh log (one analysis per process, but don't rely on it).
pub fn reset() {
    LOG.with(|log| *log.borrow_mut() = Assumptions::default());
}

/// This thread's log, as the --ASSUMPTIONS-- section (see Assumptions::report).
pub fn report() -> String {
    LOG.with(|log| log.borrow().report())
}
