use crate::interp::InterpPass;
use crate::timing::TimingCat;
use crate::rustc_public::CrateDef;
use rustc_public::mir::mono::Instance;

use rustc_public::DefId;
use rustc_public::mir::{Body, LocalDecl, Mutability, Operand, Place, ProjectionElem};
use rustc_public::ty::{
    AdtDef, Binder, ClosureDef, ExistentialPredicate, FnDef, GenericArgs, Span, TraitDef,
    VariantIdx,
};

use crate::merge::merge_mapvals;
use crate::sig_collect::SigVal;
use crate::wto::BBDeps;

use log::{debug, warn};

use im::HashMap as ImHashMap;
use im::HashSet as ImHashSet;
use indexmap::IndexSet;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::rc::{Rc, Weak};

// Once a base's own disjunct count has already proven itself this large,
// write_field's per-disjunct loop (clone one, insert the new field into
// it) costs O(base size) per call regardless of how cheap each individual
// clone is - and that cost compounds across every subsequent write to the
// same place. Matches the 50-entry convention used for the exact_memo/
// summaries caps elsewhere in this codebase.
const WRITE_FIELD_WIDEN_THRESHOLD: usize = 50;

// Separate, much higher threshold for checking the *incoming* value's own
// recursive size (constraints_size, not just top-level disjunct count).
// Ordinary, non-pathological values legitimately reach into the low
// thousands here (e.g. a struct's own field values observed up to ~3,330);
// this only needs to catch cases orders of magnitude beyond that.
const NEW_VALUE_WIDEN_THRESHOLD: usize = 5_000;

/// The two field maps' keys whose values differ (a key only one has counts),
/// stopping once more than `limit` are found.
fn differing_fields(a: &ADTFields, b: &ADTFields, limit: usize) -> Vec<usize> {
    let mut out = Vec::new();
    for k in a.keys().chain(b.keys().filter(|k| !a.contains_key(k))) {
        if a.get(k) != b.get(k) {
            out.push(*k);
            if out.len() > limit {
                break;
            }
        }
    }
    out
}

/// If `a` and `b` are the same ADT (def, generic args, variant) or closure
/// (def, generic args), with the same trait-object type if any, and differ
/// in at most one field / capture position (counting cfc and toc fields
/// together), the single disjunct equal to both: that position's value is
/// their union. Lossless - with every other field equal, no combination of
/// field values is added - unlike merging disjuncts that differ in two
/// fields, which would lose which values go together. Provenance is joined.
fn merge_if_one_field_differs(a: &Constraint, b: &Constraint) -> Option<Constraint> {
    fn cfc_fields(c: &Option<RunningConstraint>) -> Option<&ADTFields> {
        match c {
            Some(RunningConstraint::Adt(_, _, _, f)) | Some(RunningConstraint::Closure(_, _, f)) => Some(f),
            _ => None,
        }
    }
    fn toc_fields(t: &TraitObjConstraint) -> &ADTFields {
        match t {
            TraitObjConstraint::Adt(_, _, _, f) | TraitObjConstraint::Closure(_, _, f) => f,
        }
    }
    let same_cfc_shape = match (&a.cfc, &b.cfc) {
        (Some(RunningConstraint::Adt(d1, g1, v1, _)), Some(RunningConstraint::Adt(d2, g2, v2, _))) => {
            d1 == d2 && v1 == v2 && g1 == g2
        }
        (Some(RunningConstraint::Closure(d1, g1, _)), Some(RunningConstraint::Closure(d2, g2, _))) => {
            d1 == d2 && g1 == g2
        }
        _ => false,
    };
    if !same_cfc_shape {
        return None;
    }
    let same_toc_shape = match (&a.toc, &b.toc) {
        (None, None) => true,
        (Some((t1, c1)), Some((t2, c2))) => {
            t1 == t2
                && match (c1, c2) {
                    (TraitObjConstraint::Adt(d1, g1, v1, _), TraitObjConstraint::Adt(d2, g2, v2, _)) => {
                        d1 == d2 && v1 == v2 && g1 == g2
                    }
                    (TraitObjConstraint::Closure(d1, g1, _), TraitObjConstraint::Closure(d2, g2, _)) => {
                        d1 == d2 && g1 == g2
                    }
                    _ => false,
                }
        }
        _ => false,
    };
    if !same_toc_shape {
        return None;
    }
    let cfc_diff = differing_fields(cfc_fields(&a.cfc)?, cfc_fields(&b.cfc)?, 1);
    let toc_diff = match (&a.toc, &b.toc) {
        (Some((_, c1)), Some((_, c2))) => differing_fields(toc_fields(c1), toc_fields(c2), 1),
        _ => Vec::new(),
    };
    if cfc_diff.len() + toc_diff.len() > 1 {
        return None;
    }
    // A field one side lacks is unknown there (see step_field_one): the
    // union would replace "unknown" with the other side's value, so those
    // stay separate disjuncts.
    let present_on_both = |fa: Option<&ADTFields>, fb: Option<&ADTFields>, k: usize| {
        fa.is_some_and(|f| f.contains_key(&k)) && fb.is_some_and(|f| f.contains_key(&k))
    };
    if let Some(&k) = cfc_diff.first()
        && !present_on_both(cfc_fields(&a.cfc), cfc_fields(&b.cfc), k)
    {
        return None;
    }
    if let Some(&k) = toc_diff.first()
        && !present_on_both(
            a.toc.as_ref().map(|(_, c)| toc_fields(c)),
            b.toc.as_ref().map(|(_, c)| toc_fields(c)),
            k,
        )
    {
        return None;
    }
    let mut merged = a.clone();
    let union_at = |into: &mut ADTFields, from: &ADTFields, k: usize| {
        if let Some(v) = from.get(&k) {
            into.entry(k).or_insert_with(Constraints::new).append(v.clone());
        }
    };
    if let Some(&k) = cfc_diff.first() {
        let from = cfc_fields(&b.cfc).expect("same shape").clone();
        if let Some(RunningConstraint::Adt(_, _, _, f)) | Some(RunningConstraint::Closure(_, _, f)) = &mut merged.cfc {
            union_at(f, &from, k);
        }
    }
    if let Some(&k) = toc_diff.first() {
        let from = toc_fields(&b.toc.as_ref().expect("same shape").1).clone();
        if let Some((_, TraitObjConstraint::Adt(_, _, _, f) | TraitObjConstraint::Closure(_, _, f))) = &mut merged.toc {
            union_at(f, &from, k);
        }
    }
    merged.prov.join(&b.prov);
    Some(merged)
}
/// `value`, widened (ADT fields / captures dropped, recorded under `cause`)
/// if its recursive size exceeds NEW_VALUE_WIDEN_THRESHOLD - the bound
/// write_field applies to written values, for values built elsewhere.
pub fn bound_value_size(value: Constraints, cause: crate::assumptions::Reason) -> Constraints {
    if constraints_size(&value) > NEW_VALUE_WIDEN_THRESHOLD {
        crate::assumptions::with_widen_cause(cause, || widen_constraints(&value))
    } else {
        value
    }
}

pub fn unique_push<T: PartialEq>(vec: &mut Vec<T>, elem: T) -> Option<T> {
    if vec.contains(&elem) {
        Some(elem)
    } else {
        vec.push(elem);
        None
    }
}

pub fn unique_append<T: PartialEq>(vec: &mut Vec<T>, to_append: Vec<T>) {
    for elem in to_append {
        unique_push(vec, elem);
    }
}

/// Using `Instance` as unique ID (internal objects are interned so this is apparently cheap)
///
/// The third field is a k-CFA-style bounded call context: the last `k`
/// immediate callers' own (Instance, GenericArgs, call-site Span)
/// identities, most recent first, where `k` is `--context-depth` (see
/// util/options.rs). The Span is which specific call site *within* that
/// caller produced this scope - without it, two structurally distinct
/// calls from the same caller function (e.g. `main`'s two separate
/// calls to `get_animal`, once for a Cat, once for a Dog) would still
/// collapse onto one identity, since the caller's own (Instance,
/// GenericArgs) alone doesn't distinguish "which of its call sites".
/// This is what lets two different call sites reaching the same callee
/// be distinguished as genuinely different scopes, rather than
/// silently merged - which is what caused the REFS/WTOS MERGE CONFLICT
/// panics (see interp.rs) to fire on cases like diff_sites, where
/// first_speak and second_speak both dynamically dispatch to the same
/// Cat::speak but with genuinely different receiver aliasing per call
/// site. k=0 reproduces the previous (unfixed) context-insensitive
/// behavior exactly - an empty context is indistinguishable from no
/// context tracking at all.
///
/// Constructed via `push_caller_context` below, never directly, so
/// direct/mutual recursion still terminates: each entry is the
/// caller's own *base* identity (plus this one call site) only, never
/// folding in the caller's own context recursively, which would
/// otherwise make the context grow unboundedly with recursion depth
/// and break the `call_stack.contains(...)`-based cycle detection that
/// recursive interpretation relies on. A recursive call's own call
/// site is the same source-level expression on every recursive step
/// (e.g. `sum`'s own `n + sum(next)`), so this doesn't fragment
/// recursion the way it correctly does fragment get_animal's two
/// distinct call sites.
pub type CallContext = Vec<(Instance, GenericArgs, Span)>;
pub type VOID = (Instance, GenericArgs, CallContext);

/// Builds the CallContext for a new scope being entered from
/// `caller_scope` via `call_site`, bounded to `k` entries. Prepends the
/// caller's own base (Instance, GenericArgs) plus this call site - not
/// `caller_scope.clone()` as a whole, which would fold in the caller's
/// own context and grow unboundedly across recursion depth - then
/// keeps only as much of the caller's own existing context as still
/// fits within the bound.
pub fn push_caller_context(caller_scope: &VOID, call_site: Span, k: usize) -> CallContext {
    if k == 0 {
        return Vec::new();
    }
    let mut ctx = Vec::with_capacity(k);
    ctx.push((caller_scope.0.clone(), caller_scope.1.clone(), call_site));
    ctx.extend(caller_scope.2.iter().take(k - 1).cloned());
    ctx
}

/// The scope-identity component used for `cmap` (the main
/// constraint-propagation store) and the "summary cache" family
/// (summaries, exact_memo, scope_epoch, scope_exact_memo_count,
/// scope_summaries_count). Currently uses the *same* full k-deep
/// CallContext as VOID itself - i.e. cmap is exactly as context-
/// sensitive as refs/wtos/call_stack are, not a coarser projection of
/// it. An *immediate-call-site-only* version was tried first (enough
/// to distinguish get_animal's two call sites from each other), but a
/// case one level of indirection deeper defeats it: if `wrapper()` is
/// called from two different places but itself has only one call site
/// to some `helper()`, an immediate-only projection gives every call
/// to `helper()` the same identity regardless of which `wrapper()`
/// invocation is running, since `helper()`'s own call site never
/// changes - only the *chain* leading up to it does. Using the full
/// k-deep context here closes that gap. This is a deliberate
/// correctness-first choice: full k-CFA sensitivity for cmap risks
/// fragmenting the sharing flatten_all's memoization (and friends)
/// rely on for tractable performance at scale, more so than the
/// immediate-only version did - if that turns out to matter in
/// practice, revisiting this (e.g. a separate, shallower depth just
/// for cmap, decoupled from --context-depth) is the place to look,
/// once there's a concrete performance case motivating it rather than
/// a theoretical one. Constructed only via `base_scope_id` below,
/// never directly.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BaseScopeId(pub Instance, pub GenericArgs, pub CallContext);

pub fn base_scope_id(scope: &VOID) -> BaseScopeId {
    BaseScopeId(scope.0.clone(), scope.1.clone(), scope.2.clone())
}

/// The summary-cache family's own key shape, mirroring SummaryKey's
/// (scope, args) structure but with the caller-tag projected down to
/// just the immediate call site via BaseScopeId - unlike SummaryKey
/// itself, which must keep the full VOID (see prepare_call:
/// `call_stack.push(key.0.clone())` needs the full caller-tag chain
/// for correct recursion detection).
pub type MemoKey = (BaseScopeId, ArgSet);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapKey {
    Var(Place),
    ScopeId(BaseScopeId),
    Static(DefId),
}

/// The union of two stores' enclosing scopes - the same Rc back when the
/// union adds nothing, so identity (and merge::mapvals_identical) survives.
pub fn merge_enclosing_scopes(cur: &EnclosingScopes, new: &EnclosingScopes) -> EnclosingScopes {
    match (cur, new) {
        (Some(c), Some(n)) if Rc::ptr_eq(c, n) => Some(c.clone()),
        (Some(c), Some(n)) => {
            let mut v = (**c).clone();
            unique_append(&mut v, n.to_vec());
            if v.len() == c.len() { Some(c.clone()) } else { Some(Rc::new(v)) }
        }
        (Some(c), None) => Some(c.clone()),
        (None, Some(n)) => Some(n.clone()),
        (None, None) => None,
    }
}

/// Shared (Rc), so cloning a store - e.g. when a persistent map copies a node -
/// shares the list instead of copying it, and identity checks can use ptr_eq.
pub type EnclosingScopes = Option<Rc<Vec<VOID>>>;

#[derive(Debug, Clone, PartialEq)]
pub enum MapValue {
    Store(ConstraintStore, EnclosingScopes),
    Constraints(Constraints),
}

pub fn adt_field_idx(elem: &ProjectionElem) -> usize {
    match elem {
        ProjectionElem::Field(idx, _) => *idx,
        _ => panic!("expected Field projection: {:?}", elem),
    }
}

// Set of positive constraints; negative constraints are resolved immediately by removing them from the set
#[derive(Default, Clone)]
pub struct Constraints {
    pub inner: Rc<IndexSet<Constraint>>,
    /// Cached content hash (see content_hash). Values nest deeply (fields,
    /// captures), and every push into an IndexSet hashes the whole disjunct,
    /// so recomputing it each time made merges quadratic in value size.
    /// Reset by everything that changes `inner` (push, write_field) - code
    /// that replaces `inner` directly must call `invalidate_hash`.
    hash: Cell<Option<u64>>,
}

impl std::fmt::Debug for Constraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Constraints").field("inner", &self.inner).finish()
    }
}

impl Constraints {
    /// Order-independent hash of the disjuncts, computed once per value.
    fn content_hash(&self) -> u64 {
        if let Some(h) = self.hash.get() {
            return h;
        }
        let mut combined: u64 = 0;
        for elem in self.inner.iter() {
            let mut h = DefaultHasher::new();
            elem.hash(&mut h);
            combined ^= h.finish();
        }
        self.hash.set(Some(combined));
        combined
    }

    pub fn invalidate_hash(&mut self) {
        self.hash.set(None);
    }
}

impl Hash for Constraints {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.content_hash().hash(state);
    }
}

impl PartialEq for Constraints {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
            || (self.inner.len() == other.inner.len()
                && self.content_hash() == other.content_hash()
                && self.inner == other.inner)
    }
}
impl Eq for Constraints {}

impl Constraints {
    pub fn new() -> Constraints {
        Self { inner: Rc::new(IndexSet::new()), hash: Cell::new(None) }
    }

    pub fn from(constraint: Constraint) -> Constraints {
        let mut set = IndexSet::with_capacity(1);
        set.insert(constraint);
        Self { inner: Rc::new(set), hash: Cell::new(None) }
    }

    pub fn from_vec(inner: Vec<Constraint>) -> Constraints {
        Self { inner: Rc::new(inner.into_iter().collect()), hash: Cell::new(None) }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn at(&self, idx: usize) -> &Constraint {
        self.inner
            .get_index(idx)
            .unwrap_or_else(|| panic!("Constraints::at: index {} out of bounds", idx))
    }

    pub fn push(&mut self, new_constraint: Constraint) {
        // insert() dedups on (toc, cfc) via the custom Eq/Hash impl above,
        // but a plain insert() would silently drop the incoming prov when a
        // duplicate is already present. Merge provenance into the existing
        // entry (via replace(), which keeps insertion order) instead of
        // just discarding it.
        let mut new_constraint = new_constraint;
        // Lossless merging (see merge_if_one_field_differs): absorb, one at a
        // time, every existing disjunct that differs from the incoming one in
        // at most one field, so values differing only in e.g. a scalar
        // capacity don't multiply into separate disjuncts.
        //
        // The merge is computed once, for the disjunct it absorbs: it unions
        // the differing field, which pushes into that field's value, so
        // merging once to test and again to apply doubled the work at every
        // nesting level - 2^depth for deeply nested values (tock's
        // statically wired objects stalled for hours).
        while let Some((i, merged)) = self.inner.iter().enumerate().find_map(|(i, c)| {
            if *c == new_constraint {
                return None;
            }
            merge_if_one_field_differs(c, &new_constraint).map(|m| (i, m))
        }) {
            new_constraint = merged;
            Rc::make_mut(&mut self.inner).shift_remove_index(i);
        }
        if let Some(existing) = self.inner.get(&new_constraint) {
            let mut merged = existing.clone();
            merged.prov.join(&new_constraint.prov);
            Rc::make_mut(&mut self.inner).replace(merged);
        } else {
            Rc::make_mut(&mut self.inner).insert(new_constraint);
        }
        self.invalidate_hash();
    }

    pub fn append(&mut self, new_constraints: Constraints) {
        for c in new_constraints.inner.iter() {
            self.push(c.clone());
        }
    }

    // Write: strong-update the field within EVERY disjunct currently in scope.
    // This is what makes {A, B}.f = C become {A{f:C}, B} instead of touching a global table.
    pub fn write_field(&mut self, projection: Vec<ProjectionElem>, new: Constraints) {
        // An array's or slice's value is the union of its elements (index
        // projections are no-ops on values), so a write to one element is a
        // weak update: the strong one, joined with the old value - the other
        // elements keep theirs. (Done as a strong update, `t[0] = a; t[1] =
        // b` left only `b`.)
        let is_index = |e: &ProjectionElem| {
            matches!(
                e,
                ProjectionElem::Index(_) | ProjectionElem::ConstantIndex { .. } | ProjectionElem::Subslice { .. }
            )
        };
        if projection.iter().any(is_index) {
            let mut updated = self.clone();
            updated.write_field(projection.into_iter().filter(|e| !is_index(e)).collect(), new);
            self.append(updated);
            return;
        }
        // The base-size cap below only catches "many base disjuncts, each
        // getting a copy of new". It misses the other half: new itself
        // already being huge before it ever reaches here, in which case
        // even a single base disjunct ends up holding a multi-million-node
        // value. Recursive size (not just new.inner.len()) is what matters,
        // since new's own size could be hiding in nested fields rather
        // than at its own top level.
        let new = if constraints_size(&new) > NEW_VALUE_WIDEN_THRESHOLD {
            crate::assumptions::with_widen_cause(crate::assumptions::Reason::WidenWriteValue, || {
                widen_constraints(&new)
            })
        } else {
            new
        };

        let target_variant = projection.iter().find_map(|e| match e {
            ProjectionElem::Downcast(v) => Some(*v),
            _ => None,
        });

        let field: Vec<ProjectionElem> = projection
            .into_iter()
            .filter(|e| matches!(e, ProjectionElem::Field(..)))
            .collect();

        match (field.len(), target_variant) {
            // *x = v - no field, no downcast: replace the whole pointee outright.
            (0, None) => {
                *self = new;
            }

            // (x as Variant) = v with no further Field: writing a variant's entire
            // payload as one unit rather than through a named field.
            (0, Some(_v)) => {
                todo!(
                    "whole-variant write without a Field projection: {:?}",
                    target_variant
                );
            }

            // The ordinary case: update one field, honoring the same variant-scoping
            // as filter_variant on the read side.
            (1, target_variant) => {
                let idx = adt_field_idx(&field[0]);
                if self.inner.len() > WRITE_FIELD_WIDEN_THRESHOLD {
                    *self = crate::assumptions::with_widen_cause(
                        crate::assumptions::Reason::WidenWriteDisjuncts,
                        || widen_constraints(self),
                    );
                }
                let old = std::mem::take(&mut self.inner);
                let new_set: IndexSet<Constraint> = old
                    .iter()
                    .map(|c| {
                        let mut c = c.clone();
                        if let Some(RunningConstraint::Adt(_, _, variant, fields)) = &mut c.cfc {
                            let applies = match target_variant {
                                Some(v) => variant.is_none() || *variant == Some(v),
                                None => true,
                            };
                            if applies {
                                fields.insert(idx, new.clone());
                            }
                        } else if let Some(RunningConstraint::Closure(_, _, captures)) = &mut c.cfc {
                            // a by-value capture written by an FnMut closure
                            captures.insert(idx, new.clone());
                        }
                        c
                    })
                    .collect();
                self.inner = Rc::new(new_set);
                self.invalidate_hash();
            }

            (_, _) => {
                let (first, rest) = field.split_first().expect("len >= 2 per match arm");
                let idx = adt_field_idx(first);
                let rest = rest.to_vec();

                if self.inner.len() > WRITE_FIELD_WIDEN_THRESHOLD {
                    *self = crate::assumptions::with_widen_cause(
                        crate::assumptions::Reason::WidenWriteDisjuncts,
                        || widen_constraints(self),
                    );
                }
                let old = std::mem::take(&mut self.inner);
                let new_set: IndexSet<Constraint> = old
                    .iter()
                    .map(|c| {
                        let mut c = c.clone();
                        if let Some(RunningConstraint::Adt(_, _, variant, fields)) = &mut c.cfc {
                            let applies = match target_variant {
                                Some(v) => variant.is_none() || *variant == Some(v),
                                None => true,
                            };
                            if applies {
                                let mut nested =
                                    fields.get(&idx).cloned().unwrap_or_else(|| field_skeleton(first));
                                nested.write_field(rest.clone(), new.clone());
                                fields.insert(idx, nested);
                            }
                        } else if let Some(RunningConstraint::Closure(_, _, captures)) = &mut c.cfc {
                            let mut nested =
                                captures.get(&idx).cloned().unwrap_or_else(|| field_skeleton(first));
                            nested.write_field(rest.clone(), new.clone());
                            captures.insert(idx, nested);
                        }
                        c
                    })
                    .collect();
                self.inner = Rc::new(new_set);
                self.invalidate_hash();
            }
        }
    }

    pub fn filter_variant(
        &self,
        vidx: VariantIdx,
        scope: Option<&VOID>,
        timing: Option<&InterpPass>,
    ) -> Constraints {
        let mut out = Constraints::new();
        for c in self.inner.iter() {
            match &c.cfc {
                Some(RunningConstraint::Adt(_, _, variant, _)) => {
                    if variant.is_none() || *variant == Some(vidx) {
                        let cc = c.clone();
                        let _g = scope.zip(timing).map(|(s, p)| {
                            p.timing_span(TimingCat::GetConstraintsFilterVariantPush1, s)
                        });
                        out.push(cc);
                    }
                }
                // Unknown-yet placeholder from a parametric summary: we
                // can't know at summary-build time whether the real
                // argument's variant will match `vidx`, so conservatively
                // assume it might and record the downcast onto the path -
                // substitute_params replays it against the real value later.
                Some(RunningConstraint::Param(i, path)) => {
                    let mut new_path = path.clone();
                    new_path.push(ProjStep::Downcast(vidx));
                    let c = Constraint::new(None, Some(RunningConstraint::Param(*i, new_path)));
                    let _g = scope.zip(timing).map(|(s, p)| {
                        p.timing_span(TimingCat::GetConstraintsFilterVariantPush2, s)
                    });
                    out.push(c);
                }
                _ => {}
            }
        }
        out
    }
}

// Maybe organize TraitObjConstraints by trait..? Like if we have two potentially obfuscating
// dynamic calls (one for Option and one for inner TraitObj)
#[derive(Debug, Clone)]
pub struct Constraint {
    pub toc: Option<(TraitObjTy, TraitObjConstraint)>,
    pub cfc: Option<RunningConstraint>,
    pub prov: TagProv,
}

impl PartialEq for Constraint {
    fn eq(&self, o: &Self) -> bool {
        self.toc == o.toc && self.cfc == o.cfc
    }
}
impl Eq for Constraint {}
impl Hash for Constraint {
    fn hash<H: Hasher>(&self, s: &mut H) {
        self.toc.hash(s);
        self.cfc.hash(s);
    }
}

impl Constraint {
    pub fn new(
        toc: Option<(TraitObjTy, TraitObjConstraint)>,
        cfc: Option<RunningConstraint>,
    ) -> Constraint {
        Self {
            toc,
            cfc,
            prov: TagProv::Unknown,
        }
    }

    pub fn with_prov(mut self, prov: TagProv) -> Self {
        self.prov = prov;
        self
    }

    pub fn is_cfc_closure(&self) -> bool {
        if self.cfc.is_none() {
            return false;
        }

        match self.cfc.as_ref().unwrap() {
            RunningConstraint::Closure(..) => true,
            _ => false,
        }
    }
}

pub type ADTFields = BTreeMap<usize, Constraints>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TraitObjConstraint {
    Adt(AdtDef, GenericArgs, Option<VariantIdx>, ADTFields),
    /// A closure value with its captured values (upvar index -> value), like
    /// an ADT's fields; a capture never written reads as empty.
    Closure(ClosureDef, GenericArgs, ADTFields),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    pub scope: Option<DefId>,
    pub bb: usize,
    pub stmt: usize,
}

impl Location {
    pub fn unknown() -> Self {
        Self {
            scope: None,
            bb: 0,
            stmt: 0,
        }
    }

    pub fn new_at(scope: DefId, bb: usize, stmt: usize) -> Self {
        Self {
            scope: Some(scope),
            bb,
            stmt,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagProv {
    Unknown,
    Tags(
        HashSet<(
            DefId, /* scope */
            usize, /* bb */
            usize, /* stmt */
        )>,
    ),
}

impl TagProv {
    pub fn join(&mut self, other: &TagProv) {
        match (&mut *self, other) {
            (TagProv::Unknown, _) => {}
            (_, TagProv::Unknown) => *self = TagProv::Unknown,
            (TagProv::Tags(a), TagProv::Tags(b)) => a.extend(b.iter().cloned()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunningConstraint {
    Scalar(Option<i128>),
    Float,
    Adt(AdtDef, GenericArgs, Option<VariantIdx>, ADTFields),
    Ptr(Box<Constraint>),
    //Ref(Box<Constraint>),
    /// A closure value with its captured values (upvar index -> value), like
    /// an ADT's fields; a capture never written reads as empty.
    Closure(ClosureDef, GenericArgs, ADTFields),
    FnDef(FnDef, GenericArgs),
    FnPtr(SigVal),
    Dynamic(Vec<TraitObjTy>),
    List(Box<Constraint>),
    Tuple(Vec<Constraints>),
    Idk(Box<Constraints>),
    /// Symbolic placeholder used only while building a *parametric function
    /// summary*: "the value in argument position `usize`, with the given
    /// projection path already applied."
    Param(usize, Vec<ProjStep>),
}

/// One step of a projection path recorded onto a `Param` placeholder while
/// building a parametric summary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjStep {
    Field(usize),
    Downcast(VariantIdx),
}

/// Replays a `Param` placeholder's recorded projection path against a real
/// argument's constraints. Deliberately duplicates (rather than reuses)
/// `Context::step_field_one`/`filter_variant`'s Adt/Tuple matching: those
/// take a `&ProjectionElem`/`VariantIdx` tied to real MIR type info, and
/// there's no legitimate `Ty` to fabricate for a placeholder step recorded
/// during summary-building - operating on a raw field index instead avoids
/// needing one.
fn apply_field_idx(base: &Constraints, idx: usize) -> Constraints {
    let mut out = Constraints::new();
    for c in base.inner.iter() {
        match &c.cfc {
            Some(RunningConstraint::Adt(_, _, _, fields))
            | Some(RunningConstraint::Closure(_, _, fields)) => {
                out.append(fields.get(&idx).cloned().unwrap_or_else(Constraints::new));
            }
            Some(RunningConstraint::Tuple(inner)) => {
                out.append(inner.get(idx).cloned().unwrap_or_else(Constraints::new));
            }
            Some(RunningConstraint::Ptr(box inner)) => {
                out.append(apply_field_idx(&Constraints::from(inner.clone()), idx));
            }
            Some(RunningConstraint::Idk(box inner_cs)) => {
                out.append(apply_field_idx(inner_cs, idx));
            }
            _ => {}
        }
    }
    out
}

fn apply_proj_path(base: &Constraints, path: &[ProjStep]) -> Constraints {
    let mut cur = base.clone();
    for step in path {
        cur = match step {
            ProjStep::Field(idx) => apply_field_idx(&cur, *idx),
            ProjStep::Downcast(vidx) => cur.filter_variant(*vidx, None, None),
        };
    }
    cur
}

fn substitute_toc(
    toc: &Option<(TraitObjTy, TraitObjConstraint)>,
    actual_args: &[Constraints],
) -> Option<(TraitObjTy, TraitObjConstraint)> {
    toc.as_ref().map(|(ty, tc)| {
        let new_tc = match tc {
            TraitObjConstraint::Adt(def, genargs, variant, fields) => {
                let mut new_fields = ADTFields::new();
                for (k, v) in fields {
                    new_fields.insert(*k, substitute_params(v, actual_args));
                }
                TraitObjConstraint::Adt(def.clone(), genargs.clone(), *variant, new_fields)
            }
            TraitObjConstraint::Closure(cdef, genargs, captures) => {
                let mut new_captures = ADTFields::new();
                for (k, v) in captures {
                    new_captures.insert(*k, substitute_params(v, actual_args));
                }
                TraitObjConstraint::Closure(*cdef, genargs.clone(), new_captures)
            }
        };
        (ty.clone(), new_tc)
    })
}

pub fn memoize_by_rc<K, T>(
    cache: &RefCell<HashMap<(usize, K), (Weak<IndexSet<Constraint>>, T)>>,
    cs: &Constraints,
    extra_key: K,
    compute: impl FnOnce() -> T,
) -> T
where
    K: Hash + Eq,
    T: Clone,
{
    let full_key = (Rc::as_ptr(&cs.inner) as usize, extra_key);
    if let Some((weak, cached)) = cache.borrow().get(&full_key) {
        if weak.upgrade().is_some() {
            return cached.clone();
        }
    }
    let result = compute();
    let mut cache = cache.borrow_mut();
    // Entries outlive their keys: the result is held strongly, and the Weak
    // keeps the key's allocation. Drop dead entries every PURGE_EVERY
    // insertions into this cache (amortized O(live entries / PURGE_EVERY)
    // per insert) - without it every value ever memoized stayed alive until
    // the end of the run. Purging only at power-of-two sizes (the first fix)
    // still let dead entries pile up between purges: in ripgrep ~200k dead
    // widen_constraints results (~100 KB each) held 20-30 GB, until the
    // memory guard stopped the run before the cache reached 2^20.
    // Also at power-of-two sizes, as before: early on, while the caches are
    // small, that purges more often than every PURGE_EVERY insertions.
    const PURGE_EVERY: u64 = 1 << 14;
    thread_local!(static INSERTS: RefCell<HashMap<usize, u64>> = RefCell::new(HashMap::new()));
    let purge = INSERTS.with(|m| {
        let mut m = m.borrow_mut();
        let n = m.entry(&*cache as *const HashMap<_, _> as usize).or_default();
        *n += 1;
        *n % PURGE_EVERY == 0
    });
    if purge || (cache.len() >= 4096 && cache.len().is_power_of_two()) {
        cache.retain(|_, (weak, _)| weak.strong_count() > 0);
    }
    cache.insert(full_key, (Rc::downgrade(&cs.inner), result.clone()));
    result
}

pub fn hash_val<T: Hash>(val: &T) -> u64 {
    let mut h = DefaultHasher::new();
    val.hash(&mut h);
    h.finish()
}

thread_local! {
    static CONSTRAINTS_SIZE_CACHE: RefCell<HashMap<(usize, ()), (Weak<IndexSet<Constraint>>, usize)>> = RefCell::new(HashMap::new());
    static CONTAINS_PARAM_CACHE: RefCell<HashMap<(usize, ()), (Weak<IndexSet<Constraint>>, bool)>> = RefCell::new(HashMap::new());
    static WIDEN_CONSTRAINTS_CACHE: RefCell<HashMap<(usize, ()), (Weak<IndexSet<Constraint>>, Constraints)>> = RefCell::new(HashMap::new());
    static WIDEN_SCALARS_CACHE: RefCell<HashMap<(usize, ()), (Weak<IndexSet<Constraint>>, Constraints)>> = RefCell::new(HashMap::new());
    // Real cache (not diagnostic) - flatten_all's own output, keyed on the
    // same Rc identity the diagnostic above confirmed is redundant ~99.96%
    // of the time. Separate from FLATTEN_ALL_SEEN, which stays purely as a
    // hit-rate counter so we can confirm this cache is actually paying off.
    static FLATTEN_ALL_CACHE: RefCell<HashMap<(usize, ()), (Weak<IndexSet<Constraint>>, Constraints)>> = RefCell::new(HashMap::new());
    // Diagnostic only (no caching/behavior change): tracks whether flatten_all
    // is repeatedly called on the *same* underlying Rc<IndexSet<Constraint>>
    // (same allocation, confirmed still alive via Weak::upgrade - not just a
    // reused address) - a high hit rate would mean flatten_all is a strong
    // candidate for the same memoize_by_rc treatment already used above.
    static FLATTEN_ALL_SEEN: RefCell<HashMap<usize, Weak<IndexSet<Constraint>>>> =
        RefCell::new(HashMap::new());
    static FLATTEN_ALL_HITS: Cell<u64> = Cell::new(0);
    static FLATTEN_ALL_MISSES: Cell<u64> = Cell::new(0);
}

/// Diagnostics (VERIFOPT_CACHE_SIZES): entries / dead entries (key dropped)
/// of a memoize_by_rc cache.
pub fn rc_cache_size<K, T>(cache: &RefCell<HashMap<(usize, K), (Weak<IndexSet<Constraint>>, T)>>) -> (usize, usize) {
    let cache = cache.borrow();
    (cache.len(), cache.values().filter(|(w, _)| w.strong_count() == 0).count())
}

/// Diagnostics: this module's memoize_by_rc caches, as "name=entries/dead ...".
pub fn rc_cache_sizes() -> String {
    let mut out = String::new();
    let mut add = |name: &str, (n, dead): (usize, usize)| out.push_str(&format!("{name}={n}/{dead} "));
    CONTAINS_PARAM_CACHE.with(|c| add("contains_param", rc_cache_size(c)));
    WIDEN_CONSTRAINTS_CACHE.with(|c| add("widen", rc_cache_size(c)));
    FLATTEN_ALL_CACHE.with(|c| add("flatten_all", rc_cache_size(c)));
    FLATTEN_ALL_SEEN.with(|c| {
        let c = c.borrow();
        add("flatten_seen", (c.len(), c.values().filter(|w| w.strong_count() == 0).count()))
    });
    out
}

/// Diagnostic only - records whether this exact `Constraints` allocation
/// (not just an equal one) has been seen by `flatten_all` before. Returns
/// nothing; updates thread-local hit/miss counters that `dump_flatten_all_cache_stats`
/// reports. Cheap: one hashmap probe + insert per call, no recursion, no
/// change to flatten_all's actual output.
fn record_flatten_all_rc_identity(constraints: &Constraints) {
    let ptr = Rc::as_ptr(&constraints.inner) as usize;
    let is_hit = FLATTEN_ALL_SEEN.with(|seen| {
        let mut seen = seen.borrow_mut();
        // A hit only counts if the *same allocation* is still alive -
        // if the old Rc at this address was dropped and a new, unrelated
        // one happened to be allocated at the same freed address, upgrade()
        // fails and we correctly treat this as a miss, not a false hit.
        let hit = seen
            .get(&ptr)
            .map(|w| w.upgrade().is_some())
            .unwrap_or(false);
        // Same purge as memoize_by_rc: a Weak keeps its allocation alive.
        if seen.len() >= 4096 && seen.len().is_power_of_two() {
            seen.retain(|_, w| w.strong_count() > 0);
        }
        seen.insert(ptr, Rc::downgrade(&constraints.inner));
        hit
    });
    if is_hit {
        FLATTEN_ALL_HITS.with(|c| c.set(c.get() + 1));
    } else {
        FLATTEN_ALL_MISSES.with(|c| c.set(c.get() + 1));
    }
}

/// Diagnostic only - prints the running flatten_all Rc-identity hit rate.
/// Call this from the same periodic/final checkpoints that print
/// TOTAL WALL CLOCK, so partial data survives even if the run panics
/// before finishing.
pub fn dump_flatten_all_cache_stats(label: &str) {
    let hits = FLATTEN_ALL_HITS.with(|c| c.get());
    let misses = FLATTEN_ALL_MISSES.with(|c| c.get());
    let total = hits + misses;
    warn!(
        "FLATTEN_ALL RC IDENTITY STATS [{}]: hits={} misses={} total={} hit_rate={:.2}%",
        label,
        hits,
        misses,
        total,
        100.0 * hits as f64 / total.max(1) as f64
    );
}

pub fn constraints_size(cs: &Constraints) -> usize {
    CONSTRAINTS_SIZE_CACHE
        .with(|cache| memoize_by_rc(cache, cs, (), || cs.inner.iter().map(constraint_size).sum()))
}

fn constraint_size(c: &Constraint) -> usize {
    1 + c.cfc.as_ref().map(rc_size).unwrap_or(0)
        + c.toc.as_ref().map(|(_, tc)| toc_size(tc)).unwrap_or(0)
}

fn rc_size(rc: &RunningConstraint) -> usize {
    match rc {
        RunningConstraint::Adt(_, _, _, fields) | RunningConstraint::Closure(_, _, fields) => {
            fields.values().map(constraints_size).sum()
        }
        RunningConstraint::Ptr(inner) => constraint_size(inner),
        RunningConstraint::List(inner) => constraint_size(inner),
        RunningConstraint::Tuple(elems) => elems.iter().map(constraints_size).sum(),
        RunningConstraint::Idk(inner) => constraints_size(inner),
        _ => 0,
    }
}

fn toc_size(tc: &TraitObjConstraint) -> usize {
    match tc {
        TraitObjConstraint::Adt(_, _, _, fields) | TraitObjConstraint::Closure(_, _, fields) => {
            fields.values().map(constraints_size).sum()
        }
    }
}

pub fn widen_constraints(cs: &Constraints) -> Constraints {
    WIDEN_CONSTRAINTS_CACHE.with(|cache| {
        memoize_by_rc(cache, cs, (), || {
            let mut out = Constraints::new();
            for c in cs.inner.iter() {
                out.push(widen_constraint(c));
            }
            out
        })
    })
}

/// Forgets the scalars in `cs` (and enum variants, with their fields) but
/// keeps its structure: struct fields, the objects pointers lead to (a
/// pointer's value is its pointee's), trait objects' concrete types. For
/// values a skipped call could have written through a `&mut` whose type's
/// pointer fields aren't public (InterpPass::havoc_mut_args, `fmt::Formatter`):
/// the callee can change data, not which objects those fields point to -
/// unlike widen_constraints, which forgets the pointers too, so the writer
/// behind a Formatter became every `fmt::Write` impl.
pub fn widen_scalars(cs: &Constraints) -> Constraints {
    WIDEN_SCALARS_CACHE.with(|cache| {
        memoize_by_rc(cache, cs, (), || {
            let mut out = Constraints::new();
            for c in cs.inner.iter() {
                out.push(widen_scalars_one(c));
            }
            out
        })
    })
}

/// Diagnosis only (VERIFOPT_CUT_KEEP_ENUMS=1): widen_scalars keeps enum
/// variants and their fields, like a struct's.
fn keep_enums() -> bool {
    static KEEP: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *KEEP.get_or_init(|| std::env::var_os("VERIFOPT_CUT_KEEP_ENUMS").is_some())
}

fn widen_scalars_fields(fields: &ADTFields) -> ADTFields {
    fields.iter().map(|(k, v)| (*k, widen_scalars(v))).collect()
}

fn widen_scalars_one(c: &Constraint) -> Constraint {
    let toc = c.toc.as_ref().map(|(ty, tc)| {
        let tc = match tc {
            TraitObjConstraint::Adt(def, genargs, variant, fields) => {
                if def.kind().is_enum() && !keep_enums() {
                    TraitObjConstraint::Adt(def.clone(), genargs.clone(), None, ADTFields::new())
                } else {
                    TraitObjConstraint::Adt(def.clone(), genargs.clone(), *variant, widen_scalars_fields(fields))
                }
            }
            TraitObjConstraint::Closure(cdef, genargs, captures) => {
                TraitObjConstraint::Closure(*cdef, genargs.clone(), widen_scalars_fields(captures))
            }
        };
        (ty.clone(), tc)
    });
    let cfc = c.cfc.as_ref().map(|rc| match rc {
        RunningConstraint::Scalar(_) => RunningConstraint::Scalar(None),
        RunningConstraint::Adt(def, genargs, _, _) if def.kind().is_enum() && !keep_enums() => {
            RunningConstraint::Adt(def.clone(), genargs.clone(), None, ADTFields::new())
        }
        RunningConstraint::Adt(def, genargs, variant, fields) => {
            RunningConstraint::Adt(def.clone(), genargs.clone(), *variant, widen_scalars_fields(fields))
        }
        RunningConstraint::Ptr(inner) => RunningConstraint::Ptr(Box::new(widen_scalars_one(inner))),
        RunningConstraint::Closure(cdef, genargs, captures) => {
            RunningConstraint::Closure(*cdef, genargs.clone(), widen_scalars_fields(captures))
        }
        RunningConstraint::List(inner) => RunningConstraint::List(Box::new(widen_scalars_one(inner))),
        RunningConstraint::Tuple(elems) => RunningConstraint::Tuple(elems.iter().map(widen_scalars).collect()),
        RunningConstraint::Idk(inner) => RunningConstraint::Idk(Box::new(widen_scalars(inner))),
        other => other.clone(),
    });
    Constraint::new(toc, cfc)
}

fn widen_constraint(c: &Constraint) -> Constraint {
    let toc = c.toc.as_ref().map(|(ty, tc)| (ty.clone(), widen_toc(tc)));
    let cfc = c.cfc.as_ref().map(widen_rc);
    Constraint::new(toc, cfc)
}

fn widen_toc(tc: &TraitObjConstraint) -> TraitObjConstraint {
    match tc {
        TraitObjConstraint::Adt(def, genargs, _, fields) => {
            if !fields.is_empty() {
                crate::assumptions::note_dropped_fields(&def.0.name());
            }
            TraitObjConstraint::Adt(def.clone(), genargs.clone(), None, ADTFields::new())
        }
        TraitObjConstraint::Closure(cdef, genargs, captures) => {
            if !captures.is_empty() {
                crate::assumptions::note_dropped_fields(&cdef.0.name());
            }
            TraitObjConstraint::Closure(*cdef, genargs.clone(), ADTFields::new())
        }
    }
}

fn widen_rc(rc: &RunningConstraint) -> RunningConstraint {
    match rc {
        RunningConstraint::Scalar(_) => RunningConstraint::Scalar(None),
        RunningConstraint::Float => RunningConstraint::Float,
        RunningConstraint::Adt(def, genargs, _, fields) => {
            if !fields.is_empty() {
                crate::assumptions::note_dropped_fields(&def.0.name());
            }
            RunningConstraint::Adt(def.clone(), genargs.clone(), None, ADTFields::new())
        }
        RunningConstraint::Ptr(inner) => RunningConstraint::Ptr(Box::new(widen_constraint(inner))),
        RunningConstraint::Closure(cdef, genargs, captures) => {
            if !captures.is_empty() {
                crate::assumptions::note_dropped_fields(&cdef.0.name());
            }
            RunningConstraint::Closure(*cdef, genargs.clone(), ADTFields::new())
        }
        RunningConstraint::FnDef(fndef, genargs) => {
            RunningConstraint::FnDef(*fndef, genargs.clone())
        }
        RunningConstraint::FnPtr(sig) => RunningConstraint::FnPtr(sig.clone()),
        RunningConstraint::Dynamic(tys) => RunningConstraint::Dynamic(tys.clone()),
        RunningConstraint::List(inner) => {
            RunningConstraint::List(Box::new(widen_constraint(inner)))
        }
        RunningConstraint::Tuple(elems) => {
            RunningConstraint::Tuple(elems.iter().map(widen_constraints).collect())
        }
        RunningConstraint::Idk(inner) => RunningConstraint::Idk(Box::new(widen_constraints(inner))),
        // Drop the projection path entirely, not just its shape
        RunningConstraint::Param(i, _) => RunningConstraint::Param(*i, vec![]),
    }
}

pub fn contains_param(cs: &Constraints) -> bool {
    CONTAINS_PARAM_CACHE.with(|cache| {
        memoize_by_rc(cache, cs, (), || {
            cs.inner.iter().any(constraint_contains_param)
        })
    })
}

fn constraint_contains_param(c: &Constraint) -> bool {
    match &c.cfc {
        Some(RunningConstraint::Param(..)) => true,
        Some(RunningConstraint::Adt(_, _, _, fields))
        | Some(RunningConstraint::Closure(_, _, fields)) => fields.values().any(contains_param),
        Some(RunningConstraint::Tuple(inner)) => inner.iter().any(contains_param),
        Some(RunningConstraint::Ptr(box inner)) => constraint_contains_param(inner),
        Some(RunningConstraint::List(box inner)) => constraint_contains_param(inner),
        Some(RunningConstraint::Idk(box inner_cs)) => contains_param(inner_cs),
        _ => false,
    }
}

pub fn substitute_params(summary: &Constraints, actual_args: &[Constraints]) -> Constraints {
    let mut out = Constraints::new();
    for c in summary.inner.iter() {
        out.append(substitute_params_constraint(c, actual_args));
    }
    out
}

fn substitute_params_constraint(c: &Constraint, actual_args: &[Constraints]) -> Constraints {
    let toc = substitute_toc(&c.toc, actual_args);

    match &c.cfc {
        Some(RunningConstraint::Param(i, path)) => {
            // A bare Param can carry its own toc from resolve_arg (e.g. a
            // trait-object argument passed straight through), which the
            // caller-substituted toc above would double up on - the actual
            // argument's own toc (if any) is exactly what belongs here
            // instead, since this whole disjunct *is* that argument.
            let base = actual_args
                .get(*i)
                .cloned()
                .unwrap_or_else(Constraints::new);
            apply_proj_path(&base, path)
        }
        Some(RunningConstraint::Adt(def, genargs, variant, fields)) => {
            let mut new_fields = ADTFields::new();
            for (k, v) in fields {
                new_fields.insert(*k, substitute_params(v, actual_args));
            }
            Constraints::from(
                Constraint::new(
                    toc,
                    Some(RunningConstraint::Adt(
                        def.clone(),
                        genargs.clone(),
                        *variant,
                        new_fields,
                    )),
                )
                .with_prov(c.prov.clone()),
            )
        }
        Some(RunningConstraint::Tuple(inner)) => {
            let new_inner = inner
                .iter()
                .map(|cs| substitute_params(cs, actual_args))
                .collect();
            Constraints::from(
                Constraint::new(toc, Some(RunningConstraint::Tuple(new_inner)))
                    .with_prov(c.prov.clone()),
            )
        }
        Some(RunningConstraint::Ptr(inner)) => {
            let substituted = substitute_params(&Constraints::from((**inner).clone()), actual_args);
            let mut out = Constraints::new();
            for sc in substituted.inner.iter() {
                out.push(
                    Constraint::new(
                        toc.clone(),
                        Some(RunningConstraint::Ptr(Box::new(sc.clone()))),
                    )
                    .with_prov(c.prov.clone()),
                );
            }
            out
        }
        Some(RunningConstraint::List(inner)) => {
            let substituted = substitute_params(&Constraints::from((**inner).clone()), actual_args);
            let mut out = Constraints::new();
            for sc in substituted.inner.iter() {
                out.push(
                    Constraint::new(
                        toc.clone(),
                        Some(RunningConstraint::List(Box::new(sc.clone()))),
                    )
                    .with_prov(c.prov.clone()),
                );
            }
            out
        }
        Some(RunningConstraint::Idk(inner_cs)) => {
            let substituted = substitute_params(inner_cs, actual_args);
            Constraints::from(
                Constraint::new(toc, Some(RunningConstraint::Idk(Box::new(substituted))))
                    .with_prov(c.prov.clone()),
            )
        }
        Some(RunningConstraint::Closure(cdef, genargs, captures)) => {
            let mut new_captures = ADTFields::new();
            for (k, v) in captures {
                new_captures.insert(*k, substitute_params(v, actual_args));
            }
            Constraints::from(
                Constraint::new(
                    toc,
                    Some(RunningConstraint::Closure(*cdef, genargs.clone(), new_captures)),
                )
                .with_prov(c.prov.clone()),
            )
        }
        // Scalar/Float/Dynamic/FnDef/FnPtr/None: nothing nested that
        // summary-building could have planted a Param inside.
        _ => Constraints::from(Constraint::new(toc, c.cfc.clone()).with_prov(c.prov.clone())),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitObjTy {
    //pub bound_tys: Vec<(DefId, String)>,
    //pub bound_regions: Vec<(DefId, String)>,
    pub def: TraitDef,
    pub genargs: GenericArgs,
}

impl TraitObjTy {
    pub fn new_from_bound_existential(binder: &Binder<ExistentialPredicate>) -> Option<TraitObjTy> {
        //let mut bound_tys = Vec::new();
        //let mut bound_regions = Vec::new();
        if !binder.bound_vars.is_empty() {
            //debug!("handle bound vars");
            //    for bound_var in &binder.bound_vars {
            //        match bound_var {
            //            BoundVariableKind::Ty(ty) => match ty {
            //                BoundTyKind::Param(def, s) => bound_tys.push((def.0, s.clone())),
            //                _ => {}
            //            },
            //            BoundVariableKind::Region(region) => match region {
            //                BoundRegionKind::BrNamed(def, s) => bound_regions.push((def.0, s.clone())),
            //                _ => {}
            //            },
            //            _ => {}
            //        }
            //    }
        }

        match binder.clone().skip_binder() {
            ExistentialPredicate::Trait(trait_ref) => {
                return Some(Self {
                    def: trait_ref.def_id,
                    genargs: trait_ref.generic_args,
                });
            }
            ExistentialPredicate::Projection(proj) => {
                return Some(Self {
                    def: proj.def_id,
                    genargs: proj.generic_args,
                });
            }
            ExistentialPredicate::AutoTrait(_trait_def) => None,
        }
    }

    pub fn is_fn_trait(&self) -> bool {
        matches!(
            self.def.name().as_str(),
            "std::ops::Fn"
                | "std::ops::FnMut"
                | "std::ops::FnOnce"
                | "std::ops::Fn::Output"
                | "std::ops::FnMut::Output"
                | "std::ops::FnOnce::Output"
        )
    }

    pub fn is_universal_trait(&self) -> bool {
        matches!(self.def.name().as_str(), "core::error::Erased")
    }
}

/// Whether `trait_path` names one of the traits a closure (or fn item) can
/// be dispatched through: Fn/FnMut/FnOnce and their async counterparts.
/// Unlike TraitObjTy::is_fn_trait this takes a plain path, and accepts both
/// `std::` and `core::` spellings (no_std crates print the latter).
pub fn is_fn_family_trait_path(trait_path: &str) -> bool {
    let Some(name) = trait_path
        .strip_prefix("std::ops::")
        .or_else(|| trait_path.strip_prefix("core::ops::"))
    else {
        return false;
    };
    matches!(name, "Fn" | "FnMut" | "FnOnce" | "AsyncFn" | "AsyncFnMut" | "AsyncFnOnce")
}

/// Whether a closure is a valid dispatch target for a call through
/// `trait_defid`. A closure only implements the Fn-family traits (plus
/// auto/marker traits, which have no methods to dispatch), so it must
/// never be offered as a candidate for anything else - e.g. a closure
/// stored in `Map<I, F>`'s `f` field is not an implementation of
/// `Iterator`, even though it's reachable from a `dyn Iterator` value.
pub fn closure_can_implement(trait_defid: &DefId) -> bool {
    is_fn_family_trait_path(&trait_defid.name())
}

/// Same check as closure_can_implement, but from a trait *method*'s DefId
/// (e.g. `std::ops::FnMut::call_mut`), by stripping the method segment.
pub fn is_fn_family_method(method_defid: &DefId) -> bool {
    let name = method_defid.name();
    match name.rsplit_once("::") {
        Some((trait_path, _method)) => is_fn_family_trait_path(trait_path),
        None => false,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArgSet {
    pub args: Vec<HashSet<Constraint>>,
}

impl ArgSet {
    pub fn new(constraints: &[Constraints]) -> Self {
        let args = constraints
            .iter()
            .map(|cs| cs.inner.iter().cloned().collect::<HashSet<Constraint>>())
            .collect();

        ArgSet { args }
    }
}

impl Hash for ArgSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for arg in &self.args {
            let mut acc: u64 = 0;

            for c in arg {
                let mut h = DefaultHasher::new();
                c.hash(&mut h);
                acc = acc.wrapping_add(h.finish());
            }

            acc.hash(state);
        }
    }
}

pub type SummaryKey = (VOID, ArgSet);

pub fn summary_key(
    ipass: &InterpPass,
    scope: VOID,
    ctxt: &Context,
    term_span: &Span,
    caller_scope: &VOID,
    body: &Body,
    local_decls: &[LocalDecl],
    args: &Vec<Operand>,
    is_closure: bool,
) -> SummaryKey {
    let cs: Vec<Constraints> = ipass.collect_resolved_args(
        ctxt,
        term_span,
        caller_scope,
        &body,
        local_decls,
        args,
        is_closure,
    );

    (scope, ArgSet::new(&cs))
}

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub cstore: ConstraintStore,
    pub wtos: ImHashMap<VOID, BBDeps>,
    pub bb_written_places: HashMap<VOID, HashSet<Place>>,
}

impl Context {
    pub fn new(cstore: ConstraintStore, wtos: ImHashMap<VOID, BBDeps>) -> Context {
        Self {
            cstore,
            wtos,
            bb_written_places: HashMap::new(),
        }
    }

    pub fn empty() -> Context {
        Self {
            cstore: ConstraintStore::new(),
            wtos: ImHashMap::default(),
            bb_written_places: HashMap::new(),
        }
    }

    pub fn get_wto(&self, scope: &VOID) -> Option<&BBDeps> {
        self.wtos.get(scope)
    }

    pub fn set_wto(&mut self, scope: &VOID, bbdeps: &BBDeps) {
        self.wtos.insert(scope.clone(), bbdeps.clone());
    }

    pub fn set_scoped_constraints(
        &mut self,
        scope: &VOID,
        place: &Place,
        constraints: Constraints,
        timing: Option<&InterpPass>,
    ) {
        self.set_constraints(scope, place, constraints, timing, true);
    }

    /// The write of an assignment to `written` (a statement's or a call's
    /// destination), stored at `target` (`written` itself, or its base local
    /// for a field write): through alias edges only if `written` goes
    /// through a Deref - otherwise it's an assignment to the place itself
    /// (see set_local_constraints).
    pub fn assign_constraints(
        &mut self,
        scope: &VOID,
        written: &Place,
        target: &Place,
        constraints: Constraints,
        timing: Option<&InterpPass>,
    ) {
        let through_pointer = written.projection.contains(&ProjectionElem::Deref);
        self.set_constraints(scope, target, constraints, timing, through_pointer);
    }

    /// set_scoped_constraints for an assignment *to* `place` rather than
    /// through it: `place` has no Deref (e.g. `_2 = copy _5`, a call's
    /// destination), so it's written itself, without following its alias
    /// edges - for a pointer-typed local that's a rebinding, not a write
    /// into its old pointee.
    pub fn set_local_constraints(
        &mut self,
        scope: &VOID,
        place: &Place,
        constraints: Constraints,
        timing: Option<&InterpPass>,
    ) {
        self.set_constraints(scope, place, constraints, timing, false);
    }

    fn set_constraints(
        &mut self,
        scope: &VOID,
        place: &Place,
        constraints: Constraints,
        timing: Option<&InterpPass>,
        follow_refs: bool,
    ) {
        let _g = timing.map(|p| p.timing_span(TimingCat::SetScopedConstraints, scope));
        let already_written = self
            .bb_written_places
            .get(scope)
            .map(|s| s.contains(place))
            .unwrap_or(false);
        if already_written {
            debug!("REPLACE value (already written)");
            let _g = timing.map(|p| p.timing_span(TimingCat::SetScopedConstraintsReplace, scope));
            let (key, value) = (MapKey::Var(place.clone()), Box::new(MapValue::Constraints(constraints)));
            if follow_refs {
                self.cstore.scoped_replace(scope, key, value);
            } else {
                self.cstore.scoped_replace_local(scope, key, value);
            }
        } else {
            debug!("UPDATE value");
            self.bb_written_places
                .entry(scope.clone())
                .or_default()
                .insert(place.clone());
            let _g = timing.map(|p| p.timing_span(TimingCat::SetScopedConstraintsUpdate, scope));
            let (key, value) = (MapKey::Var(place.clone()), Box::new(MapValue::Constraints(constraints)));
            if follow_refs {
                self.cstore.scoped_update(scope, key, value, timing);
            } else {
                self.cstore.scoped_update_local(scope, key, value, timing);
            }
        }
    }

    pub fn step_field(
        &self,
        scope: &VOID,
        constraints: &Constraints,
        elem: &ProjectionElem,
        timing: Option<&InterpPass>,
    ) -> Constraints {
        let mut out = Constraints::new();
        for constraint in constraints.inner.iter() {
            let res = self.step_field_one(scope, constraint, elem, timing);
            let _g = timing.map(|p| p.timing_span(TimingCat::StepFieldAppend, scope));
            out.append(res);
        }
        out
    }

    /// What a read through a Box/Arc/Rc's own internals yields: the pointer
    /// to its contents - which, since references and raw pointers are
    /// represented by their pointee's constraint (see convert's Rvalue::Ref
    /// / AddressOf, and Deref being a no-op in get_constraints), is just the
    /// contents themselves.
    ///
    /// Previously every such read went through flatten_all, which returns
    /// the wrapper itself *and* everything nested anywhere inside it. For a
    /// `Box<dyn Iterator>` dispatch - whose receiver is read through
    /// `((*self).0: Unique).0: NonNull` after Box-deref lowering - that made
    /// the receiver a mix of the Box (with and without its trait-object
    /// constraint), the iterator, and the iterator's own scalar fields, so
    /// the Box's forwarding `<Box<I, A> as Iterator>::next` became a
    /// spurious dispatch target (ripgrep's search loop, `box_dyn_iter`).
    ///
    /// Only wrappers in the shape stub_wrapper_new builds - a single field 0
    /// holding the contents - are unwrapped; anything else (Idk, a wrapper
    /// converted from its type, ...) keeps the imprecise-but-sound flatten.
    /// Nested wrappers are unwrapped one level only: for `Box<Box<T>>` the
    /// result is the inner Box, the real object (see `box_box_dyn_iter`).
    fn wrapper_contents_or_flatten(
        &self,
        constraints: &Constraints,
        scope: &VOID,
        timing: Option<&InterpPass>,
    ) -> Constraints {
        let mut out = Constraints::new();
        for constraint in constraints.inner.iter() {
            match &constraint.cfc {
                Some(RunningConstraint::Adt(adtdef, _, _, fields))
                    if crate::convert::is_pointer_wrapper_defid(adtdef)
                        && fields.len() == 1
                        && fields.get(&0).is_some_and(|c| !c.inner.is_empty()) =>
                {
                    out.append(fields[&0].clone());
                }
                _ => out.append(self.flatten_one(constraint, scope, timing)),
            }
        }
        out
    }

    /// Recursively unions every constraint reachable inside `constraints`,
    /// discarding all field/variant/tuple structure. Used in place of
    /// `step_field`/`filter_variant` for reads through types we've decided
    /// not to model with precise field indices (see `is_opaque_internal`)
    /// - safe even though it's imprecise, since it can only ever surface
    /// *more* candidates than a precise lookup would, never fewer
    pub fn flatten_all(
        &self,
        constraints: &Constraints,
        scope: &VOID,
        timing: Option<&InterpPass>,
    ) -> Constraints {
        debug!("flatten_all: constraints_size={}", constraints.inner.len());
        record_flatten_all_rc_identity(constraints);
        let _g = timing.map(|p| p.timing_span(TimingCat::FlattenAll, scope));
        FLATTEN_ALL_CACHE.with(|cache| {
            memoize_by_rc(cache, constraints, (), || {
                let mut out = Constraints::new();
                for constraint in constraints.inner.iter() {
                    let _g1 = timing.map(|p| p.timing_span(TimingCat::FlattenOne, scope));
                    let flat = self.flatten_one(constraint, scope, timing);
                    drop(_g1);

                    let _g2 = timing.map(|p| p.timing_span(TimingCat::FlattenAllAppend, scope));
                    out.append(flat);
                    drop(_g2);
                }
                out
            })
        })
    }

    fn flatten_one(
        &self,
        constraint: &Constraint,
        scope: &VOID,
        timing: Option<&InterpPass>,
    ) -> Constraints {
        let mut out = Constraints::new();

        // The constraint itself is always part of the union
        out.push(constraint.clone());

        match &constraint.cfc {
            Some(RunningConstraint::Adt(_, _, _, fields))
            | Some(RunningConstraint::Closure(_, _, fields)) => {
                for (_key, field_constraints) in fields {
                    let res = self.flatten_all(field_constraints, scope, timing);
                    let _g = timing.map(|p| p.timing_span(TimingCat::FlattenOneAppend, scope));
                    out.append(res);
                }
            }
            Some(RunningConstraint::Tuple(inner)) => {
                for elem_constraints in inner {
                    let res = self.flatten_all(elem_constraints, scope, timing);
                    let _g = timing.map(|p| p.timing_span(TimingCat::FlattenOneAppend, scope));
                    out.append(res);
                }
            }
            Some(RunningConstraint::Ptr(box inner)) => {
                let res = self.flatten_one(inner, scope, timing);
                let _g = timing.map(|p| p.timing_span(TimingCat::FlattenOneAppend, scope));
                out.append(res);
            }
            Some(RunningConstraint::List(box inner)) => {
                let res = self.flatten_one(inner, scope, timing);
                let _g = timing.map(|p| p.timing_span(TimingCat::FlattenOneAppend, scope));
                out.append(res);
            }
            Some(RunningConstraint::Idk(box inner_cs)) => {
                let res = self.flatten_all(inner_cs, scope, timing);
                let _g = timing.map(|p| p.timing_span(TimingCat::FlattenOneAppend, scope));
                out.append(res);
            }
            // Scalar/Float/Dynamic/FnDef/FnPtr: no further
            // structure to descend into
            _ => {}
        }

        out
    }

    fn step_field_one(
        &self,
        scope: &VOID,
        constraint: &Constraint,
        elem: &ProjectionElem,
        timing: Option<&InterpPass>,
    ) -> Constraints {
        match &constraint.cfc {
            // A field the value doesn't have - widened away, or of an "any
            // value of the type" value - is any value of the field's type,
            // not nothing: reading it as nothing lost trait objects and made
            // branches on it unreachable (plan A, merge_widened_field).
            Some(RunningConstraint::Adt(_, _, _, fields)) => {
                let _g = timing.map(|p| p.timing_span(TimingCat::StepFieldOneAdt, scope));
                fields
                    .get(&adt_field_idx(elem))
                    .cloned()
                    .unwrap_or_else(|| unknown_field_value(elem, scope, timing))
            }
            // An upvar read in a closure body: `(*_1).i` / `_1.i`.
            Some(RunningConstraint::Closure(_, _, captures)) => {
                let _g = timing.map(|p| p.timing_span(TimingCat::StepFieldOneAdt, scope));
                captures
                    .get(&adt_field_idx(elem))
                    .cloned()
                    .unwrap_or_else(|| unknown_field_value(elem, scope, timing))
            }
            Some(RunningConstraint::Tuple(inner)) => {
                let _g = timing.map(|p| p.timing_span(TimingCat::StepFieldOneTuple, scope));
                match elem {
                    ProjectionElem::Field(idx, _) => inner
                        .get(*idx)
                        .cloned()
                        .unwrap_or_else(|| unknown_field_value(elem, scope, timing)),
                    _ => Constraints::new(),
                }
            }
            Some(RunningConstraint::Ptr(box inner)) => {
                let _g = timing.map(|p| p.timing_span(TimingCat::StepFieldOnePtr, scope));
                self.step_field_one(scope, inner, elem, timing)
            }
            // A sequence's elements: like Idk below, a field of "an element"
            // (a pointer to a List is also a pointer to its elements).
            Some(RunningConstraint::List(box inner)) => {
                let _g = timing.map(|p| p.timing_span(TimingCat::StepFieldOneIdk, scope));
                self.step_field_one(scope, inner, elem, timing)
            }
            Some(RunningConstraint::Idk(box inner_cs)) => {
                let _g = timing.map(|p| p.timing_span(TimingCat::StepFieldOneIdk, scope));
                let mut out = Constraints::new();
                for ic in inner_cs.inner.iter() {
                    out.append(self.step_field_one(scope, ic, elem, timing));
                }
                out
            }
            // Unknown-yet placeholder from a parametric summary: don't
            // collapse to "no info" the moment code reads a field off a
            // parameter - extend the path instead, so substitute_params can
            // replay this exact field access against the real argument.
            Some(RunningConstraint::Param(i, path)) => {
                let _g = timing.map(|p| p.timing_span(TimingCat::StepFieldOneParam, scope));
                let mut new_path = path.clone();
                new_path.push(ProjStep::Field(adt_field_idx(elem)));
                Constraints::from(Constraint::new(
                    None,
                    Some(RunningConstraint::Param(*i, new_path)),
                ))
            }
            // Scalar/Float/Dynamic/Closure/etc: this disjunct has no field structure at all,
            // so it contributes no information to the projection - not an error.
            _ => Constraints::new(),
        }
    }

    pub fn get_constraints(
        &self,
        scope: &VOID,
        local_decls: &[LocalDecl],
        place: &Place,
        is_closure: bool,
        timing: Option<&InterpPass>,
    ) -> Option<Constraints> {
        if place.projection.is_empty() {
            let _g = timing.map(|p| p.timing_span(TimingCat::GetConstraintsIfBlock, scope));
            match self
                .cstore
                .scoped_get(scope, &MapKey::Var(place.clone()), is_closure)
            {
                Some(MapValue::Constraints(constraints)) => Some(constraints),
                None => None,
                _ => panic!("got store instead of constraints"),
            }
        } else {
            let _g = timing.map(|p| p.timing_span(TimingCat::GetConstraintsElseBlock, scope));
            let base = Place {
                local: place.local,
                projection: vec![],
            };
            match self.get_constraints(scope, local_decls, &base, is_closure, timing) {
                Some(base_constraints) => {
                    // Collect every matching projection in the set of constraints
                    let mut cur = base_constraints;
                    // Tracks everything accumulated *before* the current
                    // projection element
                    let mut prefix = base.clone();
                    // Once any step along this projection chain turns out
                    // to be opaque, every remaining step is skipped
                    let mut opaque_from_here = false;

                    let _g_loop =
                        timing.map(|p| p.timing_span(TimingCat::GetConstraintsProjLoop, scope));
                    for elem in &place.projection {
                        if !opaque_from_here {
                            // (is opaque, is a pointer wrapper)
                            let (is_opaque, is_wrapper) = {
                                let _g = timing.map(|p| {
                                    p.timing_span(TimingCat::GetConstraintsIsOpaqueInternal, scope)
                                });
                                if let Ok(prefix_ty) = prefix.ty(local_decls) {
                                    (
                                        crate::convert::is_opaque_internal(&prefix_ty),
                                        crate::convert::is_pointer_wrapper(&prefix_ty),
                                    )
                                } else {
                                    (false, false)
                                }
                            };
                            if is_opaque {
                                opaque_from_here = true;
                                let _g = timing.map(|p| {
                                    p.timing_span(TimingCat::GetConstraintsFlattenAll, scope)
                                });
                                cur = if is_wrapper {
                                    self.wrapper_contents_or_flatten(&cur, scope, timing)
                                } else {
                                    self.flatten_all(&cur, scope, timing)
                                };
                            }
                        }

                        if !opaque_from_here {
                            match elem {
                                ProjectionElem::Downcast(vidx) => {
                                    let _g = timing.map(|p| {
                                        p.timing_span(TimingCat::GetConstraintsFilterVariant, scope)
                                    });
                                    cur = cur.filter_variant(*vidx, Some(scope), timing);
                                }
                                ProjectionElem::Field(..) => {
                                    let _g = timing.map(|p| {
                                        p.timing_span(TimingCat::GetConstraintsStepField, scope)
                                    });
                                    cur = self.step_field(scope, &cur, elem, timing);
                                }
                                _ => {}
                            }
                        }

                        prefix.projection.push(elem.clone());
                    }
                    drop(_g_loop);

                    Some(cur)
                }
                None => None,
            }
        }
    }

    pub fn set_cstore_scope(
        &mut self,
        scope: &VOID,
        store: ConstraintStore,
        enclosing_scope: EnclosingScopes,
    ) {
        self.cstore.cmap.insert(
            MapKey::ScopeId(base_scope_id(scope)),
            Box::new(MapValue::Store(store, enclosing_scope)),
        );
    }

    pub fn get_cstore_scope(&self, scope: &VOID) -> Option<&Box<MapValue>> {
        self.cstore.cmap.get(&MapKey::ScopeId(base_scope_id(scope)))
    }

    pub fn get_static(&self, defid: DefId) -> Option<Constraints> {
        match self.cstore.cmap.get(&MapKey::Static(defid)) {
            Some(box MapValue::Constraints(cs)) => Some(cs.clone()),
            _ => None,
        }
    }

    pub fn set_static(&mut self, defid: DefId, constraints: Constraints) {
        self.cstore.cmap.insert(
            MapKey::Static(defid),
            Box::new(MapValue::Constraints(constraints)),
        );
    }
}

/// A store's map, recording which keys it mutated (inserted, borrowed
/// mutably, removed) while tracking is on (track_writes) - so merging a
/// candidate's context back can visit just the scopes the candidates wrote
/// rather than every scope (InterpPass::merge_cstores_timed). The same
/// methods as the ImHashMap it wraps; equality and Debug ignore the record.
#[derive(Clone, Default)]
pub struct TrackedMap {
    map: ImHashMap<MapKey, Box<MapValue>>,
    dirty: Option<ImHashSet<MapKey>>,
}

impl std::fmt::Debug for TrackedMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.map.fmt(f)
    }
}

impl PartialEq for TrackedMap {
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

impl TrackedMap {
    fn mark(&mut self, key: &MapKey) {
        if let Some(dirty) = &mut self.dirty {
            dirty.insert(key.clone());
        }
    }

    pub fn insert(&mut self, key: MapKey, value: Box<MapValue>) -> Option<Box<MapValue>> {
        self.mark(&key);
        self.map.insert(key, value)
    }

    pub fn get_mut(&mut self, key: &MapKey) -> Option<&mut Box<MapValue>> {
        self.mark(key);
        self.map.get_mut(key)
    }

    pub fn remove(&mut self, key: &MapKey) -> Option<Box<MapValue>> {
        self.mark(key);
        self.map.remove(key)
    }

    pub fn get(&self, key: &MapKey) -> Option<&Box<MapValue>> {
        self.map.get(key)
    }

    pub fn iter(&self) -> im::hashmap::Iter<'_, MapKey, Box<MapValue>> {
        self.map.iter()
    }

    pub fn values(&self) -> im::hashmap::Values<'_, MapKey, Box<MapValue>> {
        self.map.values()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn ptr_eq(&self, other: &Self) -> bool {
        self.map.ptr_eq(&other.map)
    }

    /// Starts recording mutated keys - keeping any keys already recorded, so
    /// a context cloned inside an outer simulation stays a superset of what
    /// changed since the outermost one began.
    pub fn track_writes(&mut self) {
        if self.dirty.is_none() {
            self.dirty = Some(ImHashSet::new());
        }
    }

    pub fn stop_tracking(&mut self) {
        self.dirty = None;
    }

    /// The keys mutated since tracking began; None when not tracking.
    pub fn dirty(&self) -> Option<&ImHashSet<MapKey>> {
        self.dirty.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintStore {
    pub cmap: TrackedMap,
    /// Each place can alias *multiple* targets - e.g. the same shared,
    /// type-erased stdlib call chain (like RawVecInner's allocation
    /// machinery, generic over element size/align rather than the
    /// concrete element type) can be reached from genuinely unrelated
    /// parts of a program, each recording a different alias target for
    /// the same key. A single-valued map forces choosing one target and
    /// silently discarding the other on merge - a real soundness gap,
    /// not just an imprecision, since the discarded fact could still
    /// matter. Retaining the full set is the sound alternative: nothing
    /// gets discarded, readers consider every possible target, at the
    /// cost of resolving to potentially more than one place.
    ///
    /// Keyed by scope first (see Refs), so a finished scope's aliases can be
    /// dropped in one step (drop_scope_refs).
    pub refs: Refs,
}

/// Alias edges by the scope of the aliasing place, then the place itself:
/// (place, scope) -> its possible targets. Each scope - with its call
/// context - is stored once rather than in every entry's key.
pub type Refs = ImHashMap<VOID, ImHashMap<Place, ImHashSet<((Place, VOID), Mutability)>>>;

/// The union of two alias maps (every target either has).
pub fn union_refs(a: Refs, b: Refs) -> Refs {
    a.union_with(b, |m1, m2| m1.union_with(m2, |s1, s2| s1.union(s2)))
}

impl ConstraintStore {
    pub fn new() -> ConstraintStore {
        Self {
            cmap: TrackedMap::default(),
            refs: ImHashMap::default(),
        }
    }

    pub fn add_ref(&mut self, from: (Place, VOID), to: (Place, VOID), bk: Mutability) {
        let (place, scope) = from;
        self.refs
            .entry(scope)
            .or_default()
            .entry(place)
            .or_insert_with(ImHashSet::new)
            .insert((to, bk));
    }

    /// Forgets the alias edges out of `scope`'s places - for a scope that has
    /// finished (see InterpPass::finish_frame); a later call of it registers
    /// its own again (resolve_args, and its `&place` statements).
    pub fn drop_scope_refs(&mut self, scope: &VOID) {
        self.refs.remove(scope);
    }

    fn ref_targets(&self, cur: &(Place, VOID)) -> Option<&ImHashSet<((Place, VOID), Mutability)>> {
        self.refs.get(&cur.1).and_then(|m| m.get(&cur.0))
    }

    /// The places `place` in `scope` denotes once its alias edges are
    /// followed - including through a Deref *inside* an alias target: a
    /// target `(*_1).0.0` (e.g. `&raw const (*self).value`, or `&mut
    /// (*x).f`) is resolved further through `_1`'s edges, with `.0.0`
    /// appended to whatever `_1` reaches. Terminals can then carry a field
    /// path (`x.0.0`); scoped_get / the writes handle those as field reads /
    /// field writes of their base. A Deref whose base has no edge (a pointer
    /// read out of a value) stays as it is. Collects every visited place
    /// (for write_log's ports).
    fn canonical_places(
        &self,
        place: Place,
        scope: VOID,
        visited: &mut ImHashSet<(Place, VOID)>,
        out: &mut ImHashSet<(Place, VOID)>,
        depth: usize,
    ) {
        const MAX_DEPTH: usize = 16;
        if place.projection.first() == Some(&ProjectionElem::Deref) {
            let base = Place { local: place.local, projection: vec![] };
            let rest = place.projection[1..].to_vec();
            let (terms, vis) = self.resolve_mut_ref_visiting(base.clone(), scope.clone());
            visited.extend(vis);
            for (tplace, tscope) in terms {
                if tplace == base && tscope == scope {
                    out.insert((place.clone(), scope.clone()));
                } else {
                    let mut projection = tplace.projection.clone();
                    projection.extend(rest.iter().cloned());
                    let next = Place { local: tplace.local, projection };
                    if depth < MAX_DEPTH {
                        self.canonical_places(next, tscope, visited, out, depth + 1);
                    } else {
                        out.insert((next, tscope));
                    }
                }
            }
        } else {
            let (terms, vis) = self.resolve_mut_ref_visiting(place.clone(), scope.clone());
            visited.extend(vis);
            for t in terms {
                if t.0.projection.first() == Some(&ProjectionElem::Deref) && t != (place.clone(), scope.clone()) && depth < MAX_DEPTH {
                    self.canonical_places(t.0, t.1, visited, out, depth + 1);
                } else {
                    out.insert(t);
                }
            }
        }
    }

    /// Every place the alias walk from `place` in `scope` visits (itself
    /// included), and the places it denotes (canonical_places) - see
    /// InterpPass::interp_return.
    pub fn alias_walk(&self, place: Place, scope: VOID) -> (ImHashSet<(Place, VOID)>, ImHashSet<(Place, VOID)>) {
        let mut visited = ImHashSet::new();
        let mut targets = ImHashSet::new();
        self.canonical_places(place, scope, &mut visited, &mut targets, 0);
        (visited, targets)
    }

    /// The terminal places of `place`'s alias edges (reads and writes alike),
    /// and every place the walk visited (for write_log: which calls' ports a
    /// write passes through). It follows mutable *and* shared borrows: a
    /// write through a shared reference is interior mutability (Cell,
    /// RefCell, OptionalCell, Mutex, atomics - anything else is undefined
    /// behavior), and it reaches the borrowed place just the same; stopping
    /// at shared borrows, as the write walk used to, kept those writes in
    /// the callee's own local (`write_interior_mut`, `cell_client_wiring`).
    fn resolve_mut_ref_visiting(
        &self,
        place: Place,
        scope: VOID,
    ) -> (ImHashSet<(Place, VOID)>, ImHashSet<(Place, VOID)>) {
        let mut results = ImHashSet::new();
        let mut visited: ImHashSet<(Place, VOID)> = ImHashSet::new();
        let mut worklist = vec![(place, scope)];
        while let Some(cur) = worklist.pop() {
            if visited.contains(&cur) {
                continue;
            }
            visited.insert(cur.clone());
            match self.ref_targets(&cur) {
                Some(targets) if !targets.is_empty() => {
                    for (to, _bk) in targets.iter() {
                        worklist.push(to.clone());
                    }
                }
                _ => {
                    results.insert(cur);
                }
            }
        }
        (results, visited)
    }

    /// The places a write to `key` in `scope` goes to (resolve's mutable
    /// alias walk), after logging the write for write_log: a Var write by
    /// the places its walk visited, a Static write by its static.
    fn write_targets(&self, scope: &VOID, key: &MapKey, value: &MapValue) -> Vec<(VOID, MapKey)> {
        match key {
            MapKey::Var(place) => {
                let mut visited = ImHashSet::new();
                let mut targets = ImHashSet::new();
                self.canonical_places(place.clone(), scope.clone(), &mut visited, &mut targets, 0);
                crate::write_log::record_write(visited.iter(), value);
                targets.into_iter().map(|(p, s)| (s, MapKey::Var(p))).collect()
            }
            MapKey::Static(def) => {
                crate::write_log::record_static(*def, value);
                vec![(scope.clone(), key.clone())]
            }
            MapKey::ScopeId(_) => vec![(scope.clone(), key.clone())],
        }
    }

    pub fn scoped_get(&self, scope: &VOID, key: &MapKey, is_closure: bool) -> Option<MapValue> {
        let targets: Vec<(VOID, MapKey)> = match key {
            MapKey::Var(place) => {
                let mut visited = ImHashSet::new();
                let mut out = ImHashSet::new();
                self.canonical_places(place.clone(), scope.clone(), &mut visited, &mut out, 0);
                out.into_iter().map(|(p, s)| (s, MapKey::Var(p))).collect()
            }
            MapKey::ScopeId(_) => vec![(scope.clone(), key.clone())],
            MapKey::Static(_) => panic!("use get_static/set_static"),
        };

        // A Var lookup can now resolve to multiple possible places (see
        // `refs`'s own field doc) - merge every target's own result
        // rather than assuming there's exactly one. Constraints merge
        // by appending (disjuncts accumulate, nothing gets discarded);
        // Store/Static results shouldn't ever arise from resolving
        // multiple *Var* targets, so keeping the first rather than
        // trying to merge them is fine - that combination isn't
        // reachable by construction, not something being silently
        // papered over.
        let mut merged: Option<MapValue> = None;
        for (target_scope, target_key) in targets {
            let single = self.scoped_get_single(&target_scope, &target_key, is_closure);
            merged = match (merged, single) {
                (None, other) => other,
                (Some(existing), None) => Some(existing),
                (Some(MapValue::Constraints(mut cs)), Some(MapValue::Constraints(other_cs))) => {
                    cs.append(other_cs);
                    Some(MapValue::Constraints(cs))
                }
                (Some(existing), Some(_)) => Some(existing),
            };
        }
        merged
    }

    fn scoped_get_single(&self, scope: &VOID, key: &MapKey, is_closure: bool) -> Option<MapValue> {
        // A field path (an alias target like `x.0.0`, see canonical_places):
        // the base's value, projected.
        if let MapKey::Var(place) = key
            && !place.projection.is_empty()
            && !self.has_own_entry(scope, key)
        {
            let base = MapKey::Var(Place { local: place.local, projection: vec![] });
            return match self.scoped_get_single(scope, &base, is_closure)? {
                MapValue::Constraints(cs) => project_value(&cs, &place.projection).map(MapValue::Constraints),
                other => Some(other),
            };
        }
        note_static_read(scope);
        match self.cmap.get(&MapKey::ScopeId(base_scope_id(scope))) {
            Some(vartype) => match *vartype.clone() {
                MapValue::Store(store, enclosing_scopes) => {
                    // Is key in inner_cmap? if not:
                    // - Is nested func: return None
                    // - Is closure: follow backptr to enclosing scope
                    match store.cmap.get(key) {
                        Some(boxed) => Some(*boxed.clone()),
                        None => {
                            if is_closure && enclosing_scopes.is_some() {
                                // Check enclosing scopes for missing key(s)
                                let constraints =
                                    self.get_from_enclosing_scopes(&enclosing_scopes, key);
                                Some(MapValue::Constraints(constraints))
                            } else {
                                None
                            }
                        }
                    }
                }
                _ => panic!("not a scope: {:?}", scope),
            },
            None => None,
        }
    }

    fn get_from_enclosing_scopes(
        &self,
        enclosing_scopes: &EnclosingScopes,
        key: &MapKey,
    ) -> Constraints {
        let mut all_constraints = Constraints::new();
        for enclosing_scope in enclosing_scopes.as_ref().unwrap().iter() {
            match self.scoped_get(&enclosing_scope, key, false) {
                Some(val) => match val {
                    MapValue::Constraints(constraints) => {
                        all_constraints.append(constraints);
                    }
                    _ => panic!("got scope"),
                },
                None => {}
            }
        }
        all_constraints
    }

    pub fn scoped_update(
        &mut self,
        scope: &VOID,
        key: MapKey,
        value: Box<MapValue>,
        timing: Option<&InterpPass>,
    ) {
        let _resolve_guard = timing.map(|p| p.timing_span(TimingCat::ScopedUpdateResolve, scope));
        let targets = self.write_targets(scope, &key, &value);
        drop(_resolve_guard);

        // A write that can only reach one static's whole value overwrites it
        // (a strong update): a static is one location, and on this path it
        // definitely holds the new value now - which a field write also
        // computes from its current value. Without it a `static mut` that
        // starts as any value of its type (static_place) would stay "any"
        // after its first write - every static_init! buffer.
        if let [(target_scope, MapKey::Var(p))] = targets.as_slice()
            && p.projection.is_empty()
            && static_of_scope(target_scope).is_some()
        {
            self.replace_targets(targets, value);
            return;
        }

        // A Var write can now resolve to multiple possible places (see
        // `refs`'s own field doc) - apply the same write to every one
        // of them, since which one is "real" isn't known at analysis
        // time. Each target's own write still merges with whatever's
        // already there (via scoped_update_single), it just happens
        // once per target instead of once total.
        for (target_scope, target_key) in targets {
            self.scoped_update_single(&target_scope, target_key, value.clone(), timing);
        }
    }

    /// Whether `scope`'s store has an entry under exactly `key`.
    pub fn has_own_entry(&self, scope: &VOID, key: &MapKey) -> bool {
        match self.cmap.get(&MapKey::ScopeId(base_scope_id(scope))) {
            Some(b) => matches!(&**b, MapValue::Store(store, _) if store.cmap.get(key).is_some()),
            None => false,
        }
    }

    /// A write of `value` to the field path of `key` (an alias target like
    /// `x.0.0`, see canonical_places): the base's value with that field
    /// written - the base value to store instead, if `key` has a path and
    /// the base has a value.
    fn field_write(&self, scope: &VOID, key: &MapKey, value: &MapValue) -> Option<(MapKey, Box<MapValue>)> {
        let MapKey::Var(place) = key else { return None };
        if place.projection.is_empty() {
            return None;
        }
        let MapValue::Constraints(new) = value else { return None };
        let base = MapKey::Var(Place { local: place.local, projection: vec![] });
        let Some(MapValue::Constraints(mut cs)) = self.scoped_get_single(scope, &base, false) else {
            return None;
        };
        cs.write_field(place.projection.clone(), new.clone());
        Some((base, Box::new(MapValue::Constraints(cs))))
    }

    fn scoped_update_single(
        &mut self,
        scope: &VOID,
        key: MapKey,
        value: Box<MapValue>,
        timing: Option<&InterpPass>,
    ) {
        let (key, value) = self.field_write(scope, &key, &value).unwrap_or((key, value));
        let _get_guard = timing.map(|p| p.timing_span(TimingCat::ScopedUpdateGetScope, scope));
        let mapres = self.cmap.get(&MapKey::ScopeId(base_scope_id(scope)));
        drop(_get_guard);

        match mapres {
            Some(vartype) => match *vartype.clone() {
                MapValue::Store(mut store, enclosing_scope) => {
                    let _g0 =
                        timing.map(|p| p.timing_span(TimingCat::ScopedUpdateStorePre, scope));
                    let mut new_val = value.clone();
                    let old_val = store.cmap.get(&key);
                    drop(_g0);

                    match old_val {
                        Some(old_val_) => {
                            let _g1 = timing
                                .map(|p| p.timing_span(TimingCat::ScopedUpdateStoreMerge, scope));
                            let merged =
                                merge_mapvals(old_val_, &value, timing.map(|p| (p, scope)));
                            drop(_g1);

                            match &merged {
                                MapValue::Constraints(constraints) => {
                                    debug!(
                                        "scoped_update: MERGE scope={:?} disjuncts={}",
                                        scope.0.name(),
                                        constraints_size(constraints),
                                    );
                                }
                                _ => {}
                            }
                            new_val = Box::new(merged);
                        }
                        None => {}
                    }

                    // modify scope w new key/val
                    let _g2 =
                        timing.map(|p| p.timing_span(TimingCat::ScopedUpdateStorePost, scope));
                    note_static_write(scope, store.cmap.get(&key), &new_val);
                    store.cmap.insert(key, new_val);
                    self.cmap.insert(
                        MapKey::ScopeId(base_scope_id(scope)),
                        Box::new(MapValue::Store(store, enclosing_scope)),
                    );
                }
                MapValue::Constraints(..) => {
                    panic!("defid is not a scope: {:?}", scope);
                }
            },
            None => {
                // initialize new scope w key/val
                let _g = timing.map(|p| p.timing_span(TimingCat::ScopedUpdateInitScope, scope));
                note_static_write(scope, None, &value);
                let mut new_store = ConstraintStore::new();
                new_store.cmap.insert(key, value);
                self.cmap.insert(
                    MapKey::ScopeId(base_scope_id(scope)),
                    Box::new(MapValue::Store(new_store, Some(Rc::new(vec![scope.clone()])))),
                );
            }
        }
    }

    // Same as scoped_update, but never merges with whatever's already
    // there - always overwrites outright. Correct specifically for a
    // second (or later) write to a place already written during the
    // current basic-block visit: within one basic block there's no
    // branching by MIR's own definition, so there's no alternative
    // incoming path the prior value could represent.
    pub fn scoped_replace(&mut self, scope: &VOID, key: MapKey, value: Box<MapValue>) {
        let targets = self.write_targets(scope, &key, &value);
        self.replace_targets(targets, value);
    }

    /// A write to `key` itself, not through it: a plain assignment to a
    /// pointer-typed local (`_2 = copy _5`) rebinds the local, it doesn't
    /// write into what the local pointed at before - so no alias edges are
    /// followed (see Context::set_scoped_constraints).
    pub fn scoped_replace_local(&mut self, scope: &VOID, key: MapKey, value: Box<MapValue>) {
        self.replace_targets(vec![(scope.clone(), key)], value);
    }

    /// scoped_update's merge into `key` itself (see scoped_replace_local).
    pub fn scoped_update_local(
        &mut self,
        scope: &VOID,
        key: MapKey,
        value: Box<MapValue>,
        timing: Option<&InterpPass>,
    ) {
        self.scoped_update_single(scope, key, value, timing);
    }

    fn replace_targets(&mut self, targets: Vec<(VOID, MapKey)>, value: Box<MapValue>) {
        // A field-path target is a field write into its base: a merge with
        // the base's other fields' values, so through scoped_update_single.
        if targets.len() == 1
            && let MapKey::Var(p) = &targets[0].1
            && !p.projection.is_empty()
        {
            let (scope, key) = targets.into_iter().next().unwrap();
            self.scoped_update_single(&scope, key, value, None);
            return;
        }
        // The "always overwrites outright" contract only holds when
        // there's exactly one possible target - the original
        // rationale (no branching within a BB, so no alternative
        // incoming path the prior value could represent) doesn't
        // extend to "and we're certain which of several alias targets
        // is the real one." With more than one target, resolving to
        // multiple possible places itself means we're not certain,
        // so fall back to the same merge-based write scoped_update
        // uses - overwriting every possible target unconditionally
        // would risk discarding legitimate, unrelated information for
        // whichever targets weren't actually the one written to here.
        if targets.len() == 1 {
            let (scope, key) = targets.into_iter().next().unwrap();
            match self.cmap.get(&MapKey::ScopeId(base_scope_id(&scope))) {
                Some(vartype) => match *vartype.clone() {
                    MapValue::Store(mut store, enclosing_scope) => {
                        note_static_write(&scope, store.cmap.get(&key), &value);
                        store.cmap.insert(key, value);
                        self.cmap.insert(
                            MapKey::ScopeId(base_scope_id(&scope)),
                            Box::new(MapValue::Store(store, enclosing_scope)),
                        );
                    }
                    MapValue::Constraints(..) => {
                        panic!("defid is not a scope: {:?}", scope);
                    }
                },
                None => {
                    // initialize new scope w key/val
                    let mut new_store = ConstraintStore::new();
                    new_store.cmap.insert(key, value);
                    self.cmap.insert(
                        MapKey::ScopeId(base_scope_id(&scope)),
                        Box::new(MapValue::Store(new_store, Some(Rc::new(vec![scope.clone()])))),
                    );
                }
            }
        } else {
            for (target_scope, target_key) in targets {
                self.scoped_update_single(&target_scope, target_key, value.clone(), None);
            }
        }
    }
}

/// Diagnostics: the `top` node kinds making up `cs` by (tree) node count,
/// e.g. "Adt(std::vec::Vec)=812345 Closure(..)=...", plus the maximum depth.
pub fn size_histogram(cs: &Constraints, top: usize) -> String {
    fn walk_cs(cs: &Constraints, depth: usize, counts: &mut HashMap<String, usize>, max_depth: &mut usize) {
        for c in cs.inner.iter() {
            walk_c(c, depth, counts, max_depth);
        }
    }
    fn walk_c(c: &Constraint, depth: usize, counts: &mut HashMap<String, usize>, max_depth: &mut usize) {
        *max_depth = (*max_depth).max(depth);
        if let Some((_, tc)) = &c.toc {
            match tc {
                TraitObjConstraint::Adt(_, _, _, f) | TraitObjConstraint::Closure(_, _, f) => {
                    for v in f.values() {
                        walk_cs(v, depth + 1, counts, max_depth);
                    }
                }
            }
        }
        let label = match &c.cfc {
            Some(RunningConstraint::Adt(d, ..)) => format!("Adt({})", d.0.name()),
            Some(RunningConstraint::Closure(d, ..)) => format!("Closure({})", d.0.name()),
            Some(other) => format!("{:?}", other).split(['(', ' ', '{']).next().unwrap_or("?").to_string(),
            None => "None".to_string(),
        };
        *counts.entry(label).or_default() += 1;
        match &c.cfc {
            Some(RunningConstraint::Adt(_, _, _, f)) | Some(RunningConstraint::Closure(_, _, f)) => {
                for v in f.values() {
                    walk_cs(v, depth + 1, counts, max_depth);
                }
            }
            Some(RunningConstraint::Ptr(inner)) | Some(RunningConstraint::List(inner)) => {
                walk_c(inner, depth + 1, counts, max_depth)
            }
            Some(RunningConstraint::Tuple(elems)) => {
                for e in elems {
                    walk_cs(e, depth + 1, counts, max_depth);
                }
            }
            Some(RunningConstraint::Idk(inner)) => walk_cs(inner, depth + 1, counts, max_depth),
            _ => {}
        }
    }
    let mut counts = HashMap::new();
    let mut max_depth = 0;
    walk_cs(cs, 0, &mut counts, &mut max_depth);
    let mut v: Vec<(String, usize)> = counts.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    let parts: Vec<String> = v.iter().take(top).map(|(k, n)| format!("{k}={n}")).collect();
    format!("max depth {max_depth}; {}", parts.join(" "))
}

/// Diagnostics: total (tree) node count of a memo key's argument values.
pub fn argset_size(a: &ArgSet) -> usize {
    a.args.iter().flatten().map(constraint_size).sum()
}

/// Diagnostics: (variables, total node count) over a store and the stores
/// nested in it.
pub fn store_size(store: &ConstraintStore) -> (usize, usize) {
    let (mut vars, mut nodes) = (0, 0);
    for v in store.cmap.values() {
        match &**v {
            MapValue::Constraints(cs) => {
                vars += 1;
                nodes += constraints_size(cs);
            }
            MapValue::Store(inner, _) => {
                let (v2, n2) = store_size(inner);
                vars += v2;
                nodes += n2;
            }
        }
    }
    (vars, nodes)
}

/// `cs` projected along `projection` (the field path of an alias target, see
/// ConstraintStore::canonical_places): fields of ADTs, closures and tuples,
/// through pointers and sequence elements, downcasts filtering variants;
/// Deref and indexing are no-ops on values (a pointer's value is its
/// pointee's, an array's the union of its elements). None if some step
/// can't be followed (a type with no field structure, a parametric
/// placeholder) - "unknown", like a missing entry.
pub fn project_value(cs: &Constraints, projection: &[ProjectionElem]) -> Option<Constraints> {
    fn field(c: &Constraint, idx: usize, out: &mut Constraints) -> bool {
        match &c.cfc {
            // A missing field is unknown (see Context::step_field_one), so
            // the whole projected read is - the caller falls back to any
            // value of the read place's type.
            Some(RunningConstraint::Adt(_, _, _, fields)) | Some(RunningConstraint::Closure(_, _, fields)) => {
                match fields.get(&idx) {
                    Some(v) => {
                        out.append(v.clone());
                        true
                    }
                    None => false,
                }
            }
            Some(RunningConstraint::Tuple(inner)) => match inner.get(idx) {
                Some(v) => {
                    out.append(v.clone());
                    true
                }
                None => false,
            },
            Some(RunningConstraint::Ptr(box inner)) | Some(RunningConstraint::List(box inner)) => {
                field(inner, idx, out)
            }
            Some(RunningConstraint::Idk(box inner)) => inner.inner.iter().all(|ic| field(ic, idx, out)),
            // No field structure, and nothing unknown about that: contributes
            // nothing (e.g. a MaybeUninit's uninitialized `()` field).
            None
            | Some(RunningConstraint::Scalar(_))
            | Some(RunningConstraint::Float)
            | Some(RunningConstraint::FnDef(..))
            | Some(RunningConstraint::FnPtr(_)) => true,
            // A parametric placeholder or an unknown object: unknown.
            _ => false,
        }
    }
    let mut cur = cs.clone();
    for elem in projection {
        cur = match elem {
            ProjectionElem::Field(idx, _) => {
                let mut out = Constraints::new();
                for c in cur.inner.iter() {
                    if !field(c, *idx, &mut out) {
                        return None;
                    }
                }
                out
            }
            ProjectionElem::Downcast(v) => cur.filter_variant(*v, None, None),
            _ => cur,
        };
    }
    Some(cur)
}

// Statics as places: each static's value lives at `_0` of a synthetic scope
// of its own (static_scope), and a pointer to a static is an alias edge to
// that place (InterpPass::static_place) - so writes through any pointer to it
// (`&mut STATIC`, `&raw mut`, a pointer passed down or stored) reach it, and
// every read sees its current value, through the same alias resolution as
// other places. The scope's store is part of the flow-sensitive context.
thread_local! {
    static STATIC_SCOPES: RefCell<HashMap<Instance, DefId>> = RefCell::new(HashMap::new());
    static STATIC_VERSIONS: RefCell<HashMap<DefId, u64>> = RefCell::new(HashMap::new());
}

/// The synthetic scope holding static `def`'s value (at `_0`).
pub fn static_scope(def: DefId) -> VOID {
    let instance = Instance::from(rustc_public::mir::mono::StaticDef(def));
    STATIC_SCOPES.with(|s| s.borrow_mut().insert(instance, def));
    (instance, GenericArgs(Vec::new()), Vec::new())
}

/// The static whose synthetic scope `scope` is, if it is one.
pub fn static_of_scope(scope: &VOID) -> Option<DefId> {
    if !scope.2.is_empty() {
        return None;
    }
    STATIC_SCOPES.with(|s| s.borrow().get(&scope.0).copied())
}

/// Static `def`'s version: bumped whenever its value changes. A memoized
/// result is reused only while the statics it read keep the versions it saw
/// (write_log::reads_fresh) - per static, so one written all the time (a
/// writer, a counter) doesn't invalidate every memo entry.
pub fn static_version(def: DefId) -> u64 {
    STATIC_VERSIONS.with(|v| v.borrow().get(&def).copied().unwrap_or(0))
}

/// A read of `scope`'s store: if it is a static's, noted in the write-log
/// frames (write_log::record_static_read).
fn note_static_read(scope: &VOID) {
    if let Some(def) = static_of_scope(scope) {
        crate::write_log::record_static_read(def, static_version(def));
    }
}

/// After a write into `scope`'s store: if `scope` is a static's, and its value
/// changed, a new version (the old one noted as read: a field write computes
/// from the old value), and the write logged for memo replay
/// (write_log::record_static - the static's whole new value).
fn note_static_write(scope: &VOID, old: Option<&Box<MapValue>>, new: &MapValue) {
    let Some(def) = static_of_scope(scope) else { return };
    if old.is_some_and(|o| **o == *new) {
        return;
    }
    let version = static_version(def);
    crate::write_log::record_static_read(def, version);
    STATIC_VERSIONS.with(|v| v.borrow_mut().insert(def, version + 1));
    crate::write_log::record_static(def, new);
}

/// A missing field's value as something a nested write can go into: "any
/// value" of the field's type with no fields known (an ADT or closure of
/// that type - its other fields then read as unknown, see step_field_one).
/// Writing a nested path into a missing field used to write into nothing, so
/// the write was lost (e.g. `set_client` on a `static mut` capsule that
/// starts as any value of its type).
fn field_skeleton(elem: &ProjectionElem) -> Constraints {
    use rustc_public::ty::{RigidTy, TyKind};
    let ProjectionElem::Field(_, ty) = elem else { return Constraints::new() };
    match ty.kind() {
        TyKind::RigidTy(RigidTy::Adt(def, args)) => {
            Constraints::from(Constraint::new(None, Some(RunningConstraint::Adt(def, args, None, ADTFields::new()))))
        }
        TyKind::RigidTy(RigidTy::Closure(def, args)) => {
            Constraints::from(Constraint::new(None, Some(RunningConstraint::Closure(def, args, ADTFields::new()))))
        }
        _ => Constraints::new(),
    }
}

/// Any value of the type of the field `elem` projects to (see
/// step_field_one).
fn unknown_field_value(elem: &ProjectionElem, scope: &VOID, timing: Option<&InterpPass>) -> Constraints {
    let ProjectionElem::Field(_, ty) = elem else { return Constraints::new() };
    match timing {
        Some(p) => Constraints::from(p.converter.convert_ty(&Location::unknown(), ty, Some(scope), timing).1),
        None => field_skeleton(elem),
    }
}
