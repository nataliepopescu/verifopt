#![feature(rustc_private)]
#![feature(box_patterns)]

extern crate rustc_driver;
extern crate rustc_data_structures;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_index;
extern crate rustc_metadata;
extern crate rustc_public;
extern crate rustc_ast;

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

use rustc_ast::ast::Crate;
use rustc_hir::def_id::DefId;
use rustc_driver::{Callbacks, Compilation, run_compiler};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;
use rustc_metadata::creader::CStore;
use rustc_public::external_crates;
use rustc_middle::middle::exported_symbols::ExportedSymbol;
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
        let mut func_map = FuncMap::new();
        let mut cmap = ConstraintMap::new();

        // get entry point function DefId
        let entry_func_opt = tcx.entry_fn(());
        if entry_func_opt.is_none() {
            panic!("No main func detected");
        }
        let entry_func: DefId = entry_func_opt.unwrap().0;

        // get optimized MIR body of entry point function
        let mir_body = tcx.optimized_mir(entry_func);

        //let mut num_crates = 0;
        //for crate_ in tcx.used_crates(()).into_iter() {
        //    num_crates += 1;
        //    let def_id = crate_.as_def_id();
        //    println!();
        //    println!("Crate DefId: {:?}", def_id);

            //let engs = tcx.exported_non_generic_symbols(*crate_);
            //let egs = tcx.exported_generic_symbols(*crate_);
            //let rng = tcx.reachable_non_generics(*crate_);
            //println!("engs: {:?}", engs);
            //println!("egs: {:?}", egs);

            //for eng in engs {
            //    if let ExportedSymbol::NonGeneric(def_id) = eng.0 {
            //        println!("ng defid: {:?}", def_id);
            //        println!("ng mir available? {:?}", tcx.is_mir_available(def_id));
            //    }
            //}

            //for eg in egs {
            //    if let ExportedSymbol::Generic(def_id, _) = eg.0 {
            //        println!("g defid: {:?}", def_id);
            //        println!("g mir available? {:?}", tcx.is_mir_available(def_id));
            //    }
            //}

            //.keys().map(|&x| x).into_sorted_stable_ord();
            //for key in exports.iter() {
            //    println!("symbol defid: {:?}", key);
            //    println!("SYMBOL mir available? {:?}", tcx.is_mir_available(*key));
            //}
            //println!("CRATE mir available? {:?}", tcx.is_mir_available(def_id));

        //}
        //println!("NUM USED CRATES: {:?}", num_crates);

        //num_crates = 0;
        //for crate_ in external_crates().iter() {
        //    num_crates += 1;
        //    //let def_id = crate_.id.as_def_id();
        //    println!("Crate DefId: {:?}", crate_.id); //def_id);
        //}
        //println!("NUM EXTERNAL CRATES: {:?}", num_crates);

        //let cstore = CStore::from_tcx(tcx);
        //for (cnum, cmeta) in cstore.iter_crate_data() {
        //    println!("CRATE (cstore)");
        //    println!("crate num: {:?}", cnum);
        //    println!("crate metadata: {:?}", cmeta);
        //}

        // init + run Function Collection Pass
        // TODO collect non-local funcs too
        let mut func_collect = FuncCollectPass::new(tcx, &mut func_map);
        func_collect.run();
        //println!("func_map: {:#?}", func_map);

        //// init + run Function Signature Collection Pass
        //// https://doc.rust-lang.org/beta/nightly-rustc/rustc_middle/ty/struct.TyCtxt.html#method.fn_sig

        //// init + run Interpreter Pass
        let interp = InterpPass::new(tcx, &func_map);
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

    run_compiler(
        &rustc_args,
        &mut VerifoptCallbacks
    )
}
