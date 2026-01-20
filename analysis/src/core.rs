use rustc_hir::def::Res;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::mir::interpret::Scalar;
use rustc_span::symbol::Symbol;
use rustc_span::Ident;
use rustc_index::IndexSlice;
use rustc_middle::ty::Ty;

use crate::error::Error;

pub type Type = &'static str;

// FIXME continue to add stuff to this def as needed
#[derive(Debug, Clone, Hash)]
pub struct FuncVal {
    pub def_id: DefId,
    //pub name: Symbol,
    //pub is_method: bool,
    //pub params: Vec<(Place<'tcx>, Res)>,
    // FIXME names only for now
    pub params: Vec<Option<Ident>>, //Place<'tcx>>,
    //pub rettype: Option<Res>,
}

impl FuncVal {
    pub fn new(
        def_id: DefId,
        params: Vec<Option<Ident>>, //Place<'tcx>>,
    ) -> FuncVal {
        Self { def_id, params }
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
    Scalar(Scalar),
    Ref(Box<VerifoptRval<'tcx>>),
    IdkType(Ty<'tcx>),
    Idk(&'static str),
}

impl<'tcx> VerifoptRval<'tcx> {
    pub fn from_rvalue(
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        item: &Rvalue<'tcx>,
    ) -> Self {
        match item {
            Rvalue::Use(op) => {
                match op {
                    Operand::Constant(box co) => {
                        match co.const_ {
                            Const::Val(_, ty) => {
                                VerifoptRval::IdkType(ty)
                            },
                            _ => VerifoptRval::Idk("not-val const"),
                        }
                    }
                    _ => VerifoptRval::Idk("not-const (copy/move op)"),
                }
            },
            Rvalue::Ref(_, _, place) => {
                println!("place.loc: {:?}", place.local);
                println!("place.proj: {:?}", place.projection);
                let local = place.local_or_deref_local().unwrap();
                let local_decl = body_locals.get(local).unwrap();
                VerifoptRval::Ref(
                    Box::new(
                        VerifoptRval::IdkType(local_decl.ty)
                    )
                )
            },
            _ => VerifoptRval::Idk("idk"),
        }
    }
}

pub trait Merge<T> {
    fn merge(&self) -> Result<Option<T>, Error>;
}



