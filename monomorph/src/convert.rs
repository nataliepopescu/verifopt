use rustc_public::mir::{
    AggregateKind, BinOp, CastKind, ConstOperand, LocalDecl, Operand, Place, ProjectionElem,
    Rvalue, UnOp,
};
use rustc_public::ty::{AdtDef, Allocation, ConstantKind, GenericArgKind, RigidTy, Ty, TyKind};
use rustc_public::{CrateDef, DefId};

use crate::InterpStore;
use crate::TraitStore;
use crate::constraints::{
    ADTFields, Constraint, Constraints, Location, MapKey, MapValue, RunningConstraint,
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

    pub fn convert_op(
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
        debug!("CONVERTING CONST");
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
                                    Some(RunningConstraint::Scalar(Some(val))),
                                )]
                            }
                            _ => {
                                let (maybe_traitobjty, constraint) =
                                    self.convert_ty(span, &const_op.const_.ty());
                                debug!("HERE");
                                debug!("constraint: {:?}", constraint);
                                debug!("maybe_traitobjty: {:?}", maybe_traitobjty);
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

        //debug!("CONVERTING PLACE");
        match istore.scoped_get(cur_scope, &MapKey::Var(place.clone()), false) {
            Some(val) => match val {
                MapValue::Constraints(constraints) => {
                    debug!("found constraints for place {:?}: {:?}", place, constraints);
                    debug!("checking field projection constraints.....");

                    // FIXME implementation is similar to interp::resolve_arg()
                    match istore.field_map.get(&(place.clone(), cur_scope.clone())) {
                        Some(field_places) => {
                            debug!("\n--FIELD projections: {:?}", field_places);

                            let mut fields = Vec::new();
                            for field_place in field_places {
                                debug!("field_place: {:?}", field_place);
                                debug!("projs: {:?}", field_place.projection);
                                let field_ty = match field_place.projection
                                    [field_place.projection.len() - 1]
                                {
                                    ProjectionElem::Field(_, ty) => ty,
                                    _ => todo!(),
                                };

                                // get each field constraints
                                let (field_constraints, field_fields) = self.convert_place(
                                    istore,
                                    span,
                                    local_decls,
                                    cur_scope,
                                    &field_place,
                                    &field_ty,
                                );
                                //debug!("[ConvertPlace] field_constraints: {:?}", field_constraints);
                                if field_fields.is_some() {
                                    todo!();
                                }

                                // push into vec
                                fields.push((
                                    // full projection
                                    field_place.projection.clone(),
                                    // field constraints
                                    field_constraints,
                                ))
                            }
                            //debug!("\n--DONE FIELD places: {:?}\n", field_places);
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
                if place.projection.len() == 1 && place.projection[0] == ProjectionElem::Deref {
                    let base = Place {
                        local: place.local,
                        projection: vec![],
                    };

                    if let Some(MapValue::Constraints(cs)) =
                        istore.scoped_get(cur_scope, &MapKey::Var(base), false)
                    {
                        let deref: Constraints = cs
                            .into_iter()
                            .map(|c| match c {
                                Constraint {
                                    toc: _,
                                    cfc: Some(RunningConstraint::Ptr(inner)),
                                } => *inner,
                                other => other,
                            })
                            .collect();

                        return (deref, None);
                    }
                }

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

    fn convert_cfc_to_toc(&self, cfc: &RunningConstraint) -> TraitObjConstraint {
        match cfc {
            RunningConstraint::Adt(adtdef, genargs) => {
                TraitObjConstraint::Adt(*adtdef, genargs.clone())
            }
            RunningConstraint::Closure(cdef, genargs) => {
                TraitObjConstraint::Closure(*cdef, genargs.clone())
            }
            _ => panic!("unexpected cfc: {:?}", cfc),
        }
    }

    fn get_defid_from_cfc(&self, cfc: &RunningConstraint) -> DefId {
        match cfc {
            RunningConstraint::Adt(adtdef, _) => adtdef.0,
            RunningConstraint::Closure(cdef, _) => cdef.0,
            // FIXME this is a jumbled mess
            RunningConstraint::Idk(inner) => {
                if inner.len() > 1 {
                    todo!();
                }
                self.get_defid_from_cfc(&inner[0].cfc.as_ref().unwrap())
            }
            RunningConstraint::Dynamic(tot_vec) => {
                if tot_vec.len() > 1 {
                    todo!();
                }
                tot_vec[0].def.0
            }
            _ => todo!("cfc: {:?}", cfc),
        }
    }

    fn convert_cast_helper(
        &self,
        traitobjtys: &Vec<TraitObjTy>,
        constraints: &Constraints,
    ) -> Constraints {
        debug!("CAST HELPER");
        let mut new_constraints = Vec::new();

        debug!("traitobjtys: {:?}\n", traitobjtys);
        for traitobjty in traitobjtys {
            for constraint in constraints {
                debug!("\ntraitobjty: {:?}", traitobjty);
                debug!("constraint: {:?}", constraint);
                match constraint {
                    Constraint {
                        toc: Some(_), //(tot, toc)),
                        ..
                    } => {
                        //debug!("tot: {:?}", tot);
                        //debug!("toc: {:?}", toc);
                        debug!("HERE");

                        // Push old constraint unchanged
                        unique_push(&mut new_constraints, constraint.clone());
                    }
                    Constraint {
                        toc: None,
                        cfc: Some(cfc_),
                    } => {
                        // If no TOC but CFC exists, pull any CFC constraints that
                        // could be a traitobj for this traitobjty
                        let defid = self.get_defid_from_cfc(&cfc_);
                        debug!("DEFID: {:?}", defid);

                        match self.tstore.struct_traits.get(&defid) {
                            Some(traits) => {
                                debug!("found traits");
                                if traits.contains(&traitobjty.def.0) {
                                    // pull relevant CFC into TOC
                                    debug!("HERE1");
                                    let new_constraint = Constraint::new(
                                        Some((traitobjty.clone(), self.convert_cfc_to_toc(&cfc_))),
                                        Some(cfc_.clone()),
                                    );
                                    unique_push(&mut new_constraints, new_constraint);
                                } else {
                                    // push old constraint unchanged
                                    debug!("HERE2");
                                    unique_push(&mut new_constraints, constraint.clone());
                                }
                            }
                            None => {
                                // Is the trait one of Fn/FnMut/FnOnce? And is the current
                                // constraint a closure? If so, these traits are implicitly
                                // implemented and won't exist in our trait_store
                                if constraint.is_cfc_closure() && traitobjty.is_fn_trait() {
                                    // Pull relevant CFC into TOC
                                    debug!("HERE3");
                                    let new_constraint = Constraint::new(
                                        Some((traitobjty.clone(), self.convert_cfc_to_toc(&cfc_))),
                                        Some(cfc_.clone()),
                                    );
                                    unique_push(&mut new_constraints, new_constraint);
                                } else {
                                    todo!();
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
                debug!("CASTING existing place");
                debug!("place: {:?}", place);

                let (prev_constraints, maybe_fields) =
                    self.convert_place(istore, span, local_decls, cur_scope, place, ty);
                debug!("FIELDS in CAST? {:?}", maybe_fields);
                //if let Some(fields) = maybe_fields {
                //    todo!("fields: {:?}", fields);
                //}

                debug!("PRE CAST constraints: {:?}", prev_constraints);
                let (maybe_traitobj, post_constraint) = self.convert_ty(span, ty);
                debug!("POST CAST ty: {:?}", post_constraint);
                debug!("maybe_traitobj: {:?}", maybe_traitobj);

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
                cfc: Some(maybe_to),
            } => {
                //debug!("MAYBE PULL TOC from {:?}", maybe_to);
                match maybe_to {
                    RunningConstraint::Adt(adtdef, adt_genargs) => {
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
                    RunningConstraint::Closure(cdef, genargs) => {
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
        debug!("AGG kind: {:?}", kind);
        debug!("ops: {:?}", ops);
        debug!("destty: {:?}", destty);
        match kind {
            AggregateKind::Adt(def, _variant_idx, genargs, _, _field_idx) => {
                debug!("ADT agg");
                //debug!("field_idx: {:?}", field_idx);
                //let ty = def.ty_with_args(genargs);
                //debug!("ty: {:?}", ty);

                // Create projections here to simulate field initializers
                let mut fields = Vec::new();
                for (i, op) in ops.into_iter().enumerate() {
                    debug!("\n---op {:?}", i);
                    let (op_constraints, maybe_fields) =
                        self.convert_op(istore, span, local_decls, cur_scope, op, destty);
                    debug!("op constraints: {:?}", op_constraints);
                    // FIXME maybe_fields constraints are dropped (nested fields)
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
                        Some(RunningConstraint::Adt(*def, genargs.clone())),
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
                    //debug!("op_constraints: {:?}", op_constraints.clone());
                    unique_append(&mut inner_constraints, op_constraints);
                    if let Some(_fields) = maybe_fields {
                        // FIXME
                        //debug!("TUPLE fields: {:?}", fields);
                    }
                }
                (
                    vec![Constraint::new(
                        None,
                        Some(RunningConstraint::Tuple(inner_constraints)),
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
                        Some(RunningConstraint::Ptr(Box::new(constraint))),
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
                        Some(RunningConstraint::List(Box::new(constraint))),
                    )],
                    None,
                )
            }
            AggregateKind::Closure(def, genargs) => {
                //debug!("closure agg");
                (
                    vec![Constraint::new(
                        None,
                        Some(RunningConstraint::Closure(*def, genargs.clone())),
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
                    Constraint::new(None, Some(RunningConstraint::Scalar(None))),
                ),
                RigidTy::Float(_) => (None, Constraint::new(None, Some(RunningConstraint::Float))),
                RigidTy::Adt(def, genargs) => {
                    let mut traitobjtys = Vec::new();
                    for genarg in &genargs.0 {
                        match genarg {
                            GenericArgKind::Type(ty) => {
                                let (tot, _) = self.convert_ty(span, &ty);
                                match tot {
                                    Some(tot) => unique_append(&mut traitobjtys, tot),
                                    None => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    if traitobjtys.is_empty() {
                        debug!("NO TRAITOBJS");
                        (
                            None,
                            Constraint::new(None, Some(RunningConstraint::Adt(def, genargs))),
                        )
                    } else {
                        debug!("traitobjs!!!: {:?}", traitobjtys);
                        (
                            Some(traitobjtys),
                            Constraint::new(None, Some(RunningConstraint::Adt(def, genargs))),
                        )
                    }
                }
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
                        Constraint::new(None, Some(RunningConstraint::Idk(Box::new(inner)))),
                    )
                }
                RigidTy::Array(ty, _) | RigidTy::Slice(ty) => {
                    let (maybe_traitobj, constraint) = self.convert_ty(span, &ty);
                    (
                        maybe_traitobj,
                        Constraint::new(
                            None,
                            Some(RunningConstraint::Idk(Box::new(vec![constraint]))),
                        ),
                    )
                }
                RigidTy::Closure(def, genargs) => (
                    None,
                    Constraint::new(None, Some(RunningConstraint::Closure(def, genargs))),
                ),
                RigidTy::FnDef(def, genargs) => (
                    None,
                    Constraint::new(None, Some(RunningConstraint::FnDef(def, genargs))),
                ),
                RigidTy::FnPtr(poly_fn_sig) => {
                    let sigval = SigVal::new_from_poly(&poly_fn_sig);

                    (
                        None,
                        Constraint::new(None, Some(RunningConstraint::FnPtr(sigval))),
                    )
                }
                RigidTy::Ref(_, ty, _) => self.convert_ty(span, &ty),
                RigidTy::RawPtr(ty, _mut) => self.convert_ty(span, &ty),
                RigidTy::Char | RigidTy::Str | RigidTy::Never => {
                    (None, Constraint::new(None, None))
                }
                RigidTy::Dynamic(bound_existentials, _) => {
                    let mut traitobj_vec = Vec::new();
                    for bound_existential in bound_existentials {
                        unique_push(
                            &mut traitobj_vec,
                            TraitObjTy::new_from_bound_existential(&bound_existential),
                        );
                    }
                    (
                        Some(traitobj_vec.clone()),
                        Constraint::new(None, Some(RunningConstraint::Dynamic(traitobj_vec))),
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
            BinOp::Offset => {
                let (_, ty) = self.convert_ty(span, destty);
                ty
            }
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
            return Constraint::new(None, Some(RunningConstraint::Scalar(None)));
        }
        match (c_op1[0].clone(), c_op2[0].clone()) {
            (
                Constraint {
                    toc: None,
                    cfc: Some(RunningConstraint::Scalar(Some(val1))),
                },
                Constraint {
                    toc: None,
                    cfc: Some(RunningConstraint::Scalar(Some(val2))),
                },
            ) => Constraint::new(None, Some(RunningConstraint::Scalar(Some(f(val1, val2))))),
            (
                Constraint {
                    toc: None,
                    cfc: Some(RunningConstraint::Scalar(Some(val1))),
                },
                Constraint {
                    toc: to,
                    cfc: Some(RunningConstraint::Scalar(Some(val2))),
                },
            ) => Constraint::new(to, Some(RunningConstraint::Scalar(Some(f(val1, val2))))),
            (
                Constraint {
                    toc: to,
                    cfc: Some(RunningConstraint::Scalar(Some(val1))),
                },
                Constraint {
                    toc: None,
                    cfc: Some(RunningConstraint::Scalar(Some(val2))),
                },
            ) => Constraint::new(to, Some(RunningConstraint::Scalar(Some(f(val1, val2))))),
            (
                Constraint {
                    toc: _to1,
                    cfc: Some(RunningConstraint::Scalar(Some(_val1))),
                },
                Constraint {
                    toc: _to2,
                    cfc: Some(RunningConstraint::Scalar(Some(_val2))),
                },
            ) => {
                todo!();
                //(to, Some(RunningConstraint::Scalar(Some(f(
                //    val1, val2,
                //)))))
            }
            _ => Constraint::new(None, Some(RunningConstraint::Scalar(None))),
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
            UnOp::PtrMetadata => {
                let (_, ty) = self.convert_ty(span, destty);
                ty
            }
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
            return Constraint::new(None, Some(RunningConstraint::Scalar(None)));
        }
        match c_op[0].clone() {
            Constraint {
                toc: to,
                cfc: Some(RunningConstraint::Scalar(Some(val))),
            } => Constraint::new(to, Some(RunningConstraint::Scalar(Some(f(val))))),
            Constraint { toc: to, cfc: _ } => {
                Constraint::new(to, Some(RunningConstraint::Scalar(None)))
            } //_ => Constraint::ControlFlow(Box::new(RunningConstraint::Scalar(None))),
        }
    }

    fn is_unsafe_cell(&self, def: &AdtDef) -> bool {
        def.name().ends_with("UnsafeCell")
    }

    pub fn is_frozen(&self, ty: &Ty, seen: &mut Vec<Ty>) -> bool {
        if seen.contains(ty) {
            return true;
        }
        seen.push(*ty);

        match ty.kind() {
            TyKind::RigidTy(rty) => match rty {
                RigidTy::Bool
                | RigidTy::Int(_)
                | RigidTy::Uint(_)
                | RigidTy::Float(_)
                | RigidTy::Char
                | RigidTy::Str
                | RigidTy::Never
                | RigidTy::FnDef(..)
                | RigidTy::FnPtr(_) => true,

                RigidTy::Adt(adtdef, genargs) => {
                    if self.is_unsafe_cell(&adtdef) {
                        return false;
                    }
                    for var in adtdef.variants() {
                        for field in var.fields() {
                            let fty = field.ty_with_args(&genargs);
                            if !self.is_frozen(&fty, seen) {
                                return false;
                            }
                        }
                    }
                    true
                }

                RigidTy::Tuple(tys) => tys.iter().all(|t| self.is_frozen(t, seen)),
                RigidTy::Array(t, _) | RigidTy::Slice(t) => self.is_frozen(&t, seen),

                RigidTy::Ref(..) | RigidTy::RawPtr(..) => true,

                _ => false,
            },
            _ => false,
        }
    }

    pub fn convert_static_const(
        &self,
        span: &Location,
        ty: &Ty,
        alloc: &Allocation,
    ) -> Constraints {
        let inner = self.inner_ty(ty);

        match inner.kind() {
            TyKind::RigidTy(RigidTy::Bool)
            | TyKind::RigidTy(RigidTy::Int(_))
            | TyKind::RigidTy(RigidTy::Uint(_)) => match alloc.read_int() {
                Ok(val) => {
                    return vec![Constraint::new(
                        None,
                        Some(RunningConstraint::Scalar(Some(val))),
                    )];
                }
                Err(_) => {}
            },
            _ => {}
        }

        let (_, constraint) = self.convert_ty(span, ty);
        vec![constraint]
    }

    fn inner_ty(&self, ty: &Ty) -> Ty {
        match ty.kind() {
            TyKind::RigidTy(RigidTy::Ref(_, inner, _))
            | TyKind::RigidTy(RigidTy::RawPtr(inner, _)) => self.inner_ty(&inner),
            _ => *ty,
        }
    }
}
