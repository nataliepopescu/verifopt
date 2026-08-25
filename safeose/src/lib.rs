#![feature(rustc_private)]

extern crate rustc_public;

use rustc_public::CrateDef;
use rustc_public::mir::mono::Instance;

pub mod safeose;
pub mod unsafe_finder;
pub mod util;

use crate::util::options::AnalysisOptions;
use crate::unsafe_finder::UnsafeFinder;

use log::debug;

pub fn start_safeose(
    options: AnalysisOptions,
) {
    let entry_instance = if let Some(def_id) = options.entry_def_id {
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

    debug!("RUNNING UNSAFE FINDER TOOL");
    let unsafe_finder = UnsafeFinder::new();
    unsafe_finder.run(entry_instance);
}
