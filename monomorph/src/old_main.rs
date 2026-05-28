#![feature(rustc_private)]

extern crate rustc_public;
extern crate rustc_middle;
extern crate rustc_driver;
extern crate rustc_interface;

use rustc_public::{CrateDef, CompilerError, run};
use rustc_public::mir::mono::Instance;
use rustc_public::mir::{MirVisitor, Operand, Terminator, TerminatorKind};
use rustc_public::mir::visit::Location;
use rustc_public::ty::TyKind; //::RigidTy;
use rustc_public::ty::RigidTy;

use std::process::ExitCode;
use std::ops::ControlFlow;

fn main() -> ExitCode {
    let rustc_args: Vec<String> = std::env::args().collect();
    let result = run!(&rustc_args, start_verifopt);
    match result {
        Ok(_) | Err(CompilerError::Skipped | CompilerError::Interrupted(_)) => ExitCode::SUCCESS,
        _ => ExitCode::FAILURE,
    }
}

pub fn start_verifopt() -> ControlFlow<()> {
    let crate_name = rustc_public::local_crate().name;
    eprintln!("--- Analyzing crate: {crate_name}");

    let crate_items = rustc_public::all_local_items();
    for item in crate_items {
        eprintln!("  - {} @{:?}", item.name(), item.span())
    }

    let entry_fn = rustc_public::entry_fn().unwrap();
    let entry_instance = Instance::try_from(entry_fn).unwrap();
    //analyze_instance(entry_instance);
    ControlFlow::Break(())
}

/*
fn analyze_instance(instance: Instance) {
    eprintln!("--- Analyzing instance: {}", instance.name());
    eprintln!("  - Mangled name: {}", instance.mangled_name());
    eprintln!("  - FnABI: {:?}", instance.fn_abi().unwrap());

    let body = instance.body().unwrap();
    let mut visitor = Visitor {};
    visitor.visit_body(&body);
}

struct Visitor;

impl MirVisitor for Visitor {
    fn visit_terminator(&mut self, term: &Terminator, _location: Location) {
        match &term.kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => match func {
                Operand::Constant(co) => match co.const_.ty().kind() {
                    TyKind::RigidTy(rigid_ty) => match rigid_ty {
                        RigidTy::FnDef(defid, genargs) => {
                            let instance = Instance::resolve(defid, &genargs).unwrap();
                            println!("\n--- CALLING {:?}", defid);
                            println!("  - Body: {:#?}", instance.body());
                        }
                        _ => {}
                    }
                    _ => {}
                }
                _ => {}
            }
            _ => {}
        }
    }
}
*/

