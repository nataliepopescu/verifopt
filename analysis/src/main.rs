#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_data_structures;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;

mod constraints;

// inspiration from
// - https://github.com/lizhuohua/rust-mir-checker/blob/master/src/bin/mir-checker.rs
// - https://rustc-dev-guide.rust-lang.org/rustc-driver/interacting-with-the-ast.html

use rustc_hir::def_id::DefId;
use rustc_hir::def::{DefKind};
use rustc_driver::{Callbacks, Compilation, run_compiler};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;
use rustc_data_structures::fx::{FxHashSet as HashSet};

use std::env;

use constraints::{ConstraintMap};

struct GlobalContext<'tcx> {
    // The central data structure of the compiler
    pub tcx: TyCtxt<'tcx>,

    // Stores the DefIds that have been already checked, to avoid redundant check
    pub checked_def_ids: HashSet<DefId>,
}

impl<'tcx> GlobalContext<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> GlobalContext<'tcx> {
        Self { tcx, checked_def_ids: HashSet::default() }
    }
}

struct InterpPass<'a, 'tcx> {
    pub context: &'a mut GlobalContext<'tcx>,
    pub cmap: ConstraintMap<'tcx>
}

impl<'a, 'tcx> InterpPass<'a, 'tcx> {
    pub fn new(context: &'a mut GlobalContext<'tcx>) -> InterpPass<'a, 'tcx> {
        Self { context, cmap: ConstraintMap::new() }
    }

    pub fn run(&mut self, tcx: TyCtxt<'tcx>) {
        // what are all the DefIds in this crate?
        //for def_id in tcx.hir_body_owners() {
        //    println!("def_id: {:?}", def_id);
        //    let def_kind = tcx.def_kind(def_id);
        //    if def_kind == DefKind::Fn || def_kind == DefKind::AssocFn {
        //        println!("FUNCTION DEFID");
        //    }
        //}

        // get entry point DefId
        let mut entry_func= None;
        for def_id in tcx.hir_body_owners() {
            let item_name = tcx.item_name(def_id.to_def_id());
            //println!("item_name: {:?}", item_name.to_string());
            if item_name.to_string() == "main" {
                entry_func = Some(def_id);
            }
        }

        if entry_func.is_none() {
            panic!("No main func detected");
        }

        // call analyze_function()
    }
}

struct VerifoptCallbacks;

impl Callbacks for VerifoptCallbacks {
    //fn config(&mut self, config: &mut interface::Config) {}

    fn after_analysis(&mut self, _compiler: &Compiler, tcx: TyCtxt<'_>)-> Compilation {
        // initialize GlobalContext
        let mut global_context = GlobalContext::new(tcx);

        // initialize Analyzer/Interpreter (+ other passes later)
        let mut interp = InterpPass::new(&mut global_context);

        // run Interpreter (which modifies GlobalContext state)
        let _res = interp.run(tcx);

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
