//use rustc_hir::def::Res;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::interpret::Scalar;
use rustc_middle::mir::*;
//use rustc_span::symbol::Symbol;
//use rustc_span::Ident;
use rustc_index::IndexSlice;
use rustc_middle::ty::{Ty, TyCtxt};

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
        local_decls: &IndexSlice<Local, LocalDecl<'tcx>>,
        item: &Rvalue<'tcx>,
    ) -> Self {
        match item {
            Rvalue::Use(op) => VerifoptRval::IdkType(op.ty(local_decls, tcx)),
            Rvalue::Ref(_, _, place) => VerifoptRval::IdkType(place.ty(local_decls, tcx).ty),
            Rvalue::RawPtr(_, place) => VerifoptRval::IdkType(place.ty(local_decls, tcx).ty),
            Rvalue::Cast(_, _, ty) => VerifoptRval::IdkType(*ty),
            Rvalue::Repeat(_, _) => todo!("repeat"),
            Rvalue::ThreadLocalRef(_) => todo!("thread local ref"),
            Rvalue::BinaryOp(binop, boxed_op_tup) => VerifoptRval::IdkType(binop.ty(
                tcx,
                boxed_op_tup.0.ty(local_decls, tcx),
                boxed_op_tup.1.ty(local_decls, tcx),
            )),
            Rvalue::UnaryOp(unop, op) => {
                VerifoptRval::IdkType(unop.ty(tcx, op.ty(local_decls, tcx)))
            }
            Rvalue::Discriminant(place) => VerifoptRval::IdkType(place.ty(local_decls, tcx).ty),
            Rvalue::Aggregate(aggkind, _) => match &**aggkind {
                AggregateKind::Array(_) => todo!("array"),
                AggregateKind::Tuple => todo!("tup"),
                AggregateKind::Adt(defid, ..) => VerifoptRval::Idk(*defid),
                AggregateKind::Closure(defid, _)
                | AggregateKind::Coroutine(defid, _)
                | AggregateKind::CoroutineClosure(defid, _) => VerifoptRval::Idk(*defid),
                // FIXME ty == type of pointee, not pointer
                AggregateKind::RawPtr(ty, _) => VerifoptRval::IdkType(*ty),
            },
            Rvalue::ShallowInitBox(_, ty) => VerifoptRval::IdkType(*ty),
            Rvalue::CopyForDeref(place) => VerifoptRval::IdkType(place.ty(local_decls, tcx).ty),
            Rvalue::WrapUnsafeBinder(_, ty) => VerifoptRval::IdkType(*ty),
        }
    }

    // FIXME bring back below functions if want more control over returned types
    /*
    fn rval_from_op(
        tcx: TyCtxt<'tcx>,
        local_decls: &IndexSlice<Local, LocalDecl<'tcx>>,
        op: &Operand<'tcx>,
    ) -> VerifoptRval<'tcx> {
        match op {
            Operand::Constant(box co) => match co.const_ {
                Const::Val(_, ty) => VerifoptRval::IdkType(ty),
                _ => todo!("non-val const"), //VerifoptRval::Idk("not-val const"),
            },
            Operand::Copy(place) | Operand::Move(place) => {
                VerifoptRval::rval_from_place(local_decls, place)
            }
            _ => todo!("runtime checks"), //VerifoptRval::Idk("runtime check"),
        }
    }

    fn rval_from_place(
        tcx: TyCtxt<'tcx>,
        local_decls: &IndexSlice<Local, LocalDecl<'tcx>>,
        place: &Place<'tcx>,
    ) -> VerifoptRval<'tcx> {
        println!("place.loc: {:?}", place.local);
        println!("place.proj: {:?}", place.projection);
        let local = place.local_or_deref_local().unwrap();
        let local_decl = local_decls.get(local).unwrap();
        VerifoptRval::Ref(Box::new(VerifoptRval::IdkType(local_decl.ty)))
    }
    */
}

pub trait Merge<T> {
    fn merge(&self) -> Result<Option<T>, Error>;
}
