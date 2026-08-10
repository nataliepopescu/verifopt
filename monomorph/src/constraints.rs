use crate::interp::InterpPass;
use crate::rustc_public::CrateDef;
//use rustc_data_structures::fx::FxHashMap as HashMap;
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

//use log::debug;

use indexmap::IndexSet;
use std::collections::{BTreeMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};

// Persistent/structurally-shared map: cloning an im::HashMap is O(1) (just
// bumps a refcount on the shared tree) instead of walking and deep-cloning
// every entry, unlike std/Fx HashMap. cstore.cmap accumulates one entry per
// scope/variable visited across the *entire* program, and Context::clone()
// (derived Clone) clones it wholesale once per candidate at every dynamic-
// dispatch site - so with a plain HashMap that clone cost grows with how
// much of the program has been analyzed so far, compounding badly at
// dispatch-heavy points (e.g. a trait with 100+ impls). Aliased distinctly
// from `HashMap` above (FxHashMap) since this crate's API mirrors std's
// closely enough that call sites (.get/.get_mut/.insert/.remove/.iter) need
// no changes beyond the field's declared type.
use im::HashMap as ImHashMap;

//pub fn unique_update(ret: ConstraintsAndFields, new: ConstraintsAndFields) -> ConstraintsAndFields {
//    let (mut old_constraints, mut old_fields) = ret;
//    let (new_constraints, new_fields) = new;
//    unique_append(&mut old_constraints, new_constraints.to_vec());
//    unique_append(&mut old_fields, new_fields.to_vec());
//    (old_constraints, old_fields)
//}

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

/// Extracts the field index from a `ProjectionElem::Field`. `ADTFields` is
/// keyed on this `usize` alone - not the whole `ProjectionElem` - since a
/// field's type is fully determined by its index for a given ADT/variant,
/// and the embedded `Ty` carries an interned id that isn't guaranteed
/// identical across every derivation of "the same type" (see the sip::State
/// oscillation bug this replaced). Public so callers building/reading
/// `ADTFields` outside this module (convert.rs, interp.rs, stdlib_stubs.rs)
/// can key it the same way.
pub fn adt_field_idx(elem: &ProjectionElem) -> usize {
    match elem {
        ProjectionElem::Field(idx, _) => *idx,
        _ => panic!("expected Field projection: {:?}", elem),
    }
}

// Set of positive constraints; negative constraints are resolved immediately by removing them from the set
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Constraints {
    pub inner: IndexSet<Constraint>,
}

// IndexSet<T> intentionally doesn't implement Hash even when T: Hash - same
// reason std::collections::HashSet doesn't: its PartialEq/Eq (which it does
// provide) compares as an unordered set, so a consistent Hash needs a
// commutative combining function, which the std/indexmap authors leave to
// the caller rather than choosing one for you. Needed here because
// RunningConstraint::Idk(Box<Constraints>) requires Constraints: Hash for
// its own #[derive(Hash)] to resolve. XOR-folding each element's individual
// hash keeps this order-independent, matching the Eq above (a == b must
// imply hash(a) == hash(b), and a/b can differ in insertion order here).
impl Hash for Constraints {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut combined: u64 = 0;
        for elem in &self.inner {
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
            inner: IndexSet::new(),
        }
    }

    pub fn from(constraint: Constraint) -> Constraints {
        let mut inner = IndexSet::with_capacity(1);
        inner.insert(constraint);
        Self { inner }
    }

    pub fn from_vec(inner: Vec<Constraint>) -> Constraints {
        Self {
            inner: inner.into_iter().collect(),
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
            self.inner.replace(merged);
        } else {
            self.inner.insert(new_constraint);
        }
    }

    pub fn append(&mut self, new_constraints: Constraints) {
        for c in new_constraints.inner {
            self.push(c);
        }
    }

    // Write: strong-update the field within EVERY disjunct currently in scope.
    // This is what makes {A, B}.f = C become {A{f:C}, B} instead of touching a global table.
    pub fn write_field(&mut self, projection: Vec<ProjectionElem>, new: Constraints) {
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
                // Can't mutate elements of an IndexSet in place via iter_mut()
                // (it doesn't exist - an element IS its own hash key, so an
                // in-place edit that changes the hash would silently corrupt
                // the set's bucket layout). Take ownership of every disjunct,
                // transform it, and reinsert - insert() recomputes the hash
                // correctly for the new content.
                self.inner = std::mem::take(&mut self.inner)
                    .into_iter()
                    .map(|mut c| {
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
            }

            (_, _) => {
                let (first, rest) = field.split_first().expect("len >= 2 per match arm");
                let idx = adt_field_idx(first);
                let rest = rest.to_vec();

                self.inner = std::mem::take(&mut self.inner)
                    .into_iter()
                    .map(|mut c| {
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
            }
        }
    }

    pub fn filter_variant(&self, vidx: VariantIdx) -> Constraints {
        let mut out = Constraints::new();
        for c in &self.inner {
            match &c.cfc {
                Some(RunningConstraint::Adt(_, _, variant, _)) => {
                    if variant.is_none() || *variant == Some(vidx) {
                        out.push(c.clone());
                    }
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
    //.into_iter()
    //.map(|(cs, _)| cs)
    //.collect();

    (scope, ArgSet::new(&cs))
}

// These should only be Field ProjectionElems. The convention is that any time one of these
// field projections is used, it will be prepended by a Deref ProjectionElem
//pub type FieldProjections = Vec<ProjectionElem>;
//pub type FieldPlace = Place;

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub cstore: ConstraintStore,
    pub wtos: ImHashMap<VOID, BBDeps>,
}

impl Context {
    pub fn new(cstore: ConstraintStore, wtos: ImHashMap<VOID, BBDeps>) -> Context {
        Self { cstore, wtos }
    }

    pub fn empty() -> Context {
        Self {
            cstore: ConstraintStore::new(),
            wtos: ImHashMap::default(),
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
    ) {
        self.cstore.scoped_update(
            scope,
            MapKey::Var(place.clone()),
            Box::new(MapValue::Constraints(constraints)),
        );
    }

    pub fn step_field(
        &self,
        scope: &VOID,
        constraints: &Constraints,
        elem: &ProjectionElem,
    ) -> Constraints {
        let mut out = Constraints::new();
        for constraint in &constraints.inner {
            out.append(self.step_field_one(scope, constraint, elem));
        }
        out
    }

    /// Recursively unions every constraint reachable inside `constraints`,
    /// discarding all field/variant/tuple structure. Used in place of
    /// `step_field`/`filter_variant` for reads through types we've decided
    /// not to model with precise field indices (see `is_opaque_internal`)
    /// - safe even though it's imprecise, since it can only ever surface
    /// *more* candidates than a precise lookup would, never fewer: nothing
    /// nested inside gets lost just because we don't trust the requested
    /// index/variant.
    pub fn flatten_all(&self, constraints: &Constraints) -> Constraints {
        let mut out = Constraints::new();
        for constraint in &constraints.inner {
            out.append(self.flatten_one(constraint));
        }
        out
    }

    fn flatten_one(&self, constraint: &Constraint) -> Constraints {
        let mut out = Constraints::new();

        // The constraint itself is always part of the union
        out.push(constraint.clone());

        match &constraint.cfc {
            Some(RunningConstraint::Adt(_, _, _, fields)) => {
                for (_key, field_constraints) in fields {
                    out.append(self.flatten_all(field_constraints));
                }
            }
            Some(RunningConstraint::Tuple(inner)) => {
                for elem_constraints in inner {
                    out.append(self.flatten_all(elem_constraints));
                }
            }
            Some(RunningConstraint::Ptr(box inner)) => {
                out.append(self.flatten_one(inner));
            }
            Some(RunningConstraint::List(box inner)) => {
                out.append(self.flatten_one(inner));
            }
            Some(RunningConstraint::Idk(box inner_cs)) => {
                out.append(self.flatten_all(inner_cs));
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
    ) -> Constraints {
        match &constraint.cfc {
            Some(RunningConstraint::Adt(_, _, _, fields)) => fields
                .get(&adt_field_idx(elem))
                .cloned()
                .unwrap_or_else(Constraints::new), // unknown/never-written field -> fallback, see below
            Some(RunningConstraint::Tuple(inner)) => match elem {
                ProjectionElem::Field(idx, _) => {
                    inner.get(*idx).cloned().unwrap_or_else(Constraints::new)
                }
                _ => Constraints::new(),
            },
            Some(RunningConstraint::Ptr(box inner)) => self.step_field_one(scope, inner, elem),
            Some(RunningConstraint::Idk(box inner_cs)) => {
                let mut out = Constraints::new();
                for ic in &inner_cs.inner {
                    out.append(self.step_field_one(scope, ic, elem));
                }
                out
            }
            // Scalar/Float/Dynamic/Closure/etc: this disjunct has no field structure at all,
            // so it contributes no information to the projection - not an error.
            _ => {
                //debug!(
                //    "unexpected running constraint type to have projections: {:?}",
                //    constraint.cfc
                //);
                Constraints::new()
            }
        }
    }

    pub fn get_constraints(
        &self,
        scope: &VOID,
        local_decls: &[LocalDecl],
        place: &Place,
        is_closure: bool,
    ) -> Option<Constraints> {
        if place.projection.is_empty() {
            match self
                .cstore
                .scoped_get(scope, &MapKey::Var(place.clone()), is_closure)
            {
                Some(MapValue::Constraints(constraints)) => Some(constraints),
                None => None,
                _ => panic!("got store instead of constraints"),
            }
        } else {
            let base = Place {
                local: place.local,
                projection: vec![],
            };
            match self.get_constraints(scope, local_decls, &base, is_closure) {
                Some(base_constraints) => {
                    // Collect every matching projection in the set of constraints
                    let mut cur = base_constraints;
                    // Tracks everything accumulated *before* the current
                    // projection element
                    let mut prefix = base.clone();
                    // Once any step along this projection chain turns out
                    // to be opaque, every remaining step is skipped
                    let mut opaque_from_here = false;

                    for elem in &place.projection {
                        if !opaque_from_here {
                            if let Ok(prefix_ty) = prefix.ty(local_decls) {
                                if crate::convert::is_opaque_internal(&prefix_ty) {
                                    opaque_from_here = true;
                                    cur = self.flatten_all(&cur);
                                }
                            }
                        }

                        if !opaque_from_here {
                            match elem {
                                ProjectionElem::Downcast(vidx) => {
                                    //debug!("\ndowncast projection: {:?}", elem);
                                    cur = cur.filter_variant(*vidx);
                                }
                                ProjectionElem::Field(..) => {
                                    //debug!("\nfield projection: {:?}", elem);
                                    cur = self.step_field(scope, &cur, elem);
                                }
                                _ => {}
                            }
                        }

                        prefix.projection.push(elem.clone());
                    }

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

    pub fn scoped_update(&mut self, scope: &VOID, key: MapKey, value: Box<MapValue>) {
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
                    let mut new_val = value.clone();
                    let old_val = store.cmap.get(&key);
                    match old_val {
                        Some(old_val_) => {
                            new_val = Box::new(merge_mapvals(old_val_, &value));
                        }
                        None => {}
                    }

                    // modify scope w new key/val
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
