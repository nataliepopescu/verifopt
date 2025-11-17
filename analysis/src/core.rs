use rustc_middle::mir::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VerifoptRval {
    //I32(i32),
    //I64(i64),
    //U32(u32),
    //U64(u64),
    //USIZE(usize),
    Num(u32),
    IdkNum(),
    Var(&'static str),
    IdkVar(),
    Idk(),
}

impl<'tcx> From<&Rvalue<'tcx>> for VerifoptRval {
    fn from(item: &Rvalue<'tcx>) -> Self {
        match item {
            _ => VerifoptRval::Idk(),
        }
    }
}

