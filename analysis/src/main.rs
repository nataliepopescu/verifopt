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
use rustc_middle::ty::{InstanceKind, ReifyReason, TyCtxt};
use rustc_metadata::creader::CStore;
use rustc_public::external_crates;
use rustc_middle::middle::exported_symbols::ExportedSymbol;
//use rustc_data_structures::stable_hasher::ToStableHashKey;

use std::env;
use std::panic;

//use context::GlobalContext;
use constraints::ConstraintMap;
use func_collect::{FuncCollectPass, FuncMap};
use interp::InterpPass;
//use rewrite::RewritePass;

//impl ToStableHashKey<DefId> for DefId { }

struct VerifoptCallbacks;

impl VerifoptCallbacks {
    fn try_stuff(tcx: TyCtxt<'_>) {
        let mut num_crates: u32 = 0;
        for crate_ in tcx.used_crates(()).into_iter() {
            num_crates += 1;
            let def_id = crate_.as_def_id();
            //println!();
            //println!("Crate DefId: {:?}", def_id);

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
        }
        //println!("NUM USED CRATES: {:?}", num_crates);

        // forge DefIds to see if we can access random MIR
        //for crate_num in 1u32..2u32 { //num_crates + 1 {
        let crate_num = 0u32;
            println!("\nnew crate_num\n");
            // when we get a panic in rustc_metadata/src/rmeta/decorder.rs about
            // Option::unwrap() being called on a None value, I think we've run 
            // out of def_indices
            for def_index in 1..u32::MAX {
                //if crate_num == 1 && def_index >= 19549
                //|| crate_num == 2 && def_index >= 78916
                //|| crate_num == 3 && def_index >= 12636
                //|| crate_num == 4 && def_index >= 4970
                //|| crate_num == 5 && def_index >= 12217
                //|| crate_num == 6 && def_index >= 3
                //|| crate_num == 7 && def_index >= 94
                //|| crate_num == 8 && def_index >= 513
                //|| crate_num == 9 && def_index >= 71
                //{ continue }

                println!("\nnew def_index");
                println!("crate_num: {:?}", crate_num);
                println!("def_index: {:?}", def_index);
                let def_id = DefId { index: def_index.into(), krate: crate_num.into() };
                println!("forged defid: {:?}", def_id);
                let mir_avail = tcx.is_mir_available(def_id);
                println!("mir available? {:?}", mir_avail);

                if mir_avail {
                    println!("trying to get func body...");
                    println!("Item body: \n{:?}", tcx.instance_mir(InstanceKind::Item(def_id)));
                //    //println!("VTableShim body: \n{:?}", tcx.instance_mir(InstanceKind::VTableShim(def_id)));
                //    //println!("ReifyShim (Some(Vtable)) body: \n{:?}", tcx.instance_mir(InstanceKind::ReifyShim(def_id, Some(ReifyReason::Vtable))));
                //    //println!("ReifyShim (Some(FnPtr)) body: \n{:?}", tcx.instance_mir(InstanceKind::ReifyShim(def_id, Some(ReifyReason::FnPtr))));
                //    //println!("ReifyShim (None) body: \n{:?}", tcx.instance_mir(InstanceKind::ReifyShim(def_id, None)));
                }
            }
        //}

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
    }
}

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

        Self::try_stuff(tcx);

        // init + run Function Collection Pass
        // TODO collect non-local funcs too
        //let mut func_collect = FuncCollectPass::new(tcx, &mut func_map);
        //func_collect.run();
        //println!("func_map: {:#?}", func_map);

        //// init + run Function Signature Collection Pass
        //// https://doc.rust-lang.org/beta/nightly-rustc/rustc_middle/ty/struct.TyCtxt.html#method.fn_sig

        //// init + run Interpreter Pass
        //let interp = InterpPass::new(tcx, &func_map);
        //let res = interp.run(&mut cmap, None, entry_func, mir_body);
        //println!("\nmain res: {:?}", res);

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
