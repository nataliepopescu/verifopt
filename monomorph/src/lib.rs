#![feature(rustc_private)]
#![feature(maybe_uninit_fill)]

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
pub mod projection;
pub mod rewrite;
pub mod sig_collect;
pub mod trait_collect;
pub mod util;
pub mod wto;

use crate::constraints::InterpStore;
use crate::interp::InterpPass;
use crate::logger::VOLogger;
use crate::sig_collect::{SigCollectPass, SigStore};
use crate::trait_collect::{TraitCollectPass, TraitStore};
use crate::util::options::AnalysisOptions;

pub fn start_verifopt(_options: AnalysisOptions) -> ControlFlow<()> {
    // TODO make log filename a cmdline option
    let filename = "tool_effectiveness_log";
    let mut logger = VOLogger::new(filename);

    let entry_fn = rustc_public::entry_fn().unwrap();
    let entry_instance = Instance::try_from(entry_fn).unwrap();

    // Collect function signatures for indirect calls
    debug!("FUNC SIG PASS");
    let mut sigstore = SigStore::new();
    let sig_collect = SigCollectPass::new();
    sig_collect.run(&mut sigstore);

    // Collect trait metadata
    debug!("TRAIT PASS");
    let mut tstore = TraitStore::new();
    let trait_collect = TraitCollectPass::new();
    trait_collect.run(&mut tstore);

    // Abstractly Interpret MIR
    debug!("INTERP PASS");
    let mut istore = InterpStore::new();
    let interp = InterpPass::new(&sigstore, &tstore);
    let _ = interp.run(&mut logger, &mut istore, entry_instance);

    // TODO Rewrite MIR

    ControlFlow::Continue(())
}
