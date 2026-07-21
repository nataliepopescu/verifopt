use crate::interp::InterpPass;
use crate::rustc_public_bridge::IndexedVal;
use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_public::mir::mono::Instance;
use rustc_public::mir::{Body, LocalDecl, Operand, Place, ProjectionElem};
use rustc_public::ty::{
    AdtDef, Binder, ClosureDef, ExistentialPredicate, FnDef, GenericArgs, Span, TraitDef,
};

//use crate::common::log_scope;
use crate::error::Error;
use crate::sig_collect::SigVal;
use crate::wto::BBDeps;

use log::debug;

use std::collections::HashSet;
use std::hash::{DefaultHasher, Hash, Hasher};

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
    Store(InterpStore, EnclosingScopes),
    Constraints(Constraints),
}

pub type ADTFields = Vec<(Vec<ProjectionElem>, Constraints)>;

// Set of positive constraints; negative constraints are resolved immediately by removing them from the set
pub type Constraints = Vec<Constraint>;

// Alias around VORval to make it semantically easier to tell when we are processing generic arguments
//pub type VOGenargs = Vec<VOGenarg>;
//pub type VOGenarg = VORval;

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

//pub type TraitObjConstraint = (AdtDef, GenericArgs);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TraitObjConstraint {
    // more complex data types
    Adt(AdtDef, GenericArgs), //Option<VOGenargs>),

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

// Location portion corresponds to when this constraint was last set (at what span)
// TODO maybe refine this a bit though
//pub type RunningConstraint = (Location, RunningConstraintInner);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunningConstraint {
    // primitive data types
    Scalar(Option<i128>),
    Float,

    // more complex data types
    Adt(AdtDef, GenericArgs), //Option<VOGenargs>),

    // pointer types
    Ptr(Box<Constraint>),
    //Ref(Box<Constraint>),

    // callable types
    Closure(ClosureDef, GenericArgs),
    FnDef(FnDef, GenericArgs),
    FnPtr(SigVal), //Box<Constraints>, FnSig),

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
pub struct InterpStore {
    pub cmap: HashMap<MapKey, Box<MapValue>>,
    pub wtos: HashMap<VOID, BBDeps>,
    // Map ADT places to their field places (projections) which have constraints in cmap
    pub field_map: HashMap<(Place, VOID), Vec<Place>>,
}

impl InterpStore {
    pub fn new() -> InterpStore {
        Self {
            cmap: HashMap::default(),
            wtos: HashMap::default(),
            field_map: HashMap::default(),
        }
    }

    pub fn link_adt_field(
        &mut self,
        adt_place_and_scope: &(Place, VOID),
        field_place: &Place, //roj: &ProjectionElem,
    ) {
        debug!("\nLINKING FIELD");
        debug!("new_field_place: {:?}", field_place);

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
        match self.cmap.get(&MapKey::ScopeId(scope.clone())) {
            Some(vartype) => match *vartype.clone() {
                MapValue::Store(store, enclosing_scopes) => {
                    // Is key in inner_cmap? if not:
                    // - Is nested func: return None
                    // - Is closure: follow backptr to enclosing scope
                    match store.cmap.get(key) {
                        Some(boxed) => Some(*boxed.clone()),
                        None => {
                            //debug!("is_closure?: {:?}", is_closure);
                            //debug!("enclosing_scope: {:?}", enclosing_scopes);
                            if is_closure && enclosing_scopes.is_some() {
                                // Check enclosing scopes for missing key(s)
                                let constraints =
                                    self.get_from_enclosing_scopes(&enclosing_scopes, key);
                                Some(MapValue::Constraints(constraints))
                            } else {
                                // If this is incorrectly labeled as _not_ a closure (meaning it
                                // should be labeled a closure), can we get the needed value in the
                                // enclosing scope?
                                //debug!("SHOULD THIS SCOPE BE A CLOSURE?");
                                //log_scope(&scope);

                                //if enclosing_scopes.is_some() {
                                //    let constraints = self.get_from_enclosing_scopes(&enclosing_scopes, key);
                                //    debug!("got constraints from enclosing scope: {:?}", constraints);
                                //} else {
                                //    debug!("nope");
                                //}

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

pub fn merge_stores(
    cur_store: &InterpStore,
    cur_es: &EnclosingScopes,
    new_store: &InterpStore,
    new_es: &EnclosingScopes,
) -> (InterpStore, EnclosingScopes) {
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

    let merged_store = if *cur_store != *new_store {
        merge_stores_helper(cur_store, new_store)
    } else {
        cur_store.clone()
    };

    (merged_store, merged_es)
}

fn merge_stores_helper(cur_store: &InterpStore, new_store: &InterpStore) -> InterpStore {
    let vec = vec![cur_store.clone(), new_store.clone()];
    match vec.merge() {
        Ok(Some(merged)) => merged,
        Ok(None) => panic!("no stores to merge?"),
        e @ _ => panic!("error merging stores: {:?}", e),
    }
}

// FIXME merge span of RunningConstraints into a single vec if the RunningConstraintsInner are equal
fn merge_constraints(cur_constraints: &Constraints, new_constraints: &Constraints) -> Constraints {
    let mut merged = cur_constraints.clone();
    if merged != *new_constraints {
        unique_append(&mut merged, new_constraints.to_vec());
    }
    merged
}

fn merge_mapvals(cur_val: &MapValue, new_val: &MapValue) -> MapValue {
    match (cur_val.clone(), new_val.clone()) {
        (MapValue::Constraints(cur_constraints), MapValue::Constraints(new_constraints)) => {
            MapValue::Constraints(merge_constraints(&cur_constraints, &new_constraints))
        }
        (MapValue::Store(cur_store, cur_es), MapValue::Store(new_store, new_es)) => {
            let (store, es) = merge_stores(&cur_store, &cur_es, &new_store, &new_es);
            MapValue::Store(store, es)
        }
        _ => panic!("incomparable MapValue types"),
    }
}

pub trait Merge<T> {
    fn merge(&self) -> Result<Option<T>, Error>;
}

impl Merge<InterpStore> for Vec<InterpStore> {
    fn merge(&self) -> Result<Option<InterpStore>, Error> {
        //debug!("interp stores to merge: {:?}", self);

        if self.is_empty() {
            return Ok(None);
        }

        if self.len() == 1 {
            return Ok(Some(self[0].clone()));
        }

        let mut merged = self[0].clone();
        let mut first = true;
        for store in self.iter() {
            if first {
                first = false;
                continue;
            }
            for (key, val) in store.clone().cmap.iter() {
                match merged.cmap.get_mut(key) {
                    Some(merged_val) => {
                        let new_merged_val = merge_mapvals(merged_val, val);
                        merged.cmap.insert(key.clone(), Box::new(new_merged_val));
                    }
                    None => {
                        merged.cmap.insert(key.clone(), val.clone());
                    }
                }
            }
        }

        //debug!("merged stores: {:?}", merged);

        Ok(Some(merged))
    }
}

impl Merge<Constraints> for Vec<Constraints> {
    fn merge(&self) -> Result<Option<Constraints>, Error> {
        if self.is_empty() {
            return Ok(None);
        }

        if self.len() == 1 {
            return Ok(Some(self[0].clone()));
        }

        let mut merged_constraints = self[0].clone();
        for constraints in self.iter() {
            merged_constraints = merge_constraints(&merged_constraints, &constraints);
        }

        Ok(Some(merged_constraints))
    }
}
