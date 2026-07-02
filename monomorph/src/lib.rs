#![feature(rustc_private)]
#![feature(maybe_uninit_fill)]
#![feature(box_patterns)]

//extern crate rustc_hir;
//extern crate rustc_middle;
extern crate rustc_data_structures;
extern crate rustc_index;
extern crate rustc_public;
extern crate rustc_public_bridge;

//use rustc_public::CrateDef;
use rustc_public::mir::mono::Instance;

use log::debug;
use std::ops::ControlFlow;

pub mod common;
pub mod constraints;
pub mod convert;
pub mod error;
pub mod interp;
pub mod logger;
pub mod merge;
//pub mod projection;
pub mod rewrite;
pub mod sig_collect;
pub mod trait_collect;
pub mod util;
pub mod wto;

use crate::constraints::Context;
use crate::interp::InterpPass;
use crate::logger::VOLogger;
use crate::sig_collect::{SigCollectPass, SigStore};
use crate::trait_collect::{TraitCollectPass, TraitStore};
use crate::util::options::AnalysisOptions;

pub fn start_verifopt(_options: AnalysisOptions) -> ControlFlow<()> {
    // TODO make log filename a cmdline option
    let f_filename = "found_ex";
    let nf_filename = "notfound_ex";
    let mut logger = VOLogger::new(f_filename, nf_filename);

    let entry_fn_opt = rustc_public::entry_fn();
    if entry_fn_opt.is_none() {
        panic!("no entry function");
    }

    let entry_fn = entry_fn_opt.unwrap();
    let entry_instance = Instance::try_from(entry_fn).unwrap();

    // Collect trait metadata
    debug!("\n\nTRAIT PASS");
    let mut tstore = TraitStore::new();
    let trait_collect = TraitCollectPass::new();
    trait_collect.run(&mut tstore);

    // Collect function signatures for indirect calls
    debug!("\n\nSIG PASS");
    let mut sigstore = SigStore::new();
    let sig_collect = SigCollectPass::new(&tstore);
    sig_collect.run(&mut sigstore);

    // Abstractly Interpret MIR
    debug!("\n\nINTERP PASS");
    let mut ctxt = Context::empty();
    let interp = InterpPass::new(&sigstore, &tstore);
    let _ = interp.run(&mut logger, &mut ctxt, entry_instance);

    // TODO Rewrite MIR

    let _ = logger.log_stats();

    ControlFlow::Continue(())
}
