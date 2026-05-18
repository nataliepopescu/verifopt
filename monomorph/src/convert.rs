use rustc_data_structures::fx::FxHashSet as HashSet;
use rustc_public::DefId;
use rustc_public::mir::{AggregateKind, CastKind, LocalDecl, Operand, Place, Rvalue};
use rustc_public::ty::{GenericArgKind, GenericArgs, RigidTy, Ty, TyKind};

use crate::InterpStore;
use crate::constraints::{
    Constraints, MapKey, MapValue, ScopeId, VerifoptGenarg, VerifoptGenargs, VerifoptRval,
};
use crate::trait_collect::TraitStore;

use log::debug;

pub struct RvalConverter<'a> {
    pub tstore: &'a TraitStore,
}

impl<'a> RvalConverter<'a> {
    pub fn new(tstore: &'a TraitStore) -> RvalConverter<'a> {
        Self { tstore }
    }

    pub fn convert(
        &self,
        istore: &InterpStore,
        cur_scope: ScopeId,
        local_decls: &[LocalDecl],
        to_convert: &Rvalue,
    ) -> Constraints {
        debug!("CONVERTING RVALUE");
        match to_convert {
            Rvalue::Use(op) => {
                debug!("USE");
                debug!("op: {:?}", op);
                self.convert_op(istore, cur_scope, local_decls, op)
            }
            Rvalue::Ref(_region, _borrow_kind, place) => {
                debug!("REF");
                let inner = self.convert_place(istore, cur_scope, local_decls, place);
                self.wrap_in_ref(inner)
            }
            Rvalue::Discriminant(place) => {
                debug!("DISCRIMINANT");
                self.convert_place(istore, cur_scope, local_decls, place)
            }
            Rvalue::CopyForDeref(place) => {
                debug!("COPY FOR DEREF");
                self.convert_place(istore, cur_scope, local_decls, place)
            }
            Rvalue::Cast(kind, op, ty) => {
                debug!("CAST");
                self.convert_cast(istore, cur_scope, kind, op, ty)
            }
            Rvalue::Aggregate(kind, fields) => {
                debug!("AGGREGATE");
                self.convert_agg(istore, cur_scope, local_decls, kind, fields)
            }
            //Rvalue::AddressOf(_rawptrkind, place) => {
            //    debug!("ADDRESS OF");
            //    self.convert_place(istore, cur_scope, local_decls, place)
            //}
            _ => todo!("other rval: {:?}", to_convert),
        }
    }

    fn convert_op(
        &self,
        istore: &InterpStore,
        cur_scope: ScopeId,
        local_decls: &[LocalDecl],
        op: &Operand,
    ) -> Constraints {
        debug!("CONVERTING OP");
        match op {
            Operand::Copy(place) | Operand::Move(place) => {
                self.convert_place(istore, cur_scope, local_decls, place)
            }
            Operand::Constant(const_op) => {
                debug!("const_op: {:#?}", const_op.const_);
                let mut constraints = HashSet::default();
                constraints.insert(VerifoptRval::IdkType(const_op.const_.ty()));
                constraints
            }
            _ => todo!("runtime checks"),
        }
    }

    fn convert_place(
        &self,
        istore: &InterpStore,
        cur_scope: ScopeId,
        local_decls: &[LocalDecl],
        place: &Place,
    ) -> Constraints {
        debug!("CONVERTING PLACE");
        let backup_ty_naive = &local_decls[place.local].ty;
        let resolved_ty = place.ty(local_decls);
        debug!("resolved_ty: \n{:?}", resolved_ty.unwrap());
        debug!("backup_ty_naive: \n{:?}", backup_ty_naive);

        // If projection exists, process + return (FIXME this is incomplete)
        //if !place.projection.is_empty() {
        //    if let Some(constraints) = self.convert_projection(local_decls, place) {
        //        return constraints;
        //    }
        //}

        let clean_place = Place {
            local: place.local,
            projection: Vec::new(),
        };
        debug!("clean_place: {:?}", clean_place);

        match istore.scoped_get(cur_scope, &MapKey::Place(clean_place.clone())) {
            Some(val) => match val {
                MapValue::Constraints(constraints) => {
                    debug!(
                        "found constraints for place {:?}: {:?}",
                        clean_place, constraints
                    );
                    return constraints;
                }
                _ => panic!("value should not be a scope"),
            },
            None => match place.ty(local_decls) {
                Ok(ty) => {
                    debug!(
                        "place {:?} has not been set, use resolved (backup) type {:?}",
                        place, ty
                    );
                    let mut constraints = HashSet::default();
                    constraints.insert(VerifoptRval::IdkType(ty));
                    return constraints;
                }
                e @ Err(_) => panic!("resolving place ty error: {:?}", e),
            },
        }
    }

    /*
    fn convert_projection(&self, local_decls: &[LocalDecl], place: &Place) -> Option<Constraints> {
        debug!("CONVERTING PROJECTIONS");
        let mut constraints = HashSet::default();

        if place.projection.len() > 1 {
            let backup_ty = local_decls[place.local].ty;
            debug!("multiple projections, using backup_ty: {:?}", backup_ty);
            constraints.insert(VerifoptRval::IdkType(backup_ty));
        } else {
            match place.projection[0] {
                // FIXME essentially ignoring the deref here
                ProjectionElem::Deref => return None,
                ProjectionElem::Field(field_idx, ty) => {
                    debug!("field_idx: {:?}", field_idx);
                    debug!("ty: {:?}", ty);
                    // FIXME
                    constraints.insert(VerifoptRval::IdkType(ty));
                }
                _ => {
                    constraints.insert(VerifoptRval::Idk());
                }
            }
        }

        Some(constraints)
    }
    */

    fn wrap_in_ref(&self, inner: Constraints) -> Constraints {
        let mut outer = HashSet::default();
        for constraint in inner.clone().drain() {
            outer.insert(VerifoptRval::Ref(Box::new(constraint)));
        }
        outer
    }

    fn convert_cast(
        &self,
        istore: &InterpStore,
        cur_scope: ScopeId,
        kind: &CastKind,
        op: &Operand,
        ty: &Ty,
    ) -> Constraints {
        debug!("kind: {:?}", kind);
        debug!("op: {:?}", op);
        debug!("ty: {:?}", ty);

        let mut constraints = HashSet::default();
        match op {
            Operand::Constant(const_op) => {
                constraints.insert(VerifoptRval::IdkType(const_op.const_.ty()));
            }
            Operand::Copy(place) | Operand::Move(place) => {
                match istore.scoped_get(cur_scope, &MapKey::Place(place.clone())) {
                    Some(val) => match val {
                        MapValue::Constraints(constraints_) => {
                            debug!(
                                "found constraints for place {:?}: {:?}",
                                place, constraints_
                            );
                            for constraint in constraints_.iter() {
                                debug!("constraint to resolve: {:?}", constraint);
                                constraints.insert(self.resolve_cast(kind, ty, constraint));
                            }
                        }
                        _ => panic!("trying to cast a scope"),
                    },
                    None => panic!("no value to cast"),
                }
            }
            _ => todo!("runtime checks"),
        }

        constraints
    }

    fn resolve_cast(
        &self,
        kind: &CastKind,
        dst_ty: &Ty,
        constraint: &VerifoptRval,
    ) -> VerifoptRval {
        match constraint {
            VerifoptRval::IdkAdt(_defid, _) => constraint.clone(),
            VerifoptRval::IdkType(_) => VerifoptRval::IdkType(*dst_ty),
            VerifoptRval::Ptr(inner) => {
                VerifoptRval::Ptr(Box::new(self.resolve_cast(kind, dst_ty, &*inner)))
            }
            VerifoptRval::Ref(inner) => {
                VerifoptRval::Ref(Box::new(self.resolve_cast(kind, dst_ty, &*inner)))
            }
            VerifoptRval::Scalar(_) => match kind {
                CastKind::IntToInt
                | CastKind::FloatToInt
                | CastKind::FloatToFloat
                | CastKind::IntToFloat
                | CastKind::PtrToPtr
                | CastKind::PointerCoercion(_) => constraint.clone(),
                _ => todo!("cannot yet cast (scalar): {:?} ({:?})", constraint, kind),
            },
            _ => todo!("cannot yet cast: {:?}", constraint),
        }
    }

    fn convert_agg(
        &self,
        istore: &InterpStore,
        cur_scope: ScopeId,
        local_decls: &[LocalDecl],
        kind: &AggregateKind,
        fields: &Vec<Operand>,
    ) -> Constraints {
        let mut constraints = HashSet::default();

        match kind {
            AggregateKind::Adt(def, variant_idx, genargs, _, _) => {
                match self.convert_genargs(istore, cur_scope, def.0, genargs) {
                    // FIXME DefId -> ScopeId/VerifoptId?
                    Some(converted_genargs) => {
                        debug!("defid: {:?}", def.0);
                        debug!("converted genargs: {:?}", converted_genargs);
                        if converted_genargs.list.is_empty() {
                            panic!("why here? already checked for empty...");
                        }
                        constraints.insert(VerifoptRval::IdkAdt(def.0, Some(converted_genargs)));
                    }
                    None => {
                        constraints.insert(VerifoptRval::IdkAdt(def.0, None));
                    }
                }
            }
            _ => todo!("other agg kind: {:?}", kind),
        }

        constraints
    }

    fn convert_genargs(
        &self,
        istore: &InterpStore,
        cur_scope: ScopeId,
        defid: DefId,
        genargs: &GenericArgs,
    ) -> Option<VerifoptGenargs> {
        if genargs.0.is_empty() {
            return None;
        }
        let mut converted_genargs = Vec::new();
        for genarg in &genargs.0 {
            match genarg {
                GenericArgKind::Type(ty) => {
                    converted_genargs.push(self.convert_genarg_ty(istore, cur_scope, defid, ty));
                }
                _ => {}
            }
        }
        Some(VerifoptGenargs::new(converted_genargs))
    }

    fn convert_genarg_ty(
        &self,
        istore: &InterpStore,
        cur_scope: ScopeId,
        defid: DefId,
        genarg_ty: &Ty,
    ) -> VerifoptGenarg {
        match genarg_ty.kind() {
            TyKind::RigidTy(rigidty) => match rigidty {
                RigidTy::Uint(uintty) => VerifoptRval::Uint(),
                RigidTy::Adt(adtdef, adt_genargs) => VerifoptRval::IdkAdt(
                    adtdef.0,
                    self.convert_genargs(istore, cur_scope, defid, &adt_genargs),
                ),
                other @ _ => panic!("other rigidty: {:?}", other),
            },
            other @ _ => panic!("other ty kind: {:?}", other),
        }
    }
}
