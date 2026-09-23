extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_public;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_verifopt;

use rustc_span::def_id::{DefPathHash, LOCAL_CRATE};

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;
use rustc_public::{DefId, rustc_internal};
use rustc_verifopt::{ShapeRegistry, Store, hash_args};

use std::collections::HashMap; // FIXME FxHashMap for consistency?
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::OnceLock;

use crate::interp::TagPlan;
use crate::start_verifopt;
use crate::util::options::AnalysisOptions;

use log::debug;

static STORE: OnceLock<Mutex<Store>> = OnceLock::new();

pub fn dep_rewrite_store_path() -> PathBuf {
    resolve_store_path("verifopt_store.json")
}

pub fn needs_rewrite_pass_marker_path() -> PathBuf {
    resolve_store_path("verifopt_needs_rewrite_pass")
}

fn resolve_store_path(filename: &str) -> PathBuf {
    match std::env::var_os("VERIFOPT_STORE_DIR") {
        Some(dir) => PathBuf::from(dir).join(filename),
        None => PathBuf::from(filename),
    }
}

fn store() -> &'static Mutex<Store> {
    STORE.get_or_init(|| Mutex::new(Store::default()))
}

/// Sentinel hash -> shape for every type hashed during this run; written
/// into the store so the rewrite side can rebuild those types. The hashing
/// scheme itself lives in rustc_verifopt, shared with the compiler.
static SHAPES: ShapeRegistry = ShapeRegistry::new();

pub struct FsaCallbacks {
    pub options: AnalysisOptions,
}

impl Callbacks for FsaCallbacks {
    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        debug!(
            "[verifopt debug][after_analysis entry] crate={:?} skip_analysis={:?}",
            tcx.crate_name(LOCAL_CRATE),
            self.options.skip_analysis,
        );
        if self.options.skip_analysis {
            return Compilation::Continue;
        }

        let _ = rustc_internal::run(tcx, || {
            let Ok((targets, tags)) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                start_verifopt(self.options.clone())
            })) else {
                debug!(
                    "[verifopt debug] start_verifopt itself panicked - no store written this run"
                );
                return;
            };

            let mut store = store().lock().unwrap();

            let to_hash = |did| -> Option<DefPathHash> {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    tcx.def_path_hash(rustc_internal::internal(tcx, did))
                }))
                .inspect_err(|_| debug!("to_hash panicked on {:?}, skipping", did))
                .ok()
            };

            // One generic arg list -> one hash per arg (lifetimes included),
            // via the scheme shared with the compiler (rustc_verifopt), which
            // operates on rustc-internal types. Shapes of any sentinel hashes
            // are recorded in SHAPES for the store.
            let to_genargs_hashes = |genargs: &rustc_public::ty::GenericArgs| -> Option<Vec<DefPathHash>> {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    hash_args(tcx, rustc_internal::internal(tcx, genargs), &SHAPES)
                }))
                .inspect_err(|_| debug!("hash_args panicked on {:?}, skipping", genargs))
                .ok()
                .flatten()
            };
            let to_self_hashes = |genargs: &Option<rustc_public::ty::GenericArgs>|
             -> Option<Option<Vec<DefPathHash>>> {
                match genargs {
                    None => Some(None),
                    Some(g) => Some(Some(to_genargs_hashes(g)?)),
                }
            };

            for ((defid, bb, caller_genargs), (_, ts)) in targets {
                let Some(hash) = to_hash(defid) else {
                    continue;
                };

                // Unhashable caller args: record nothing for this site. The
                // rewrite side hashes the same (resolved) args with the same
                // function, fails identically, and leaves the call alone -
                // never falling back to a coarser key that could match a
                // different instantiation's entry.
                let Some(caller_genargs_hash) = to_genargs_hashes(&caller_genargs) else {
                    debug!(
                        "[verifopt debug][store] skipping {:?} bb{}: caller genargs {:?} not hashable",
                        defid, bb, caller_genargs
                    );
                    continue;
                };

                // All or nothing: silently dropping one unhashable target
                // (the old filter_map) could shrink a 2-target site to one,
                // which compute_edits turns into Edit::Single - a static call
                // to the wrong impl whenever the dropped one is the real
                // callee.
                let t_hashes: Option<Vec<(DefPathHash, Option<Vec<DefPathHash>>)>> = ts
                    .iter()
                    .map(|(did, genargs)| Some((to_hash(*did)?, to_self_hashes(genargs)?)))
                    .collect();
                let Some(t_hashes) = t_hashes else {
                    debug!(
                        "[verifopt debug][store] skipping {:?} bb{}: a target is not hashable",
                        defid, bb
                    );
                    continue;
                };

                store.targets.insert((hash, bb, caller_genargs_hash), t_hashes);
            }

            for ((defid, bb, caller_genargs), plan) in tags {
                let TagPlan::Tagged(sites) = plan else {
                    continue;
                };
                if sites.is_empty() {
                    continue;
                }

                let Some(hash) = to_hash(defid) else {
                    continue;
                };

                let Some(caller_genargs_hash) = to_genargs_hashes(&caller_genargs) else {
                    continue;
                };

                let mut next: u64 = 0;
                let mut assigned: HashMap<DefId, u64> = HashMap::default();

                // All or nothing, as for targets. Without a tags entry,
                // compute_edits falls back to Pointers (or leaves the call).
                let entry: Option<Vec<(usize, usize, u64, DefPathHash, Option<Vec<DefPathHash>>)>> =
                    sites
                        .iter()
                        .map(|(bb, stmt, did, genargs)| {
                            let tag = *assigned.entry(*did).or_insert_with(|| {
                                next += 1;
                                next - 1
                            });
                            Some((*bb, *stmt, tag, to_hash(*did)?, to_self_hashes(genargs)?))
                        })
                        .collect();
                let Some(entry) = entry else {
                    debug!(
                        "[verifopt debug][store] not tagging {:?} bb{}: a tag site is not hashable",
                        defid, bb
                    );
                    continue;
                };

                store.tags.insert((hash, bb, caller_genargs_hash), entry);
            }

            store.shapes = SHAPES.snapshot();

            debug!(
                "[verifopt debug][store write check] CARGO_PRIMARY_PACKAGE={:?} store.targets.len()={} store.tags.len()={} store.shapes.len()={} crate={:?} path={:?}",
                std::env::var("CARGO_PRIMARY_PACKAGE"),
                store.targets.len(),
                store.tags.len(),
                store.shapes.len(),
                tcx.crate_name(LOCAL_CRATE),
                dep_rewrite_store_path(),
            );
            if std::env::var("CARGO_PRIMARY_PACKAGE").is_ok() {
                match store.to_json() {
                    Ok(json) => {
                        debug!(
                            "[verifopt debug][store write] serialized ok, {} bytes, writing to {:?}",
                            json.len(),
                            dep_rewrite_store_path(),
                        );
                        match std::fs::write(dep_rewrite_store_path(), json) {
                            Ok(()) => debug!("[verifopt debug][store write] fs::write succeeded"),
                            Err(e) => debug!("[verifopt debug][store write] fs::write FAILED: {:?}", e),
                        }
                    }
                    Err(e) => debug!("[verifopt debug][store write] Store::to_json FAILED: {:?}", e),
                }

                let primary_crate_id = tcx.stable_crate_id(LOCAL_CRATE);
                let needs_rewrite_pass = store
                    .targets
                    .keys()
                    .chain(store.tags.keys())
                    .any(|(hash, _bb, _caller_genargs)| hash.stable_crate_id() != primary_crate_id);

                if needs_rewrite_pass {
                    let _ = std::fs::write(needs_rewrite_pass_marker_path(), "1");
                }
            }
        });

        Compilation::Continue
    }
}

