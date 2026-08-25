extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;

use crate::start_safeose;
use crate::util::options::AnalysisOptions;

pub struct SafeOSECallbacks {
    pub options: AnalysisOptions,
}

impl Callbacks for SafeOSECallbacks {
    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, _tcx: TyCtxt<'tcx>) -> Compilation {
        start_safeose(self.options.clone());

        Compilation::Continue
    }
}
