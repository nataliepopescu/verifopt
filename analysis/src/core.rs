//use rustc_hir::def::Res;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::interpret::Scalar;
use rustc_middle::mir::*;
//use rustc_span::symbol::Symbol;
//use rustc_span::Ident;
use rustc_index::IndexSlice;
use rustc_middle::ty::{Ty, TyCtxt};

use crate::ConstraintMap;
use crate::constraints::{MapKey, VarType};
use crate::error::Error;

pub type Type = &'static str;

// FIXME continue to add stuff to this def as needed
#[derive(Debug, Clone, Hash)]
pub struct FuncVal<'tcx> {
    pub def_id: DefId,
    pub is_intrinsic: bool,
    pub is_method: bool,
    //pub params: Vec<(Place<'tcx>, Res)>,
    // FIXME param names only for now
    pub params: Vec<Place<'tcx>>,
    pub rettype: Option<Ty<'tcx>>,
}

impl<'tcx> FuncVal<'tcx> {
    pub fn new(
        def_id: DefId,
        is_intrinsic: bool,
        is_method: bool,
        params: Vec<Place<'tcx>>,
        rettype: Option<Ty<'tcx>>,
    ) -> FuncVal<'tcx> {
        Self {
            def_id,
            is_intrinsic,
            is_method,
            params,
            rettype,
        }
    }

    /*
    pub fn new(
        def_id: DefId,
        name: Symbol,
        is_method: bool,
        arg_names: Vec<Place<'tcx>>,
        arg_types: Vec<Res>,
        rettype: Option<Res>,
    ) -> FuncVal<'tcx> {
        let params;
        if arg_names.len() == arg_types.len() {
            params = std::iter::zip(arg_names, arg_types).collect();
        } else {
            panic!("arg_names.len() != arg_types.len()");
        }

        Self {
            def_id, name, is_method, params, rettype
        }
    }
    */
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VerifoptRval<'tcx> {
    //Struct(DefId),
    Scalar(Scalar),
    //Ref(Box<VerifoptRval<'tcx>>),
    IdkType(Ty<'tcx>),
    Idk(DefId),
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
            Rvalue::Cast(_, op, ty) => {
                if debug {
                    println!("--cast");
                }
                VerifoptRval::rval_from_op(cmap, cur_scope, op, ty)
            }

            Rvalue::Use(op) => {
                if debug {
                    println!("--use");
                }
                VerifoptRval::IdkType(op.ty(local_decls, tcx))
            }
            Rvalue::Ref(_, _, place) => {
                if debug {
                    println!("--ref");
                }
                VerifoptRval::IdkType(place.ty(local_decls, tcx).ty)
            }
            Rvalue::RawPtr(_, place) => {
                if debug {
                    println!("--rawptr");
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
            Rvalue::Aggregate(aggkind, _) => match &**aggkind {
                AggregateKind::Adt(defid, ..) => {
                    if debug {
                        println!("--agg-adt");
                    }
                    VerifoptRval::Idk(*defid)
                }
                AggregateKind::Closure(defid, _)
                | AggregateKind::Coroutine(defid, _)
                | AggregateKind::CoroutineClosure(defid, _) => {
                    if debug {
                        println!("--agg-closure/coroutine");
                    }
                    VerifoptRval::Idk(*defid)
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

    // instead of using the type system to resolve value "constraints", use our cmap
    fn rval_from_op(
        cmap: &ConstraintMap<'tcx>,
        cur_scope: DefId,
        op: &Operand<'tcx>,
        backup_ty: &Ty<'tcx>,
    ) -> VerifoptRval<'tcx> {
        match op {
            Operand::Constant(box co) => match co.const_ {
                Const::Val(_, ty) => {
                    // FIXME don't use ty
                    VerifoptRval::IdkType(ty)
                }
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

        if place.projection.len() != 0 {
            todo!("handle projections");
        }

        match cmap.scoped_get(Some(cur_scope), &MapKey::Place(*place), false) {
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
