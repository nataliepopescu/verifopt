use rustc_public::mir::mono::Instance;
use rustc_public::mir::{AggregateKind, BinOp, CastKind, LocalDecl, Operand, Place, Rvalue, UnOp};
use rustc_public::ty::{BoundVariableKind, GenericArgKind, GenericArgs, RigidTy, Ty, TyKind};

use crate::InterpStore;
//use crate::constraints::IntTy as VOIntTy;
use crate::constraints::{Constraints, MapKey, MapValue, VOGenargs, VORval};
use crate::constraints::{unique_append, unique_push};
use crate::projection::ProjectionHandler;

use log::debug;

pub struct RvalConverter {
    //pub tstore: &'a TraitStore,
    pub projection_handler: ProjectionHandler,
}

impl RvalConverter {
    pub fn new() -> RvalConverter {
        Self {
            projection_handler: ProjectionHandler::new(),
        }
    }

    pub fn convert(
        &self,
        istore: &InterpStore,
        cur_scope: Instance,
        local_decls: &[LocalDecl],
        to_convert: &Rvalue,
    ) -> Constraints {
        debug!("CONVERTING RVALUE");
        match to_convert {
            Rvalue::Use(op) => {
                debug!("USE");
                self.convert_op(istore, cur_scope, local_decls, op)
            }
            Rvalue::Ref(_region, _borrow_kind, place) => {
                debug!("REF");
                let place_constraints = self.convert_place(istore, cur_scope, local_decls, place);
                self.wrap_in_ref(place_constraints)
            }
            Rvalue::Discriminant(place) => {
                debug!("DISCRIMINANT");
                self.convert_discr(istore, cur_scope, local_decls, place)
            }
            Rvalue::CopyForDeref(place) => {
                debug!("COPY FOR DEREF");
                self.convert_place(istore, cur_scope, local_decls, place)
            }
            Rvalue::Cast(kind, op, ty) => {
                debug!("CAST");
                self.convert_cast(istore, cur_scope, local_decls, kind, op, ty)
            }
            Rvalue::Aggregate(kind, fields) => {
                debug!("AGGREGATE");
                self.convert_agg(istore, cur_scope, local_decls, kind, fields)
            }
            Rvalue::AddressOf(_rawptrkind, place) => {
                debug!("ADDRESS OF");
                let place_constraints = self.convert_place(istore, cur_scope, local_decls, place);
                self.wrap_in_addrof(place_constraints)
            }
            Rvalue::UnaryOp(unop, op) => {
                debug!("UNOP");
                self.convert_unop(local_decls, unop, op)
            }
            Rvalue::BinaryOp(binop, op1, op2) => {
                debug!("BINOP");
                self.convert_binop(local_decls, binop, op1, op2)
            }
            Rvalue::CheckedBinaryOp(binop, op1, op2) => {
                debug!("CHECKED BINOP");
                let binop_constraints = self.convert_binop(local_decls, binop, op1, op2);
                self.wrap_in_tup_bool(binop_constraints)
            }
            _ => todo!("other rval: {:?}", to_convert),
        }
    }

    fn wrap_in_addrof(&self, inner: Constraints) -> Constraints {
        let mut outer = Vec::new();
        for constraint in inner {
            unique_push(&mut outer, VORval::AddressOf(Box::new(constraint)));
        }
        outer
    }

    fn wrap_in_ref(&self, inner: Constraints) -> Constraints {
        let mut outer = Vec::new();
        for constraint in inner {
            unique_push(&mut outer, VORval::Ref(Box::new(constraint)));
        }
        outer
    }

    fn wrap_in_tup_bool(&self, inner: Constraints) -> Constraints {
        let mut outer = Vec::new();
        for constraint in inner {
            unique_push(&mut outer, VORval::Tuple(vec![constraint, VORval::Bool]));
        }
        outer
    }

    fn convert_op(
        &self,
        istore: &InterpStore,
        cur_scope: Instance,
        local_decls: &[LocalDecl],
        op: &Operand,
    ) -> Constraints {
        debug!("CONVERTING OP");
        debug!("op: {:?}", op);
        match op {
            Operand::Copy(place) | Operand::Move(place) => {
                self.convert_place(istore, cur_scope, local_decls, place)
            }
            Operand::Constant(const_op) => {
                debug!("const_op: {:#?}", const_op.const_);
                vec![VORval::IdkType(const_op.const_.ty())]
            }
            _ => todo!("runtime checks"),
        }
    }

    fn convert_place(
        &self,
        istore: &InterpStore,
        cur_scope: Instance,
        local_decls: &[LocalDecl],
        place: &Place,
    ) -> Constraints {
        debug!("CONVERTING PLACE");
        debug!("place: {:?}", place);
        let backup_ty_naive = &local_decls[place.local].ty;
        let resolved_ty = place.ty(local_decls).unwrap();
        debug!("backup_ty_naive: \n{:?}", backup_ty_naive);
        debug!("resolved_ty: \n{:?}", resolved_ty);

        match istore.scoped_get(cur_scope, &MapKey::Local(place.local), false) {
            Some(val) => match val {
                MapValue::Constraints(constraints) => {
                    debug!("found constraints for place {:?}: {:?}", place, constraints);
                    self.projection_handler.apply_projection(
                        backup_ty_naive,
                        &resolved_ty,
                        constraints,
                        place,
                    )
                }
                _ => panic!("value should not be a scope"),
            },
            None => {
                debug!(
                    "place {:?} has not been set, use resolved (backup) type {:?}",
                    place, resolved_ty,
                );
                vec![VORval::IdkType(resolved_ty)]
            } /*
              match place.ty(local_decls) {
                  Ok(ty) => {
                      debug!(
                          "place {:?} has not been set, use resolved (backup) type {:?}",
                          place, ty
                      );
                      vec![VORval::IdkType(ty)]
                  }
                  e @ Err(_) => panic!("resolving place ty error: {:?}", e),
              },
              */
        }
    }

    fn convert_discr(
        &self,
        _istore: &InterpStore,
        _cur_scope: Instance,
        local_decls: &[LocalDecl],
        place: &Place,
    ) -> Constraints {
        debug!("CONVERTING DISCRIMINANT");
        let resolved_ty = place.ty(local_decls).unwrap();
        // FIXME get more fine-grained results
        vec![VORval::IdkType(resolved_ty)]
    }

    fn convert_cast(
        &self,
        istore: &InterpStore,
        cur_scope: Instance,
        local_decls: &[LocalDecl],
        kind: &CastKind,
        op: &Operand,
        ty: &Ty,
    ) -> Constraints {
        debug!("kind: {:?}", kind);
        debug!("op: {:?}", op);
        debug!("ty: {:?}", ty);

        let mut constraints = Vec::new();
        match op {
            Operand::Constant(const_op) => {
                unique_push(&mut constraints, VORval::IdkType(const_op.const_.ty()));
            }
            Operand::Copy(place) | Operand::Move(place) => {
                let resolved_ty = place.ty(local_decls).unwrap();
                debug!("place: {:?}", place);
                debug!("resolved_ty for place: {:?}", resolved_ty);

                match istore.scoped_get(cur_scope, &MapKey::Local(place.local), false) {
                    Some(val) => match val {
                        MapValue::Constraints(constraints_) => {
                            debug!(
                                "found constraints for place {:?}: {:?}",
                                place, constraints_
                            );
                            for constraint in constraints_.iter() {
                                debug!("constraint to convert: {:?}", constraint);
                                unique_push(
                                    &mut constraints,
                                    self.convert_constraint_cast(kind, ty, constraint),
                                );
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

    fn convert_constraint_cast(&self, kind: &CastKind, dst_ty: &Ty, constraint: &VORval) -> VORval {
        match constraint {
            VORval::Adt(_, _)
            | VORval::Bool
            | VORval::Array(_)
            | VORval::Slice(_)
            | VORval::Idk
            | VORval::Uint => constraint.clone(),
            VORval::IdkType(_) => VORval::IdkType(*dst_ty),
            VORval::AddressOf(inner) => VORval::AddressOf(Box::new(
                self.convert_constraint_cast(kind, dst_ty, &*inner),
            )),
            VORval::RawPtr(inner) => VORval::RawPtr(Box::new(
                self.convert_constraint_cast(kind, dst_ty, &*inner),
            )),
            VORval::Ref(inner) => VORval::Ref(Box::new(
                self.convert_constraint_cast(kind, dst_ty, &*inner),
            )),
            VORval::Tuple(inner) => {
                let mut converted_inner = Vec::new();
                for inner_elem in inner {
                    unique_push(
                        &mut converted_inner,
                        self.convert_constraint_cast(kind, dst_ty, &*inner_elem),
                    );
                }
                VORval::Tuple(converted_inner)
            }
            VORval::Scalar(_) => match kind {
                CastKind::IntToInt
                | CastKind::FloatToInt
                | CastKind::FloatToFloat
                | CastKind::IntToFloat
                | CastKind::PtrToPtr
                | CastKind::PointerCoercion(_)
                | CastKind::Transmute => constraint.clone(),
                _ => todo!("cannot yet cast (scalar): {:?} ({:?})", constraint, kind),
            },
            _ => todo!(),
        }
    }

    fn convert_agg(
        &self,
        istore: &InterpStore,
        cur_scope: Instance,
        local_decls: &[LocalDecl],
        kind: &AggregateKind,
        fields: &Vec<Operand>,
    ) -> Constraints {
        let mut constraints = Vec::new();
        debug!("fields: {:?}", fields);

        match kind {
            AggregateKind::Adt(def, _variant_idx, genargs, _, _) => {
                debug!("ADT agg");
                unique_push(
                    &mut constraints,
                    VORval::Adt(*def, self.convert_genargs(genargs)),
                );
            }
            AggregateKind::Tuple => {
                debug!("tuple agg");
                let mut field_constraints = Vec::new();
                for op in fields {
                    unique_append(
                        &mut field_constraints,
                        self.convert_op(istore, cur_scope, local_decls, op),
                    );
                }
                unique_push(&mut constraints, VORval::Tuple(field_constraints));
            }
            AggregateKind::RawPtr(ty, _mut) => {
                debug!("rawptr agg");
                debug!("ty: {:?}", ty);

                match fields.len() {
                    0 => todo!("no fields"),
                    1 => todo!("thin ptr (1 field)"),
                    2 => debug!("fat ptr (2 fields)"),
                    _ => todo!("more than 2 fields"),
                }

                unique_push(
                    &mut constraints,
                    VORval::RawPtr(Box::new(VORval::IdkType(*ty))),
                );
            }
            AggregateKind::Array(ty) => {
                debug!("array agg");
                unique_push(&mut constraints, VORval::Array(*ty));
            }
            AggregateKind::Closure(def, genargs) => {
                debug!("closure agg");
                unique_push(
                    &mut constraints,
                    VORval::Closure(*def, genargs.clone()), //self.convert_genargs(genargs)),
                );
            }
            _ => todo!("other agg kind: {:?}", kind),
        }

        constraints
    }

    fn convert_genargs(&self, genargs: &GenericArgs) -> Option<VOGenargs> {
        if genargs.0.is_empty() {
            return None;
        }
        let mut converted_genargs = Vec::new();
        for genarg in &genargs.0 {
            match genarg {
                GenericArgKind::Type(ty) => {
                    unique_push(&mut converted_genargs, self.convert_ty(ty));
                }
                _ => {}
            }
        }

        if converted_genargs.is_empty() {
            None
        } else {
            Some(converted_genargs)
        }
    }

    pub fn convert_ty(&self, ty: &Ty) -> VORval {
        match ty.kind() {
            TyKind::RigidTy(rigidty) => match rigidty {
                RigidTy::Bool => VORval::Bool,
                RigidTy::Int(_intty) => VORval::Int,
                //match intty {
                //    IntTy::I8 => VORval::Int(VOIntTy::I8),
                //    IntTy::I16 => VORval::Int(VOIntTy::I16),
                //    IntTy::I32 => VORval::Int(VOIntTy::I32),
                //    _ => VORval::Int(VOIntTy::Other),
                //}
                RigidTy::Uint(_uintty) => VORval::Uint,
                RigidTy::Adt(def, genargs) => VORval::Adt(def, self.convert_genargs(&genargs)),
                RigidTy::Tuple(ty_vec) => {
                    let mut inner = Vec::new();
                    for ty in ty_vec {
                        unique_push(&mut inner, self.convert_ty(&ty));
                    }
                    VORval::Tuple(inner)
                }
                RigidTy::Slice(ty) => VORval::Slice(ty),
                RigidTy::Closure(def, genargs) => {
                    VORval::Closure(def, genargs) //self.convert_genargs(&genargs))
                }
                RigidTy::FnDef(def, genargs) => VORval::FnDef(def, self.convert_genargs(&genargs)),
                RigidTy::FnPtr(poly_fn_sig) => {
                    debug!("poly_fn_sig: {:?}", poly_fn_sig);
                    if !poly_fn_sig.bound_vars.is_empty() {
                        for bound_var in poly_fn_sig.bound_vars {
                            match bound_var {
                                BoundVariableKind::Ty(_) => todo!(),
                                _ => {}
                            }
                        }
                    }
                    let mut inputs_output_vorvals = Vec::new();
                    for io in poly_fn_sig.value.inputs_and_output {
                        inputs_output_vorvals.push(self.convert_ty(&io));
                    }
                    VORval::FnPtr(inputs_output_vorvals)
                }
                RigidTy::Ref(_, ty, _) => self.convert_ty(&ty),
                RigidTy::RawPtr(ty, _mut) => VORval::RawPtr(Box::new(self.convert_ty(&ty))),
                //RigidTy::Dynamic(traitref, _region) => {
                //    todo!();
                //}
                other @ _ => panic!("other rigidty: {:?}", other),
            },
            other @ _ => panic!("other ty kind: {:?}", other),
        }
    }

    fn convert_binop(
        &self,
        local_decls: &[LocalDecl],
        binop: &BinOp,
        op1: &Operand,
        op2: &Operand,
    ) -> Constraints {
        // Currently just returning an IdkType b/c this is likely a scalar op
        // TODO do we want to step into parts?
        vec![VORval::IdkType(binop.ty(
            op1.ty(local_decls).unwrap(),
            op2.ty(local_decls).unwrap(),
        ))]
    }

    fn convert_unop(&self, local_decls: &[LocalDecl], unop: &UnOp, op: &Operand) -> Constraints {
        // Currently just returning an IdkType b/c this is likely a scalar op
        // TODO do we want to step into parts?
        vec![VORval::IdkType(unop.ty(op.ty(local_decls).unwrap()))]
    }
}
