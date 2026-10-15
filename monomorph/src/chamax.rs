//! "CHAmax": the whole-program maximum set of *instances* behind each dynamic
//! dispatch's CHA candidates.
//!
//! CHA is a sound maximum at the level of trait impls: a method that isn't
//! among a site's CHA candidates can never be called there. But a generic impl
//! (e.g. `impl<C, D> Process for ProcessStandard<'_, C, D>`) stands for one
//! function per instantiation, and which instantiations exist is only known
//! after each crate's monomorphization. Every crate compiled by the modified
//! compiler under cargo-verifopt writes an instance manifest
//! (rustc_verifopt::InstanceManifest, from rustc_monomorphize's partitioning),
//! and CHAmax is the union, over all of the program's manifests, of the
//! compiled instances of each CHA candidate.
//!
//! It's "incomplete" (a lower bound only) when a crate that could instantiate
//! a candidate has no manifest - typically the precompiled standard library,
//! whose crates can only instantiate impls from other sysroot crates (they
//! can't depend on yours), or any crate cargo didn't recompile in this run.

extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_verifopt;

use std::collections::{HashMap, HashSet};

use rustc_middle::ty::TyCtxt;
use rustc_public::ty::{GenericArgs, Span};
use rustc_public::{DefId, rustc_internal};
use rustc_span::def_id::{DefPathHash, LOCAL_CRATE};

type Key = (DefId, usize, GenericArgs);

pub struct ChaMax {
    /// Each CHA candidate, with how many compiled instances of it the
    /// program's manifests contain.
    pub per_candidate: Vec<(DefId, usize)>,
    pub total: usize,
    /// Why this is only a lower bound, if it is.
    pub incomplete: Option<String>,
    /// Why CHAmax says nothing about this site, if it doesn't: it's built
    /// from CHA's candidates, and CHA never lists closures - so at a site
    /// dispatching to closures (through an Fn-family trait) CHAmax would be
    /// an empty "maximum" that's simply wrong.
    pub not_applicable: Option<String>,
}

/// Computes CHAmax for every site in `cha`. Must run *after* the store has
/// been written: it first makes rustc monomorphize (and partition) this crate
/// - which is what writes this crate's own manifest - and the modified
/// partitioning reads the store to account for the rewrite's new calls. The
/// query's result is cached, so codegen reuses it rather than redoing it.
pub fn compute<'tcx>(
    tcx: TyCtxt<'tcx>,
    fsa: &HashMap<Key, (Span, Vec<(DefId, Option<GenericArgs>)>)>,
    cha: &HashMap<Key, (Span, Vec<(DefId, Option<GenericArgs>)>)>,
) -> HashMap<Key, ChaMax> {
    let _ = tcx.collect_and_partition_mono_items(());

    // Every crate in this program: stable crate id -> (name, loaded from sysroot?)
    let sysroots: Vec<std::path::PathBuf> =
        tcx.sess.opts.sysroot.all_paths().map(|p| p.to_path_buf()).collect();
    let mut program: HashMap<u64, (String, bool)> = HashMap::new();
    program.insert(
        tcx.stable_crate_id(LOCAL_CRATE).as_u64(),
        (tcx.crate_name(LOCAL_CRATE).to_string(), false),
    );
    for &cnum in tcx.crates(()) {
        let from_sysroot = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let src = tcx.used_crate_source(cnum);
            [&src.rlib, &src.rmeta, &src.dylib]
                .into_iter()
                .flatten()
                .any(|p| sysroots.iter().any(|s| p.starts_with(s)))
        }))
        .unwrap_or(false);
        program.insert(tcx.stable_crate_id(cnum).as_u64(), (tcx.crate_name(cnum).to_string(), from_sysroot));
    }

    // Union of this program's manifests: function -> its compiled instances' args.
    let mut instances: HashMap<DefPathHash, HashSet<Option<Vec<DefPathHash>>>> = HashMap::new();
    let mut covered: HashSet<u64> = HashSet::new();
    if let Ok(entries) = std::fs::read_dir(rustc_verifopt::manifest_dir()) {
        for entry in entries.flatten() {
            let Ok(json) = std::fs::read_to_string(entry.path()) else { continue };
            let Ok(manifest) = rustc_verifopt::InstanceManifest::from_json(&json) else { continue };
            // Manifests of other builds (other profiles, flags, targets of the
            // same crate) have other crate ids, and are ignored.
            if !program.contains_key(&manifest.crate_id) {
                continue;
            }
            covered.insert(manifest.crate_id);
            for (function, args) in manifest.instances {
                instances.entry(function).or_default().insert(args);
            }
        }
    }
    let missing_sysroot = program.iter().any(|(id, (_, sys))| *sys && !covered.contains(id));
    let mut missing_other: Vec<&str> = program
        .iter()
        .filter(|(id, (_, sys))| !*sys && !covered.contains(*id))
        .map(|(_, (name, _))| name.as_str())
        .collect();
    missing_other.sort();

    let mut out = HashMap::new();
    for (key, (_, candidates)) in cha {
        let mut per_candidate = Vec::new();
        let mut total = 0;
        let mut incomplete: Option<String> = None;
        if !missing_other.is_empty() {
            incomplete = Some(format!("no instance manifest for crate(s) {}", missing_other.join(", ")));
        }
        // Closure dispatch: CHA can't describe it (see ChaMax::not_applicable).
        let fsa_targets = fsa.get(key).map(|(_, t)| t.as_slice()).unwrap_or(&[]);
        let fsa_has_closure = fsa_targets
            .iter()
            .any(|(d, _)| tcx.is_closure_like(rustc_internal::internal(tcx, *d)));
        if fsa_has_closure || (candidates.is_empty() && !fsa_targets.is_empty()) {
            out.insert(
                key.clone(),
                ChaMax {
                    per_candidate: Vec::new(),
                    total: 0,
                    incomplete: None,
                    not_applicable: Some(
                        "dispatch to closures (CHA never lists closures, so CHAmax can't bound it)"
                            .to_string(),
                    ),
                },
            );
            continue;
        }

        // Each function once: CHA lists a trait's default method once per
        // implementing type (same DefId, different Self), but the manifests
        // count that function's instances across all of them - summing per
        // entry would multiply them by the number of implementing types.
        let mut distinct: Vec<DefId> = Vec::new();
        for (candidate, _) in candidates {
            if !distinct.contains(candidate) {
                distinct.push(*candidate);
            }
        }
        for candidate in &distinct {
            let did = rustc_internal::internal(tcx, *candidate);
            let hash = tcx.def_path_hash(did);
            let found = instances.get(&hash);
            let count = found.map_or(0, |set| set.len());
            if found.is_some_and(|set| set.contains(&None)) && incomplete.is_none() {
                incomplete = Some(format!("an instance of {} has unhashable generic args", tcx.def_path_str(did)));
            }
            let candidate_from_sysroot = program
                .get(&hash.stable_crate_id().as_u64())
                .is_some_and(|(_, sys)| *sys);
            if missing_sysroot && candidate_from_sysroot && incomplete.is_none() {
                incomplete = Some(format!(
                    "{} is from the precompiled standard library, whose crates have no instance manifests",
                    tcx.def_path_str(did)
                ));
            }
            total += count;
            per_candidate.push((*candidate, count));
        }
        out.insert(key.clone(), ChaMax { per_candidate, total, incomplete, not_applicable: None });
    }
    out
}
