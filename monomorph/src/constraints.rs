use crate::interp::{InterpPass, TimingCat};
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
pub type VOID = (Instance, GenericArgs);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapKey {
    Var(Place),
    ScopeId(VOID),
    Static(DefId),
}

pub type EnclosingScopes = Option<Vec<VOID>>;

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
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Constraints {
    pub inner: Rc<IndexSet<Constraint>>,
}

impl Hash for Constraints {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut combined: u64 = 0;
        for elem in self.inner.iter() {
            let mut h = DefaultHasher::new();
            elem.hash(&mut h);
            combined ^= h.finish();
        }
        combined.hash(state);
    }
}

impl Constraints {
    pub fn new() -> Constraints {
        Self {
            inner: Rc::new(IndexSet::new()),
        }
    }

    pub fn from(constraint: Constraint) -> Constraints {
        let mut set = IndexSet::with_capacity(1);
        set.insert(constraint);
        Self {
            inner: Rc::new(set),
        }
    }

    pub fn from_vec(inner: Vec<Constraint>) -> Constraints {
        Self {
            inner: Rc::new(inner.into_iter().collect()),
        }
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
        if let Some(existing) = self.inner.get(&new_constraint) {
            let mut merged = existing.clone();
            merged.prov.join(&new_constraint.prov);
            Rc::make_mut(&mut self.inner).replace(merged);
        } else {
            Rc::make_mut(&mut self.inner).insert(new_constraint);
        }
    }

    pub fn append(&mut self, new_constraints: Constraints) {
        for c in new_constraints.inner.iter() {
            self.push(c.clone());
        }
    }

    // Write: strong-update the field within EVERY disjunct currently in scope.
    // This is what makes {A, B}.f = C become {A{f:C}, B} instead of touching a global table.
    pub fn write_field(&mut self, projection: Vec<ProjectionElem>, new: Constraints) {
        // The base-size cap below only catches "many base disjuncts, each
        // getting a copy of new". It misses the other half: new itself
        // already being huge before it ever reaches here, in which case
        // even a single base disjunct ends up holding a multi-million-node
        // value. Recursive size (not just new.inner.len()) is what matters,
        // since new's own size could be hiding in nested fields rather
        // than at its own top level.
        let new = if constraints_size(&new) > NEW_VALUE_WIDEN_THRESHOLD {
            widen_constraints(&new)
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
                    *self = widen_constraints(self);
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
                        }
                        c
                    })
                    .collect();
                self.inner = Rc::new(new_set);
            }

            (_, _) => {
                let (first, rest) = field.split_first().expect("len >= 2 per match arm");
                let idx = adt_field_idx(first);
                let rest = rest.to_vec();

                if self.inner.len() > WRITE_FIELD_WIDEN_THRESHOLD {
                    *self = widen_constraints(self);
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
                                    fields.get(&idx).cloned().unwrap_or_else(Constraints::new);
                                nested.write_field(rest.clone(), new.clone());
                                fields.insert(idx, nested);
                            }
                        }
                        c
                    })
                    .collect();
                self.inner = Rc::new(new_set);
            }
        }
    }

    pub fn filter_variant(&self, vidx: VariantIdx) -> Constraints {
        let mut out = Constraints::new();
        for c in self.inner.iter() {
            match &c.cfc {
                Some(RunningConstraint::Adt(_, _, variant, _)) => {
                    if variant.is_none() || *variant == Some(vidx) {
                        out.push(c.clone());
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
                    out.push(Constraint::new(
                        None,
                        Some(RunningConstraint::Param(*i, new_path)),
                    ));
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
    Closure(ClosureDef, GenericArgs),
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
    Closure(ClosureDef, GenericArgs),
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
            Some(RunningConstraint::Adt(_, _, _, fields)) => {
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
            ProjStep::Downcast(vidx) => cur.filter_variant(*vidx),
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
            TraitObjConstraint::Closure(cdef, genargs) => {
                TraitObjConstraint::Closure(*cdef, genargs.clone())
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
    cache
        .borrow_mut()
        .insert(full_key, (Rc::downgrade(&cs.inner), result.clone()));
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
        RunningConstraint::Adt(_, _, _, fields) => fields.values().map(constraints_size).sum(),
        RunningConstraint::Ptr(inner) => constraint_size(inner),
        RunningConstraint::List(inner) => constraint_size(inner),
        RunningConstraint::Tuple(elems) => elems.iter().map(constraints_size).sum(),
        RunningConstraint::Idk(inner) => constraints_size(inner),
        _ => 0,
    }
}

fn toc_size(tc: &TraitObjConstraint) -> usize {
    match tc {
        TraitObjConstraint::Adt(_, _, _, fields) => fields.values().map(constraints_size).sum(),
        _ => 0,
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

fn widen_constraint(c: &Constraint) -> Constraint {
    let toc = c.toc.as_ref().map(|(ty, tc)| (ty.clone(), widen_toc(tc)));
    let cfc = c.cfc.as_ref().map(widen_rc);
    Constraint::new(toc, cfc)
}

fn widen_toc(tc: &TraitObjConstraint) -> TraitObjConstraint {
    match tc {
        TraitObjConstraint::Adt(def, genargs, _, _) => {
            TraitObjConstraint::Adt(def.clone(), genargs.clone(), None, ADTFields::new())
        }
        TraitObjConstraint::Closure(cdef, genargs) => {
            TraitObjConstraint::Closure(*cdef, genargs.clone())
        }
    }
}

fn widen_rc(rc: &RunningConstraint) -> RunningConstraint {
    match rc {
        RunningConstraint::Scalar(_) => RunningConstraint::Scalar(None),
        RunningConstraint::Float => RunningConstraint::Float,
        RunningConstraint::Adt(def, genargs, _, _) => {
            RunningConstraint::Adt(def.clone(), genargs.clone(), None, ADTFields::new())
        }
        RunningConstraint::Ptr(inner) => RunningConstraint::Ptr(Box::new(widen_constraint(inner))),
        RunningConstraint::Closure(cdef, genargs) => {
            RunningConstraint::Closure(*cdef, genargs.clone())
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
        Some(RunningConstraint::Adt(_, _, _, fields)) => fields.values().any(contains_param),
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
        // Scalar/Float/Dynamic/Closure/FnDef/FnPtr/None: nothing nested that
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
        let _g = timing.map(|p| p.timing_span(TimingCat::SetScopedConstraints, scope));
        let already_written = self
            .bb_written_places
            .get(scope)
            .map(|s| s.contains(place))
            .unwrap_or(false);
        if already_written {
            debug!("REPLACE value (already written)");
            let _g = timing.map(|p| p.timing_span(TimingCat::SetScopedConstraintsReplace, scope));
            self.cstore.scoped_replace(
                scope,
                MapKey::Var(place.clone()),
                Box::new(MapValue::Constraints(constraints)),
            );
        } else {
            debug!("UPDATE value");
            self.bb_written_places
                .entry(scope.clone())
                .or_default()
                .insert(place.clone());
            let _g = timing.map(|p| p.timing_span(TimingCat::SetScopedConstraintsUpdate, scope));
            self.cstore.scoped_update(
                scope,
                MapKey::Var(place.clone()),
                Box::new(MapValue::Constraints(constraints)),
                timing,
            );
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
            out.append(self.step_field_one(scope, constraint, elem, timing));
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
            Some(RunningConstraint::Adt(_, _, _, fields)) => {
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
            // Scalar/Float/Dynamic/Closure/FnDef/FnPtr: no further
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
            Some(RunningConstraint::Adt(_, _, _, fields)) => {
                let _g = timing.map(|p| p.timing_span(TimingCat::StepFieldOneAdt, scope));
                fields
                    .get(&adt_field_idx(elem))
                    .cloned()
                    .unwrap_or_else(Constraints::new) // unknown/never-written field -> fallback, see below
            }
            Some(RunningConstraint::Tuple(inner)) => {
                let _g = timing.map(|p| p.timing_span(TimingCat::StepFieldOneTuple, scope));
                match elem {
                    ProjectionElem::Field(idx, _) => {
                        inner.get(*idx).cloned().unwrap_or_else(Constraints::new)
                    }
                    _ => Constraints::new(),
                }
            }
            Some(RunningConstraint::Ptr(box inner)) => {
                let _g = timing.map(|p| p.timing_span(TimingCat::StepFieldOnePtr, scope));
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
                            let is_opaque = {
                                let _g = timing.map(|p| {
                                    p.timing_span(TimingCat::GetConstraintsIsOpaqueInternal, scope)
                                });
                                if let Ok(prefix_ty) = prefix.ty(local_decls) {
                                    crate::convert::is_opaque_internal(&prefix_ty)
                                } else {
                                    false
                                }
                            };
                            if is_opaque {
                                opaque_from_here = true;
                                let _g = timing.map(|p| {
                                    p.timing_span(TimingCat::GetConstraintsFlattenAll, scope)
                                });
                                cur = self.flatten_all(&cur, scope, timing);
                            }
                        }

                        if !opaque_from_here {
                            match elem {
                                ProjectionElem::Downcast(vidx) => {
                                    let _g = timing.map(|p| {
                                        p.timing_span(TimingCat::GetConstraintsFilterVariant, scope)
                                    });
                                    cur = cur.filter_variant(*vidx);
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
            MapKey::ScopeId(scope.clone()),
            Box::new(MapValue::Store(store, enclosing_scope)),
        );
    }

    pub fn get_cstore_scope(&self, scope: &VOID) -> Option<&Box<MapValue>> {
        self.cstore.cmap.get(&MapKey::ScopeId(scope.clone()))
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

#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintStore {
    pub cmap: ImHashMap<MapKey, Box<MapValue>>,
    pub refs: ImHashMap<(Place, VOID), ((Place, VOID), Mutability)>,
}

impl ConstraintStore {
    pub fn new() -> ConstraintStore {
        Self {
            cmap: ImHashMap::default(),
            refs: ImHashMap::default(),
        }
    }

    fn resolve(&self, place: Place, scope: VOID, for_mut: bool) -> (Place, VOID) {
        if place.projection.first() == Some(&ProjectionElem::Deref) {
            let base = Place {
                local: place.local,
                projection: vec![],
            };
            let rest = place.projection[1..].to_vec();

            let (tplace, tscope) = if for_mut {
                self.resolve_mut_ref(base.clone(), scope.clone())
            } else {
                self.resolve_ref(base.clone(), scope.clone())
            };

            if tplace == base && tscope == scope {
                return (place, scope);
            }

            let mut projection = tplace.projection.clone();
            projection.extend(rest);

            (
                Place {
                    local: tplace.local,
                    projection,
                },
                tscope,
            )
        } else if for_mut {
            self.resolve_mut_ref(place, scope)
        } else {
            self.resolve_ref(place, scope)
        }
    }

    pub fn add_ref(&mut self, from: (Place, VOID), to: (Place, VOID), bk: Mutability) {
        self.refs.insert(from, (to, bk));
    }

    fn resolve_ref(&self, place: Place, scope: VOID) -> (Place, VOID) {
        let mut cur = (place, scope);
        while let Some(((p, s), _)) = self.refs.get(&cur) {
            cur = (p.clone(), s.clone());
        }
        cur
    }

    fn resolve_mut_ref(&self, place: Place, scope: VOID) -> (Place, VOID) {
        let mut cur = (place, scope);
        while let Some(((p, s), bk)) = self.refs.get(&cur) {
            if matches!(bk, Mutability::Mut) {
                cur = (p.clone(), s.clone());
            } else {
                return cur;
            }
        }
        cur
    }

    pub fn scoped_get(&self, scope: &VOID, key: &MapKey, is_closure: bool) -> Option<MapValue> {
        let (scope, key) = match key {
            MapKey::Var(place) => {
                let (place, scope) = self.resolve(place.clone(), scope.clone(), false);
                (scope, MapKey::Var(place))
            }
            MapKey::ScopeId(_) | MapKey::Static(_) => (scope.clone(), key.clone()),
        };

        match self.cmap.get(&MapKey::ScopeId(scope.clone())) {
            Some(vartype) => match *vartype.clone() {
                MapValue::Store(store, enclosing_scopes) => {
                    // Is key in inner_cmap? if not:
                    // - Is nested func: return None
                    // - Is closure: follow backptr to enclosing scope
                    match store.cmap.get(&key) {
                        Some(boxed) => Some(*boxed.clone()),
                        None => {
                            if is_closure && enclosing_scopes.is_some() {
                                // Check enclosing scopes for missing key(s)
                                let constraints =
                                    self.get_from_enclosing_scopes(&enclosing_scopes, &key);
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
        for enclosing_scope in enclosing_scopes.as_ref().unwrap() {
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
        let (scope, key) = match key {
            MapKey::Var(place) => {
                let (place, scope) = self.resolve(place.clone(), scope.clone(), true);
                debug!("scoped_update: local={}", place.local);
                (scope, MapKey::Var(place))
            }
            MapKey::ScopeId(_) | MapKey::Static(_) => (scope.clone(), key.clone()),
        };
        drop(_resolve_guard);

        let _get_guard = timing.map(|p| p.timing_span(TimingCat::ScopedUpdateGetScope, &scope));
        let mapres = self.cmap.get(&MapKey::ScopeId(scope.clone()));
        drop(_get_guard);

        match mapres {
            Some(vartype) => match *vartype.clone() {
                MapValue::Store(mut store, enclosing_scope) => {
                    let _g0 =
                        timing.map(|p| p.timing_span(TimingCat::ScopedUpdateStorePre, &scope));
                    let mut new_val = value.clone();
                    let old_val = store.cmap.get(&key);
                    drop(_g0);

                    match old_val {
                        Some(old_val_) => {
                            let _g1 = timing
                                .map(|p| p.timing_span(TimingCat::ScopedUpdateStoreMerge, &scope));
                            let merged =
                                merge_mapvals(old_val_, &value, timing.map(|p| (p, &scope)));
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
                        timing.map(|p| p.timing_span(TimingCat::ScopedUpdateStorePost, &scope));
                    store.cmap.insert(key, new_val);
                    self.cmap.insert(
                        MapKey::ScopeId(scope.clone()),
                        Box::new(MapValue::Store(store, enclosing_scope)),
                    );
                }
                MapValue::Constraints(..) => {
                    panic!("defid is not a scope: {:?}", scope);
                }
            },
            None => {
                // initialize new scope w key/val
                let _g = timing.map(|p| p.timing_span(TimingCat::ScopedUpdateInitScope, &scope));
                let mut new_store = ConstraintStore::new();
                new_store.cmap.insert(key, value);
                self.cmap.insert(
                    MapKey::ScopeId(scope.clone()),
                    Box::new(MapValue::Store(new_store, Some(vec![scope.clone()]))),
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
        let (scope, key) = match key {
            MapKey::Var(place) => {
                let (place, scope) = self.resolve(place.clone(), scope.clone(), true);
                (scope, MapKey::Var(place))
            }
            MapKey::ScopeId(_) | MapKey::Static(_) => (scope.clone(), key.clone()),
        };

        match self.cmap.get(&MapKey::ScopeId(scope.clone())) {
            Some(vartype) => match *vartype.clone() {
                MapValue::Store(mut store, enclosing_scope) => {
                    store.cmap.insert(key, value);
                    self.cmap.insert(
                        MapKey::ScopeId(scope.clone()),
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
                    MapKey::ScopeId(scope.clone()),
                    Box::new(MapValue::Store(new_store, Some(vec![scope.clone()]))),
                );
            }
        }
    }
}
