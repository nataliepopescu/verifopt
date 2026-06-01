use rustc_public::mir::{AggregateKind, BinOp, CastKind, Operand, Place, Rvalue, UnOp};
use rustc_public::ty::{
    BoundVariableKind, ConstantKind, GenericArgKind, GenericArgs, RigidTy, Ty, TyKind,
};

use crate::InterpStore;
use crate::constraints::{Constraints, MapKey, MapValue, VOGenargs, VOID, VORval};
use crate::constraints::{unique_append, unique_push};

use log::debug;

pub struct RvalConverter;

impl RvalConverter {
    pub fn new() -> RvalConverter {
        Self {}
    }

    pub fn convert(
        &self,
        istore: &InterpStore,
        cur_scope: &VOID,
        to_convert: &Rvalue,
    ) -> Constraints {
        debug!("CONVERTING RVALUE");
        match to_convert {
            Rvalue::Use(op) => {
                debug!("USE");
                self.convert_op(istore, cur_scope, op)
            }
            Rvalue::Ref(_region, _borrow_kind, place) => {
                debug!("REF");
                self.convert_place(istore, cur_scope, place)
            }
            Rvalue::Discriminant(place) => {
                debug!("DISCRIMINANT");
                self.convert_place(istore, cur_scope, place)
            }
            Rvalue::CopyForDeref(place) => {
                debug!("COPY FOR DEREF");
                self.convert_place(istore, cur_scope, place)
            }
            Rvalue::Cast(kind, op, ty) => {
                debug!("CAST");
                self.convert_cast(istore, cur_scope, kind, op, ty)
            }
            Rvalue::Aggregate(kind, fields) => {
                debug!("AGGREGATE");
                self.convert_agg(istore, cur_scope, kind, fields)
            }
            Rvalue::AddressOf(_rawptrkind, place) => {
                debug!("ADDRESS OF");
                self.convert_place(istore, cur_scope, place)
            }
            Rvalue::UnaryOp(unop, op) => {
                debug!("UNOP");
                self.convert_unop(istore, cur_scope, unop, op)
            }
            Rvalue::BinaryOp(binop, op1, op2) => {
                debug!("BINOP");
                self.convert_binop(istore, cur_scope, binop, op1, op2)
            }
            Rvalue::CheckedBinaryOp(binop, op1, op2) => {
                debug!("CHECKED BINOP");
                self.convert_binop(istore, cur_scope, binop, op1, op2)
            }
            _ => todo!("other rval: {:?}", to_convert),
        }
    }

    fn convert_op(&self, istore: &InterpStore, cur_scope: &VOID, op: &Operand) -> Constraints {
        debug!("CONVERTING OP");
        debug!("op: {:?}", op);

        match op {
            Operand::Copy(place) | Operand::Move(place) => {
                self.convert_place(istore, cur_scope, place)
            }
            Operand::Constant(const_op) => match const_op.const_.kind() {
                ConstantKind::Allocated(alloc) => match alloc.read_uint() {
                    Ok(val) => vec![VORval::Scalar(Some(val))],
                    _ => vec![],
                },
                ConstantKind::ZeroSized => vec![],
                other @ _ => todo!("other constant kind: {:?}", other),
            },
            _ => todo!("runtime checks"),
        }
    }

    fn convert_place(&self, istore: &InterpStore, cur_scope: &VOID, place: &Place) -> Constraints {
        debug!("CONVERTING PLACE");
        debug!("place: {:?}", place);

        match istore.scoped_get(cur_scope, &MapKey::Local(place.local), false) {
            Some(val) => match val {
                MapValue::Constraints(constraints) => {
                    debug!(
                        "found constraints for local {:?}: {:?}",
                        place.local, constraints
                    );
                    constraints
                }
                _ => panic!("value should not be a scope"),
            },
            None => {
                debug!("place {:?} has not been set, widen to type", place,);
                vec![]
            }
        }
    }

    fn convert_cast(
        &self,
        istore: &InterpStore,
        cur_scope: &VOID,
        kind: &CastKind,
        op: &Operand,
        ty: &Ty,
    ) -> Constraints {
        debug!("kind: {:?}", kind);
        debug!("op: {:?}", op);
        debug!("ty: {:?}", ty);

        match op {
            Operand::Constant(const_op) => {
                debug!("CONST OP");
                vec![self.convert_ty(&const_op.const_.ty())]
            }
            Operand::Copy(place) | Operand::Move(place) => {
                debug!("PLACE OP");
                self.convert_place(istore, cur_scope, place)
            }
            _ => todo!("runtime checks"),
        }
    }

    fn convert_agg(
        &self,
        istore: &InterpStore,
        cur_scope: &VOID,
        kind: &AggregateKind,
        fields: &Vec<Operand>,
    ) -> Constraints {
        match kind {
            AggregateKind::Adt(def, _variant_idx, genargs, _, _) => {
                debug!("ADT agg");
                vec![VORval::Adt(*def, genargs.clone())] //self.convert_genargs(genargs))]
            }
            AggregateKind::Tuple => {
                debug!("tuple agg");
                let mut inner_constraints = Vec::new();
                for op in fields {
                    unique_append(
                        &mut inner_constraints,
                        self.convert_op(istore, cur_scope, op),
                    );
                }
                inner_constraints
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

                vec![self.convert_ty(ty)]
            }
            AggregateKind::Array(ty) => {
                debug!("array agg");
                vec![self.convert_ty(ty)]
            }
            AggregateKind::Closure(def, genargs) => {
                debug!("closure agg");
                vec![VORval::Closure(*def, genargs.clone())]
            }
            _ => todo!("other agg kind: {:?}", kind),
        }
    }

    fn convert_genargs(&self, genargs: &GenericArgs) -> Option<VOGenargs> {
        if genargs.0.is_empty() {
            return None;
        }
        let mut converted_genargs = Vec::new();
        for genarg in &genargs.0 {
            match self.convert_genarg(genarg) {
                Some(vorval) => {
                    unique_push(&mut converted_genargs, vorval);
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

    pub fn convert_genarg(&self, genarg: &GenericArgKind) -> Option<VORval> {
        match genarg {
            GenericArgKind::Type(ty) => Some(self.convert_ty(ty)),
            _ => None,
        }
    }

    pub fn convert_ty(&self, ty: &Ty) -> VORval {
        debug!("CONVERTING TY");
        debug!("ty: {:?}", ty);

        match ty.kind() {
            TyKind::RigidTy(rigidty) => match rigidty {
                RigidTy::Bool | RigidTy::Int(_) | RigidTy::Uint(_) => VORval::Scalar(None),
                RigidTy::Adt(def, genargs) => VORval::Adt(def, genargs), //self.convert_genargs(&genargs)),
                RigidTy::Tuple(ty_vec) => {
                    let mut inner = Vec::new();
                    for ty in ty_vec {
                        unique_push(&mut inner, self.convert_ty(&ty));
                    }
                    VORval::Idk(inner)
                }
                RigidTy::Array(ty, _) | RigidTy::Slice(ty) => self.convert_ty(&ty),
                RigidTy::Closure(def, genargs) => VORval::Closure(def, genargs),
                RigidTy::FnDef(def, genargs) => VORval::FnDef(def, genargs),
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
                RigidTy::RawPtr(ty, _mut) => self.convert_ty(&ty),
                RigidTy::Str => VORval::Idk(vec![]),
                other @ _ => panic!("other rigidty: {:?}", other),
            },
            other @ _ => panic!("other ty kind: {:?}", other),
        }
    }

    fn convert_binop(
        &self,
        istore: &InterpStore,
        cur_scope: &VOID,
        binop: &BinOp,
        op1: &Operand,
        op2: &Operand,
    ) -> Constraints {
        debug!("BINOP");
        debug!("binop: {:?}", binop);

        match binop {
            BinOp::Add | BinOp::AddUnchecked => {
                self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| x + y)
            }
            BinOp::Sub | BinOp::SubUnchecked => {
                self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| x - y)
            }
            BinOp::Mul | BinOp::MulUnchecked => {
                self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| x * y)
            }
            BinOp::Div => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| x / y),
            BinOp::Rem => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| x % y),
            BinOp::Eq => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| {
                if x == y { 1u128 } else { 0u128 }
            }),
            BinOp::Lt => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| {
                if x < y { 1u128 } else { 0u128 }
            }),
            BinOp::Le => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| {
                if x <= y { 1u128 } else { 0u128 }
            }),
            BinOp::Ne => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| {
                if x != y { 1u128 } else { 0u128 }
            }),
            BinOp::Ge => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| {
                if x >= y { 1u128 } else { 0u128 }
            }),
            BinOp::Gt => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| {
                if x > y { 1u128 } else { 0u128 }
            }),
            // This binop return Ord results
            BinOp::Cmp => todo!("impl cmp binop"),
            // bit-level binops
            BinOp::Shl | BinOp::ShlUnchecked => {
                self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| x << y)
            }
            BinOp::Shr | BinOp::ShrUnchecked => {
                self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| x >> y)
            }
            BinOp::BitAnd => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| x & y),
            BinOp::BitOr => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| x | y),
            BinOp::BitXor => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| x ^ y),
            _ => todo!(),
        }
    }

    fn convert_binop_helper(
        &self,
        istore: &InterpStore,
        cur_scope: &VOID,
        op1: &Operand,
        op2: &Operand,
        f: fn(u128, u128) -> u128,
    ) -> Constraints {
        let c_op1 = self.convert_op(istore, cur_scope, op1);
        let c_op2 = self.convert_op(istore, cur_scope, op2);
        if c_op1.len() != 1 || c_op2.len() != 1 {
            return vec![VORval::Scalar(None)];
        }
        match (c_op1[0].clone(), c_op2[0].clone()) {
            (VORval::Scalar(Some(val1)), VORval::Scalar(Some(val2))) => {
                vec![VORval::Scalar(Some(f(val1, val2)))]
            }
            (VORval::Scalar(_), VORval::Scalar(_)) => vec![VORval::Scalar(None)],
            _ => panic!("unexpected vorvals for binop"),
        }
    }

    fn convert_unop(
        &self,
        _istore: &InterpStore,
        _cur_scope: &VOID,
        unop: &UnOp,
        _op: &Operand,
    ) -> Constraints {
        match unop {
            //// These unops return bool/integer results, so can safely ignore any operand constraints
            //UnOp::Not | UnOp::Neg => vec![],
            //// Otherwise, retain the operand constraints
            //_ => self.convert_op(istore, cur_scope, op),
            UnOp::PtrMetadata => vec![],
            _ => todo!("unimpl unop: {:?}", unop),
        }
    }
}
