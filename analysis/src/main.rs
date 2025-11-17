#![feature(rustc_private)]
#![feature(box_patterns)]

extern crate rustc_driver;
extern crate rustc_data_structures;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;

mod constraints;
mod context;
mod core;
mod interp;

// inspiration from
// - https://github.com/lizhuohua/rust-mir-checker/blob/master/src/bin/mir-checker.rs
// - https://rustc-dev-guide.rust-lang.org/rustc-driver/interacting-with-the-ast.html

use rustc_driver::{Callbacks, Compilation, run_compiler};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;

use std::env;

use context::GlobalContext;
use interp::InterpPass;

struct VerifoptCallbacks;

impl Callbacks for VerifoptCallbacks {
    //fn config(&mut self, config: &mut interface::Config) {}

    fn after_analysis(&mut self, _compiler: &Compiler, tcx: TyCtxt<'_>)-> Compilation {
        // initialize GlobalContext
        let mut global_context = GlobalContext::new(tcx);

        // initialize Analyzer/Interpreter (TODO + other passes later)
        let mut interp = InterpPass::new(&mut global_context);

        // run Interpreter (which modifies GlobalContext state)
        let _res = interp.run();

        // TODO somehow pass GlobalContext state to our transformation pass...

        Compilation::Stop
    }
}

fn main() {
    let rustc_args = env::args_os()
        .enumerate()
        .map(|(i, arg)| {
            arg.into_string().unwrap_or_else(|arg| {
                panic!("Argument {} is not valid Unicode: {:?}", i, arg);
            })
        })
        .collect::<Vec<_>>();

    run_compiler(
        &rustc_args,
        &mut VerifoptCallbacks
    )
}
