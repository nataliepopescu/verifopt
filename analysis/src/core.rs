use rustc_hir::def::Res;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_span::symbol::Symbol;

pub type Type = &'static str;

#[derive(Debug, Clone, Hash)]
pub struct FuncVal<'tcx> {
    pub def_id: DefId,
    pub name: Symbol,
    pub is_method: bool,
    pub params: Vec<(Place<'tcx>, Res)>,
    pub rettype: Option<Res>,
    //pub body: &'a Body<'tcx>,
}

impl<'tcx> FuncVal<'tcx> {
    pub fn new(
        def_id: DefId,
        name: Symbol,
        is_method: bool,
        arg_names: Vec<Place<'tcx>>,
        arg_types: Vec<Res>,
        rettype: Option<Res>,
    ) -> FuncVal {
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VerifoptRval {
    //Num(u32),
    //IdkNum(),
    //Var(&'static str),
    //IdkVar(),

    IdkType(Type),
    Idk(),
}

impl<'tcx> From<&Rvalue<'tcx>> for VerifoptRval {
    fn from(item: &Rvalue<'tcx>) -> Self {
        match item {
            _ => VerifoptRval::Idk(),
        }
    }
}

