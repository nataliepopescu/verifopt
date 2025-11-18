#![feature(rustc_private)]
#![feature(box_patterns)]

extern crate rustc_driver;
extern crate rustc_data_structures;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;

mod constraints;
//mod context;
mod core;

mod func_collect;
mod interp;
//mod rewrite;

// inspiration from
// - https://github.com/lizhuohua/rust-mir-checker/blob/master/src/bin/mir-checker.rs
// - https://rustc-dev-guide.rust-lang.org/rustc-driver/interacting-with-the-ast.html

use rustc_hir::def_id::DefId;
use rustc_driver::{Callbacks, Compilation, run_compiler};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;

use std::env;

//use context::GlobalContext;
use constraints::ConstraintMap;
use func_collect::{FuncCollectPass, FuncMap};
use interp::InterpPass;
//use rewrite::RewritePass;

struct VerifoptCallbacks;

impl Callbacks for VerifoptCallbacks {
    //fn config(&mut self, config: &mut interface::Config) {}

    fn after_analysis(&mut self, _compiler: &Compiler, tcx: TyCtxt<'_>)-> Compilation {
        let mut func_map = FuncMap::new();
        let mut cmap = ConstraintMap::new();

        // get entry point DefId
        let mut entry_func_opt = None;
        for def_id in tcx.hir_body_owners() {
            let item_name = tcx.item_name(def_id.to_def_id());
            if item_name.to_string() == "main" {
                entry_func_opt = Some(def_id);
            }
        }

        if entry_func_opt.is_none() {
            panic!("No main func detected");
        }
        let entry_func: DefId = entry_func_opt.unwrap().into();

        // get optimized MIR body of entry point func
        let mir_body = tcx.optimized_mir(entry_func);

        // Function Collection Pass
        let mut func_collect = FuncCollectPass::new(tcx, entry_func, &mut func_map);
        func_collect.run(mir_body);

        // Function Signature Collection Pass

        // Interpreter Pass
        let mut interp = InterpPass::new(tcx, entry_func, &func_map, &mut cmap);
        interp.run(mir_body);

        // Rewriter Pass
        //let mut rw_mir_body: rustc_middle::mir::Body<'_> = mir_body.clone();
        //let rewriter = RewritePass::new(&global_context);
        //rewriter.run(&mut rw_mir_body);

        // TODO is MIR actually modified??
        // cannot turn mir_body (immut &) into a mutable &...

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
