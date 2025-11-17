use rustc_middle::mir::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VerifoptRval {
    //Num(u32),
    //IdkNum(),
    //Var(&'static str),
    //IdkVar(),

    // str == Type name
    IdkType(&'static str),
    Idk(),
}

impl<'tcx> From<&Rvalue<'tcx>> for VerifoptRval {
    fn from(item: &Rvalue<'tcx>) -> Self {
        match item {
            _ => VerifoptRval::Idk(),
        }
    }
}

