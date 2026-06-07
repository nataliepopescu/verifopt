use rustc_public::mir::{
    AggregateKind, BinOp, CastKind, ConstOperand, Operand, Place, Rvalue, UnOp,
};
use rustc_public::ty::{AdtDef, ConstantKind, GenericArgKind, GenericArgs, RigidTy, Ty, TyKind};

use crate::InterpStore;
use crate::TraitStore;
use crate::constraints::{
    Constraint, Constraints, ControlFlowConstraint, MapKey, MapValue, TraitObjConstraint,
    TraitObjDestTy, VOID,
};
use crate::constraints::{unique_append, unique_push};
use crate::sig_collect::SigVal;

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
        cur_scope: &VOID,
        maybe_trait_destty: &Option<Vec<TraitObjDestTy>>,
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
                self.convert_agg(istore, cur_scope, maybe_trait_destty, kind, fields)
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
            Rvalue::Repeat(op, _tyconst) => {
                debug!("REPEAT");
                self.convert_op(istore, cur_scope, op)
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
            Operand::Constant(const_op) => self.convert_const(&const_op),
            _ => todo!("runtime checks"),
        }
    }

    pub fn convert_const(&self, const_op: &ConstOperand) -> Constraints {
        match const_op.const_.kind() {
            ConstantKind::Allocated(alloc) => match alloc.read_int() {
                // FIXME
                Ok(val) => {
                    debug!("ALLOC CONST");
                    // Only use the constval if this is supposed to be used as an integer
                    match const_op.const_.ty().kind() {
                        TyKind::RigidTy(rigidty) => match rigidty {
                            RigidTy::Bool | RigidTy::Int(_) | RigidTy::Uint(_) => {
                                vec![Constraint::new(
                                    None,
                                    Some(ControlFlowConstraint::Scalar(Some(val))),
                                )]
                            }
                            _ => vec![],
                        },
                        _ => todo!(),
                    }
                }
                _ => vec![],
            },
            ConstantKind::ZeroSized => vec![],
            other @ _ => todo!("arg is another constant kind: {:?}", other),
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

    pub fn get_any_traitobj(
        &self,
        maybe_trait_destty: &Option<Vec<TraitObjDestTy>>,
        constraint: &Constraint,
    ) -> Option<TraitObjConstraint> {
        match constraint {
            Constraint { toc: Some(to_), .. } => {
                debug!("PROPAGATING TOC: {:?}", to_);
                return Some(to_.clone());
            }
            Constraint {
                toc: None,
                cfc: Some(maybe_to),
            } => match maybe_to {
                ControlFlowConstraint::Adt(adtdef, _adt_genargs) => {
                    // If we get Some, that means this struct/adt implements one or more
                    // traits, but that does _not_ mean that this is a trait object
                    match self.tstore.struct_traits.get(&adtdef.0) {
                        Some(_) => {
                            // If we know we are storing the result of this rval into a
                            // traitobj, then populate the traitobj constraint field
                            if maybe_trait_destty.is_some() {
                                todo!();
                                //debug!("SETTING TOC: ({:?}, {:?})", adtdef, adt_genargs);
                                //return Some((adtdef.clone(), adt_genargs.clone()));
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            _ => {}
        }

        None
    }

    fn contains_traitobj(
        &self,
        maybe_trait_destty: &Option<Vec<TraitObjDestTy>>,
        def: &AdtDef,
        genargs: &Vec<Constraint>,
    ) -> Option<TraitObjConstraint> {
        debug!("IS TO?");
        debug!("def: {:?}", def);
        debug!("genargs: {:?}", genargs);

        // TODO check def for traitobj

        // check genargs for traitobj
        let mut to = None;
        for genarg in genargs {
            match self.get_any_traitobj(maybe_trait_destty, &genarg) {
                to_ @ Some(_) => {
                    to = to_;
                    break;
                }
                _ => {}
            }
        }

        debug!("TO: {:?}", to);

        to
    }

    fn contains_controlflow(
        &self,
        def: &AdtDef,
        genargs: &Vec<Constraint>,
    ) -> Option<ControlFlowConstraint> {
        debug!("IS CF?");
        debug!("def: {:?}", def);
        debug!("genargs: {:?}", genargs);

        // TODO check def for controlflow

        // check genargs for controlflow
        let mut cf = None;
        for genarg in genargs {
            match genarg {
                Constraint {
                    toc: _,
                    cfc: Some(cf_),
                } => {
                    cf = Some(cf_.clone());
                    break;
                }
                _ => {}
            }
        }

        debug!("CF: {:?}", cf);

        cf
    }

    /*
    fn convert_repeat(
        &self,
        istore: &InterpStore,
        cur_scope: &VOID,
        op: &Operand,
    ) -> Constraints {
        match op {
            Operand::Copy(place) | Operand::Move(place) => {
                self.convert_place(istore, cur_scope, place)
            }
            Operand::Constant(const_op) => self.convert_const(&const_op),
            _ => todo!("runtime checks"),
        }
    }
    */

    fn convert_agg(
        &self,
        istore: &InterpStore,
        cur_scope: &VOID,
        maybe_trait_destty: &Option<Vec<TraitObjDestTy>>,
        kind: &AggregateKind,
        fields: &Vec<Operand>,
    ) -> Constraints {
        match kind {
            AggregateKind::Adt(def, _variant_idx, genargs, _, _) => {
                debug!("ADT agg");
                debug!("ty: {:?}", def.ty_with_args(genargs));

                if let Some(c_genargs) = self.convert_genargs(genargs) {
                    let to = self.contains_traitobj(maybe_trait_destty, def, &c_genargs);
                    let cf = self.contains_controlflow(def, &c_genargs);

                    if to.is_some() || cf.is_some() {
                        return vec![Constraint::new(to, cf)];
                    }
                }

                vec![]
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
                vec![Constraint::new(
                    None,
                    Some(ControlFlowConstraint::Closure(*def, genargs.clone())),
                )]
            }
            _ => todo!("other agg kind: {:?}", kind),
        }
    }

    fn convert_genargs(&self, genargs: &GenericArgs) -> Option<Vec<Constraint>> {
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

    pub fn convert_genarg(&self, genarg: &GenericArgKind) -> Option<Constraint> {
        match genarg {
            GenericArgKind::Type(ty) => Some(self.convert_ty(ty)),
            _ => None,
        }
    }

    pub fn convert_ty(&self, ty: &Ty) -> Constraint {
        debug!("CONVERTING TY");
        debug!("ty: {:?}", ty);

        match ty.kind() {
            TyKind::RigidTy(rigidty) => match rigidty {
                // FIXME want to create any TraitObjConstraints here?
                RigidTy::Bool | RigidTy::Int(_) | RigidTy::Uint(_) => {
                    Constraint::new(None, Some(ControlFlowConstraint::Scalar(None)))
                }
                RigidTy::Adt(def, genargs) => {
                    Constraint::new(None, Some(ControlFlowConstraint::Adt(def, genargs)))
                }
                RigidTy::Tuple(ty_vec) => {
                    let mut inner = Vec::new();
                    for ty in ty_vec {
                        unique_push(&mut inner, self.convert_ty(&ty));
                    }
                    Constraint::new(None, Some(ControlFlowConstraint::Idk(Box::new(inner))))
                }
                RigidTy::Array(ty, _) | RigidTy::Slice(ty) => self.convert_ty(&ty),
                RigidTy::Closure(def, genargs) => {
                    Constraint::new(None, Some(ControlFlowConstraint::Closure(def, genargs)))
                }
                RigidTy::FnDef(def, genargs) => {
                    Constraint::new(None, Some(ControlFlowConstraint::FnDef(def, genargs)))
                }
                RigidTy::FnPtr(poly_fn_sig) => {
                    debug!("CONVERTING FN PTR");
                    debug!("poly_fn_sig: {:?}", poly_fn_sig);

                    let sigval = SigVal::new_from_poly(&poly_fn_sig);

                    //if !poly_fn_sig.bound_vars.is_empty() {
                    //    for bound_var in poly_fn_sig.bound_vars {
                    //        match bound_var {
                    //            BoundVariableKind::Ty(_) => todo!(),
                    //            _ => {}
                    //        }
                    //    }
                    //}
                    //let mut inputs_output_vorvals = Vec::new();
                    //for io in poly_fn_sig.value.inputs_and_output {
                    //    inputs_output_vorvals.push(self.convert_ty(&io));
                    //}

                    Constraint::new(None, Some(ControlFlowConstraint::FnPtr(sigval)))
                }
                RigidTy::Ref(_, ty, _) => self.convert_ty(&ty),
                RigidTy::RawPtr(ty, _mut) => self.convert_ty(&ty),
                RigidTy::Char | RigidTy::Str | RigidTy::Never => Constraint::new(None, None),
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

        let constraint =
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
                    if x == y { 1 } else { 0 }
                }),
                BinOp::Lt => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| {
                    if x < y { 1 } else { 0 }
                }),
                BinOp::Le => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| {
                    if x <= y { 1 } else { 0 }
                }),
                BinOp::Ne => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| {
                    if x != y { 1 } else { 0 }
                }),
                BinOp::Ge => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| {
                    if x >= y { 1 } else { 0 }
                }),
                BinOp::Gt => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| {
                    if x > y { 1 } else { 0 }
                }),
                // bit-level binops
                BinOp::Shl | BinOp::ShlUnchecked => {
                    self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| x << y)
                }
                BinOp::Shr | BinOp::ShrUnchecked => {
                    self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| x >> y)
                }
                BinOp::BitAnd => {
                    self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| x & y)
                }
                BinOp::BitOr => {
                    self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| x | y)
                }
                BinOp::BitXor => {
                    self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| x ^ y)
                }
                // This binop return Ord results
                BinOp::Cmp => self.convert_binop_helper(istore, cur_scope, op1, op2, |x, y| {
                    if x < y {
                        -1
                    } else if x > y {
                        1
                    } else {
                        0
                    }
                }),
                // TODO
                BinOp::Offset => {
                    Constraint::new(None, Some(ControlFlowConstraint::Idk(Box::new(vec![]))))
                }
            };

        vec![constraint]
    }

    fn convert_binop_helper(
        &self,
        istore: &InterpStore,
        cur_scope: &VOID,
        op1: &Operand,
        op2: &Operand,
        f: fn(i128, i128) -> i128,
    ) -> Constraint {
        let c_op1 = self.convert_op(istore, cur_scope, op1);
        let c_op2 = self.convert_op(istore, cur_scope, op2);
        if c_op1.len() != 1 || c_op2.len() != 1 {
            return Constraint::new(None, Some(ControlFlowConstraint::Scalar(None)));
        }
        match (c_op1[0].clone(), c_op2[0].clone()) {
            (
                Constraint {
                    toc: None,
                    cfc: Some(ControlFlowConstraint::Scalar(Some(val1))),
                },
                Constraint {
                    toc: None,
                    cfc: Some(ControlFlowConstraint::Scalar(Some(val2))),
                },
            ) => Constraint::new(
                None,
                Some(ControlFlowConstraint::Scalar(Some(f(val1, val2)))),
            ),
            (
                Constraint {
                    toc: None,
                    cfc: Some(ControlFlowConstraint::Scalar(Some(val1))),
                },
                Constraint {
                    toc: to,
                    cfc: Some(ControlFlowConstraint::Scalar(Some(val2))),
                },
            ) => Constraint::new(to, Some(ControlFlowConstraint::Scalar(Some(f(val1, val2))))),
            (
                Constraint {
                    toc: to,
                    cfc: Some(ControlFlowConstraint::Scalar(Some(val1))),
                },
                Constraint {
                    toc: None,
                    cfc: Some(ControlFlowConstraint::Scalar(Some(val2))),
                },
            ) => Constraint::new(to, Some(ControlFlowConstraint::Scalar(Some(f(val1, val2))))),
            (
                Constraint {
                    toc: _to1,
                    cfc: Some(ControlFlowConstraint::Scalar(Some(_val1))),
                },
                Constraint {
                    toc: _to2,
                    cfc: Some(ControlFlowConstraint::Scalar(Some(_val2))),
                },
            ) => {
                todo!();
                //(to, Some(ControlFlowConstraint::Scalar(Some(f(
                //    val1, val2,
                //)))))
            }
            other @ _ => {
                debug!("BINOP HELPER OTHER: {:?}", other);
                Constraint::new(None, Some(ControlFlowConstraint::Scalar(None)))
            }
        }
    }

    fn convert_unop(
        &self,
        istore: &InterpStore,
        cur_scope: &VOID,
        unop: &UnOp,
        op: &Operand,
    ) -> Constraints {
        let constraint = match unop {
            UnOp::Neg => self.convert_unop_helper(istore, cur_scope, op, |x| -x),
            UnOp::Not => self.convert_unop_helper(istore, cur_scope, op, |x| !x),
            UnOp::PtrMetadata => {
                Constraint::new(None, Some(ControlFlowConstraint::Idk(Box::new(vec![]))))
            }
        };

        vec![constraint]
    }

    fn convert_unop_helper(
        &self,
        istore: &InterpStore,
        cur_scope: &VOID,
        op: &Operand,
        f: fn(i128) -> i128,
    ) -> Constraint {
        let c_op = self.convert_op(istore, cur_scope, op);
        if c_op.len() != 1 {
            return Constraint::new(None, Some(ControlFlowConstraint::Scalar(None)));
        }
        match c_op[0].clone() {
            Constraint {
                toc: to,
                cfc: Some(ControlFlowConstraint::Scalar(Some(val))),
            } => Constraint::new(to, Some(ControlFlowConstraint::Scalar(Some(f(val))))),
            Constraint { toc: to, cfc: _ } => {
                Constraint::new(to, Some(ControlFlowConstraint::Scalar(None)))
            } //_ => Constraint::ControlFlow(Box::new(ControlFlowConstraint::Scalar(None))),
        }
    }
}
