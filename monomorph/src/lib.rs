#![feature(rustc_private)]
#![feature(maybe_uninit_fill)]
#![feature(box_patterns)]

//extern crate rustc_hir;
//extern crate rustc_middle;
extern crate rustc_data_structures;
extern crate rustc_index;
extern crate rustc_public;
extern crate rustc_public_bridge;

use rustc_public::CrateDef;
use rustc_public::DefId;
use rustc_public::mir::mono::Instance;
use rustc_public::ty::{GenericArgs, Span};
use std::collections::HashMap;
use std::fs;

use log::debug;

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
pub mod stdlib_stubs;
pub mod trait_collect;
pub mod util;
pub mod wto;

use crate::constraints::Context;
use crate::interp::{InterpPass, TagPlan};
use crate::logger::VOLogger;
use crate::sig_collect::{SigCollectPass, SigStore};
use crate::trait_collect::{TraitCollectPass, TraitStore};
use crate::util::options::AnalysisOptions;

pub fn start_verifopt(
    options: AnalysisOptions,
) -> (
    HashMap<(DefId, usize), (Span, Vec<(DefId, Option<GenericArgs>)>)>,
    HashMap<(DefId, usize), TagPlan>,
) {
    // `stats` (opened in append mode by VOLogger::new below) and
    // `mir_dump.txt` (opened in append mode by rewrite.rs's `dump_body`,
    // written during the later RewriteCallbacks compiler session within
    // this same process) both accumulate across runs instead of being
    // overwritten. Clear them here, at the very start of the whole
    // verifopt pipeline, so each invocation starts from a clean slate.
    for f in ["stats", "mir_dump.txt"] {
        let _ = fs::remove_file(f);
    }

    // TODO make log filename a cmdline option
    let mut logger = VOLogger::new();

    // `rustc_public::entry_fn()` only resolves rustc's own notion of "the
    // crate's designated entry point" — tied to the ordinary Rust runtime
    // bootstrap (`lang_start`/`fn main`). `#![no_main]` crates (e.g. Tock's
    // board crates, which boot via their own reset handler + linker script)
    // never register anything there, regardless of what a function is
    // named or `#[no_mangle]`'d as — so this always returned `None` for
    // such crates and `--entry-func`/`--entry-id` were silently ignored
    // (note: `options` used to be named `_options` here). Resolve those
    // explicitly first, and only fall back to `rustc_public::entry_fn()`
    // when neither was supplied.
    let entry_instance = if let Some(def_id) = options.entry_def_id {
        // `rustc_public::DefId` carries crate context beyond a bare index,
        // so it can't be reconstructed from just the `u32` a user would
        // pass on the command line — there's no `From<u32>` impl, and
        // building one correctly would need a real crate-context lookup
        // (e.g. matching against a specific CrateNum) that this function
        // doesn't have inputs for yet. Left unimplemented rather than
        // guessing at a conversion; use --entry-func for now.
        let _ = def_id;
        todo!(
            "--entry-id is not yet wired up (DefId needs crate context, \
             not just a raw u32) — use --entry-func instead"
        );
    } else if !options.entry_func.is_empty() {
        let suffix = format!("::{}", options.entry_func);
        let candidates: Vec<_> = rustc_public::all_local_items()
            .into_iter()
            .filter(|item| {
                let name = item.name();
                name == options.entry_func || name.ends_with(&suffix)
            })
            .collect();

        match candidates.len() {
            0 => {
                // Nothing matched exactly or by qualified suffix. Dump any
                // item whose name merely *contains* the requested string,
                // so the actual name format (mangled? crate-qualified
                // differently? case differences?) is visible instead of
                // guessing again.
                let near_misses: Vec<String> = rustc_public::all_local_items()
                    .into_iter()
                    .map(|item| item.name())
                    .filter(|name| name.contains(&options.entry_func))
                    .collect();
                panic!(
                    "no local item named {:?} found (checked {} local items). \
                     Items containing {:?} as a substring: {:?}",
                    options.entry_func,
                    rustc_public::all_local_items().len(),
                    options.entry_func,
                    near_misses
                );
            }
            1 => Instance::try_from(candidates.into_iter().next().unwrap()).unwrap_or_else(|_| {
                panic!(
                    "could not build an Instance for {:?}",
                    options.entry_func
                )
            }),
            n => panic!(
                "{n} local items named {:?} found — disambiguate with --entry-id instead \
                 (this can happen with e.g. inherent vs. trait methods sharing a name)",
                options.entry_func
            ),
        }
    } else {
        let entry_fn_opt = rustc_public::entry_fn();
        if entry_fn_opt.is_none() {
            panic!("no entry function");
        }
        Instance::try_from(entry_fn_opt.unwrap()).unwrap()
    };

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
    let interp = InterpPass::new(&sigstore, &tstore, options.context_depth);
    let _ = interp.run(&mut ctxt, entry_instance);

    let incomplete = &interp.incomplete.borrow();
    let confirmed: HashMap<Span, bool> = interp
        .dependencies
        .borrow()
        .iter()
        .map(|(&s, ds)| (s, !ds.iter().any(|d| incomplete.contains(d))))
        .collect();

    let cha = &interp.dispatch_cha.borrow();

    let fsa = interp
        .dispatch_targets
        .borrow()
        .iter()
        .filter_map(|(&key, (span, impls))| {
            if *confirmed.get(&span).unwrap_or(&false) {
                Some((key, (span.clone(), impls.clone())))
            } else {
                cha.get(&key).map(|c| (key, (span.clone(), c.clone().1)))
            }
        })
        .collect();

    let tags: HashMap<(DefId, usize), TagPlan> = interp
        .dispatch_tags
        .borrow()
        .iter()
        .map(|(&k, p)| {
            let ok = interp
                .dispatch_targets
                .borrow()
                .get(&k)
                .map_or(false, |(s, _)| *confirmed.get(s).unwrap_or(&false));
            (k, if ok { p.clone() } else { TagPlan::Poisoned })
        })
        .collect();

    // log_stats' signature is pinned to std::HashMap (a purely diagnostic,
    // one-time-at-the-very-end call, unlike the InterpPass fields above -
    // no need to touch its signature just for this). interp.dispatch_cha
    // is `im::HashMap` now (see interp.rs's ImHashMap import) so build_
    // param_summary's per-attempt snapshot/restore stays O(1) instead of
    // O(program size so far) - this one conversion back to std::HashMap
    // happens exactly once for the whole run, not per summary-build
    // attempt, so it isn't the cost that migration was about avoiding.
    let cha_std: HashMap<(DefId, usize), (Span, Vec<(DefId, Option<GenericArgs>)>)> =
        cha.iter().map(|(k, v)| (*k, v.clone())).collect();
    let _ = logger.log_stats(&fsa, &cha_std);

    (fsa, tags)
}
