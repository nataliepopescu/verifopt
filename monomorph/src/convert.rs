use rustc_public::mir::{
    AggregateKind, BinOp, CastKind, ConstOperand, LocalDecl, Operand, Place, ProjectionElem,
    Rvalue, UnOp,
};
use rustc_public::ty::{ConstantKind, GenericArgKind, RigidTy, Ty, TyKind};

use crate::InterpStore;
use crate::TraitStore;
use crate::constraints::{
    ADTFields, Constraint, Constraints, Location, MapKey, MapValue, RunningConstraintInner,
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
    ) -> (Constraints, Option<ADTFields>) {
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
            Rvalue::Aggregate(kind, ops) => {
                self.convert_agg(istore, span, local_decls, cur_scope, destty, kind, ops)
            }
            Rvalue::AddressOf(_rawptrkind, place) => {
                self.convert_place(istore, span, local_decls, cur_scope, place, destty)
            }
            Rvalue::UnaryOp(unop, op) => (
                self.convert_unop(istore, span, local_decls, cur_scope, destty, unop, op),
                None,
            ),
            Rvalue::BinaryOp(binop, op1, op2) => (
                self.convert_binop(
                    istore,
                    span,
                    local_decls,
                    cur_scope,
                    destty,
                    binop,
                    op1,
                    op2,
                ),
                None,
            ),
            Rvalue::CheckedBinaryOp(binop, op1, op2) => (
                self.convert_binop(
                    istore,
                    span,
                    local_decls,
                    cur_scope,
                    destty,
                    binop,
                    op1,
                    op2,
                ),
                None,
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
    ) -> (Constraints, Option<ADTFields>) {
        match op {
            Operand::Copy(place) | Operand::Move(place) => {
                self.convert_place(istore, span, local_decls, cur_scope, place, destty)
            }
            Operand::Constant(const_op) => (self.convert_const(span, &const_op), None),
            _ => todo!("runtime checks"),
        }
    }

    pub fn convert_const(&self, span: &Location, const_op: &ConstOperand) -> Constraints {
        //debug!("CONVERTING CONST");
        match const_op.const_.kind() {
            ConstantKind::Allocated(alloc) => match alloc.read_int() {
                // FIXME
                Ok(val) => {
                    //debug!("ALLOC CONST");
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
                                let (_maybe_traitobjty, constraint) =
                                    self.convert_ty(span, &const_op.const_.ty());
                                vec![constraint]
                            }
                        },
                        _ => todo!(),
                    }
                }
                _ => {
                    let (maybe_traitobj, constraint) = self.convert_ty(span, &const_op.const_.ty());
                    if maybe_traitobj.is_some() {
                        todo!("const contains dyn");
                    }
                    vec![constraint]
                }
            },
            ConstantKind::ZeroSized => {
                let (maybe_traitobj, constraint) = self.convert_ty(span, &const_op.const_.ty());
                if let Some(_traitobjty) = maybe_traitobj {
                    //debug!("traitobjty: {:?}", traitobjty);
                    todo!("const contains dyn");
                }
                vec![constraint]
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
    ) -> (Constraints, Option<ADTFields>) {
        //debug!("current place ty: {:?}", place.ty(local_decls).unwrap());
        // TODO use current place ty instead of *just* getting existing place constraints

        debug!("CONVERTING PLACE");
        match istore.scoped_get(cur_scope, &MapKey::Var(place.clone()), false) {
            Some(val) => match val {
                MapValue::Constraints(constraints) => {
                    debug!("found constraints for place {:?}: {:?}", place, constraints);
                    debug!("checking field projection constraints.....");

                    // FIXME implementation is similar to interp::resolve_arg()
                    match istore.field_map.get(&(place.clone(), cur_scope.clone())) {
                        Some(field_projections) => {
                            debug!("\n--FIELD projections: {:?}", field_projections);

                            let mut fields = Vec::new();
                            for field_proj in field_projections {
                                let field_ty = match field_proj {
                                    ProjectionElem::Field(_, ty) => ty,
                                    _ => panic!("unexpected proj elem: {:?}", field_proj),
                                };
                                debug!("\nfield_proj: {:?}", field_proj);
                                let proj = vec![ProjectionElem::Deref, field_proj.clone()];
                                let field_place = Place {
                                    local: place.local,
                                    projection: proj.clone(),
                                };
                                // get each field constraints
                                let (field_constraints, field_fields) = self.convert_place(
                                    istore,
                                    span,
                                    local_decls,
                                    cur_scope,
                                    &field_place,
                                    field_ty,
                                );
                                debug!("[ConvertPlace] field_constraints: {:?}", field_constraints);
                                if field_fields.is_some() {
                                    todo!();
                                }

                                // push into vec
                                fields.push((
                                    // full projection
                                    proj,
                                    // field constraints
                                    field_constraints,
                                ))
                            }
                            debug!("\n--DONE FIELD projections: {:?}\n", field_projections);
                            if fields.is_empty() {
                                (constraints, None)
                            } else {
                                (constraints, Some(fields))
                            }
                        }
                        None => {
                            debug!("NO FIELD CONSTRAINTS");
                            (constraints, None)
                        }
                    }
                }
                _ => panic!("value should not be a scope"),
            },
            None => {
                debug!("place {:?} has not been set, widen to type", place);
                for proj in &place.projection {
                    debug!("PROJ: {:?}", proj);
                }
                let (_maybe_traitobj, constraint) = self.convert_ty(span, destty);
                //debug!("constraint (from ty): {:?}", constraint);
                //if let Some(traitobj) = maybe_traitobj {
                //    todo!("place ty contains dyn {:?}", traitobj);
                //}
                (vec![constraint], None)
            }
        }
    }

    fn convert_cfc_to_toc(&self, cfc: &RunningConstraintInner) -> TraitObjConstraint {
        match cfc {
            RunningConstraintInner::Adt(adtdef, genargs) => {
                TraitObjConstraint::Adt(*adtdef, genargs.clone())
            }
            RunningConstraintInner::Closure(cdef, genargs) => {
                TraitObjConstraint::Closure(*cdef, genargs.clone())
            }
            _ => panic!("unexpected cfc: {:?}", cfc),
        }
    }

    fn convert_cast_helper(
        &self,
        traitobjtys: &Vec<TraitObjTy>,
        constraints: &Constraints,
    ) -> Constraints {
        let mut new_constraints = Vec::new();

        //debug!("\ntraitobjtys: {:?}", traitobjtys);
        for traitobjty in traitobjtys {
            for constraint in constraints {
                //debug!("\ntraitobjty: {:?}", traitobjty);
                //debug!("constraint: {:?}", constraint);
                match constraint {
                    Constraint {
                        toc: Some(_toc_), ..
                    } => {
                        // Replace any Dynamic(TraitObjTy) with any TOC from constraints
                        todo!();
                    }
                    Constraint {
                        toc: None,
                        cfc: Some(cfc_),
                    } => {
                        // If no TOC but CFC exists, pull any CFC constraints that
                        // could be a traitobj for this traitobjty
                        let defid;
                        match &cfc_.1 {
                            RunningConstraintInner::Adt(adtdef, _) => defid = adtdef.0,
                            RunningConstraintInner::Closure(cdef, _) => defid = cdef.0,
                            // FIXME this is a jumbled mess
                            RunningConstraintInner::Idk(inner) => {
                                if inner.len() > 1 {
                                    todo!();
                                }
                                match &inner[0].cfc.as_ref().unwrap().1 {
                                    RunningConstraintInner::Dynamic(tot_vec) => {
                                        if tot_vec.len() > 1 {
                                            todo!();
                                        }
                                        defid = tot_vec[0].def.0;
                                    }
                                    _ => todo!(),
                                }
                            }
                            _ => todo!("cfc: {:?}", cfc_.1),
                        }
                        //debug!("DEFID: {:?}", defid);

                        match self.tstore.struct_traits.get(&defid) {
                            Some(traits) => {
                                //debug!("found traits");
                                if traits.contains(&traitobjty.def.0) {
                                    // pull relevant CFC into TOC
                                    let new_constraint = Constraint::new(
                                        Some((
                                            traitobjty.clone(),
                                            self.convert_cfc_to_toc(&cfc_.1),
                                        )),
                                        Some(cfc_.clone()),
                                    );
                                    unique_push(&mut new_constraints, new_constraint);
                                } else {
                                    // push old constraint unchanged
                                    unique_push(&mut new_constraints, constraint.clone());
                                }
                            }
                            None => {
                                //debug!("did NOT find traits in store");
                                // Is the trait one of Fn/FnMut/FnOnce? And is the current
                                // constraint a closure? If so, these traits are implicitly
                                // implemented and won't exist in our trait_store
                                //debug!("\ntraitobjty: {:?}", traitobjty);
                                //debug!("constraint: {:?}", constraint);
                                if constraint.is_cfc_closure() && traitobjty.is_fn_trait() {
                                    //debug!("fn trait / closure");
                                    // pull relevant CFC into TOC
                                    let new_constraint = Constraint::new(
                                        Some((
                                            traitobjty.clone(),
                                            self.convert_cfc_to_toc(&cfc_.1),
                                        )),
                                        Some(cfc_.clone()),
                                    );
                                    unique_push(&mut new_constraints, new_constraint);
                                } else {
                                    //debug!("NOT fn trait / closure");
                                }
                            }
                        }
                    }
                    _ => {
                        // push old constraint unchanged
                        unique_push(&mut new_constraints, constraint.clone());
                    }
                }
            }
        }

        //debug!("\nNEW CONSTRAINTS: {:?}", new_constraints);

        new_constraints
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
    ) -> (Constraints, Option<ADTFields>) {
        match op {
            Operand::Constant(const_op) => {
                let (maybe_traitobj, constraint) = self.convert_ty(span, &const_op.const_.ty());
                if maybe_traitobj.is_some() {
                    todo!("cast const contains dyn");
                }
                (vec![constraint], None)
            }
            Operand::Copy(place) | Operand::Move(place) => {
                //debug!("CASTING existing place");
                let (prev_constraints, maybe_fields) =
                    self.convert_place(istore, span, local_decls, cur_scope, place, ty);
                debug!("FIELDS in CAST? {:?}", maybe_fields);
                debug!("PRE CAST constraints: {:?}", prev_constraints);
                let (maybe_traitobj, post_constraint) = self.convert_ty(span, ty);
                debug!("POST CAST ty: {:?}", post_constraint);

                if let Some(traitobjtys) = maybe_traitobj {
                    (
                        self.convert_cast_helper(&traitobjtys, &prev_constraints),
                        maybe_fields,
                    )
                } else {
                    (vec![post_constraint], maybe_fields)
                }
            }
            _ => todo!("runtime checks"),
        }
    }

    pub fn get_any_traitobj(
        &self,
        maybe_trait_ty: &Option<Vec<TraitObjTy>>,
        constraint: &Constraint,
    ) -> Option<(TraitObjTy, TraitObjConstraint)> {
        //debug!("maybe_trait_ty: {:?}", maybe_trait_ty);
        match constraint {
            Constraint { toc: Some(to_), .. } => {
                //debug!("PROPAGATING TOC: {:?}", to_);
                return Some(to_.clone());
            }
            Constraint {
                toc: None,
                cfc: Some((_, maybe_to)),
            } => {
                //debug!("MAYBE PULL TOC from {:?}", maybe_to);
                match maybe_to {
                    RunningConstraintInner::Adt(adtdef, adt_genargs) => {
                        // If we get Some, that means this struct/adt implements one or more
                        // traits, but that does _not_ mean that this is a trait object
                        match self.tstore.struct_traits.get(&adtdef.0) {
                            Some(_possible_traits) => {
                                // Once we know we are storing the result of this rval into a
                                // traitobj, only _then_ can we populate the traitobj constraint field
                                if let Some(trait_ty) = maybe_trait_ty {
                                    //debug!("ADT (adtdef): {:?}", adtdef);
                                    //debug!("SETTING TOC: ({:?}, {:?})", adtdef, adt_genargs);
                                    //debug!("trait_ty: {:?}", trait_ty);
                                    if trait_ty.len() > 1 {
                                        todo!();
                                    }

                                    return Some((
                                        trait_ty[0].clone(),
                                        TraitObjConstraint::Adt(
                                            adtdef.clone(),
                                            adt_genargs.clone(),
                                        ),
                                    ));
                                }
                            }
                            _ => {} //debug!("no possible traits?"),
                        }
                    }
                    RunningConstraintInner::Closure(cdef, genargs) => {
                        // This case is expected if the traits in maybe_trait_ty are one of: Fn, FnMut, FnOnce

                        if let Some(trait_ty) = maybe_trait_ty {
                            //debug!("CLOSURE (cdef): {:?}", cdef);
                            //debug!("SETTING TOC: ({:?}, {:?})", cdef, genargs);
                            //debug!("trait_ty: {:?}", trait_ty);
                            if trait_ty.len() > 1 {
                                todo!();
                            }

                            return Some((
                                trait_ty[0].clone(),
                                TraitObjConstraint::Closure(cdef.clone(), genargs.clone()),
                            ));
                        }
                    }
                    _ => {} //debug!("another CFC kind"),
                }
            }
            _ => {} //debug!("CFC is NONE"),
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
        ops: &Vec<Operand>,
    ) -> (Constraints, Option<ADTFields>) {
        match kind {
            AggregateKind::Adt(def, _variant_idx, genargs, _, field_idx) => {
                debug!("ADT agg");
                debug!("ops: {:?}", ops);
                debug!("destty: {:?}", destty);
                debug!("field_idx: {:?}", field_idx);
                let ty = def.ty_with_args(genargs);
                debug!("ty: {:?}", ty);

                // Create projections here to simulate field initializers
                let mut fields = Vec::new();
                for (i, op) in ops.into_iter().enumerate() {
                    debug!("\n---op {:?}", i);
                    let (op_constraints, maybe_fields) =
                        self.convert_op(istore, span, local_decls, cur_scope, op, destty);
                    debug!("op constraints: {:?}", op_constraints);
                    // FIXME maybe_fields constraints are dropped
                    debug!("maybe_fields: {:?}", maybe_fields);

                    let op_ty;
                    match op {
                        Operand::Copy(place) | Operand::Move(place) => {
                            op_ty = place.ty(local_decls).unwrap();
                        }
                        Operand::Constant(co) => {
                            op_ty = co.const_.ty();
                        }
                        _ => todo!("op: {:?}", op),
                    }

                    let proj = vec![ProjectionElem::Deref, ProjectionElem::Field(i, op_ty)];
                    debug!("PROJ: {:?}", proj);
                    fields.push((
                        // obj deref + field access
                        proj,
                        // field constraints
                        op_constraints,
                    ));
                    debug!("---done op {:?}\n", i);
                }

                (
                    vec![Constraint::new(
                        None,
                        Some((
                            span.clone(),
                            RunningConstraintInner::Adt(*def, genargs.clone()),
                        )),
                    )],
                    Some(fields),
                )
            }
            AggregateKind::Tuple => {
                //debug!("tuple agg");
                let mut inner_constraints = Vec::new();
                for op in ops {
                    let (op_constraints, maybe_fields) =
                        self.convert_op(istore, span, local_decls, cur_scope, op, destty);
                    debug!("op_constraints: {:?}", op_constraints.clone());
                    unique_append(&mut inner_constraints, op_constraints);
                    if let Some(fields) = maybe_fields {
                        // FIXME
                        debug!("TUPLE fields: {:?}", fields);
                    }
                }
                (
                    vec![Constraint::new(
                        None,
                        Some((
                            span.clone(),
                            RunningConstraintInner::Tuple(inner_constraints),
                        )),
                    )],
                    None,
                )
            }
            AggregateKind::RawPtr(ty, _mut) => {
                //debug!("rawptr agg");
                //debug!("ty: {:?}", ty);

                match ops.len() {
                    0 => todo!("no operands"),
                    1 => todo!("thin ptr (1 operand)"),
                    2 => {} //debug!("fat ptr (2 operands)"),
                    _ => todo!("more than 2 operands"),
                }

                let (maybe_traitobj, constraint) = self.convert_ty(span, ty);
                if maybe_traitobj.is_some() {
                    todo!("rawptr contains dyn");
                }

                (
                    vec![Constraint::new(
                        None,
                        Some((
                            span.clone(),
                            RunningConstraintInner::Ptr(Box::new(constraint)),
                        )),
                    )],
                    None,
                )
            }
            AggregateKind::Array(ty) => {
                //debug!("array agg");
                let (maybe_traitobj, constraint) = self.convert_ty(span, ty);
                if maybe_traitobj.is_some() {
                    todo!("array contains dyn");
                }
                (
                    vec![Constraint::new(
                        None,
                        Some((
                            span.clone(),
                            RunningConstraintInner::List(Box::new(constraint)),
                        )),
                    )],
                    None,
                )
            }
            AggregateKind::Closure(def, genargs) => {
                //debug!("closure agg");
                (
                    vec![Constraint::new(
                        None,
                        Some((
                            span.clone(),
                            RunningConstraintInner::Closure(*def, genargs.clone()),
                        )),
                    )],
                    None,
                )
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
                let (maybe_traitobj, constraint) = self.convert_ty(span, ty);
                if maybe_traitobj.is_some() {
                    todo!("genarg contains dyn");
                }
                Some(constraint)
            }
            _ => None,
        }
    }

    pub fn convert_ty(&self, span: &Location, ty: &Ty) -> (Option<Vec<TraitObjTy>>, Constraint) {
        //debug!("CONVERTING TY");
        //debug!("ty: {:?}", ty);

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

                    let maybe_traitobjty = if traitobj_vec.is_empty() {
                        None
                    } else {
                        Some(traitobj_vec)
                    };

                    (
                        maybe_traitobjty,
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
                    //debug!("SETTING INTO DYNAMIC");
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
        let (c_op1, _) = self.convert_op(istore, span, local_decls, cur_scope, op1, destty);
        let (c_op2, _) = self.convert_op(istore, span, local_decls, cur_scope, op2, destty);
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
            _ => Constraint::new(
                None,
                Some((span.clone(), RunningConstraintInner::Scalar(None))),
            ),
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
        let (c_op, _) = self.convert_op(istore, span, local_decls, cur_scope, op, destty);
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
