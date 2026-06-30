use crate::rustc_public_bridge::IndexedVal;
use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_public::mir::mono::Instance;
use rustc_public::mir::{Place, ProjectionElem};
use rustc_public::ty::{
    AdtDef, Binder, ClosureDef, ExistentialPredicate, FnDef, GenericArgs, TraitDef,
};

//use crate::common::log_scope;
use crate::merge::merge_mapvals;
use crate::sig_collect::SigVal;
use crate::wto::BBDeps;

use log::debug;

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
    Store(InterpStore, EnclosingScopes),
    Constraints(Constraints),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MapFieldValue {
    Store(InterpStore),
    Fields(Vec<Place>),
}

pub type ADTFields = Vec<(Vec<ProjectionElem>, Constraints)>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstraintsAndFields {
    pub constraints: Vec<Constraint>,
    pub fields: Vec<Place>,
    pub scope: VOID,
}

impl ConstraintsAndFields {
    pub fn new(
        constraints: Vec<Constraint>,
        fields: Vec<Place>,
        scope: VOID,
    ) -> ConstraintsAndFields {
        Self {
            constraints,
            fields,
            scope,
        }
    }

    pub fn empty(scope: VOID) -> ConstraintsAndFields {
        Self {
            constraints: Vec::new(),
            fields: Vec::new(),
            scope,
        }
    }

    pub fn update(&mut self, new_cafs: ConstraintsAndFields) {
        unique_append(&mut self.constraints, new_cafs.constraints);
        unique_append(&mut self.fields, new_cafs.fields);
    }
}

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

// These should only be Field ProjectionElems. The convention is that any time one of these
// field projections is used, it will be prepended by a Deref ProjectionElem
//pub type FieldProjections = Vec<ProjectionElem>;
//pub type FieldPlace = Place;

#[derive(Debug, Clone, PartialEq)]
pub struct InterpStore {
    pub cmap: HashMap<MapKey, Box<MapValue>>,
    pub wtos: HashMap<VOID, BBDeps>,
    // Map ADT places to their field places (projections) which have constraints in cmap
    pub field_map: HashMap<MapKey, Box<MapFieldValue>>,
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
        //adt_place_and_scope: &(Place, VOID),
        scope: &VOID,
        place: &Place,
        new_field_places: &Vec<Place>, //proj: &ProjectionElem,
    ) {
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
        */
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

    pub fn scoped_field_get(&self, scope: &VOID, key: &MapKey) -> Option<MapFieldValue> {
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

    pub fn scoped_field_update(&mut self, scope: &VOID, key: MapKey, value: Box<MapFieldValue>) {
        match self.field_map.get(&MapKey::ScopeId(scope.clone())) {
            Some(vartype) => match *vartype.clone() {
                MapFieldValue::Store(mut store) => {
                    let new_val = value.clone();
                    let old_val = store.cmap.get(&key);
                    match old_val {
                        Some(old_val_) => {
                            debug!("old val: {:?}", old_val_);
                            debug!("new val: {:?}", new_val);
                            todo!();
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
            None => panic!("undefined scope: {:?}", scope),
        }
    }
}
