#![feature(rustc_private)]
#![feature(box_patterns)]

extern crate rustc_ast;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_interface;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_public;
extern crate rustc_span;

mod constraints;
mod core;
mod error;
mod wto;

mod func_collect;
mod interp;
//mod rewrite;

// inspiration from
// - https://github.com/lizhuohua/rust-mir-checker/blob/master/src/bin/mir-checker.rs
// - https://rustc-dev-guide.rust-lang.org/rustc-driver/interacting-with-the-ast.html

//use rustc_ast::ast::Crate;
use rustc_driver::{Callbacks, Compilation, run_compiler};
//use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;
//use rustc_metadata::creader::CStore;
//use rustc_public::external_crates;
//use rustc_middle::middle::exported_symbols::ExportedSymbol;
//use rustc_data_structures::stable_hasher::ToStableHashKey;

use std::env;

//use context::GlobalContext;
use constraints::ConstraintMap;
use func_collect::{FuncCollectPass, FuncMap};
use interp::InterpPass;
//use rewrite::RewritePass;

//impl ToStableHashKey<DefId> for DefId { }

struct VerifoptCallbacks;

impl Callbacks for VerifoptCallbacks {
    //fn config(&mut self, config: &mut interface::Config) {}

    fn after_analysis(&mut self, _compiler: &Compiler, tcx: TyCtxt<'_>) -> Compilation {
        // get entry point function DefId
        let entry_func_opt = tcx.entry_fn(());
        if entry_func_opt.is_none() {
            panic!("No main func detected");
        }
        let entry_func: DefId = entry_func_opt.unwrap().0;

        // get optimized MIR body of entry point function
        let mir_body = tcx.optimized_mir(entry_func);

        // init + run Function Collection Pass
        // TODO collect non-local funcs too
        let mut funcs = FuncMap::new();
        let func_collect = FuncCollectPass::new(tcx, false);
        func_collect.run(&mut funcs);
        //println!("funcs: {:#?}", funcs);

        //// init + run Function Signature Collection Pass
        //// https://doc.rust-lang.org/beta/nightly-rustc/rustc_middle/ty/struct.TyCtxt.html#method.fn_sig

        //// init + run Interpreter Pass
        let mut cmap = ConstraintMap::new();
        let interp = InterpPass::new(tcx, &funcs, false);
        let res = interp.run(&mut cmap, None, entry_func, mir_body);
        println!("\nmain res: {:?}", res);

        // init + run Rewriter Pass
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

    run_compiler(&rustc_args, &mut VerifoptCallbacks)
}
