//use rustc_hir::def::Res;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::interpret::Scalar;
use rustc_middle::mir::*;
//use rustc_span::symbol::Symbol;
//use rustc_span::Ident;
use rustc_abi::FieldIdx;
use rustc_index::{IndexSlice, IndexVec};
use rustc_middle::ty::{GenericArgKind, List, ParamTy, Ty, TyCtxt, TyKind};
//use rustc_data_structures::fx::{FxHashSet as HashSet};

use crate::ConstraintMap;
use crate::FuncMap;
use crate::constraints::{MapKey, VarType};
use crate::error::Error;

pub type Type = &'static str;

#[derive(Debug, Clone, Hash)]
pub struct FuncVal<'tcx> {
    pub def_id: DefId,
    pub is_intrinsic: bool,
    pub self_arg: Option<Place<'tcx>>,
    pub params: Vec<(Place<'tcx>, Ty<'tcx>)>,
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
    Scalar(Scalar),
    ConstSlice(),
    Ptr(Box<VerifoptRval<'tcx>>),
    Ref(Box<VerifoptRval<'tcx>>),
    IdkStruct(DefId, Option<Vec<Vec<VerifoptRval<'tcx>>>>),
    IdkStr(), //Const<'tcx>),
    // FIXME don't want types
    IdkType(Ty<'tcx>),
    IdkDefId(DefId),
    Idk(),
    Undef(),
}

impl<'tcx> VerifoptRval<'tcx> {
    pub fn get_first_field_op(fields: &IndexVec<FieldIdx, Operand<'tcx>>) -> Option<Operand<'tcx>> {
        let fields_slice = fields.as_slice();
        if fields_slice.len() == 1 {
            let op = &fields_slice[FieldIdx::from_u32(0)];
            return Some(op.clone());
        } else {
            return None;
        }
    }

    pub fn from_rvalue(
        tcx: TyCtxt<'tcx>,
        funcs: &FuncMap<'tcx>,
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        local_decls: &IndexSlice<Local, LocalDecl<'tcx>>,
        item: &Rvalue<'tcx>,
        debug: bool,
    ) -> Self {
        if debug {
            println!("\n### IN FROM_RVALUE, item is: {:?}", item);
        }

        match item {
            Rvalue::Cast(kind, op, ty) => {
                if debug {
                    println!("--cast");
                    println!("kind: {:?}", kind);
                    println!("op: {:?}", op);
                    println!("ty: {:?}", ty);
                }
                VerifoptRval::rval_from_cast(cmap, cur_scope, kind, op, ty, debug)
            }
            Rvalue::Aggregate(aggkind, fields) => {
                if debug {
                    println!("--agg");
                    println!("aggkind: {:?}", aggkind);
                    println!("fields: {:?}", fields);
                }
                VerifoptRval::rval_from_agg(
                    tcx,
                    funcs,
                    cmap,
                    cur_scope,
                    local_decls,
                    aggkind,
                    fields,
                    debug,
                )
            }
            Rvalue::Use(op) => {
                if debug {
                    println!("--use");
                    println!("op: {:?}", op);
                }
                VerifoptRval::rval_from_op(cmap, cur_scope, op, &op.ty(local_decls, tcx), debug)
            }
            Rvalue::RawPtr(kind, place) => {
                if debug {
                    println!("--rawptr");
                    println!("kind: {:?}", kind);
                    println!("place: {:?}", place);
                }
                let inner = VerifoptRval::rval_from_place(
                    cmap,
                    cur_scope,
                    place,
                    &place.ty(local_decls, tcx).ty,
                    debug,
                );
                VerifoptRval::Ptr(Box::new(inner))
            }
            Rvalue::Ref(_, _, place) => {
                if debug {
                    println!("--ref");
                }
                VerifoptRval::Ref(Box::new(VerifoptRval::rval_from_place(
                    cmap,
                    cur_scope,
                    place,
                    &place.ty(local_decls, tcx).ty,
                    debug,
                )))
            }
            /////////////////////////////////////
            // the below rvals all widen to Type
            /////////////////////////////////////
            Rvalue::BinaryOp(binop, boxed_op_tup) => {
                if debug {
                    println!("--binop");
                }
                VerifoptRval::IdkType(binop.ty(
                    tcx,
                    boxed_op_tup.0.ty(local_decls, tcx),
                    boxed_op_tup.1.ty(local_decls, tcx),
                ))
            }
            Rvalue::UnaryOp(unop, op) => {
                if debug {
                    println!("--unop");
                }
                VerifoptRval::IdkType(unop.ty(tcx, op.ty(local_decls, tcx)))
            }
            Rvalue::Discriminant(place) => {
                if debug {
                    println!("--discr");
                }
                VerifoptRval::IdkType(place.ty(local_decls, tcx).ty)
            }
            Rvalue::ShallowInitBox(_, ty) => {
                if debug {
                    println!("--shallowinitbox");
                }
                VerifoptRval::IdkType(*ty)
            }
            Rvalue::CopyForDeref(place) => {
                if debug {
                    println!("--copyforderef");
                }
                VerifoptRval::IdkType(place.ty(local_decls, tcx).ty)
            }
            Rvalue::WrapUnsafeBinder(_, ty) => {
                if debug {
                    println!("--wrapunsafebinder");
                }
                VerifoptRval::IdkType(*ty)
            }
            Rvalue::Repeat(_, _) => todo!("repeat"),
            Rvalue::ThreadLocalRef(_) => todo!("thread local ref"),
        }
    }

    fn resolve_cast(kind: &CastKind, dst_ty: &Ty<'tcx>, constraint: &VerifoptRval<'tcx>, debug: bool) -> VerifoptRval<'tcx> {
        match constraint {
            VerifoptRval::IdkStruct(_, _)
            | VerifoptRval::IdkStr()
            | VerifoptRval::IdkType(_)
            | VerifoptRval::IdkDefId(_)
            | VerifoptRval::Idk() => {
                // FIXME if we always use the dst_ty then we'll always be loosing
                // information
                let ret = VerifoptRval::IdkType(*dst_ty);
                if debug {
                    println!("dst_ty: {:?}", dst_ty);
                    println!("*dst_ty: {:?}", *dst_ty);
                    println!("ret: {:?}", ret);
                }
                return ret;
                //todo!("just change the type: {:?}", constraint);
            }
            VerifoptRval::Ptr(inner) => VerifoptRval::Ptr(Box::new(Self::resolve_cast(kind, dst_ty, &*inner, debug))),
            _ => todo!("cannot yet cast: {:?}", constraint),
        }
    }

    fn rval_from_cast(
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        kind: &CastKind,
        op: &Operand<'tcx>,
        ty: &Ty<'tcx>,
        debug: bool,
    ) -> VerifoptRval<'tcx> {
        if debug {
            println!("in rval_from_cast");
        }

        // TODO perhaps use CastKind to help

        match op {
            Operand::Copy(place) | Operand::Move(place) => {
                if debug {
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
                            if debug {
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
                                    if debug {
                                        println!("constraints: {:?}", constraints);
                                    }
                                    if constraints.len() != 1 {
                                        panic!(
                                            "handle other constraint lens: {:?}",
                                            constraints.len()
                                        );
                                    }
                                    for constraint in constraints.iter() {
                                        return constraint.clone();
                                    }
                                    // FIXME
                                    if debug {
                                        println!("should not print......");
                                    }
                                    return VerifoptRval::IdkType(ty);
                                }
                                _ => panic!("unexpected res from scoped_get"),
                            }
                        }
                        _ => return VerifoptRval::Idk(),
                    }
                }

                match cmap.scoped_get(Some(cur_scope), &MapKey::Place(*place), false) {
                    Some(vartype) => match vartype {
                        VarType::Values(constraints) => {
                            if debug {
                                println!("constraints: {:?}", constraints);
                            }
                            if constraints.len() != 1 {
                                todo!("handle other lengths: {:?}", constraints.len());
                            }
                            // FIXME change the type!
                            for constraint in constraints.iter() {
                                if debug {
                                    println!("GOT CONSTRAINT: {:?}", constraint);
                                }
                                return Self::resolve_cast(kind, ty, constraint, debug);
                            }
                            // FIXME
                            if debug {
                                println!("should not print......");
                            }
                            VerifoptRval::IdkType(*ty)
                        }
                        _ => panic!("should not get scope (cast)"),
                    },
                    None => panic!("no val (cast)"),
                }
            }
            // FIXME
            Operand::Constant(_) => VerifoptRval::IdkType(*ty),
            _ => todo!("runtime checks (cast)"),
        }
    }

    fn rval_from_agg(
        tcx: TyCtxt<'tcx>,
        funcs: &FuncMap<'tcx>,
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        local_decls: &IndexSlice<Local, LocalDecl<'tcx>>,
        aggkind: &Box<AggregateKind<'tcx>>,
        fields: &IndexVec<FieldIdx, Operand<'tcx>>,
        debug: bool,
    ) -> VerifoptRval<'tcx> {
        match &**aggkind {
            AggregateKind::Adt(defid, vidx, genargsref, maybe_usertyannot, maybe_fidx) => {
                if debug {
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
                    return VerifoptRval::IdkStruct(*defid, None);
                } else {
                    let mut genarg_vec = vec![];
                    for i in 0..genargsref.len() {
                        if debug {
                            println!("genargsref at ({:?}): {:?}", i, genargsref[i]);
                        }
                        match genargsref[i].kind() {
                            GenericArgKind::Type(ty) => match ty.kind() {
                                TyKind::Param(param) => {
                                    if debug {
                                        println!("cur_scope: {:?}", cur_scope);
                                        println!("param.name: {:?}", param.name);
                                    }
                                    match cmap.scoped_get(
                                        Some(cur_scope),
                                        &MapKey::Generic(param.name),
                                        false,
                                    ) {
                                        Some(VarType::Values(constraints)) => {
                                            // turn HashSet constraints into Vec so can store
                                            // in HashMap (derive `Hash` trait)
                                            let mut constraint_vec = vec![];
                                            for constraint in constraints.iter() {
                                                constraint_vec.push(constraint.clone());
                                            }
                                            genarg_vec.push(constraint_vec);
                                        }
                                        Some(_) => panic!("unexpected generic mapping"),
                                        None => {
                                            if debug {
                                                println!("{:?}", funcs.struct_generics.get(defid));
                                            }
                                            panic!("no generic mapping");
                                        }
                                    }
                                }
                                // TODO
                                TyKind::Adt(def, genargs) => {
                                    if debug {
                                        println!("adt def: {:?}", def);
                                        println!("adt genargs: {:?}", genargs);
                                    }
                                }
                                _ => {
                                    if debug {
                                        println!("other ty kind: {:?}", ty.kind());
                                    }
                                }
                            },
                            GenericArgKind::Const(_) => {
                                if debug {
                                    println!("const genarg");
                                }
                            }
                            GenericArgKind::Lifetime(_) => {
                                if debug {
                                    println!("lifetime genarg");
                                }
                            }
                        }
                    }
                    if debug {
                        println!(
                            "returningval: defid = {:?}, genargs = {:?}",
                            defid,
                            Some(genarg_vec.clone())
                        );
                    }
                    return VerifoptRval::IdkStruct(*defid, Some(genarg_vec));
                }
            }
            AggregateKind::Closure(defid, _)
            | AggregateKind::Coroutine(defid, _)
            | AggregateKind::CoroutineClosure(defid, _) => {
                if debug {
                    println!("--agg-closure/coroutine");
                }
                return VerifoptRval::IdkDefId(*defid);
            }
            // FIXME ty == type of pointee, not pointer
            AggregateKind::RawPtr(ty, _) => {
                if debug {
                    println!("--agg-rawptr");
                }
                return VerifoptRval::IdkType(*ty);
            }
            AggregateKind::Array(ty) => {
                if debug {
                    println!("array");
                    println!("ty: {:?}", ty);
                    println!("fields: {:?}", fields);
                }
                match Self::get_first_field_op(fields) {
                    Some(op) => {
                        if debug {
                            println!("first field op: {:?}", op);
                        }
                        return VerifoptRval::rval_from_op(
                            cmap,
                            cur_scope,
                            &op,
                            &op.ty(local_decls, tcx),
                            debug,
                        );
                    }
                    None => todo!("array w no fields"),
                }
            }
            AggregateKind::Tuple => {
                if debug {
                    println!("tup");
                    println!("fields: {:?}", fields);
                }
                match Self::get_first_field_op(fields) {
                    Some(op) => {
                        if debug {
                            println!("first field op: {:?}", op);
                        }
                        return VerifoptRval::rval_from_op(
                            cmap,
                            cur_scope,
                            &op,
                            &op.ty(local_decls, tcx),
                            debug,
                        );
                    }
                    None => todo!("tup w no fields"),
                }
            }
        }
    }

    fn rval_from_op(
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        op: &Operand<'tcx>,
        backup_ty: &Ty<'tcx>,
        debug: bool,
    ) -> VerifoptRval<'tcx> {
        match op {
            Operand::Constant(box co) => match co.const_ {
                Const::Val(constval, ty) => match constval {
                    ConstValue::Scalar(scalar) => {
                        if debug {
                            println!("scalar");
                        }
                        VerifoptRval::Scalar(scalar)
                    }
                    ConstValue::ZeroSized => {
                        if debug {
                            println!("zerosized");
                        }
                        VerifoptRval::IdkType(ty)
                    }
                    ConstValue::Slice { alloc_id, meta } => {
                        if debug {
                            println!("slice");
                            println!("alloc: {:?}", alloc_id);
                            println!("meta: {:?}", meta);
                            println!("ty: {:?}", ty);
                        }
                        if ty.is_str() || ty.is_imm_ref_str() {
                            if debug {
                                println!("got str!");
                            }
                            return VerifoptRval::IdkStr();
                        } else {
                            if debug {
                                println!("not str");
                            }
                            return VerifoptRval::ConstSlice();
                        }
                    }
                    ConstValue::Indirect { .. } => {
                        if debug {
                            println!("indirect");
                        }
                        VerifoptRval::IdkType(ty)
                    }
                },
                _ => todo!("non-val const"),
            },
            Operand::Copy(place) | Operand::Move(place) => {
                VerifoptRval::rval_from_place(cmap, cur_scope, place, backup_ty, debug)
            }
            _ => todo!("runtime checks"),
        }
    }

    fn rval_from_place(
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        place: &Place<'tcx>,
        backup_ty: &Ty<'tcx>,
        debug: bool,
    ) -> VerifoptRval<'tcx> {
        if debug {
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

        let mut newplace = *place;
        if place.projection.len() != 0 {
            // not dealing w complicated projections right now, widen to backup_ty
            if place.projection.len() > 1 {
                if debug {
                    println!("multiple projections, using backup_ty: {:?}", backup_ty);
                }
                return VerifoptRval::IdkType(*backup_ty);
            }

            match place.projection[0] {
                PlaceElem::Deref => {
                    // FIXME essentially ignoring the deref here, when would this be wrong?
                    newplace = Place {
                        local: Local::from_u32(place.local.as_u32()),
                        projection: List::empty(),
                    };
                    if debug {
                        println!("newplace: {:?}", newplace);
                    }
                }
                PlaceElem::Field(field_idx, ty) => {
                    if debug {
                        println!("field_idx: {:?}", field_idx);
                        println!("ty: {:?}", ty);
                    }
                    // FIXME
                    return VerifoptRval::IdkType(ty);
                }
                _ => return VerifoptRval::Idk(),
            }
        }

        let mut ret_rval = VerifoptRval::IdkType(*backup_ty);
        match cmap.scoped_get(Some(cur_scope), &MapKey::Place(newplace), false) {
            Some(vartype) => match vartype {
                VarType::Values(constraints) => {
                    if debug {
                        println!("constraints: {:?}", constraints);
                        println!("backup_ty: {:?}", backup_ty);
                    }
                    // FIXME think about how to return multiple possible VerifoptRvals
                    // (for a constraint set of len > 1)
                    if constraints.len() != 1 {
                        panic!("unexpected constraint length: {:?}", constraints.len());
                    }
                    for sole_constraint in constraints.clone().drain() {
                        ret_rval = sole_constraint;
                    }
                }
                _ => panic!("value should not be a scope"),
            },
            None => {
                if debug {
                    println!("no val for place {:?} in scope {:?}", newplace, cur_scope);
                    println!("using backup_ty: {:?}", backup_ty);
                }
            }
        }

        ret_rval
    }
}

pub trait Merge<T> {
    fn merge(&self) -> Result<Option<T>, Error>;
}
