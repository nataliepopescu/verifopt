use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;

pub type FuncName = &'static str;

pub type Type = &'static str;

#[derive(Debug, Clone, Hash)]
pub struct FuncVal {
    pub name: &'static str,
    pub is_method: bool,
    pub params: Vec<(&'static str, Type)>,
    pub rettype: Option<Box<Type>>,
    pub body_def_id: DefId,
    // FIXME DefId instead
    //pub body: &'a Body<'tcx>,
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

