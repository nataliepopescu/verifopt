#![feature(rustc_private)]
#![feature(box_patterns)]

extern crate rustc_driver;
extern crate rustc_data_structures;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;

mod constraints;
mod core;

// inspiration from
// - https://github.com/lizhuohua/rust-mir-checker/blob/master/src/bin/mir-checker.rs
// - https://rustc-dev-guide.rust-lang.org/rustc-driver/interacting-with-the-ast.html

use rustc_hir::def_id::DefId;
//use rustc_hir::def::DefKind;
use rustc_driver::{Callbacks, Compilation, run_compiler};
use rustc_interface::interface::Compiler;
use rustc_middle::mir::*;
use rustc_middle::mir::visit::Visitor;
use rustc_middle::ty::TyCtxt;
use rustc_data_structures::fx::{FxHashSet as HashSet};

use std::env;

use constraints::{ConstraintMap, MapKey, VarType};
use core::VerifoptRval;

struct GlobalContext<'tcx> {
    // The central data structure of the compiler
    pub tcx: TyCtxt<'tcx>,

    // Stores the DefIds that have been already checked, to avoid redundant check
    // TODO when to update: before or after?
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

impl<'a, 'tcx> Visitor<'tcx> for InterpPass<'a, 'tcx> {
    fn visit_body(&mut self, body: &Body<'tcx>) {
        // FIXME how do loops affect this order?
        for (bb, data) in traversal::preorder(body) {
            self.visit_basic_block_data(bb, data);
        }
    }

    fn visit_basic_block_data(
        &mut self,
        block: BasicBlock,
        data: &BasicBlockData<'tcx>,
    ) {
        for (statement_index, stmt) in data.statements.iter().enumerate() {
            let loc = Location { block, statement_index };
            self.visit_statement(stmt, loc);
        }

        // TODO
        // visit_terminator
    }

    fn visit_statement(
        &mut self,
        statement: &Statement<'tcx>,
        _location: Location
    ) {
        match statement.kind {
            StatementKind::Assign(box (place, ref rvalue)) => {
                println!("assignment!");
                println!("place: {:?}", place);
                println!("rval: {:?}", rvalue);
                let mut set = HashSet::default();

                // FIXME need to clone?
                // Rc for cheaper clone?
                set.insert(rvalue.into());
                self.cmap.scoped_set(
                    None,
                    MapKey::Place(place),
                    Box::new(VarType::Values(set)),
                );
                println!("CMAP: {:?}", self.cmap);
            },
            _ => println!("another statement kind"),
        }
    }
}

impl<'a, 'tcx> InterpPass<'a, 'tcx> {
    pub fn new(context: &'a mut GlobalContext<'tcx>) -> InterpPass<'a, 'tcx> {
        Self { context, cmap: ConstraintMap::new() }
    }

    pub fn run(&mut self) {
        // what are all the DefIds in this crate?
        //for def_id in tcx.hir_body_owners() {
        //    println!("def_id: {:?}", def_id);
        //    let def_kind = tcx.def_kind(def_id);
        //    if def_kind == DefKind::Fn || def_kind == DefKind::AssocFn {
        //        println!("FUNCTION DEFID");
        //    }
        //}

        // get entry point DefId
        let mut entry_func = None;
        for def_id in self.context.tcx.hir_body_owners() {
            let item_name = self.context.tcx.item_name(def_id.to_def_id());
            //println!("item_name: {:?}", item_name.to_string());
            if item_name.to_string() == "main" {
                entry_func = Some(def_id);
            }
        }

        if entry_func.is_none() {
            panic!("No main func detected");
        }

        let body = self.context.tcx.optimized_mir(entry_func.unwrap());
        self.visit_body(body);
    }
}

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
