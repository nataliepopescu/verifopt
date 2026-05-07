#![feature(rustc_private)]

//extern crate rustc_hir;
//extern crate rustc_middle;
extern crate rustc_public;

use rustc_public::CrateDef;
use rustc_public::mir::mono::Instance;

use std::ops::ControlFlow;

pub mod util;

pub fn start_verifopt() -> ControlFlow<()> {
    let crate_name = rustc_public::local_crate().name;
    eprintln!("--- Analyzing crate: {crate_name}");

    let crate_items = rustc_public::all_local_items();
    for item in crate_items {
        eprintln!("  - {} @{:?}", item.name(), item.span())
    }

    let entry_fn = rustc_public::entry_fn().unwrap();
    let _entry_instance = Instance::try_from(entry_fn).unwrap();
    //analyze_instance(entry_instance);

    ControlFlow::Continue(())
}

