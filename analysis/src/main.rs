// this feature flag enables us to use compiler internals
#![feature(rustc_private)]

extern crate rustc_hir;
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;

// inspiration from
// - https://github.com/lizhuohua/rust-mir-checker/blob/master/src/bin/mir-checker.rs
// - https://rustc-dev-guide.rust-lang.org/rustc-driver/interacting-with-the-ast.html

use rustc_driver::{Callbacks, Compilation, run_compiler};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;

use std::env;

struct GlobalContext;

struct VerifoptCallbacks;

impl Callbacks for VerifoptCallbacks {
    //fn config(&mut self, config: &mut interface::Config) {}

    fn after_analysis(&mut self, _compiler: &Compiler, tcx: TyCtxt<'_>)-> Compilation {
        // Iterate over the top-level items in the crate, looking for the main function.
        // FIXME why _hir_?
        for id in tcx.hir_free_items() {
            let item = &tcx.hir_item(id);
            if let rustc_hir::ItemKind::Fn { body, .. } = item.kind {
                //let expr = &tcx.hir_body(body).value;
                let def_id = item.hir_id().owner.def_id;
                println!("\n--------\ndef_id: {:?}", def_id);
            }
        }

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
