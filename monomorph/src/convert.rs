use rustc_public::mir::{
    AggregateKind, BinOp, CastKind, ConstOperand, LocalDecl, Operand, Place, Rvalue, UnOp,
};
use rustc_public::ty::{ConstantKind, GenericArgKind, RigidTy, Ty, TyKind};

use crate::InterpStore;
use crate::TraitStore;
use crate::constraints::{
    Constraint, Constraints, Location, MapKey, MapValue, RunningConstraintInner,
    TraitObjConstraint, TraitObjTy, VOID,
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
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        destty: &Ty,
        to_convert: &Rvalue,
    ) -> Constraints {
        match to_convert {
            Rvalue::Use(op) => self.convert_op(istore, span, local_decls, cur_scope, op, destty),
            Rvalue::Ref(_region, _borrow_kind, place) => {
                self.convert_place(istore, span, local_decls, cur_scope, place, destty)
            }
            Rvalue::Discriminant(place) => {
                self.convert_place(istore, span, local_decls, cur_scope, place, destty)
            }
            Rvalue::CopyForDeref(place) => {
                self.convert_place(istore, span, local_decls, cur_scope, place, destty)
            }
            Rvalue::Cast(kind, op, ty) => {
                self.convert_cast(istore, span, local_decls, cur_scope, kind, op, ty)
            }
            Rvalue::Aggregate(kind, fields) => {
                self.convert_agg(istore, span, local_decls, cur_scope, destty, kind, fields)
            }
            Rvalue::AddressOf(_rawptrkind, place) => {
                self.convert_place(istore, span, local_decls, cur_scope, place, destty)
            }
            Rvalue::UnaryOp(unop, op) => {
                self.convert_unop(istore, span, local_decls, cur_scope, destty, unop, op)
            }
            Rvalue::BinaryOp(binop, op1, op2) => self.convert_binop(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                binop,
                op1,
                op2,
            ),
            Rvalue::CheckedBinaryOp(binop, op1, op2) => self.convert_binop(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                binop,
                op1,
                op2,
            ),
            Rvalue::Repeat(op, _tyconst) => {
                self.convert_op(istore, span, local_decls, cur_scope, op, destty)
            }
            _ => todo!("other rval: {:?}", to_convert),
        }
    }

    fn convert_op(
        &self,
        istore: &InterpStore,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        op: &Operand,
        destty: &Ty,
    ) -> Constraints {
        match op {
            Operand::Copy(place) | Operand::Move(place) => {
                self.convert_place(istore, span, local_decls, cur_scope, place, destty)
            }
            Operand::Constant(const_op) => self.convert_const(span, &const_op),
            _ => todo!("runtime checks"),
        }
    }

    pub fn convert_const(&self, span: &Location, const_op: &ConstOperand) -> Constraints {
        debug!("CONVERTING CONST");
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
                                    Some((span.clone(), RunningConstraintInner::Scalar(Some(val)))),
                                )]
                            }
                            _ => {
                                let (maybe_traitobj, ty) =
                                    self.convert_ty(span, &const_op.const_.ty());
                                if maybe_traitobj.is_some() {
                                    todo!("const contains dyn");
                                }
                                vec![ty]
                            }
                        },
                        _ => todo!(),
                    }
                }
                _ => {
                    let (maybe_traitobj, ty) = self.convert_ty(span, &const_op.const_.ty());
                    if maybe_traitobj.is_some() {
                        todo!("const contains dyn");
                    }
                    vec![ty]
                }
            },
            ConstantKind::ZeroSized => {
                let (maybe_traitobj, ty) = self.convert_ty(span, &const_op.const_.ty());
                if maybe_traitobj.is_some() {
                    todo!("const contains dyn");
                }
                vec![ty]
            }
            other @ _ => todo!("arg is another constant kind: {:?}", other),
        }
    }

    fn convert_place(
        &self,
        istore: &InterpStore,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        place: &Place,
        destty: &Ty,
    ) -> Constraints {
        debug!("current place ty: {:?}", place.ty(local_decls).unwrap());

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
                let (maybe_traitobj, ty) = self.convert_ty(span, destty);
                if maybe_traitobj.is_some() {
                    todo!("place ty contains dyn");
                }
                vec![ty]
            }
        }
    }

    fn convert_cast(
        &self,
        istore: &InterpStore,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        _kind: &CastKind,
        op: &Operand,
        ty: &Ty,
    ) -> Constraints {
        match op {
            Operand::Constant(const_op) => {
                let (maybe_traitobj, ty) = self.convert_ty(span, &const_op.const_.ty());
                if maybe_traitobj.is_some() {
                    todo!("cast const contains dyn");
                }
                vec![ty]
            }
            Operand::Copy(place) | Operand::Move(place) => {
                debug!("CASTING existing place");
                let prev_constraints =
                    self.convert_place(istore, span, local_decls, cur_scope, place, ty);
                debug!("PRE CAST constraints: {:?}", prev_constraints);
                let (maybe_traitobj, post_ty) = self.convert_ty(span, ty);
                debug!("POST CAST ty: {:?}", post_ty);

                if maybe_traitobj.is_some() {
                    debug!("maybe_traitobj: {:?}", maybe_traitobj);
                    todo!("dest ty is traitobj");
                }

                vec![post_ty]
            }
            _ => todo!("runtime checks"),
        }
    }

    pub fn get_any_traitobj(
        &self,
        maybe_trait_ty: &Option<Vec<TraitObjTy>>,
        constraint: &Constraint,
    ) -> Option<TraitObjConstraint> {
        match constraint {
            Constraint { toc: Some(to_), .. } => {
                debug!("PROPAGATING TOC: {:?}", to_);
                return Some(to_.clone());
            }
            Constraint {
                toc: None,
                cfc: Some((_, maybe_to)),
            } => {
                debug!("MAYBE PULL TOC from {:?}", maybe_to);
                match maybe_to {
                    RunningConstraintInner::Adt(adtdef, adt_genargs) => {
                        debug!("ADT (adtdef): {:?}", adtdef);

                        // If we get Some, that means this struct/adt implements one or more
                        // traits, but that does _not_ mean that this is a trait object
                        match self.tstore.struct_traits.get(&adtdef.0) {
                            Some(_possible_traits) => {
                                // Once we know we are storing the result of this rval into a
                                // traitobj, only _then_ can we populate the traitobj constraint field
                                if maybe_trait_ty.is_some() {
                                    debug!("SETTING TOC: ({:?}, {:?})", adtdef, adt_genargs);
                                    return Some(TraitObjConstraint::Adt(
                                        adtdef.clone(),
                                        adt_genargs.clone(),
                                    ));
                                }
                            }
                            _ => debug!("no possible traits?"),
                        }
                    }
                    RunningConstraintInner::Closure(cdef, genargs) => {
                        // This case is expected if the traits in maybe_trait_ty are one of: Fn, FnMut, FnOnce
                        debug!("CLOSURE (cdef): {:?}", cdef);

                        if maybe_trait_ty.is_some() {
                            debug!("SETTING TOC: ({:?}, {:?})", cdef, genargs);
                            return Some(TraitObjConstraint::Closure(
                                cdef.clone(),
                                genargs.clone(),
                            ));
                        }
                    }
                    _ => debug!("another CFC kind"),
                }
            }
            _ => debug!("CFC is NONE"),
        }

        None
    }

    /*
    fn contains_traitobj(
        &self,
        maybe_trait_destty: &Option<Vec<TraitObjTy>>,
        //def: &AdtDef,
        genargs: &Vec<Constraint>,
    ) -> Option<TraitObjConstraint> {
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

        to
    }

    fn contains_controlflow(
        &self,
        _def: &AdtDef,
        genargs: &Vec<Constraint>,
    ) -> Option<RunningConstraint> {
        // TODO check def for controlflow

        // check genargs for controlflow
        let mut cf = None;
        for genarg in genargs {
            match genarg {
                Constraint {
                    toc: _,
                    cfc: Some((span, cf_)),
                } => {
                    cf = Some((span.clone(), cf_.clone()));
                    break;
                }
                _ => {}
            }
        }

        cf
    }
    */

    fn convert_agg(
        &self,
        istore: &InterpStore,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        destty: &Ty,
        kind: &AggregateKind,
        fields: &Vec<Operand>,
    ) -> Constraints {
        match kind {
            AggregateKind::Adt(def, _variant_idx, genargs, _, _) => {
                debug!("ADT agg");
                debug!("ty: {:?}", def.ty_with_args(genargs));

                vec![Constraint::new(
                    None,
                    Some((
                        span.clone(),
                        RunningConstraintInner::Adt(*def, genargs.clone()),
                    )),
                )]
            }
            AggregateKind::Tuple => {
                debug!("tuple agg");
                let mut inner_constraints = Vec::new();
                for op in fields {
                    unique_append(
                        &mut inner_constraints,
                        self.convert_op(istore, span, local_decls, cur_scope, op, destty),
                    );
                }
                vec![Constraint::new(
                    None,
                    Some((
                        span.clone(),
                        RunningConstraintInner::Tuple(inner_constraints),
                    )),
                )]
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

                let (maybe_traitobj, converted_ty) = self.convert_ty(span, ty);
                if maybe_traitobj.is_some() {
                    todo!("rawptr contains dyn");
                }

                vec![Constraint::new(
                    None,
                    Some((
                        span.clone(),
                        RunningConstraintInner::Ptr(Box::new(converted_ty)),
                    )),
                )]
            }
            AggregateKind::Array(ty) => {
                debug!("array agg");
                let (maybe_traitobj, converted_ty) = self.convert_ty(span, ty);
                if maybe_traitobj.is_some() {
                    todo!("array contains dyn");
                }
                vec![Constraint::new(
                    None,
                    Some((
                        span.clone(),
                        RunningConstraintInner::List(Box::new(converted_ty)),
                    )),
                )]
            }
            AggregateKind::Closure(def, genargs) => {
                debug!("closure agg");
                vec![Constraint::new(
                    None,
                    Some((
                        span.clone(),
                        RunningConstraintInner::Closure(*def, genargs.clone()),
                    )),
                )]
            }
            _ => todo!("other agg kind: {:?}", kind),
        }
    }

    /*
    fn convert_genargs(&self, span: &Location, genargs: &GenericArgs) -> Option<Vec<Constraint>> {
        if genargs.0.is_empty() {
            return None;
        }
        let mut converted_genargs = Vec::new();
        for genarg in &genargs.0 {
            match self.convert_genarg(span, genarg) {
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
    */

    pub fn convert_genarg(&self, span: &Location, genarg: &GenericArgKind) -> Option<Constraint> {
        match genarg {
            GenericArgKind::Type(ty) => {
                let (maybe_traitobj, converted_ty) = self.convert_ty(span, ty);
                if maybe_traitobj.is_some() {
                    todo!("genarg contains dyn");
                }
                Some(converted_ty)
            }
            _ => None,
        }
    }

    pub fn convert_ty(&self, span: &Location, ty: &Ty) -> (Option<Vec<TraitObjTy>>, Constraint) {
        debug!("CONVERTING TY");
        debug!("ty: {:?}", ty);

        match ty.kind() {
            TyKind::RigidTy(rigidty) => match rigidty {
                RigidTy::Bool | RigidTy::Int(_) | RigidTy::Uint(_) => (
                    None,
                    Constraint::new(
                        None,
                        Some((span.clone(), RunningConstraintInner::Scalar(None))),
                    ),
                ),
                RigidTy::Adt(def, genargs) => (
                    None,
                    Constraint::new(
                        None,
                        Some((span.clone(), RunningConstraintInner::Adt(def, genargs))),
                    ),
                ),
                RigidTy::Tuple(ty_vec) => {
                    let mut inner = Vec::new();
                    let mut traitobj_vec = Vec::new();
                    for ty in ty_vec {
                        // FIXME
                        let (maybe_traitobj, constraint) = self.convert_ty(span, &ty);
                        unique_push(&mut inner, constraint);
                        match maybe_traitobj {
                            Some(to) => unique_append(&mut traitobj_vec, to),
                            _ => {}
                        }
                    }
                    (
                        Some(traitobj_vec),
                        Constraint::new(
                            None,
                            Some((span.clone(), RunningConstraintInner::Idk(Box::new(inner)))),
                        ),
                    )
                }
                RigidTy::Array(ty, _) | RigidTy::Slice(ty) => {
                    let (maybe_traitobj, constraint) = self.convert_ty(span, &ty);
                    (
                        maybe_traitobj,
                        Constraint::new(
                            None,
                            Some((
                                span.clone(),
                                RunningConstraintInner::Idk(Box::new(vec![constraint])),
                            )),
                        ),
                    )
                }
                RigidTy::Closure(def, genargs) => (
                    None,
                    Constraint::new(
                        None,
                        Some((span.clone(), RunningConstraintInner::Closure(def, genargs))),
                    ),
                ),
                RigidTy::FnDef(def, genargs) => (
                    None,
                    Constraint::new(
                        None,
                        Some((span.clone(), RunningConstraintInner::FnDef(def, genargs))),
                    ),
                ),
                RigidTy::FnPtr(poly_fn_sig) => {
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

                    (
                        None,
                        Constraint::new(
                            None,
                            Some((span.clone(), RunningConstraintInner::FnPtr(sigval))),
                        ),
                    )
                }
                RigidTy::Ref(_, ty, _) => self.convert_ty(span, &ty),
                RigidTy::RawPtr(ty, _mut) => self.convert_ty(span, &ty),
                RigidTy::Char | RigidTy::Str | RigidTy::Never => {
                    (None, Constraint::new(None, None))
                }
                RigidTy::Dynamic(bound_existentials, _) => {
                    debug!("SETTING INTO DYNAMIC");
                    let mut traitobj_vec = Vec::new();
                    for bound_existential in bound_existentials {
                        unique_push(
                            &mut traitobj_vec,
                            TraitObjTy::new_from_bound_existential(&bound_existential),
                        );
                    }
                    (
                        Some(traitobj_vec.clone()),
                        Constraint::new(
                            None,
                            Some((span.clone(), RunningConstraintInner::Dynamic(traitobj_vec))),
                        ),
                    )
                }
                other @ _ => panic!("other rigidty: {:?}", other),
            },
            other @ _ => panic!("other ty kind: {:?}", other),
        }
    }

    fn convert_binop(
        &self,
        istore: &InterpStore,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        destty: &Ty,
        binop: &BinOp,
        op1: &Operand,
        op2: &Operand,
    ) -> Constraints {
        let constraint = match binop {
            BinOp::Add | BinOp::AddUnchecked => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x + y,
            ),
            BinOp::Sub | BinOp::SubUnchecked => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x - y,
            ),
            BinOp::Mul | BinOp::MulUnchecked => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x * y,
            ),
            BinOp::Div => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x / y,
            ),
            BinOp::Rem => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x % y,
            ),
            BinOp::Eq => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| {
                    if x == y { 1 } else { 0 }
                },
            ),
            BinOp::Lt => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| {
                    if x < y { 1 } else { 0 }
                },
            ),
            BinOp::Le => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| {
                    if x <= y { 1 } else { 0 }
                },
            ),
            BinOp::Ne => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| {
                    if x != y { 1 } else { 0 }
                },
            ),
            BinOp::Ge => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| {
                    if x >= y { 1 } else { 0 }
                },
            ),
            BinOp::Gt => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| {
                    if x > y { 1 } else { 0 }
                },
            ),
            // bit-level binops
            BinOp::Shl | BinOp::ShlUnchecked => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x << y,
            ),
            BinOp::Shr | BinOp::ShrUnchecked => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x >> y,
            ),
            BinOp::BitAnd => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x & y,
            ),
            BinOp::BitOr => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x | y,
            ),
            BinOp::BitXor => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x ^ y,
            ),
            // This binop return Ord results
            BinOp::Cmp => self.convert_binop_helper(
                istore,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| {
                    if x < y {
                        -1
                    } else if x > y {
                        1
                    } else {
                        0
                    }
                },
            ),
            // TODO
            BinOp::Offset => Constraint::new(
                None,
                Some((span.clone(), RunningConstraintInner::Idk(Box::new(vec![])))),
            ),
        };

        vec![constraint]
    }

    fn convert_binop_helper(
        &self,
        istore: &InterpStore,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        destty: &Ty,
        op1: &Operand,
        op2: &Operand,
        f: fn(i128, i128) -> i128,
    ) -> Constraint {
        let c_op1 = self.convert_op(istore, span, local_decls, cur_scope, op1, destty);
        let c_op2 = self.convert_op(istore, span, local_decls, cur_scope, op2, destty);
        if c_op1.len() != 1 || c_op2.len() != 1 {
            return Constraint::new(
                None,
                Some((span.clone(), RunningConstraintInner::Scalar(None))),
            );
        }
        match (c_op1[0].clone(), c_op2[0].clone()) {
            (
                Constraint {
                    toc: None,
                    cfc: Some((_, RunningConstraintInner::Scalar(Some(val1)))),
                },
                Constraint {
                    toc: None,
                    cfc: Some((_, RunningConstraintInner::Scalar(Some(val2)))),
                },
            ) => Constraint::new(
                None,
                Some((
                    span.clone(),
                    RunningConstraintInner::Scalar(Some(f(val1, val2))),
                )),
            ),
            (
                Constraint {
                    toc: None,
                    cfc: Some((_, RunningConstraintInner::Scalar(Some(val1)))),
                },
                Constraint {
                    toc: to,
                    cfc: Some((_, RunningConstraintInner::Scalar(Some(val2)))),
                },
            ) => Constraint::new(
                to,
                Some((
                    span.clone(),
                    RunningConstraintInner::Scalar(Some(f(val1, val2))),
                )),
            ),
            (
                Constraint {
                    toc: to,
                    cfc: Some((_, RunningConstraintInner::Scalar(Some(val1)))),
                },
                Constraint {
                    toc: None,
                    cfc: Some((_, RunningConstraintInner::Scalar(Some(val2)))),
                },
            ) => Constraint::new(
                to,
                Some((
                    span.clone(),
                    RunningConstraintInner::Scalar(Some(f(val1, val2))),
                )),
            ),
            (
                Constraint {
                    toc: _to1,
                    cfc: Some((_, RunningConstraintInner::Scalar(Some(_val1)))),
                },
                Constraint {
                    toc: _to2,
                    cfc: Some((_, RunningConstraintInner::Scalar(Some(_val2)))),
                },
            ) => {
                todo!();
                //(to, Some(RunningConstraintInner::Scalar(Some(f(
                //    val1, val2,
                //)))))
            }
            other @ _ => {
                debug!("BINOP HELPER OTHER: {:?}", other);
                Constraint::new(
                    None,
                    Some((span.clone(), RunningConstraintInner::Scalar(None))),
                )
            }
        }
    }

    fn convert_unop(
        &self,
        istore: &InterpStore,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        destty: &Ty,
        unop: &UnOp,
        op: &Operand,
    ) -> Constraints {
        let constraint = match unop {
            UnOp::Neg => {
                self.convert_unop_helper(istore, span, local_decls, cur_scope, destty, op, |x| -x)
            }
            UnOp::Not => {
                self.convert_unop_helper(istore, span, local_decls, cur_scope, destty, op, |x| !x)
            }
            UnOp::PtrMetadata => Constraint::new(
                None,
                Some((span.clone(), RunningConstraintInner::Idk(Box::new(vec![])))),
            ),
        };

        vec![constraint]
    }

    fn convert_unop_helper(
        &self,
        istore: &InterpStore,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        destty: &Ty,
        op: &Operand,
        f: fn(i128) -> i128,
    ) -> Constraint {
        let c_op = self.convert_op(istore, span, local_decls, cur_scope, op, destty);
        if c_op.len() != 1 {
            return Constraint::new(
                None,
                Some((span.clone(), RunningConstraintInner::Scalar(None))),
            );
        }
        match c_op[0].clone() {
            Constraint {
                toc: to,
                cfc: Some((_, RunningConstraintInner::Scalar(Some(val)))),
            } => Constraint::new(
                to,
                Some((span.clone(), RunningConstraintInner::Scalar(Some(f(val))))),
            ),
            Constraint { toc: to, cfc: _ } => Constraint::new(
                to,
                Some((span.clone(), RunningConstraintInner::Scalar(None))),
            ), //_ => Constraint::ControlFlow(Box::new(RunningConstraintInner::Scalar(None))),
        }
    }
}
