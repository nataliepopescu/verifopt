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

#[derive(Debug, Clone, PartialEq)]
pub enum MapFieldValue {
    Store(FieldStore),
    Fields(Vec<Place>),
}

pub type ADTFields = Vec<(Vec<ProjectionElem>, Constraints)>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CAFs {
    pub constraints: Constraints,
    pub fields: Vec<Place>,
    pub scope: VOID,
}

impl CAFs {
    pub fn new(constraints: Constraints, fields: Vec<Place>, scope: VOID) -> CAFs {
        Self {
            constraints,
            fields,
            scope,
        }
    }

    pub fn empty(scope: VOID) -> CAFs {
        Self {
            constraints: Vec::new(),
            fields: Vec::new(),
            scope,
        }
    }

    pub fn update(&mut self, new_cafs: CAFs) {
        unique_append(&mut self.constraints, new_cafs.constraints);
        unique_append(&mut self.fields, new_cafs.fields);
    }
}

// Set of positive constraints; negative constraints are resolved immediately by removing them from the set
pub type Constraints = Vec<Constraint>;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TraitObjConstraint {
    // more complex data types
    Adt(AdtDef, GenericArgs),

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
    Adt(AdtDef, GenericArgs),

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
            .map(|cs| cs.iter().cloned().collect::<HashSet<Constraint>>())
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
    istore: &InterpStore,
    term_span: &Span,
    caller_scope: &VOID,
    body: &Body,
    local_decls: &[LocalDecl],
    args: &Vec<Operand>,
    is_closure: bool,
) -> SummaryKey {
    let cs: Vec<Constraints> = ipass
        .collect_resolved_args(
            istore,
            term_span,
            caller_scope,
            &body,
            local_decls,
            args,
            is_closure,
        )
        .into_iter()
        .map(|(cs, _)| cs)
        .collect();

    (scope, ArgSet::new(&cs))
}

// These should only be Field ProjectionElems. The convention is that any time one of these
// field projections is used, it will be prepended by a Deref ProjectionElem
//pub type FieldProjections = Vec<ProjectionElem>;
//pub type FieldPlace = Place;

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub cstore: ConstraintStore,
    pub fstore: FieldStore,
    pub wtos: HashMap<VOID, BBDeps>,
    // Map ADT places to their field places (projections) which have constraints in cmap
    //pub field_map: HashMap<(Place, VOID), Vec<Place>>,
    pub refs: HashMap<(Place, VOID), ((Place, VOID), Mutability)>,
}

impl Context {
    pub fn new(
        cstore: ConstraintStore,
        fstore: FieldStore,
        wtos: HashMap<VOID, BBDeps>,
    ) -> Context {
        Self {
            cstore,
            fstore,
            wtos,
            refs: HashMap::default(),
        }
    }

    pub fn empty() -> Context {
        Self {
            cstore: ConstraintStore::new(),
            fstore: FieldStore::new(),
            wtos: HashMap::default(),
        }
    }

    //pub fn cstore(&self) -> &ConstraintStore {
    //    &self.cstore
    //}

    //pub fn fstore(&self) -> &FieldStore {
    //    &self.fstore
    //}

    //pub fn wtos(&self) -> &HashMap<VOID, BBDeps> {
    //    &self.wtos
    //}

    pub fn get_wto(&self, scope: &VOID) -> Option<&BBDeps> {
        self.wtos.get(scope)
    }

    pub fn set_wto(&mut self, scope: &VOID, bbdeps: &BBDeps) {
        self.wtos.insert(scope.clone(), bbdeps.clone());
    }

    fn set_constraints(&mut self, place: &Place, constraints: Constraints) {
        self.cstore.cmap.insert(
            MapKey::Var(place.clone()),
            Box::new(MapValue::Constraints(constraints)),
        );
    }

    fn set_scoped_constraints(&mut self, scope: &VOID, place: &Place, constraints: Constraints) {
        self.cstore.scoped_update(
            scope,
            MapKey::Var(place.clone()),
            Box::new(MapValue::Constraints(constraints)),
        );
    }

    // currently getting in:
    // - interp_direct_call (getting field constraints + lifting TOC)
    // - get_place_constraints (only used during arg res - also for getting field constraints + lifting TOC)
    // - get_fsa_tyconstraints (get traitobj constraints)
    // - interp_switchint (get discriminant constraints)
    //
    // above cases seem to be safe to access without CAF API b/c not using fields, only using the
    // precise object asked about (with the ~exception of interp_direct_call and
    // get_place_constraints, which could be merged/improved)
    pub fn get_constraints(
        &self,
        scope: &VOID,
        place: &Place,
        is_closure: bool,
    ) -> Option<MapValue> {
        self.cstore
            .scoped_get(scope, &MapKey::Var(place.clone()), is_closure)
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

    fn get_fields(&self, scope: &VOID, place: &Place) -> Option<MapFieldValue> {
        self.fstore.scoped_get(scope, &MapKey::Var(place.clone()))
    }

    fn set_fields(&mut self, scope: &VOID, place: &Place, new_field_places: &Vec<Place>) {
        self.fstore.link_adt_fields(scope, place, new_field_places);
    }

    pub fn update_cafs(
        &mut self,
        scope: &VOID,
        base: (Place, Constraints),
        fields: Vec<(Place, Constraints)>,
    ) {
        // add field mappings -> fieldStore
        if !fields.is_empty() {
            let field_places: Vec<Place> = fields.clone().into_iter().map(|(x, _)| x).collect();
            self.set_fields(scope, &base.0, &field_places);
        }

        // add base + field constraints -> constraintStore
        self.set_constraints(&base.0, base.1);
        for field in fields {
            self.set_constraints(&field.0, field.1);
        }
    }

    pub fn update_scoped_cafs(
        &mut self,
        scope: &VOID,
        base: (Place, Constraints),
        fields: Vec<(Place, Constraints)>,
    ) {
        // add field mappings -> fieldStore
        if !fields.is_empty() {
            let field_places: Vec<Place> = fields.clone().into_iter().map(|(x, _)| x).collect();
            self.set_fields(scope, &base.0, &field_places);
        }

        // add base + field constraints -> constraintStore
        self.set_scoped_constraints(scope, &base.0, base.1);
        for field in fields {
            self.set_scoped_constraints(scope, &field.0, field.1);
        }
    }

    pub fn get_cafs(&self, scope: &VOID, place: &Place, is_closure: bool) -> Option<CAFs> {
        match self.get_constraints(scope, place, is_closure) {
            Some(ret) => match ret {
                MapValue::Constraints(constraints) => {
                    let fields = match self.get_fields(scope, place) {
                        Some(MapFieldValue::Fields(fields_)) => fields_,
                        Some(_) => panic!("got scope"),
                        None => vec![],
                    };

                    Some(CAFs::new(constraints, fields, scope.clone()))
                }
                _ => panic!("got scope"),
            },
            None => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintStore {
    pub cmap: HashMap<MapKey, Box<MapValue>>,
}

impl ConstraintStore {
    pub fn new() -> ConstraintStore {
        Self {
            cmap: HashMap::default(),
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

    pub fn link_adt_field(&mut self, scope: &VOID, place: &Place, new_field_places: &Vec<Place>) {
        debug!("\nLINKING FIELD");
        debug!("new_field_places: {:?}", new_field_places);

        self.scoped_field_update(
            scope,
            MapKey::Var(place.clone()),
            Box::new(MapFieldValue::Fields(new_field_places.to_vec())),
        );

        /*
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
        let mut all_constraints = Vec::new();
        for enclosing_scope in enclosing_scopes.as_ref().unwrap() {
            match self.scoped_get(&enclosing_scope, key, false) {
                Some(val) => match val {
                    MapValue::Constraints(constraints) => {
                        unique_append(&mut all_constraints, constraints)
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
            None => panic!("undefined scope: {:?}", scope),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldStore {
    pub field_map: HashMap<MapKey, Box<MapFieldValue>>,
}

impl FieldStore {
    pub fn new() -> FieldStore {
        Self {
            field_map: HashMap::default(),
        }
    }

    pub fn link_adt_fields(&mut self, scope: &VOID, place: &Place, new_field_places: &Vec<Place>) {
        debug!("\nLINKING FIELD");
        debug!("new_field_places: {:?}", new_field_places);

        self.scoped_update(
            scope,
            MapKey::Var(place.clone()),
            Box::new(MapFieldValue::Fields(new_field_places.to_vec())),
        );
    }

    pub fn scoped_get(&self, scope: &VOID, key: &MapKey) -> Option<MapFieldValue> {
        match self.field_map.get(&MapKey::ScopeId(scope.clone())) {
            Some(vartype) => match *vartype.clone() {
                MapFieldValue::Store(store) => match store.field_map.get(key) {
                    Some(boxed) => Some(*boxed.clone()),
                    None => None,
                },
                _ => panic!("not a scope: {:?}", scope),
            },
            None => None,
        }
    }

    pub fn scoped_update(&mut self, scope: &VOID, key: MapKey, value: Box<MapFieldValue>) {
        match self.field_map.get(&MapKey::ScopeId(scope.clone())) {
            Some(vartype) => match *vartype.clone() {
                MapFieldValue::Store(mut store) => {
                    let new_val = value.clone();
                    let old_val = store.field_map.get(&key);
                    match old_val {
                        Some(old_val_) => {
                            debug!("old val: {:?}", old_val_);
                            debug!("new val: {:?}", new_val);
                            if *old_val_ != new_val {
                                todo!();
                            }
                            //new_val = Box::new(merge_mapvals(old_val_, &value));
                        }
                        None => {}
                    }

                    // modify scope w new key/val
                    store.field_map.insert(key, new_val);
                    self.field_map.insert(
                        MapKey::ScopeId(scope.clone()),
                        Box::new(MapFieldValue::Store(store)),
                    );
                }
                MapFieldValue::Fields(..) => {
                    panic!("defid is not a scope: {:?}", scope);
                }
            },
            None => {
                // initialize new scope w key/val
                debug!("undefined scope: {:?} INIT NEW SUBSTORE", scope);
                let mut new_store = FieldStore::new();
                new_store.field_map.insert(key, value);
                self.field_map.insert(
                    MapKey::ScopeId(scope.clone()),
                    Box::new(MapFieldValue::Store(new_store)),
                );
            }
        }
    }
}
