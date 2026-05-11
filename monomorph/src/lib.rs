#![feature(rustc_private)]

//extern crate rustc_hir;
//extern crate rustc_middle;
extern crate rustc_data_structures;
extern crate rustc_index;
extern crate rustc_public;

//use rustc_public::CrateDef;
use rustc_public::mir::mono::Instance;

use log::debug;
use std::ops::ControlFlow;

pub mod common;
pub mod fsa;
pub mod util;

use crate::util::options::AnalysisOptions;
use crate::fsa::func_collect::{FuncCollectPass, FuncMap};
use crate::fsa::interp::InterpPass;
use crate::fsa::constraints::ConstraintMap;

pub fn start_verifopt(_options: AnalysisOptions) -> ControlFlow<()> {
    //eprintln!(" OPTIONS: {:?}", options);

    //let crate_name = rustc_public::local_crate().name;
    //eprintln!("--- Analyzing crate: {crate_name}");

    //let crate_items = rustc_public::all_local_items();
    //for item in crate_items {
    //    eprintln!("  - {} @{:?}", item.name(), item.span())
    //}

    let entry_fn = rustc_public::entry_fn().unwrap();
    let entry_instance = Instance::try_from(entry_fn).unwrap();

    //debug!("trait_impls: {:#?}", rustc_public::all_trait_impls());

    // Collect function and trait metadata
    //let mut fmap = FuncMap::new();
    //let func_collect = FuncCollectPass::new();
    //func_collect.run(&mut fmap);

    // Interpret MIR
    let mut cmap = ConstraintMap::new();
    let interp = InterpPass::new();
    let _ = interp.run(&mut cmap, entry_fn.0, entry_instance);

    // Rewrite MIR

    ControlFlow::Continue(())
}

