//use rustc_hir::def::Res;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::interpret::Scalar;
use rustc_middle::mir::*;
//use rustc_span::symbol::Symbol;
//use rustc_span::Ident;
use rustc_index::IndexSlice;
use rustc_middle::ty::{GenericArgKind, List, Ty, TyCtxt, TyKind};
//use rustc_data_structures::fx::{FxHashSet as HashSet};

use crate::ConstraintMap;
use crate::constraints::{MapKey, VarType};
use crate::error::Error;

pub type Type = &'static str;

#[derive(Debug, Clone, Hash)]
pub struct FuncVal<'tcx> {
    pub def_id: DefId,
    pub is_intrinsic: bool,
    pub is_method: bool,
    pub params: Vec<(Place<'tcx>, Ty<'tcx>)>,
    pub rettype: Option<Ty<'tcx>>,
}

impl<'tcx> FuncVal<'tcx> {
    pub fn new(
        def_id: DefId,
        is_intrinsic: bool,
        is_method: bool,
        arg_names: Vec<Place<'tcx>>,
        arg_types_opt: Option<Vec<Ty<'tcx>>>,
        rettype: Option<Ty<'tcx>>,
    ) -> FuncVal<'tcx> {
        let params;
        if let Some(arg_types) = arg_types_opt {
            params = std::iter::zip(arg_names, arg_types).collect();
        } else {
            // dummy value for functions that do not have MIR available (and therefore cannot get
            // argument types the way we generally get them0 -- this is probably fine since we
            // won't be executing this particular MIR body anyway
            params = vec![];
        }

        Self {
            def_id,
            is_intrinsic,
            is_method,
            params,
            rettype,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VerifoptRval<'tcx> {
    IdkStruct(DefId, Option<Vec<Vec<VerifoptRval<'tcx>>>>),
    Scalar(Scalar),
    Str(Const<'tcx>),
    ConstSlice(),
    Ptr(Box<VerifoptRval<'tcx>>),
    //Ref(Box<VerifoptRval<'tcx>>),
    IdkType(Ty<'tcx>),
    IdkDefId(DefId),
    Idk(),
    Undef(),
}

impl<'tcx> VerifoptRval<'tcx> {
    pub fn from_rvalue(
        tcx: TyCtxt<'tcx>,
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        local_decls: &IndexSlice<Local, LocalDecl<'tcx>>,
        item: &Rvalue<'tcx>,
        debug: bool,
    ) -> Self {
        println!("\n### IN FROM_RVALUE, item is: {:?}", item);
        match item {
            Rvalue::Cast(kind, op, ty) => {
                if debug {
                    println!("--cast");
                    println!("kind: {:?}", kind);
                    println!("op: {:?}", op);
                    println!("ty: {:?}", ty);
                }
                VerifoptRval::rval_from_cast(cmap, cur_scope, kind, op, ty)
            }
            Rvalue::Aggregate(aggkind, fields) => match &**aggkind {
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
                            println!("genargsref at ({:?}): {:?}", i, genargsref[i]);
                            match genargsref[i].kind() {
                                GenericArgKind::Type(ty) => match ty.kind() {
                                    TyKind::Param(param) => {
                                        println!("param.name: {:?}", param.name);
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
                                            _ => panic!("unexpected generic mapping"),
                                        }
                                    }
                                    _ => {}
                                },
                                GenericArgKind::Const(_) => println!("const"),
                                _ => {}
                            }
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
                    VerifoptRval::IdkDefId(*defid)
                }
                // FIXME ty == type of pointee, not pointer
                AggregateKind::RawPtr(ty, _) => {
                    if debug {
                        println!("--agg-rawpotr");
                    }
                    VerifoptRval::IdkType(*ty)
                }
                AggregateKind::Array(_) => todo!("array"),
                AggregateKind::Tuple => todo!("tup"),
            },
            Rvalue::Use(op) => {
                if debug {
                    println!("--use");
                    println!("op: {:?}", op);
                }
                VerifoptRval::rval_from_op(cmap, cur_scope, op, &op.ty(local_decls, tcx))
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
                );
                VerifoptRval::Ptr(Box::new(inner))
            }
            /////////////////////////////////////
            Rvalue::Ref(_, _, place) => {
                if debug {
                    println!("--ref");
                }
                VerifoptRval::IdkType(place.ty(local_decls, tcx).ty)
            }
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

    fn rval_from_cast(
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        _kind: &CastKind,
        op: &Operand<'tcx>,
        ty: &Ty<'tcx>,
    ) -> VerifoptRval<'tcx> {
        match op {
            Operand::Copy(place) | Operand::Move(place) => {
                println!("place.local: {:?}", place.local);
                println!("place.projection: {:?}", place.projection);
                //println!("cur_scope: {:?}", cur_scope);
                //println!(
                //    "cur_scope cmap: {:?}",
                //    cmap.cmap.get(&MapKey::ScopeId(cur_scope))
                //);
                //println!(
                //    "scoped_get: {:?}",
                //    cmap.scoped_get(Some(cur_scope), &MapKey::Place(*place), false)
                //);

                if place.projection.len() != 0 {
                    match place.projection[0] {
                        //PlaceElem::Deref => {
                        //    // FIXME essentially ignoring the deref here, when would this be wrong?
                        //    newplace = Place {
                        //        local: Local::from_u32(place.local.as_u32()),
                        //        projection: List::empty(),
                        //    };
                        //    println!("newplace: {:?}", newplace);
                        //}
                        PlaceElem::Field(field_idx, ty) => {
                            println!("field_idx: {:?}", field_idx);
                            println!("ty: {:?}", ty);
                            //todo!("field");
                            // FIXME
                            return VerifoptRval::IdkType(ty);
                        }
                        _ => return VerifoptRval::Idk(),
                    }
                }

                match cmap.scoped_get(Some(cur_scope), &MapKey::Place(*place), false) {
                    Some(vartype) => match vartype {
                        VarType::Values(constraints) => {
                            println!("constraints: {:?}", constraints);
                            // FIXME
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

    fn rval_from_op(
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        op: &Operand<'tcx>,
        backup_ty: &Ty<'tcx>,
    ) -> VerifoptRval<'tcx> {
        match op {
            Operand::Constant(box co) => match co.const_ {
                Const::Val(constval, ty) => match constval {
                    ConstValue::Scalar(scalar) => {
                        println!("scalar");
                        VerifoptRval::Scalar(scalar)
                    }
                    ConstValue::ZeroSized => {
                        println!("zerosized");
                        VerifoptRval::IdkType(ty)
                    }
                    ConstValue::Slice { alloc_id, meta } => {
                        println!("slice");
                        println!("alloc: {:?}", alloc_id);
                        println!("meta: {:?}", meta);
                        println!("ty: {:?}", ty);
                        if ty.is_str() || ty.is_imm_ref_str() {
                            println!("got str!");
                            return VerifoptRval::Str(co.const_);
                        } else {
                            println!("not str");
                            return VerifoptRval::ConstSlice();
                        }
                    }
                    ConstValue::Indirect { .. } => {
                        println!("indirect");
                        VerifoptRval::IdkType(ty)
                    }
                },
                _ => todo!("non-val const"),
            },
            Operand::Copy(place) | Operand::Move(place) => {
                VerifoptRval::rval_from_place(cmap, cur_scope, place, backup_ty)
            }
            _ => todo!("runtime checks"),
        }
    }

    fn rval_from_place(
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        place: &Place<'tcx>,
        backup_ty: &Ty<'tcx>,
    ) -> VerifoptRval<'tcx> {
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

        let mut newplace = *place;
        if place.projection.len() != 0 {
            match place.projection[0] {
                PlaceElem::Deref => {
                    // FIXME essentially ignoring the deref here, when would this be wrong?
                    newplace = Place {
                        local: Local::from_u32(place.local.as_u32()),
                        projection: List::empty(),
                    };
                    println!("newplace: {:?}", newplace);
                }
                PlaceElem::Field(field_idx, ty) => {
                    println!("field_idx: {:?}", field_idx);
                    println!("ty: {:?}", ty);
                    //todo!("field");
                    // FIXME
                    return VerifoptRval::IdkType(ty);
                }
                _ => return VerifoptRval::Idk(),
            }
        }

        match cmap.scoped_get(Some(cur_scope), &MapKey::Place(newplace), false) {
            Some(vartype) => match vartype {
                VarType::Values(constraints) => {
                    println!("constraints: {:?}", constraints);
                    println!("backup_ty: {:?}", backup_ty);
                    // FIXME think about how to return multiple possible VerifoptRvals
                    // (for a constraint set of len > 1)
                    if constraints.len() != 1 {
                        panic!("unexpected constraint length: {:?}", constraints.len());
                    }
                    let mut ret = VerifoptRval::IdkType(*backup_ty);
                    for sole_constraint in constraints.clone().drain() {
                        ret = sole_constraint;
                    }
                    ret
                }
                _ => panic!("value should not be a scope"),
            },
            None => {
                panic!("no val!");
            }
        }
    }
}

pub trait Merge<T> {
    fn merge(&self) -> Result<Option<T>, Error>;
}
