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

extern crate rustc_hir;
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
    /// The instances themselves, as (function, its own generic-arg hashes)
    /// from the manifests - only filled in when there are few of them (see
    /// max_listed_instances); used to record a provably single target as an
    /// exact instance (Store::exact).
    pub instances: Vec<(DefPathHash, Option<Vec<DefPathHash>>)>,
    /// "VTmax": the same, restricted to instances that sit in the slots of
    /// some vtable the program builds (the manifests' `vtables`). A `dyn` call
    /// only reaches methods through vtables, so this bounds the site too, and
    /// tighter: CHAmax also counts instances compiled only for static calls
    /// (e.g. a type that implements the trait but is never turned into this
    /// trait object). `None` when no manifest lists vtables (built before the
    /// compiler recorded them). Same `incomplete`/`not_applicable` as CHAmax.
    pub vt_total: Option<usize>,
    pub vt_per_candidate: Vec<(DefId, usize)>,
    pub vt_instances: Vec<(DefPathHash, Option<Vec<DefPathHash>>)>,
}

/// Which bound proves a site's single target (Store::exact): env
/// VERIFOPT_EXACT_FROM=chamax (default) or vtables - so their effects can be
/// compared on the same analysis.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ExactFrom {
    ChaMax,
    Vtables,
}

pub fn exact_from() -> ExactFrom {
    match std::env::var("VERIFOPT_EXACT_FROM").as_deref() {
        Ok("vtables") => ExactFrom::Vtables,
        _ => ExactFrom::ChaMax,
    }
}

impl ChaMax {
    /// The bound `exact_from()` selects: (instance count, the instances when
    /// listed). `None` for VTmax without vtable data.
    pub fn exact_bound(&self) -> Option<(usize, &[(DefPathHash, Option<Vec<DefPathHash>>)])> {
        match exact_from() {
            ExactFrom::ChaMax => Some((self.total, &self.instances)),
            ExactFrom::Vtables => self.vt_total.map(|t| (t, self.vt_instances.as_slice())),
        }
    }

    fn not_bounding(incomplete: Option<String>, not_applicable: Option<String>) -> ChaMax {
        ChaMax {
            per_candidate: Vec::new(),
            total: 0,
            incomplete,
            not_applicable,
            instances: Vec::new(),
            vt_total: None,
            vt_per_candidate: Vec::new(),
            vt_instances: Vec::new(),
        }
    }
}

/// K: the most candidates a Pointers rewrite compares (`--max-candidates`,
/// env VERIFOPT_MAX_CANDIDATES, default 4 - as the compiler reads it).
pub fn max_candidates() -> usize {
    std::env::var("VERIFOPT_MAX_CANDIDATES")
        .ok()
        .and_then(|v| v.trim().parse::<usize>().ok())
        .filter(|n| *n >= 1)
        .unwrap_or(4)
}

/// Up to how many instances a site's ChaMax lists individually: enough for a
/// Pointers rewrite over them (K), and at least 4.
fn max_listed_instances() -> usize {
    max_candidates().max(4)
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
) -> (HashMap<Key, ChaMax>, Vec<(DefPathHash, rustc_verifopt::ArgShape)>) {
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
    // Shapes of the manifests' sentinel hashes (first one wins; a hash's shape
    // is the same wherever it was recorded).
    let mut shapes: HashMap<DefPathHash, rustc_verifopt::ArgShape> = HashMap::new();
    // Every instance that sits in some vtable's slots, and whether any
    // manifest lists vtables at all.
    let mut in_vtable: HashSet<(DefPathHash, Option<Vec<DefPathHash>>)> = HashSet::new();
    let mut have_vtables = false;
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
            for (hash, shape) in manifest.shapes {
                shapes.entry(hash).or_insert(shape);
            }
            have_vtables |= !manifest.vtables.is_empty();
            for vtable in manifest.vtables {
                in_vtable.extend(vtable.methods);
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
        let mut listed: Vec<(DefPathHash, Option<Vec<DefPathHash>>)> = Vec::new();
        let mut total = 0;
        let mut vt_per_candidate = Vec::new();
        let mut vt_listed: Vec<(DefPathHash, Option<Vec<DefPathHash>>)> = Vec::new();
        let mut vt_total = 0;
        let mut incomplete: Option<String> = None;
        if !missing_other.is_empty() {
            incomplete = Some(format!("no instance manifest for crate(s) {}", missing_other.join(", ")));
        }
        // Closure dispatch: CHA can't describe it (see ChaMax::not_applicable).
        let fsa_targets = fsa.get(key).map(|(_, t)| t.as_slice()).unwrap_or(&[]);
        let fsa_has_closure = fsa_targets
            .iter()
            .any(|(d, _)| tcx.is_closure_like(rustc_internal::internal(tcx, *d)));
        // Decided by the dispatched trait too, not only by what FSA found:
        // FSA can miss the closures (or fall back to CHA), and CHA then lists
        // only the trait's explicit impls (e.g. `<Box<F, A> as Fn<Args>>`),
        // whose instance count says nothing about the closures behind it.
        let builtin_impl_trait = candidates
            .iter()
            .chain(fsa_targets)
            .filter_map(|(d, _)| candidate_trait(tcx, rustc_internal::internal(tcx, *d)))
            .find(|t| has_builtin_impls(tcx, *t));
        // Any other site where FSA found targets but CHA has none means CHA
        // dropped real impls (an analysis bug, also logged as "CHA missing
        // impl"): CHAmax is then no bound at all, so it must never prove a
        // single target - and it isn't closure dispatch, so don't call it that.
        if !fsa_has_closure
            && builtin_impl_trait.is_none()
            && candidates.is_empty()
            && !fsa_targets.is_empty()
        {
            out.insert(
                key.clone(),
                ChaMax::not_bounding(Some("CHA lacks FSA's targets (analysis bug)".to_string()), None),
            );
            continue;
        }
        if fsa_has_closure || builtin_impl_trait.is_some() {
            let why = match builtin_impl_trait {
                Some(t) => format!(
                    "dispatch through `{}`, which closures, fn items or coroutines implement \
                     without an impl item (CHA can't list those, so CHAmax can't bound it)",
                    tcx.def_path_str(t)
                ),
                None => "dispatch to closures (CHA never lists closures, so CHAmax can't bound it)"
                    .to_string(),
            };
            out.insert(key.clone(), ChaMax::not_bounding(None, Some(why)));
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
            if let Some(set) = found
                && listed.len() + set.len() <= max_listed_instances()
            {
                // Sorted: a stable order for the rewrite's candidate list.
                let mut sorted: Vec<&Option<Vec<DefPathHash>>> = set.iter().collect();
                sorted.sort();
                listed.extend(sorted.into_iter().map(|args| (hash, args.clone())));
            }
            // VTmax: the instances found in some vtable (an unhashable one,
            // `None`, is kept - it can't be matched, and CHAmax is already
            // incomplete for it).
            let mut in_vt: Vec<&Option<Vec<DefPathHash>>> = found
                .map(|set| {
                    set.iter()
                        .filter(|args| args.is_none() || in_vtable.contains(&(hash, (*args).clone())))
                        .collect()
                })
                .unwrap_or_default();
            in_vt.sort();
            vt_total += in_vt.len();
            vt_per_candidate.push((*candidate, in_vt.len()));
            if vt_listed.len() + in_vt.len() <= max_listed_instances() {
                vt_listed.extend(in_vt.into_iter().map(|args| (hash, args.clone())));
            }
        }
        if listed.len() != total {
            listed.clear(); // too many to list: listed only when complete
        }
        if vt_listed.len() != vt_total {
            vt_listed.clear();
        }
        out.insert(
            key.clone(),
            ChaMax {
                per_candidate,
                total,
                incomplete,
                not_applicable: None,
                instances: listed,
                vt_total: have_vtables.then_some(vt_total),
                vt_per_candidate,
                vt_instances: vt_listed,
            },
        );
    }
    let mut shapes: Vec<(DefPathHash, rustc_verifopt::ArgShape)> = shapes.into_iter().collect();
    shapes.sort_by_key(|(h, _)| *h);
    (out, shapes)
}

/// The trait a CHA candidate belongs to: an impl item's trait, or the trait of
/// a default method. None for anything else (e.g. a closure).
fn candidate_trait(tcx: TyCtxt<'_>, d: rustc_span::def_id::DefId) -> Option<rustc_span::def_id::DefId> {
    tcx.trait_of_assoc(d).or_else(|| tcx.trait_impl_of_assoc(d).map(|imp| tcx.impl_trait_id(imp)))
}

/// Dyn-compatible traits that closures, fn items, fn pointers, coroutines or
/// async blocks implement through compiler-builtin impls - no impl item, so
/// CHA can't list those implementors. (Not `Iterator`: only unstable `gen`
/// blocks implement it that way.)
fn has_builtin_impls(tcx: TyCtxt<'_>, t: rustc_span::def_id::DefId) -> bool {
    use rustc_hir::LangItem;
    tcx.fn_trait_kind_from_def_id(t).is_some()
        || tcx.async_fn_trait_kind_from_def_id(t).is_some()
        || [LangItem::Future, LangItem::Coroutine, LangItem::AsyncIterator]
            .into_iter()
            .any(|item| tcx.is_lang_item(t, item))
}

/// Every function instance this program compiles, for the CHAmax-only mode
/// (`VERIFOPT_CHA_ONLY`, InterpPass::enumerate_sites): this crate's mono items,
/// plus each dependency's non-generic functions (compiled in that crate; the
/// second pass rewrites them there), plus every instance with a dynamic
/// dispatch that some crate's instance manifest lists
/// (InstanceManifest::dispatchers) - which covers the generic instances a
/// dependency compiles for its own use, not instantiated here. Without the
/// last (a manifest from an older compiler, or none), those were missed.
///
/// Runs the partitioning before the store exists: the partitioning hook then
/// adds no rewrite edges in this pass - the second pass (which a store with
/// dependency sites always triggers) rebuilds this crate with them.
pub fn compiled_instances<'tcx>(tcx: TyCtxt<'tcx>) -> Vec<rustc_public::mir::mono::Instance> {
    use rustc_middle::mir::mono::MonoItem;
    let mut out = Vec::new();
    for cgu in tcx.collect_and_partition_mono_items(()).codegen_units {
        for (item, _) in cgu.items() {
            if let MonoItem::Fn(instance) = item
                && let Ok(i) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    rustc_internal::stable(*instance)
                }))
            {
                out.push(i);
            }
        }
    }
    for krate in rustc_public::external_crates() {
        // Proc-macro crates (and any other whose item list can't be read
        // from metadata) panic in the decoder: none of their code runs in
        // the program anyway.
        let Ok(fn_defs) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| krate.fn_defs())) else {
            continue;
        };
        for fd in fn_defs {
            let did = rustc_internal::internal(tcx, rustc_public::CrateDef::def_id(&fd));
            let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                !tcx.generics_of(did).requires_monomorphization(tcx) && tcx.is_mir_available(did)
            }))
            .unwrap_or(false);
            if !ok {
                continue;
            }
            if let Ok(i) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                rustc_internal::stable(rustc_middle::ty::Instance::mono(tcx, did))
            })) {
                out.push(i);
            }
        }
    }
    out.extend(manifest_dispatchers(tcx));
    out
}

/// The instances with a dynamic dispatch that this program's instance
/// manifests list (InstanceManifest::dispatchers), rebuilt from their hashes
/// with each manifest's shapes. Manifests of other builds (crate ids not in
/// this program) are skipped; an entry that can't be rebuilt (unhashable
/// args, or a function this crate can't name) is dropped - its sites then go
/// unlisted, as before the manifests listed them.
fn manifest_dispatchers<'tcx>(tcx: TyCtxt<'tcx>) -> Vec<rustc_public::mir::mono::Instance> {
    let mut program: HashSet<u64> = HashSet::new();
    program.insert(tcx.stable_crate_id(LOCAL_CRATE).as_u64());
    for &cnum in tcx.crates(()) {
        program.insert(tcx.stable_crate_id(cnum).as_u64());
    }
    let mut seen: HashSet<rustc_middle::ty::Instance<'tcx>> = HashSet::new();
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(rustc_verifopt::manifest_dir()) else { return out };
    for entry in entries.flatten() {
        let Ok(json) = std::fs::read_to_string(entry.path()) else { continue };
        let Ok(manifest) = rustc_verifopt::InstanceManifest::from_json(&json) else { continue };
        if !program.contains(&manifest.crate_id) {
            continue;
        }
        let shapes = rustc_verifopt::ShapeRegistry::new();
        shapes.seed(&manifest.shapes.into_iter().collect());
        for (function, args) in manifest.dispatchers {
            let Some(args) = args else { continue };
            let Some(did) = rustc_verifopt::def_id_from_hash(tcx, function) else { continue };
            let Some(args) = args
                .iter()
                .map(|h| rustc_verifopt::arg_from_hash(tcx, *h, &shapes))
                .collect::<Option<Vec<_>>>()
            else {
                continue;
            };
            if args.len() != tcx.generics_of(did).count() {
                continue;
            }
            let instance = rustc_middle::ty::Instance::new_raw(did, tcx.mk_args(&args));
            if !seen.insert(instance) {
                continue;
            }
            if let Ok(i) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| rustc_internal::stable(instance))) {
                out.push(i);
            }
        }
    }
    out
}
