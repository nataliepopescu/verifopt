use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_public::DefId;
use rustc_public::abi::FieldsShape;
use rustc_public::mir::{
    AggregateKind, BinOp, CastKind, ConstOperand, LocalDecl, Operand, Place, ProjectionElem,
    Rvalue, UnOp,
};
use rustc_public::ty::{
    AdtDef, Allocation, ConstantKind, GenericArgKind, GenericArgs, ProvenanceMap, RigidTy, Ty,
    TyKind,
};

//use crate::InterpStore;
use crate::TraitStore;
use crate::constraints::{ADTFields, unique_append, unique_push};
use crate::constraints::{
    Constraint, Constraints, Context, Location, Prov, RunningConstraint, TraitObjConstraint,
    TraitObjTy, VOID,
};
use crate::sig_collect::SigVal;

use log::debug;
use std::cell::RefCell;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WrapperKind {
    Box,
    Unique,
    NonNull,
    Option,
    Result,
}

pub struct RvalConverter<'a> {
    pub tstore: &'a TraitStore,
    pub wrapper_cache: RefCell<HashMap<DefId, Option<WrapperKind>>>,
}

impl<'a> RvalConverter<'a> {
    pub fn new(tstore: &'a TraitStore) -> RvalConverter<'a> {
        Self {
            tstore,
            wrapper_cache: RefCell::new(HashMap::default()),
        }
    }

    pub fn convert(
        &self,
        ctxt: &Context,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        destty: &Ty,
        to_convert: &Rvalue,
    ) -> Constraints {
        match to_convert {
            Rvalue::Use(op) => self.convert_op(ctxt, span, local_decls, cur_scope, op, destty),
            Rvalue::Ref(_region, _borrow_kind, place) => {
                self.convert_place(ctxt, span, local_decls, cur_scope, place, destty)
            }
            Rvalue::Discriminant(place) => {
                self.convert_place(ctxt, span, local_decls, cur_scope, place, destty)
            }
            Rvalue::CopyForDeref(place) => {
                self.convert_place(ctxt, span, local_decls, cur_scope, place, destty)
            }
            Rvalue::Cast(kind, op, ty) => {
                self.convert_cast(ctxt, span, local_decls, cur_scope, kind, op, ty)
            }
            Rvalue::Aggregate(kind, ops) => {
                self.convert_agg(ctxt, span, local_decls, cur_scope, destty, kind, ops)
            }
            Rvalue::AddressOf(_rawptrkind, place) => {
                self.convert_place(ctxt, span, local_decls, cur_scope, place, destty)
            }
            Rvalue::UnaryOp(unop, op) => {
                self.convert_unop(ctxt, span, local_decls, cur_scope, destty, unop, op)
            }
            Rvalue::BinaryOp(binop, op1, op2) => {
                self.convert_binop(ctxt, span, local_decls, cur_scope, destty, binop, op1, op2)
            }
            Rvalue::CheckedBinaryOp(binop, op1, op2) => self.convert_checked_binop(
                ctxt,
                span,
                local_decls,
                cur_scope,
                destty,
                binop,
                op1,
                op2,
            ),
            Rvalue::Repeat(op, _tyconst) => {
                self.convert_op(ctxt, span, local_decls, cur_scope, op, destty)
            }
            _ => todo!("other rval: {:?}", to_convert),
        }
    }

    fn convert_op(
        &self,
        ctxt: &Context,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        op: &Operand,
        destty: &Ty,
    ) -> Constraints {
        match op {
            Operand::Copy(place) | Operand::Move(place) => {
                self.convert_place(ctxt, span, local_decls, cur_scope, place, destty)
            }
            Operand::Constant(const_op) => self.convert_const(span, &const_op),
            _ => todo!("runtime checks"),
        }
    }

    pub fn convert_const(&self, span: &Location, const_op: &ConstOperand) -> Constraints {
        debug!("CONVERTING CONST");
        let ty = const_op.const_.ty();

        match const_op.const_.kind() {
            ConstantKind::Allocated(alloc) => self.convert_allocated_const(span, &ty, alloc),
            ConstantKind::ZeroSized => self.convert_zero_sized_const(span, &ty),
            other => todo!("arg is another constant kind: {:?}", other),
        }
    }

    fn convert_allocated_const(&self, span: &Location, ty: &Ty, alloc: &Allocation) -> Constraints {
        match ty.kind() {
            TyKind::RigidTy(RigidTy::Bool | RigidTy::Int(_) | RigidTy::Uint(_)) => {
                match alloc.read_int() {
                    Ok(val) => Constraints::from(Constraint::new(
                        None,
                        Some(RunningConstraint::Scalar(Some(val))),
                    )),
                    Err(_) => Constraints::from(Constraint::new(
                        None,
                        Some(RunningConstraint::Scalar(None)),
                    )),
                }
            }
            TyKind::RigidTy(RigidTy::Adt(adtdef, genargs)) => Constraints::from(Constraint::new(
                None,
                Some(RunningConstraint::Adt(
                    adtdef.clone(),
                    genargs.clone(),
                    None,
                    self.convert_allocated_adt(span, &adtdef, &genargs, ty, alloc),
                )),
            )),
            _ => self.convert_const_fallback(span, ty),
        }
    }

    fn convert_zero_sized_const(&self, span: &Location, ty: &Ty) -> Constraints {
        match ty.kind() {
            // A zero-sized ADT (unit struct, empty enum variant, etc.) — build
            // the Adt shape directly rather than falling through to convert_ty,
            // same as the non-empty case, just with an empty field list.
            TyKind::RigidTy(RigidTy::Adt(adtdef, genargs)) => Constraints::from(Constraint::new(
                None,
                Some(RunningConstraint::Adt(
                    adtdef,
                    genargs.clone(),
                    None,
                    Vec::new(),
                )),
            )),
            // Zero-sized non-ADT (e.g. `()`, a zero-length array/tuple, a ZST
            // closure) — no field structure to build, so the generic fallback
            // is the right answer here, not a gap to fill in later.
            _ => self.convert_const_fallback(span, ty),
        }
    }

    fn convert_allocated_adt(
        &self,
        span: &Location,
        adtdef: &AdtDef,
        genargs: &GenericArgs,
        ty: &Ty,
        alloc: &Allocation,
    ) -> ADTFields {
        debug!("CONVERT ADT");
        let layout = ty.layout().expect("no layout for a concrete, sized ADT");
        let shape = layout.shape();

        let offsets = match &shape.fields {
            FieldsShape::Arbitrary { offsets, .. } => offsets,
            // Primitive/Union/Array shapes shouldn't reach here for a
            // plain struct; fall back gracefully rather than panic if they do
            _ => return Vec::new(),
        };

        debug!("offsets: {:?}", offsets);

        let mut fields = Vec::new();
        for (i, field_def) in adtdef.variants()[0].fields().iter().enumerate() {
            let field_ty = field_def.ty_with_args(genargs);
            let field_layout = field_ty.layout().expect("field has no layout");
            let offset = offsets[i].bytes();
            let size = field_layout.shape().size.bytes();

            let sub_bytes = alloc.bytes[offset..offset + size].to_vec();
            let sub_provenance = Self::slice_provenance(&alloc.provenance, offset, size);

            let sub_alloc = Allocation {
                bytes: sub_bytes,
                provenance: sub_provenance,
                align: field_layout.shape().abi_align,
                mutability: alloc.mutability,
            };

            // Recurse: a scalar field bottoms out via read_int as today;
            // a nested struct field recurses back into this same function.
            let field_constraint = self.convert_sub_alloc(span, &field_ty, &sub_alloc);
            fields.push((
                ProjectionElem::Field(i, field_ty.clone()),
                Constraints::from(field_constraint),
            ));
        }
        fields
    }

    /// Decode a sub-allocation (one field's worth of bytes, sliced out of a
    /// parent struct's Allocation) into a Constraint, dispatching on the
    /// field's own type the same way convert_const dispatches on the whole
    /// operand's type.
    fn convert_sub_alloc(&self, span: &Location, ty: &Ty, alloc: &Allocation) -> Constraint {
        match ty.kind() {
            TyKind::RigidTy(RigidTy::Bool | RigidTy::Int(_) | RigidTy::Uint(_)) => {
                match alloc.read_int() {
                    Ok(val) => Constraint::new(None, Some(RunningConstraint::Scalar(Some(val)))),
                    Err(_) => Constraint::new(None, Some(RunningConstraint::Scalar(None))),
                }
            }

            TyKind::RigidTy(RigidTy::Adt(adtdef, genargs)) => Constraint::new(
                None,
                Some(RunningConstraint::Adt(
                    adtdef.clone(),
                    genargs.clone(),
                    None,
                    self.convert_allocated_adt(span, &adtdef, &genargs, ty, alloc),
                )),
            ),

            // Pointers, references, dyn, etc. inside a compile-time constant are
            // real but much rarer (a `&'static` reference to a static, mostly) —
            // not worth a bytes-level decoder yet. Fall back to the type-only
            // reconstruction rather than mis-decoding raw pointer bytes as if
            // they were meaningful without provenance resolution.
            _ => {
                let (maybe_traitobj, constraint) = self.convert_ty(span, ty);
                if maybe_traitobj.is_some() {
                    todo!("const field contains dyn: {:?}", ty);
                }
                constraint
            }
        }
    }

    fn slice_provenance(provenance: &ProvenanceMap, offset: usize, size: usize) -> ProvenanceMap {
        let ptrs = provenance
            .ptrs
            .iter()
            .filter(|(byte_off, _)| *byte_off >= offset && *byte_off < offset + size)
            .map(|(byte_off, prov)| (byte_off - offset, prov.clone()))
            .collect();
        ProvenanceMap { ptrs }
    }

    fn convert_const_fallback(&self, span: &Location, ty: &Ty) -> Constraints {
        let (maybe_traitobj, constraint) = self.convert_ty(span, ty);
        if let Some(traitobj) = maybe_traitobj {
            todo!("const contains dyn: {:?}", traitobj);
        }
        Constraints::from(constraint)
    }

    fn convert_place(
        &self,
        ctxt: &Context,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        place: &Place,
        destty: &Ty,
    ) -> Constraints {
        debug!("\nCONVERTING PLACE: {:?}", place);

        match ctxt.get_constraints(cur_scope, place, false) {
            Some(constraints) => {
                debug!("CONSTRAINTS EXIST: {:?}", constraints);
                constraints
            }
            None => {
                debug!("CONSTRAINTS DNE");
                //for proj in &place.projection {
                //    debug!("\nPROJ: {:?}", proj);
                //}
                debug!("DEST TY: {:?}", destty);
                //let (maybe_traitobj, constraint) = self.convert_ty(span, destty);
                let place_ty = place.ty(local_decls).unwrap_or(*destty);
                let (maybe_traitobj, constraint) = self.convert_ty(span, &place_ty);
                debug!("CONSTRAINT: {:?}", constraint);

                if let Some(_traitobj) = maybe_traitobj {
                    //todo!("place ty contains dyn {:?}", traitobj);
                }

                Constraints::from(constraint)
            }
        }
    }

    fn convert_cfc_to_toc(&self, cfc: &RunningConstraint) -> TraitObjConstraint {
        match cfc {
            RunningConstraint::Adt(adtdef, genargs, variant_idx, fields) => {
                TraitObjConstraint::Adt(*adtdef, genargs.clone(), *variant_idx, fields.to_vec())
            }
            RunningConstraint::Closure(cdef, genargs) => {
                TraitObjConstraint::Closure(*cdef, genargs.clone())
            }
            _ => panic!("unexpected cfc: {:?}", cfc),
        }
    }

    fn get_defid_from_cfc(&self, cfc: &RunningConstraint) -> Option<DefId> {
        match cfc {
            RunningConstraint::Adt(adtdef, _, _, _) => Some(adtdef.0),
            RunningConstraint::Closure(cdef, _) => Some(cdef.0),
            // FIXME this is a jumbled mess
            RunningConstraint::Idk(inner) => match inner.len() {
                0 => None,
                1 => self.get_defid_from_cfc(&inner.at(0).cfc.as_ref().unwrap()),
                n => todo!("Idk with {} inner constraints: {:?}", n, inner),
            },
            RunningConstraint::Dynamic(tot_vec) => match tot_vec.len() {
                0 => None,
                1 => Some(tot_vec[0].def.0),
                n => todo!("Dynamic with {} tot vector: {:?}", n, tot_vec),
            },
            _ => todo!("cfc: {:?}", cfc),
        }
    }

    fn convert_cast_helper(
        &self,
        traitobjtys: &Vec<TraitObjTy>,
        constraints: &Constraints,
        span: &Location,
    ) -> Constraints {
        debug!("CAST HELPER");
        let mut new_constraints = Constraints::new();

        debug!("traitobjtys: {:?}\n", traitobjtys);
        for traitobjty in traitobjtys {
            for constraint in &constraints.inner {
                debug!("\ntraitobjty: {:?}", traitobjty);
                debug!("constraint: {:?}", constraint);
                match constraint {
                    Constraint { toc: Some(_), .. } => {
                        new_constraints.push(constraint.clone());
                    }
                    Constraint {
                        toc: None,
                        cfc: Some(cfc_),
                        prov: _,
                    } => {
                        match self.get_defid_from_cfc(&cfc_) {
                            Some(defid) => {
                                debug!("DEFID: {:?}", defid);
                                debug!("CFC: {:?}", cfc_);

                                match self.tstore.struct_traits.get(&defid) {
                                    Some(traits) => {
                                        debug!("found traits");
                                        if traits.contains(&traitobjty.def.0) {
                                            // pull relevant CFC into TOC
                                            let new_constraint = Constraint::new(
                                                Some((traitobjty.clone(), self.convert_cfc_to_toc(&cfc_))),
                                                Some(cfc_.clone()),
                                            )
                                            .with_prov(match span.scope {
                                                Some(s) => Prov::Tags(
                                                    [(s, span.bb, span.stmt)].into_iter().collect(),
                                                ),
                                                None => Prov::Unknown,
                                            });
                                            new_constraints.push(new_constraint);
                                        } else {
                                            // push old constraint unchanged
                                            new_constraints.push(constraint.clone());
                                        }
                                    }
                                    None => {
                                        // Is the trait one of Fn/FnMut/FnOnce? And is the current
                                        // constraint a closure? If so, these traits are implicitly
                                        // implemented and won't exist in our trait_store
                                        if constraint.is_cfc_closure() && traitobjty.is_fn_trait() {
                                            // Pull relevant CFC into TOC
                                            let new_constraint = Constraint::new(
                                                Some((traitobjty.clone(), self.convert_cfc_to_toc(&cfc_))),
                                                Some(cfc_.clone()),
                                            )
                                            .with_prov(match span.scope {
                                                Some(s) => Prov::Tags(
                                                    [(s, span.bb, span.stmt)].into_iter().collect(),
                                                ),
                                                None => Prov::Unknown,
                                            });
                                            new_constraints.push(new_constraint);
                                        //} else if traitobjty.is_fn_trait() {
                                        //    // Use collected constraints
                                        //    todo!();
                                        } else {
                                            todo!();
                                        }
                                    }
                                }
                            }
                            None => new_constraints.push(constraint.clone()),
                        }
                    }
                    _ => {
                        new_constraints.push(constraint.clone());
                    }
                }
            }
        }

        new_constraints
    }

    fn convert_cast(
        &self,
        ctxt: &Context,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        _kind: &CastKind,
        op: &Operand,
        ty: &Ty,
    ) -> Constraints {
        match op {
            Operand::Constant(const_op) => {
                let prev_constraints = self.convert_const(span, &const_op);
                let (maybe_traitobj, post_constraint) = self.convert_ty(span, ty);
                if let Some(traitobjtys) = maybe_traitobj {
                    self.convert_cast_helper(&traitobjtys, &prev_constraints, span)
                } else {
                    Constraints::from(post_constraint)
                }
            }
            Operand::Copy(place) | Operand::Move(place) => {
                debug!("CASTING existing place");
                debug!("place: {:?}", place);

                let prev_constraints =
                    self.convert_place(ctxt, span, local_decls, cur_scope, place, ty);

                debug!("\n\nPRE CAST constraints: {:?}", prev_constraints);
                let (maybe_traitobj, post_constraint) = self.convert_ty(span, ty);
                debug!("POST CAST ty: {:?}", post_constraint);
                debug!("maybe_traitobj: {:?}", maybe_traitobj);

                if let Some(traitobjtys) = maybe_traitobj {
                    self.convert_cast_helper(&traitobjtys, &prev_constraints, span)
                } else {
                    match &prev_constraints.inner.first().and_then(|c| c.cfc.as_ref()) {
                        Some(RunningConstraint::Adt(adtdef, _, _, _))
                            if self.wrapper_kind(adtdef).is_some() =>
                        {
                            let unwrapped = ctxt.step_field(
                                cur_scope,
                                &prev_constraints,
                                &ProjectionElem::Field(0, ty.clone()),
                            );
                            if unwrapped.inner.is_empty() {
                                Constraints::from(post_constraint)
                            } else {
                                unwrapped
                            }
                        }
                        _ => prev_constraints,
                    }
                }
            }
            _ => todo!("runtime checks"),
        }
    }

    pub fn wrapper_kind(&self, def: &AdtDef) -> Option<WrapperKind> {
        if let Some(cached) = self.wrapper_cache.borrow().get(&def.0) {
            return *cached;
        }

        let name = def.0.name();
        let suffix = name.splitn(2, "::").nth(1).unwrap_or("");
        let kind = match suffix {
            "boxed::Box" => Some(WrapperKind::Box),
            "ptr::Unique" => Some(WrapperKind::Unique),
            "ptr::NonNull" => Some(WrapperKind::NonNull),
            "option::Option" => Some(WrapperKind::Option),
            "result::Result" => Some(WrapperKind::Result),
            _ => None,
        };

        self.wrapper_cache.borrow_mut().insert(def.0, kind);
        kind
    }

    pub fn get_any_traitobj(
        &self,
        maybe_trait_ty: &Option<Vec<TraitObjTy>>,
        constraint: &Constraint,
    ) -> Option<(TraitObjTy, TraitObjConstraint)> {
        match constraint {
            Constraint { toc: Some(to_), .. } => {
                return Some(to_.clone());
            }
            Constraint {
                toc: None,
                cfc: Some(maybe_to),
                prov: _,
            } => {
                match maybe_to {
                    RunningConstraint::Adt(adtdef, adt_genargs, variant_idx, fields) => {
                        // If we get Some, that means this struct/adt implements one or more
                        // traits, but that does _not_ mean that this is a trait object
                        match self.tstore.struct_traits.get(&adtdef.0) {
                            Some(_possible_traits) => {
                                // Once we know we are storing the result of this rval into a
                                // traitobj, only _then_ can we populate the traitobj constraint field
                                if let Some(trait_ty) = maybe_trait_ty {
                                    if trait_ty.len() > 1 {
                                        todo!();
                                    }

                                    return Some((
                                        trait_ty[0].clone(),
                                        TraitObjConstraint::Adt(
                                            adtdef.clone(),
                                            adt_genargs.clone(),
                                            variant_idx.clone(),
                                            fields.clone(),
                                        ),
                                    ));
                                }
                            }
                            _ => {}
                        }
                    }
                    RunningConstraint::Closure(cdef, genargs) => {
                        // This case is expected if the traits in maybe_trait_ty are one of: Fn, FnMut, FnOnce
                        if let Some(trait_ty) = maybe_trait_ty {
                            if trait_ty.len() > 1 {
                                todo!();
                            }

                            return Some((
                                trait_ty[0].clone(),
                                TraitObjConstraint::Closure(cdef.clone(), genargs.clone()),
                            ));
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
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
        ctxt: &Context,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        destty: &Ty,
        kind: &AggregateKind,
        ops: &Vec<Operand>,
    ) -> Constraints {
        debug!("AGG kind: {:?}", kind);
        debug!("ops: {:?}", ops);
        debug!("destty: {:?}", destty);
        match kind {
            AggregateKind::Adt(def, variant_idx, genargs, _, _field_idx) => {
                debug!("ADT agg");

                // Create projections here to simulate field initializers
                let mut fields = Vec::new();
                for (i, op) in ops.into_iter().enumerate() {
                    debug!("\n---op {:?}", i);
                    let op_constraints =
                        self.convert_op(ctxt, span, local_decls, cur_scope, op, destty);
                    debug!("op constraints: {:?}", op_constraints);

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

                    let proj = ProjectionElem::Field(i, op_ty);
                    debug!("PROJ: {:?}", proj);
                    fields.push((proj, op_constraints));
                    debug!("---done op {:?}\n", i);
                }

                Constraints::from(Constraint::new(
                    None,
                    Some(RunningConstraint::Adt(
                        *def,
                        genargs.clone(),
                        Some(*variant_idx),
                        fields,
                    )),
                ))
            }
            AggregateKind::Tuple => {
                let mut inner_constraints = Vec::new();
                for op in ops {
                    let op_constraints =
                        self.convert_op(ctxt, span, local_decls, cur_scope, op, destty);
                    inner_constraints.push(op_constraints); // preserve position; no merging across slots
                }
                Constraints::from(Constraint::new(
                    None,
                    Some(RunningConstraint::Tuple(inner_constraints)),
                ))
            }
            AggregateKind::RawPtr(ty, _mut) => {
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

                Constraints::from(Constraint::new(
                    None,
                    Some(RunningConstraint::Ptr(Box::new(constraint))),
                ))
            }
            AggregateKind::Array(ty) => {
                //debug!("array agg");
                let (maybe_traitobj, constraint) = self.convert_ty(span, ty);
                if maybe_traitobj.is_some() {
                    todo!("array contains dyn");
                }
                Constraints::from(Constraint::new(
                    None,
                    Some(RunningConstraint::List(Box::new(constraint))),
                ))
            }
            AggregateKind::Closure(def, genargs) => {
                //debug!("closure agg");
                Constraints::from(Constraint::new(
                    None,
                    Some(RunningConstraint::Closure(*def, genargs.clone())),
                ))
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
        debug!("\nCONVERTING GENARG: {:?}", genarg);
        match genarg {
            GenericArgKind::Type(ty) => {
                let (maybe_traitobj, constraint) = self.convert_ty(span, ty);
                debug!("genarg constraint: {:?}", constraint);
                if maybe_traitobj.is_some() {
                    todo!("genarg contains dyn");
                }
                Some(constraint)
            }
            _ => None,
        }
    }

    pub fn convert_ty(&self, span: &Location, ty: &Ty) -> (Option<Vec<TraitObjTy>>, Constraint) {
        debug!("IN CONVERT_TY");
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
                        debug!("NO TRAITOBJS in genargs");
                        (
                            None,
                            // FIXME fields is empty
                            Constraint::new(
                                None,
                                Some(RunningConstraint::Adt(def, genargs, None, vec![])),
                            ),
                        )
                    } else {
                        debug!("traitobjs in genargs!!!: {:?}", traitobjtys);
                        (
                            Some(traitobjtys),
                            // FIXME fields is empty
                            Constraint::new(
                                None,
                                Some(RunningConstraint::Adt(def, genargs, None, vec![])),
                            ),
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
                        Constraint::new(
                            None,
                            Some(RunningConstraint::Idk(Box::new(Constraints::from_vec(
                                inner,
                            )))),
                        ),
                    )
                }
                RigidTy::Array(ty, _) | RigidTy::Slice(ty) => {
                    let (maybe_traitobj, constraint) = self.convert_ty(span, &ty);
                    (
                        maybe_traitobj,
                        Constraint::new(
                            None,
                            Some(RunningConstraint::Idk(Box::new(Constraints::from(
                                constraint,
                            )))),
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

    fn convert_checked_binop(
        &self,
        ctxt: &Context,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        destty: &Ty,
        binop: &BinOp,
        op1: &Operand,
        op2: &Operand,
    ) -> Constraints {
        let first = self
            .convert_binop(ctxt, span, local_decls, cur_scope, destty, binop, op1, op2)
            .at(0)
            .clone();

        let second = Constraint::new(None, Some(RunningConstraint::Scalar(None)));

        let constraint = Constraint::new(
            None,
            Some(RunningConstraint::Tuple(vec![
                Constraints::from(first),
                Constraints::from(second),
            ])),
        );
        Constraints::from(constraint)
    }

    fn convert_binop(
        &self,
        ctxt: &Context,
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
                ctxt,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x + y,
            ),
            BinOp::Sub | BinOp::SubUnchecked => self.convert_binop_helper(
                ctxt,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x - y,
            ),
            BinOp::Mul | BinOp::MulUnchecked => self.convert_binop_helper(
                ctxt,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x * y,
            ),
            BinOp::Div => self.convert_binop_helper(
                ctxt,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x / y,
            ),
            BinOp::Rem => self.convert_binop_helper(
                ctxt,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x % y,
            ),
            BinOp::Eq => self.convert_binop_helper(
                ctxt,
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
                ctxt,
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
                ctxt,
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
                ctxt,
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
                ctxt,
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
                ctxt,
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
                ctxt,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x << y,
            ),
            BinOp::Shr | BinOp::ShrUnchecked => self.convert_binop_helper(
                ctxt,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x >> y,
            ),
            BinOp::BitAnd => self.convert_binop_helper(
                ctxt,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x & y,
            ),
            BinOp::BitOr => self.convert_binop_helper(
                ctxt,
                span,
                local_decls,
                cur_scope,
                destty,
                op1,
                op2,
                |x, y| x | y,
            ),
            BinOp::BitXor => self.convert_binop_helper(
                ctxt,
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
                ctxt,
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

        Constraints::from(constraint)
    }

    fn convert_binop_helper(
        &self,
        ctxt: &Context,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        destty: &Ty,
        op1: &Operand,
        op2: &Operand,
        f: fn(i128, i128) -> i128,
    ) -> Constraint {
        let c_op1 = self.convert_op(ctxt, span, local_decls, cur_scope, op1, destty);
        let c_op2 = self.convert_op(ctxt, span, local_decls, cur_scope, op2, destty);
        if c_op1.len() != 1 || c_op2.len() != 1 {
            return Constraint::new(None, Some(RunningConstraint::Scalar(None)));
        }
        match (c_op1.at(0).clone(), c_op2.at(0).clone()) {
            (
                Constraint {
                    toc: None,
                    cfc: Some(RunningConstraint::Scalar(Some(val1))),
                    prov: _,
                },
                Constraint {
                    toc: None,
                    cfc: Some(RunningConstraint::Scalar(Some(val2))),
                    prov: _,
                },
            ) => Constraint::new(None, Some(RunningConstraint::Scalar(Some(f(val1, val2))))),
            (
                Constraint {
                    toc: None,
                    cfc: Some(RunningConstraint::Scalar(Some(val1))),
                    prov: _,
                },
                Constraint {
                    toc: to,
                    cfc: Some(RunningConstraint::Scalar(Some(val2))),
                    prov: _,
                },
            ) => Constraint::new(to, Some(RunningConstraint::Scalar(Some(f(val1, val2))))),
            (
                Constraint {
                    toc: to,
                    cfc: Some(RunningConstraint::Scalar(Some(val1))),
                    prov: _,
                },
                Constraint {
                    toc: None,
                    cfc: Some(RunningConstraint::Scalar(Some(val2))),
                    prov: _,
                },
            ) => Constraint::new(to, Some(RunningConstraint::Scalar(Some(f(val1, val2))))),
            (
                Constraint {
                    toc: _to1,
                    cfc: Some(RunningConstraint::Scalar(Some(_val1))),
                    prov: _,
                },
                Constraint {
                    toc: _to2,
                    cfc: Some(RunningConstraint::Scalar(Some(_val2))),
                    prov: _,
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
        ctxt: &Context,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        destty: &Ty,
        unop: &UnOp,
        op: &Operand,
    ) -> Constraints {
        let constraint = match unop {
            UnOp::Neg => {
                self.convert_unop_helper(ctxt, span, local_decls, cur_scope, destty, op, |x| -x)
            }
            UnOp::Not => {
                self.convert_unop_helper(ctxt, span, local_decls, cur_scope, destty, op, |x| !x)
            }
            UnOp::PtrMetadata => {
                let (_, ty) = self.convert_ty(span, destty);
                ty
            }
        };

        Constraints::from(constraint)
    }

    fn convert_unop_helper(
        &self,
        ctxt: &Context,
        span: &Location,
        local_decls: &[LocalDecl],
        cur_scope: &VOID,
        destty: &Ty,
        op: &Operand,
        f: fn(i128) -> i128,
    ) -> Constraint {
        let c_op = self.convert_op(ctxt, span, local_decls, cur_scope, op, destty);
        if c_op.len() != 1 {
            return Constraint::new(None, Some(RunningConstraint::Scalar(None)));
        }
        match c_op.at(0).clone() {
            Constraint {
                toc: to,
                cfc: Some(RunningConstraint::Scalar(Some(val))),
                prov: _,
            } => Constraint::new(to, Some(RunningConstraint::Scalar(Some(f(val))))),
            Constraint {
                toc: to,
                cfc: _,
                prov: _,
            } => Constraint::new(to, Some(RunningConstraint::Scalar(None))), //_ => Constraint::ControlFlow(Box::new(RunningConstraint::Scalar(None))),
        }
    }
}
