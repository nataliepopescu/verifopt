extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;
use rustc_public::rustc_internal;

use crate::start_safeose;
use crate::util::options::AnalysisOptions;

pub struct SafeOSECallbacks {
    pub options: AnalysisOptions,
}

impl Callbacks for SafeOSECallbacks {
    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        rustc_internal::run(tcx, || {
            start_safeose(self.options.clone());
        }).unwrap();

        Compilation::Continue
    }
}
