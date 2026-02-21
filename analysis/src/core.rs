//use rustc_hir::def::Res;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::interpret::Scalar;
use rustc_middle::mir::*;
//use rustc_span::symbol::Symbol;
//use rustc_span::Ident;
use rustc_abi::FieldIdx;
use rustc_data_structures::fx::FxHashSet as HashSet;
use rustc_index::{IndexSlice, IndexVec};
use rustc_middle::ty::{GenericArg, GenericArgKind, List, ParamTy, ScalarInt, Ty, TyCtxt, TyKind};

use crate::ConstraintMap;
use crate::FuncMap;
use crate::constraints::{Constraints, MapKey, VarType};
use crate::error::Error;

pub type Type = &'static str;

#[derive(Debug, Clone, Hash)]
pub struct FuncVal<'tcx> {
    pub def_id: DefId,
    pub is_intrinsic: bool,
    pub self_arg: Option<Place<'tcx>>,
    pub params: Vec<(Place<'tcx>, Ty<'tcx>)>,
    pub param_generics: Option<Vec<ParamTy>>,
    pub rettype: Option<Ty<'tcx>>,
    pub ret_did: Option<DefId>,
    pub ret_generic: Option<ParamTy>,
}

impl<'tcx> FuncVal<'tcx> {
    pub fn new(
        def_id: DefId,
        is_intrinsic: bool,
        self_arg: Option<Place<'tcx>>,
        arg_names: Vec<Place<'tcx>>,
        arg_types_opt: Option<Vec<Ty<'tcx>>>,
        param_generics: Option<Vec<ParamTy>>,
        rettype: Option<Ty<'tcx>>,
        ret_did: Option<DefId>,
        ret_generic: Option<ParamTy>,
    ) -> FuncVal<'tcx> {
        let params;
        if let Some(arg_types) = arg_types_opt {
            params = std::iter::zip(arg_names, arg_types).collect();
        } else {
            // dummy value for functions that do not have MIR available (and therefore cannot get
            // argument types the way we generally get them -- this is probably fine since we
            // won't be executing this particular MIR body anyway
            params = vec![];
        }

        Self {
            def_id,
            is_intrinsic,
            self_arg,
            params,
            param_generics,
            rettype,
            ret_did,
            ret_generic,
        }
    }
}

pub fn is_box(def_id: DefId) -> bool {
    // FIXME does this ever change....
    def_id.krate.as_usize() == 3 && def_id.index.as_usize() == 662
}

//pub fn is_option(def_id: DefId) -> bool {
//    // FIXME does this ever change....
//    def_id.krate.as_usize() == 2 && def_id.index.as_usize() == 49010
//}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VerifoptRval<'tcx> {
    Scalar(ScalarInt),
    Ptr(Box<VerifoptRval<'tcx>>),
    Ref(Box<VerifoptRval<'tcx>>),
    ConstSlice(),
    IndirectConst(Ty<'tcx>),
    IdkStruct(DefId, Option<Vec<Vec<VerifoptRval<'tcx>>>>),
    //IdkGeneric(Symbol),
    IdkStr(), //Const<'tcx>),
    // FIXME don't want types
    IdkType(Ty<'tcx>),
    IdkDefId(DefId),
    Idk(),
    Undef(),
}

pub struct VerifoptConverter<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub funcs: &'a FuncMap<'tcx>,
    pub debug: bool,
}

impl<'a, 'tcx> VerifoptConverter<'a, 'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, funcs: &'a FuncMap<'tcx>, debug: bool) -> Self {
        Self { tcx, funcs, debug }
    }

    fn wrap_inner_constraints_ref(&self, inner: Constraints<'tcx>) -> Constraints<'tcx> {
        let mut outer = HashSet::default();
        // for each constraint, wrap in a pointer value
        for constraint in inner.clone().drain() {
            outer.insert(VerifoptRval::Ref(Box::new(constraint)));
        }
        outer
    }

    fn wrap_inner_constraints_ptr(&self, inner: Constraints<'tcx>) -> Constraints<'tcx> {
        let mut outer = HashSet::default();
        // for each constraint, wrap in a pointer value
        for constraint in inner.clone().drain() {
            outer.insert(VerifoptRval::Ptr(Box::new(constraint)));
        }
        outer
    }

    pub fn from_rvalue(
        &self,
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        local_decls: &IndexSlice<Local, LocalDecl<'tcx>>,
        item: &Rvalue<'tcx>,
    ) -> Constraints<'tcx> {
        if self.debug {
            println!("\n### IN FROM_RVALUE, item is: {:?}", item);
        }

        let mut ret_constraints = HashSet::default();
        match item {
            Rvalue::Cast(kind, op, ty) => {
                if self.debug {
                    println!("--cast");
                    println!("kind: {:?}", kind);
                    println!("op: {:?}", op);
                    println!("ty: {:?}", ty);
                }
                ret_constraints = self.rval_from_cast(cmap, cur_scope, kind, op, ty);
            }
            Rvalue::Aggregate(aggkind, fields) => {
                if self.debug {
                    println!("--agg");
                    println!("aggkind: {:?}", aggkind);
                    println!("fields: {:?}", fields);
                }
                ret_constraints = self.rval_from_agg(cmap, cur_scope, local_decls, aggkind, fields);
            }
            Rvalue::Use(op) => {
                if self.debug {
                    println!("--use");
                    println!("op: {:?}", op);
                }
                ret_constraints =
                    self.rval_from_op(cmap, cur_scope, op, &op.ty(local_decls, self.tcx));
            }
            Rvalue::RawPtr(kind, place) => {
                if self.debug {
                    println!("--rawptr");
                    println!("kind: {:?}", kind);
                    println!("place: {:?}", place);
                }
                let inner = self.rval_from_place(
                    cmap,
                    cur_scope,
                    place,
                    &place.ty(local_decls, self.tcx).ty,
                );
                ret_constraints = self.wrap_inner_constraints_ptr(inner);
            }
            Rvalue::Ref(_, _, place) => {
                if self.debug {
                    println!("--ref");
                }
                let inner = self.rval_from_place(
                    cmap,
                    cur_scope,
                    place,
                    &place.ty(local_decls, self.tcx).ty,
                );
                ret_constraints = self.wrap_inner_constraints_ref(inner);
            }
            /////////////////////////////////////
            // the below rvals all widen to Type
            /////////////////////////////////////
            Rvalue::BinaryOp(binop, boxed_op_tup) => {
                if self.debug {
                    println!("--binop");
                }
                ret_constraints.insert(VerifoptRval::IdkType(binop.ty(
                    self.tcx,
                    boxed_op_tup.0.ty(local_decls, self.tcx),
                    boxed_op_tup.1.ty(local_decls, self.tcx),
                )));
            }
            Rvalue::UnaryOp(unop, op) => {
                if self.debug {
                    println!("--unop");
                }
                ret_constraints.insert(VerifoptRval::IdkType(
                    unop.ty(self.tcx, op.ty(local_decls, self.tcx)),
                ));
            }
            Rvalue::Discriminant(place) => {
                if self.debug {
                    println!("--discr");
                }
                ret_constraints.insert(VerifoptRval::IdkType(place.ty(local_decls, self.tcx).ty));
            }
            Rvalue::ShallowInitBox(_, ty) => {
                if self.debug {
                    println!("--shallowinitbox");
                }
                ret_constraints.insert(VerifoptRval::IdkType(*ty));
            }
            Rvalue::CopyForDeref(place) => {
                if self.debug {
                    println!("--copyforderef");
                }
                ret_constraints.insert(VerifoptRval::IdkType(place.ty(local_decls, self.tcx).ty));
            }
            Rvalue::WrapUnsafeBinder(_, ty) => {
                if self.debug {
                    println!("--wrapunsafebinder");
                }
                ret_constraints.insert(VerifoptRval::IdkType(*ty));
            }
            Rvalue::Repeat(_, _) => todo!("repeat"),
            Rvalue::ThreadLocalRef(_) => todo!("thread local ref"),
        }

        ret_constraints
    }

    fn get_first_field_op(
        &self,
        fields: &IndexVec<FieldIdx, Operand<'tcx>>,
    ) -> Option<Operand<'tcx>> {
        let fields_slice = fields.as_slice();
        if fields_slice.len() > 0 {
            let op = &fields_slice[FieldIdx::from_u32(0)];
            return Some(op.clone());
        } else {
            return None;
        }
    }

    fn resolve_cast(
        &self,
        kind: &CastKind,
        dst_ty: &Ty<'tcx>,
        constraint: &VerifoptRval<'tcx>,
    ) -> VerifoptRval<'tcx> {
        match constraint {
            VerifoptRval::IdkStruct(struct_defid, _) => {
                if is_box(*struct_defid) {
                    // do not cast (will lose constraint info)
                    return constraint.clone();
                } else {
                    if self.debug {
                        println!("casting from non-box struct: {:?}", struct_defid);
                    }
                    return constraint.clone();
                }
            }
            VerifoptRval::IdkDefId(_) => todo!("casting from defid"),
            VerifoptRval::IdkStr() | VerifoptRval::IdkType(_) | VerifoptRval::Idk() => {
                let ret = VerifoptRval::IdkType(*dst_ty);
                if self.debug {
                    println!("constraint: {:?}", constraint);
                    println!("dst_ty: {:?}", dst_ty);
                    println!("ret: {:?}", ret);
                }
                return ret;
            }
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
                | CastKind::IntToFloat => return constraint.clone(),
                _ => todo!("cannot yet cast: {:?}", constraint),
            },
            _ => todo!("cannot yet cast: {:?}", constraint),
        }
    }

    fn rval_from_cast(
        &self,
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        kind: &CastKind,
        op: &Operand<'tcx>,
        ty: &Ty<'tcx>,
    ) -> Constraints<'tcx> {
        if self.debug {
            println!("in rval_from_cast");
        }

        // TODO perhaps use CastKind to help

        let mut ret_constraints = HashSet::default();
        match op {
            Operand::Copy(place) | Operand::Move(place) => {
                if self.debug {
                    println!("place.local: {:?}", place.local);
                    println!("place.projection: {:?}", place.projection);
                    println!("cur_scope: {:?}", cur_scope);
                    println!(
                        "cur_scope cmap: {:?}",
                        cmap.cmap.get(&MapKey::ScopeId(cur_scope))
                    );
                    println!(
                        "scoped_get: {:?}",
                        cmap.scoped_get(Some(cur_scope), &MapKey::Place(*place), false)
                    );
                }

                if place.projection.len() != 0 {
                    match place.projection[0] {
                        //PlaceElem::Deref => {
                        //    // FIXME essentially ignoring the deref here, when would this be wrong?
                        //    let newplace = Place {
                        //        local: Local::from_u32(place.local.as_u32()),
                        //        projection: List::empty(),
                        //    };
                        //    println!("newplace: {:?}", newplace);
                        //}
                        PlaceElem::Field(field_idx, ty) => {
                            if self.debug {
                                println!("field_idx: {:?}", field_idx);
                                println!("ty: {:?}", ty);
                            }
                            // FIXME essentially ignoring the deref here, when would this be wrong?
                            let newplace = Place {
                                local: Local::from_u32(place.local.as_u32()),
                                projection: List::empty(),
                            };
                            match cmap.scoped_get(Some(cur_scope), &MapKey::Place(newplace), false)
                            {
                                Some(VarType::Values(constraints)) => {
                                    if self.debug {
                                        println!("constraints: {:?}", constraints);
                                    }
                                    return constraints;
                                }
                                _ => panic!("unexpected res from scoped_get"),
                            }
                        }
                        _ => {
                            ret_constraints.insert(VerifoptRval::Idk());
                            return ret_constraints;
                        }
                    }
                }

                match cmap.scoped_get(Some(cur_scope), &MapKey::Place(*place), false) {
                    Some(vartype) => match vartype {
                        VarType::Values(constraints) => {
                            if self.debug {
                                println!("constraints: {:?}", constraints);
                            }
                            for constraint in constraints.iter() {
                                if self.debug {
                                    println!("GOT CONSTRAINT: {:?}", constraint);
                                }
                                ret_constraints.insert(self.resolve_cast(kind, ty, constraint));
                            }
                        }
                        _ => panic!("should not get scope (cast)"),
                    },
                    None => panic!("no val (cast)"),
                }
            }
            // FIXME
            Operand::Constant(_) => {
                ret_constraints.insert(VerifoptRval::IdkType(*ty));
            }
            _ => todo!("runtime checks (cast)"),
        }

        ret_constraints
    }

    // resolve generic params when constructing VerifoptRval
    fn handle_gen_param(
        &self,
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        defid: &DefId,
        param: &ParamTy,
    ) -> Vec<VerifoptRval<'tcx>> {
        if self.debug {
            println!("cur_scope: {:?}", cur_scope);
            println!("param.name: {:?}", param.name);
        }

        match cmap.scoped_get(Some(cur_scope), &MapKey::Generic(param.name), false) {
            Some(VarType::Values(constraints)) => {
                if self.debug {
                    println!("constraints: {:?}", constraints);
                }

                // turn HashSet constraints into Vec so can store
                // in HashMap (derive `Hash` trait)
                let mut constraint_vec = vec![];
                for constraint in constraints.iter() {
                    constraint_vec.push(constraint.clone());
                }
                return constraint_vec;
            }
            Some(_) => panic!("unexpected generic mapping (subscope)"),
            None => {
                if let Some(struct_generics) = self.funcs.struct_generics.get(defid) {
                    if self.debug {
                        println!("struct generics for {:?}: {:?}", defid, struct_generics);
                    }
                    todo!(
                        "need to add this generic to func scope/during arg res: {:?}",
                        param.name
                    );
                } else {
                    panic!("no generic mapping");
                }
            }
        }
    }

    fn resolve_genargtype(
        &self,
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        defid: &DefId,
        genarg_ty: Ty<'tcx>,
    ) -> Option<Vec<VerifoptRval<'tcx>>> {
        match genarg_ty.kind() {
            TyKind::Param(param) => {
                return Some(self.handle_gen_param(cmap, cur_scope, defid, param));
            }
            TyKind::Adt(def, genargs) => {
                if self.debug {
                    println!("adt def: {:?}", def);
                    println!("adt genargs: {:?}", genargs);
                }
                for inner_genarg in genargs.as_slice().iter() {
                    if self.debug {
                        println!("resolving: {:?}", inner_genarg);
                    }
                    return self.resolve_genargkind(cmap, cur_scope, defid, *inner_genarg);
                }
            }
            TyKind::Slice(s) => match s.kind() {
                TyKind::Int(_) | TyKind::Uint(_) => {}
                TyKind::Param(param) => {
                    return Some(self.handle_gen_param(cmap, cur_scope, defid, param));
                }
                _ => todo!("other slice tykind: {:?}", s.kind()),
            },
            TyKind::Int(_) | TyKind::Uint(_) => {}
            TyKind::Tuple(tylist) => {
                if self.debug {
                    println!("tuple types: {:?}", tylist);
                }
                if tylist.len() > 0 {
                    for ty in tylist.as_slice().iter() {
                        match ty.kind() {
                            TyKind::Param(_param) => {
                                todo!("generic in tup");
                            }
                            _ => {}
                        }
                    }
                }
            }
            TyKind::Ref(_, ty, _) => match ty.kind() {
                TyKind::Param(param) => {
                    return Some(self.handle_gen_param(cmap, cur_scope, defid, param));
                }
                _ => todo!("other ref tykind: {:?}", ty.kind()),
            },
            _ => todo!("other tykind: {:?}", genarg_ty.kind()),
        }

        return None;
    }

    fn resolve_genargkind(
        &self,
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        defid: &DefId,
        genargsref: GenericArg<'tcx>,
    ) -> Option<Vec<VerifoptRval<'tcx>>> {
        match genargsref.kind() {
            GenericArgKind::Type(ty) => return self.resolve_genargtype(cmap, cur_scope, defid, ty),
            GenericArgKind::Const(_) => {
                if self.debug {
                    println!("const genarg");
                }
                return None;
            }
            GenericArgKind::Lifetime(_) => {
                if self.debug {
                    println!("lifetime genarg");
                }
                return None;
            }
        }
    }

    fn rval_from_agg(
        &self,
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        local_decls: &IndexSlice<Local, LocalDecl<'tcx>>,
        aggkind: &Box<AggregateKind<'tcx>>,
        fields: &IndexVec<FieldIdx, Operand<'tcx>>,
    ) -> Constraints<'tcx> {
        let mut ret_constraints = HashSet::default();
        match &**aggkind {
            AggregateKind::Adt(defid, vidx, genargsref, maybe_usertyannot, maybe_fidx) => {
                if self.debug {
                    println!("--agg-adt");
                    println!("aggkind: {:?}", aggkind);
                    println!("fields: {:?}", fields);
                    println!("defid: {:?}", defid);
                    println!("vidx: {:?}", vidx);
                    println!("genargsref: {:?}", genargsref);
                    println!("maybe_usertyannot: {:?}", maybe_usertyannot);
                    println!("maybe_fidx: {:?}", maybe_fidx);

                    println!("-cur_scope: {:?}", cur_scope);
                    println!(
                        "cur_scope cmap: {:?}",
                        cmap.cmap.get(&MapKey::ScopeId(cur_scope))
                    );
                }

                if genargsref.is_empty() {
                    ret_constraints.insert(VerifoptRval::IdkStruct(*defid, None));
                } else {
                    let mut genarg_vec = vec![];
                    for i in 0..genargsref.len() {
                        if self.debug {
                            println!("genargsref at ({:?}): {:?}", i, genargsref[i]);
                        }
                        match self.resolve_genargkind(cmap, cur_scope, defid, genargsref[i]) {
                            Some(resolved) => {
                                if self.debug {
                                    println!("updating genarg_vec with: {:?}", resolved);
                                }
                                genarg_vec.push(resolved);
                            }
                            _ => {
                                if self.debug {
                                    println!("no generic args to ADT");
                                }
                            }
                        }
                    }

                    let resolved_genarg_vec = match genarg_vec.len() {
                        0 => None,
                        _ => Some(genarg_vec),
                    };
                    if self.debug {
                        println!(
                            "returningval: defid = {:?}, genargs = {:?}",
                            defid, resolved_genarg_vec,
                        );
                    }
                    ret_constraints.insert(VerifoptRval::IdkStruct(*defid, resolved_genarg_vec));
                }
            }
            AggregateKind::Closure(defid, _)
            | AggregateKind::Coroutine(defid, _)
            | AggregateKind::CoroutineClosure(defid, _) => {
                if self.debug {
                    println!("--agg-closure/coroutine");
                }
                ret_constraints.insert(VerifoptRval::IdkDefId(*defid));
            }
            // FIXME ty == type of pointee, not pointer
            AggregateKind::RawPtr(ty, _) => {
                if self.debug {
                    println!("--agg-rawptr");
                }
                ret_constraints.insert(VerifoptRval::IdkType(*ty));
            }
            AggregateKind::Array(ty) => {
                if self.debug {
                    println!("array");
                    println!("ty: {:?}", ty);
                    println!("fields: {:?}", fields);
                }
                match self.get_first_field_op(fields) {
                    Some(op) => {
                        if self.debug {
                            println!("first field op: {:?}", op);
                        }
                        return self.rval_from_op(
                            cmap,
                            cur_scope,
                            &op,
                            &op.ty(local_decls, self.tcx),
                        );
                    }
                    None => todo!("array w no fields"),
                }
            }
            AggregateKind::Tuple => {
                if self.debug {
                    println!("tup");
                    println!("fields: {:?}", fields);
                }
                match self.get_first_field_op(fields) {
                    Some(op) => {
                        if self.debug {
                            println!("first field op: {:?}", op);
                        }
                        return self.rval_from_op(
                            cmap,
                            cur_scope,
                            &op,
                            &op.ty(local_decls, self.tcx),
                        );
                    }
                    None => todo!("tup w no fields"),
                }
            }
        }

        ret_constraints
    }

    fn rval_from_op(
        &self,
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        op: &Operand<'tcx>,
        backup_ty: &Ty<'tcx>,
    ) -> Constraints<'tcx> {
        let mut constraints = HashSet::default();
        match op {
            Operand::Constant(box co) => match co.const_ {
                Const::Val(constval, ty) => match constval {
                    ConstValue::Scalar(Scalar::Int(scalar)) => {
                        if self.debug {
                            println!("scalar: {:?}", scalar);
                        }
                        constraints.insert(VerifoptRval::Scalar(scalar));
                    }
                    ConstValue::Scalar(Scalar::Ptr(_ptr, _size)) => {
                        constraints.insert(VerifoptRval::Ptr(Box::new(VerifoptRval::Idk())));
                    }
                    ConstValue::ZeroSized => {
                        if self.debug {
                            println!("zerosized");
                        }
                        constraints.insert(VerifoptRval::IdkType(ty));
                    }
                    ConstValue::Slice { alloc_id, meta } => {
                        if self.debug {
                            println!("slice");
                            println!("alloc: {:?}", alloc_id);
                            println!("meta: {:?}", meta);
                            println!("ty: {:?}", ty);
                        }
                        if ty.is_str() || ty.is_imm_ref_str() {
                            if self.debug {
                                println!("got str!");
                            }
                            constraints.insert(VerifoptRval::IdkStr());
                        } else {
                            if self.debug {
                                println!("not str");
                            }
                            constraints.insert(VerifoptRval::ConstSlice());
                        }
                    }
                    ConstValue::Indirect { .. } => {
                        if self.debug {
                            println!("indirect const");
                        }
                        constraints.insert(VerifoptRval::IndirectConst(ty));
                    }
                },
                _ => todo!("non-val const"),
            },
            Operand::Copy(place) | Operand::Move(place) => {
                constraints = self.rval_from_place(cmap, cur_scope, place, backup_ty);
            }
            _ => todo!("runtime checks"),
        }

        constraints
    }

    fn rval_from_place(
        &self,
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        place: &Place<'tcx>,
        backup_ty: &Ty<'tcx>,
    ) -> Constraints<'tcx> {
        if self.debug {
            println!("place.local: {:?}", place.local);
            println!("place.projection: {:?}", place.projection);
            println!("cur_scope: {:?}", cur_scope);
            println!(
                "cur_scope cmap: {:?}",
                cmap.cmap.get(&MapKey::ScopeId(cur_scope))
            );
            println!(
                "scoped_get: {:?}",
                cmap.scoped_get(Some(cur_scope), &MapKey::Place(*place), false)
            );
        }

        let mut ret_constraints = HashSet::default();
        let mut newplace = *place;
        if place.projection.len() != 0 {
            // not dealing w complicated projections right now, widen to backup_ty
            if place.projection.len() > 1 {
                if self.debug {
                    println!("multiple projections, using backup_ty: {:?}", backup_ty);
                }
                ret_constraints.insert(VerifoptRval::IdkType(*backup_ty));
                return ret_constraints;
            }

            match place.projection[0] {
                PlaceElem::Deref => {
                    // FIXME essentially ignoring the deref here, when would this be wrong?
                    newplace = Place {
                        local: Local::from_u32(place.local.as_u32()),
                        projection: List::empty(),
                    };
                    if self.debug {
                        println!("newplace: {:?}", newplace);
                    }
                }
                PlaceElem::Field(field_idx, ty) => {
                    if self.debug {
                        println!("field_idx: {:?}", field_idx);
                        println!("ty: {:?}", ty);
                    }
                    // FIXME
                    ret_constraints.insert(VerifoptRval::IdkType(ty));
                    return ret_constraints;
                }
                _ => {
                    ret_constraints.insert(VerifoptRval::Idk());
                    return ret_constraints;
                }
            }
        }

        match cmap.scoped_get(Some(cur_scope), &MapKey::Place(newplace), false) {
            Some(vartype) => match vartype {
                VarType::Values(constraints) => {
                    if self.debug {
                        println!("constraints: {:?}", constraints);
                        println!("backup_ty: {:?}", backup_ty);
                    }
                    ret_constraints = constraints;
                }
                _ => panic!("value should not be a scope"),
            },
            None => {
                if self.debug {
                    println!("no val for place {:?} in scope {:?}", newplace, cur_scope);
                    println!("using backup_ty: {:?}", backup_ty);
                }
                ret_constraints.insert(VerifoptRval::IdkType(*backup_ty));
            }
        }

        ret_constraints
    }
}

pub trait Merge<T> {
    fn merge(&self) -> Result<Option<T>, Error>;
}
