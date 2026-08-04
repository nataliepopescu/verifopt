use crate::rustc_public_bridge::IndexedVal;
use rustc_public::mir::{LocalDecl, Operand, Place, ProjectionElem};
use rustc_public::ty::{
    AdtDef, FnDef, GenericArgs, PolyFnSig, RigidTy, Span, Ty, TyKind, VariantIdx,
};

use crate::Context;
use crate::InterpPass;
use crate::constraints::{ADTFields, Constraint, Constraints, RunningConstraint, VOID};
use crate::convert::WrapperKind;
use crate::error::Error;

use log::debug;

/// Everything we need about a call's receiver to decide whether (and how)
/// to stub it as a BTreeSet/BTreeMap method.
struct CollectionRecv {
    place: Place,
    adtdef: AdtDef,
    genargs: GenericArgs,
    kind: WrapperKind,                 // BTreeSet or BTreeMap
    key_field: ProjectionElem,         // field 0: the Set's element, or the Map's key
    val_field: Option<ProjectionElem>, // field 1: only present for BTreeMap
}

/// Same idea, but for one of *our own fabricated* iterator values
/// (Iter/IntoIter/Keys/Values over a BTreeSet/BTreeMap).
struct IterRecv {
    place: Place,
    elem_field: ProjectionElem,
}

impl<'a> InterpPass<'a> {
    /// Entry point: returns Some(result) if this call is a std
    /// collection/iterator method we're modeling with a summary instead of
    /// interpreting its real body.
    pub fn stdlib_stub(
        &self,
        ctxt: &mut Context,
        caller_scope: &VOID,
        term_span: &Span,
        local_decls: &[LocalDecl],
        fndef: &FnDef,
        args: &Vec<Operand>,
    ) -> Option<Result<Option<Constraints>, Error>> {
        debug!("STDLIB_STUB");
        let method = Self::method_name(fndef);

        if self.is_box_new(fndef) {
            return Some(Ok(self.stub_box_new(
                ctxt,
                caller_scope,
                term_span,
                local_decls,
                args,
                fndef,
            )));
        }

        // Iterator methods (currently just `next`) take priority, since an
        // Iter<...> value is never also a BTreeSet/BTreeMap.
        if let Some(recv) = self.iter_receiver(local_decls, args) {
            let result = match method.as_str() {
                "next" => self.stub_next(ctxt, caller_scope, local_decls, fndef, &recv),
                _ => panic!(
                    "stdlib_stub: no summary for iterator method {} - add one",
                    method
                ),
            };
            return Some(Ok(result));
        }

        let recv = self.collection_receiver(local_decls, args)?;

        let result = match method.as_str() {
            "new" | "default" => self.stub_constructor(&recv),
            "insert" | "extend" => {
                self.stub_insert(ctxt, caller_scope, term_span, local_decls, &recv, args)
            }
            "get" | "first" | "last" => {
                self.stub_get_like(ctxt, caller_scope, local_decls, fndef, &recv)
            }
            "iter" | "into_iter" | "range" | "keys" | "values" => {
                self.stub_make_iter(ctxt, caller_scope, local_decls, &recv)
            }
            "len" | "is_empty" | "contains" | "clear" | "remove" => {
                return Some(self.retty_fallback_from_poly(fndef.fn_sig()));
            }
            _ => panic!(
                "stdlib_stub: no summary for {:?}::{} - add one or handle it in wrapper_kind",
                recv.adtdef, method
            ),
        };

        Some(Ok(result))
    }

    // ---------- receiver identification ----------

    /// Returns None if the receiver isn't a BTreeSet/BTreeMap (or there's no
    /// receiver at all), in which case the caller should fall through to normal
    /// interpretation.
    fn collection_receiver(
        &self,
        local_decls: &[LocalDecl],
        args: &Vec<Operand>,
    ) -> Option<CollectionRecv> {
        let place = self.receiver_place(args)?;
        let (adtdef, genargs) = self.receiver_adt(local_decls, &place)?;
        let kind = self.converter.wrapper_kind(&adtdef)?;
        if !matches!(kind, WrapperKind::BTreeSet | WrapperKind::BTreeMap) {
            return None;
        }

        let key_field = ProjectionElem::Field(0, genargs.0[0].expect_ty().clone());
        let val_field = matches!(kind, WrapperKind::BTreeMap)
            .then(|| ProjectionElem::Field(1, genargs.0[1].expect_ty().clone()));

        Some(CollectionRecv {
            place,
            adtdef,
            genargs,
            kind,
            key_field,
            val_field,
        })
    }

    /// Recognizes our own synthetic iterator values by the real iterator
    /// ADT's name suffix (Iter/IntoIter/Keys/Values over btree::map/set).
    /// NOTE: this is *not* a general "wrapper" concept for resolve_adt_helper
    /// - it only matters to our own stubs - so it's kept local here rather
    /// than added to `wrapper_kind` in convert.rs.
    fn iter_receiver(&self, local_decls: &[LocalDecl], args: &Vec<Operand>) -> Option<IterRecv> {
        let place = self.receiver_place(args)?;
        let (adtdef, _genargs) = self.receiver_adt(local_decls, &place)?;

        let name = adtdef.0.name();
        let suffix = name.splitn(2, "::").nth(1).unwrap_or("");
        if !crate::convert::is_btree_iter_suffix(suffix) {
            return None;
        }

        // Field 0 of our fabricated Iter value carries the element
        // constraints - see `stub_make_iter`. We don't know its Ty here
        // (we're not tracking one for the synthetic wrapper), so reuse
        // whatever's already stored rather than needing a type annotation.
        Some(IterRecv {
            place,
            elem_field: ProjectionElem::Field(0, Ty::bool_ty()),
        })
    }

    /// Pulls the first argument's Place out of an Operand, if it has one.
    fn receiver_place(&self, args: &Vec<Operand>) -> Option<Place> {
        match args.get(0) {
            Some(Operand::Copy(p)) | Some(Operand::Move(p)) => Some(p.clone()),
            _ => None,
        }
    }

    /// Resolves a place's type to its AdtDef + GenericArgs, stripping one level
    /// of `&`/`&mut` if the place is a reference (as method receivers usually are).
    fn receiver_adt(
        &self,
        local_decls: &[LocalDecl],
        place: &Place,
    ) -> Option<(AdtDef, GenericArgs)> {
        let ty = place.ty(local_decls).ok()?;
        let result = match ty.kind() {
            TyKind::RigidTy(RigidTy::Ref(_, inner_ty, _)) => match inner_ty.kind() {
                TyKind::RigidTy(RigidTy::Adt(adtdef, genargs)) => Some((adtdef, genargs)),
                _ => None,
            },
            TyKind::RigidTy(RigidTy::Adt(adtdef, genargs)) => Some((adtdef, genargs)),
            _ => None,
        };
        if let Some((adtdef, _)) = &result {
            debug!("receiver_adt: adtdef name = {}", adtdef.0.name());
        }
        result
    }

    /// Last path segment of a called function's name, e.g. "insert" from
    /// "collections::btree::set::BTreeSet::<impl>::insert".
    fn method_name(fndef: &FnDef) -> String {
        let full = fndef.0.name();
        full.rsplit("::").next().unwrap_or(&full).to_string()
    }

    // ---------- collection method handlers ----------

    /// `BTreeSet::new()` / `BTreeMap::new()` - a fresh, empty synthetic value.
    fn stub_constructor(&self, recv: &CollectionRecv) -> Option<Constraints> {
        Some(Constraints::from(self.fresh_collection_constraint(recv)))
    }

    /// `insert`/`extend` - merge the inserted value's constraints into the
    /// element slot, write the updated collection back to the caller's place,
    /// and return an empty result (insert's own return is just a bool).
    fn stub_insert(
        &self,
        ctxt: &mut Context,
        caller_scope: &VOID,
        term_span: &Span,
        local_decls: &[LocalDecl],
        recv: &CollectionRecv,
        args: &Vec<Operand>,
    ) -> Option<Constraints> {
        let mut cur = ctxt
            .get_constraints(caller_scope, local_decls, &recv.place, false)
            .unwrap_or_else(|| Constraints::from(self.fresh_collection_constraint(recv)));

        let resolve = |idx: usize| -> Constraints {
            match args.get(idx) {
                Some(op) => {
                    self.resolve_arg(ctxt, term_span, caller_scope, &None, local_decls, op, false)
                }
                None => Constraints::new(),
            }
        };

        match recv.kind {
            WrapperKind::BTreeSet => {
                // insert(&mut self, value) - args[1] is the sole element
                cur.write_field(vec![recv.key_field.clone()], resolve(1));
            }
            WrapperKind::BTreeMap => {
                // insert(&mut self, key, value) - args[1]=key, args[2]=value
                //cur.write_field(vec![recv.key_field.clone()], resolve(1));
                //if let Some(val_field) = &recv.val_field {
                //    cur.write_field(vec![val_field.clone()], resolve(2));
                //}
                let key = resolve(1);
                let val = resolve(2);
                debug!(
                    "stub_insert: kind={:?} key={:?} val={:?}",
                    recv.kind, key, val
                );

                cur.write_field(vec![recv.key_field.clone()], key);
                if let Some(val_field) = &recv.val_field {
                    cur.write_field(vec![val_field.clone()], val);
                }
            }
            _ => unreachable!(),
        }

        ctxt.set_scoped_constraints(caller_scope, &recv.place, cur);
        Some(Constraints::new())
    }

    /// `get`/`first`/`last` all return `Option<...>` wrapping the element -
    /// read the element slot, then wrap it the same way `next()` does.
    /// BTreeSet::get returns the element itself; BTreeMap::get returns the
    /// *value*, not the key - so the field we read differs by kind.
    fn stub_get_like(
        &self,
        ctxt: &Context,
        caller_scope: &VOID,
        local_decls: &[LocalDecl],
        fndef: &FnDef,
        recv: &CollectionRecv,
    ) -> Option<Constraints> {
        let cur = ctxt.get_constraints(caller_scope, local_decls, &recv.place, false)?;
        let field = match recv.kind {
            WrapperKind::BTreeSet => &recv.key_field,
            WrapperKind::BTreeMap => recv.val_field.as_ref()?,
            _ => unreachable!(),
        };
        let elem = ctxt.step_field(caller_scope, &cur, field);
        self.wrap_in_option(&fndef.fn_sig(), elem)
    }

    /// `iter`/`into_iter`/`range`/`keys`/`values` - build a fresh synthetic
    /// iterator value whose field 0 *carries a copy of* the collection's
    /// current element constraints, so a later `.next()` call can hand it
    /// off.
    fn stub_make_iter(
        &self,
        ctxt: &Context,
        caller_scope: &VOID,
        local_decls: &[LocalDecl],
        recv: &CollectionRecv,
    ) -> Option<Constraints> {
        let cur = ctxt.get_constraints(caller_scope, local_decls, &recv.place, false)?;

        let elem = match recv.kind {
            WrapperKind::BTreeSet => ctxt.step_field(caller_scope, &cur, &recv.key_field),
            WrapperKind::BTreeMap => {
                let key = ctxt.step_field(caller_scope, &cur, &recv.key_field);
                let val_field = recv
                    .val_field
                    .as_ref()
                    .expect("BTreeMap always has a value field");
                let val = ctxt.step_field(caller_scope, &cur, val_field);
                Constraints::from(Constraint::new(
                    None,
                    Some(RunningConstraint::Tuple(vec![key, val])),
                ))
            }
            _ => unreachable!(),
        };
        debug!("stub_make_iter: kind={:?} elem={:?}", recv.kind, elem);

        let fields: ADTFields = vec![(recv.key_field.clone(), elem)];
        Some(Constraints::from(Constraint::new(
            None,
            Some(RunningConstraint::Adt(
                recv.adtdef.clone(),
                recv.genargs.clone(),
                None,
                fields,
            )),
        )))
    }

    /// `Iterator::next()` on one of our synthetic iterator values - read
    /// back the carried element constraints and wrap them in `Option`.
    fn stub_next(
        &self,
        ctxt: &Context,
        caller_scope: &VOID,
        local_decls: &[LocalDecl],
        fndef: &FnDef,
        recv: &IterRecv,
    ) -> Option<Constraints> {
        let cur = ctxt.get_constraints(caller_scope, local_decls, &recv.place, false)?;
        let elem = ctxt.step_field(caller_scope, &cur, &recv.elem_field);
        debug!("stub_next: elem = {:?}", elem);
        self.wrap_in_option(&fndef.fn_sig(), elem)
    }

    /// Builds an `Option<T>`-shaped Constraints value carrying `inner` in
    /// the `Some` variant's field 0. Uses the *real* Option AdtDef/GenericArgs
    /// straight from the callee's own signature (rather than fabricating
    /// one) so that anything downstream that keys off Option's identity -
    /// e.g. `wrapper_kind` - still recognizes it correctly.
    fn wrap_in_option(&self, sig: &PolyFnSig, inner: Constraints) -> Option<Constraints> {
        let output_ty = sig.value.output();
        let (adtdef, genargs) = match output_ty.kind() {
            TyKind::RigidTy(RigidTy::Adt(adtdef, genargs)) => (adtdef, genargs),
            _ => return None, // signature isn't Option<..>-shaped; let caller fall back
        };

        // Find "Some" by name rather than assuming a fixed index - don't
        // want to depend on discriminant ordering.
        let some_idx = adtdef.variants().iter().position(|v| v.name() == "Some")?;

        let fields: ADTFields = vec![(
            ProjectionElem::Field(0, genargs.0[0].expect_ty().clone()),
            inner,
        )];

        Some(Constraints::from(Constraint::new(
            None,
            Some(RunningConstraint::Adt(
                adtdef,
                genargs,
                Some(VariantIdx::to_val(some_idx)),
                fields,
            )),
        )))
    }

    fn fresh_collection_constraint(&self, recv: &CollectionRecv) -> Constraint {
        let fields: ADTFields = vec![(recv.key_field.clone(), Constraints::new())];
        Constraint::new(
            None,
            Some(RunningConstraint::Adt(
                recv.adtdef.clone(),
                recv.genargs.clone(),
                None,
                fields,
            )),
        )
    }

    // ---------- Box::new() handlers ----------

    /// `Box::new` has no receiver (`self`) to key off of - it's a bare
    /// constructor - so we recognize it by return type instead: if the
    /// callee's own signature says it returns something `wrapper_kind`
    /// already calls Box, and the method is literally named `new`, treat it
    /// as a stub target.
    fn is_box_new(&self, fndef: &FnDef) -> bool {
        if Self::method_name(fndef) != "new" {
            return false;
        }
        match fndef.fn_sig().value.output().kind() {
            TyKind::RigidTy(RigidTy::Adt(adtdef, _)) => {
                matches!(self.converter.wrapper_kind(&adtdef), Some(WrapperKind::Box))
            }
            _ => false,
        }
    }

    /// Builds a clean `Box<T>` constraint directly from the argument being
    /// boxed, instead of letting the interpreter walk Box::new's real
    /// allocator-touching body.
    fn stub_box_new(
        &self,
        ctxt: &Context,
        caller_scope: &VOID,
        term_span: &Span,
        local_decls: &[LocalDecl],
        args: &Vec<Operand>,
        fndef: &FnDef,
    ) -> Option<Constraints> {
        let inner = match args.get(0) {
            Some(op) => {
                self.resolve_arg(ctxt, term_span, caller_scope, &None, local_decls, op, false)
            }
            None => Constraints::new(),
        };
        debug!("stub_box_new: {:?}", inner);

        let (adtdef, genargs) = match fndef.fn_sig().value.output().kind() {
            TyKind::RigidTy(RigidTy::Adt(adtdef, genargs)) => (adtdef, genargs),
            _ => return None,
        };

        let fields: ADTFields = vec![(
            ProjectionElem::Field(0, genargs.0[0].expect_ty().clone()),
            inner,
        )];
        Some(Constraints::from(Constraint::new(
            None,
            Some(RunningConstraint::Adt(adtdef, genargs, None, fields)),
        )))
    }
}
