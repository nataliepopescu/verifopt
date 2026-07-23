use crate::interp::InterpPass;
use crate::rustc_public_bridge::IndexedVal;
use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_public::mir::mono::Instance;

use rustc_public::mir::{Body, LocalDecl, Mutability, Operand, Place, ProjectionElem};
use rustc_public::ty::{
    AdtDef, Binder, ClosureDef, ExistentialPredicate, FnDef, GenericArgs, Span, TraitDef,
};

//use crate::common::log_scope;
use crate::merge::merge_mapvals;
use crate::sig_collect::SigVal;
use crate::wto::BBDeps;

use log::debug;

use std::collections::HashSet;
use std::hash::{DefaultHasher, Hash, Hasher};

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
}

pub type EnclosingScopes = Option<Vec<VOID>>;

#[derive(Debug, Clone, PartialEq)]
pub enum MapValue {
    Store(ConstraintStore, EnclosingScopes),
    Constraints(Constraints),
}

// Set of positive constraints; negative constraints are resolved immediately by removing them from the set
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Constraints {
    pub inner: Vec<Constraint>,
}

impl Constraints {
    pub fn new() -> Constraints {
        Self { inner: Vec::new() }
    }

    pub fn from(constraint: Constraint) -> Constraints {
        Self {
            inner: vec![constraint],
        }
    }

    pub fn from_vec(inner: Vec<Constraint>) -> Constraints {
        Self { inner }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0
    }

    pub fn at(&self, idx: usize) -> &Constraint {
        &self.inner[idx]
    }

    pub fn push(&mut self, new_constraint: Constraint) {
        unique_push(&mut self.inner, new_constraint);
    }

    pub fn append(&mut self, new_constraints: Constraints) {
        unique_append(&mut self.inner, new_constraints.inner);
    }

    // Write: strong-update the field within EVERY disjunct currently in scope.
    // This is what makes {A, B}.f = C become {A{f:C}, B} instead of touching a global table.
    pub fn write_field(&mut self, elem: Vec<ProjectionElem>, new: Constraints) {
        for c in self.inner.iter_mut() {
            if let Some(RunningConstraint::Adt(_, _, fields)) = &mut c.cfc {
                fields.retain(|(e, _)| e != &elem);
                fields.push((elem.clone(), new.clone()));
            }
        }
    }
}

// Maybe organize TraitObjConstraints by trait..? Like if we have two potentially obfuscating
// dynamic calls (one for Option and one for inner TraitObj)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Constraint {
    pub toc: Option<(TraitObjTy, TraitObjConstraint)>,
    pub cfc: Option<RunningConstraint>,
}

impl Constraint {
    pub fn new(
        toc: Option<(TraitObjTy, TraitObjConstraint)>,
        cfc: Option<RunningConstraint>,
    ) -> Constraint {
        Self { toc, cfc }
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

pub type ADTFields = Vec<(Vec<ProjectionElem>, Constraints)>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TraitObjConstraint {
    // more complex data types
    //Adt(AdtDef, GenericArgs, Vec<Place>),
    Adt(AdtDef, GenericArgs, ADTFields),

    // callable types
    Closure(ClosureDef, GenericArgs),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location;

impl Location {
    pub fn new() -> Location {
        Self {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunningConstraint {
    // primitive data types
    Scalar(Option<i128>),
    Float,

    // more complex data types
    //Adt(AdtDef, GenericArgs, Vec<Place>),
    Adt(AdtDef, GenericArgs, ADTFields),

    // pointer types
    Ptr(Box<Constraint>),
    //Ref(Box<Constraint>),

    // callable types
    Closure(ClosureDef, GenericArgs),
    FnDef(FnDef, GenericArgs),
    FnPtr(SigVal),

    // dynamic types
    Dynamic(Vec<TraitObjTy>),

    // fallback types
    //IdkType(Ty),
    List(Box<Constraint>),
    Tuple(Vec<Constraint>),
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
    pub fn new_from_bound_existential(binder: &Binder<ExistentialPredicate>) -> TraitObjTy {
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
                return Self {
                    def: trait_ref.def_id,
                    genargs: trait_ref.generic_args,
                };
            }
            ExistentialPredicate::Projection(proj) => {
                return Self {
                    def: proj.def_id,
                    genargs: proj.generic_args,
                };
            }
            _ => todo!(),
        }
    }

    pub fn is_fn_trait(&self) -> bool {
        // FnMut
        if self.def.0.to_index() == 150 {
            //debug!("FNMUT TRAIT");
            true
        } else {
            //debug!("NOT A FN TRAIT");
            false
        }
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
    pub wtos: HashMap<VOID, BBDeps>,
}

impl Context {
    pub fn new(cstore: ConstraintStore, wtos: HashMap<VOID, BBDeps>) -> Context {
        Self { cstore, wtos }
    }

    pub fn empty() -> Context {
        Self {
            cstore: ConstraintStore::new(),
            wtos: HashMap::default(),
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

    fn step_field(
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

    fn field_idx(&self, elem: &ProjectionElem) -> usize {
        match elem {
            ProjectionElem::Field(idx, _) => *idx,
            _ => panic!("expected Field projection: {:?}", elem),
        }
    }
    fn step_field_one(
        &self,
        scope: &VOID,
        constraint: &Constraint,
        elem: &ProjectionElem,
    ) -> Constraints {
        match &constraint.cfc {
            Some(RunningConstraint::Adt(_, _, fields)) => fields
                .iter()
                .find(|(key, _)| key.len() == 1 && self.field_idx(&key[0]) == self.field_idx(elem))
                .map(|(_, cs)| cs.clone())
                .unwrap_or_else(Constraints::new), // unknown/never-written field -> fallback, see below
            Some(RunningConstraint::Tuple(inner)) => match elem {
                ProjectionElem::Field(idx, _) => Constraints::from(inner[*idx].clone()),
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
            // so it contributes no information to the projection — not an error.
            _ => {
                debug!(
                    "unexpected running constraint type to have projections: {:?}",
                    constraint.cfc
                );
                Constraints::new()
            }
        }
    }

    pub fn get_constraints(
        &self,
        scope: &VOID,
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
            match self.get_constraints(scope, &base, is_closure) {
                Some(base_constraints) => {
                    debug!("\nBASE: {:?}", base);
                    debug!("BASE CONSTRAINTS: {:?}", base_constraints);
                    debug!("PROJECTION: {:?}", place.projection);

                    // Collect every matching projection in the set of constraints
                    let mut cur = base_constraints;
                    for elem in &place.projection {
                        // Ignore non-field projections
                        if matches!(elem, ProjectionElem::Field(..)) {
                            cur = self.step_field(scope, &cur, elem);
                        }
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintStore {
    pub cmap: HashMap<MapKey, Box<MapValue>>,
    pub refs: HashMap<(Place, VOID), ((Place, VOID), Mutability)>,
}

impl ConstraintStore {
    pub fn new() -> ConstraintStore {
        Self {
            cmap: HashMap::default(),
            refs: HashMap::default(),
        }
    }

    /*
    pub fn link_adt_field(&mut self, scope: &VOID, place: &Place, new_field_places: &Vec<Place>) {
        debug!("\nLINKING FIELD");
        debug!("new_field_places: {:?}", new_field_places);

        self.scoped_field_update(
            scope,
            MapKey::Var(place.clone()),
            Box::new(MapFieldValue::Fields(new_field_places.to_vec())),
        );

        match self.field_map.get_mut(adt_place_and_scope) {
            Some(field_places) => {
                //debug!("ADDING FIELDS for ADT at {:?}", adt_place_and_scope);
                //debug!("old field projections: {:?}", field_places);
                //debug!("new field projection: {:?}", field_place);

                let mut new_field_places = Vec::new();
                for old_field_place in field_places.clone() {
                    unique_push(&mut new_field_places, old_field_place.clone());
                    unique_push(&mut new_field_places, field_place.clone());
                }
                debug!("NEW FIELD PROJECTIONS: {:?}", new_field_places);
                *field_places = new_field_places;
            }
            None => {
                //debug!("INITING FIELDS for ADT at {:?}", adt_place_and_scope);
                //debug!("new field projection: {:?}", field_place);
                self.field_map
                    .insert(adt_place_and_scope.clone(), vec![field_place.clone()]);
            }
        }
    }
    */

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
            MapKey::ScopeId(_) => (scope.clone(), key.clone()),
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
            MapKey::ScopeId(_) => (scope.clone(), key.clone()),
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
                debug!("undefined scope: {:?} INIT NEW SUBSTORE", scope);
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
