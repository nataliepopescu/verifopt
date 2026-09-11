extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_public;
extern crate rustc_session;
extern crate rustc_span;

use rustc_data_structures::fingerprint::Fingerprint;
use rustc_span::def_id::{DefPathHash, LOCAL_CRATE};

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;
use rustc_public::{DefId, rustc_internal};

use std::collections::HashMap; // FIXME FxHashMap for consistency?
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::interp::TagPlan;
use crate::start_verifopt;
use crate::util::options::AnalysisOptions;

#[derive(Default)]
pub struct Store {
    pub targets:
        HashMap<(DefPathHash, usize, Vec<DefPathHash>), Vec<(DefPathHash, Option<Vec<DefPathHash>>)>>,
    pub tags: HashMap<
        (DefPathHash, usize, Vec<DefPathHash>),
        Vec<(
            usize,                     /* bb */
            usize,                     /* stmt */
            u64,                       /* tag */
            DefPathHash,               /* impl fn */
            Option<Vec<DefPathHash>>,  /* concrete generic args, when resolvable */
        )>,
    >,
}

static STORE: OnceLock<Mutex<Store>> = OnceLock::new();

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct SerializableDefPathHash([u8; 16]);

impl From<DefPathHash> for SerializableDefPathHash {
    fn from(dph: DefPathHash) -> Self {
        SerializableDefPathHash(dph.0.to_le_bytes())
    }
}

impl From<SerializableDefPathHash> for DefPathHash {
    fn from(s: SerializableDefPathHash) -> Self {
        DefPathHash(Fingerprint::from_le_bytes(s.0))
    }
}

#[derive(Serialize, Deserialize, Default)]
struct SerializableStore {
    targets: Vec<(
        (SerializableDefPathHash, usize, Vec<SerializableDefPathHash>),
        Vec<(SerializableDefPathHash, Option<Vec<SerializableDefPathHash>>)>,
    )>,
    tags: Vec<(
        (SerializableDefPathHash, usize, Vec<SerializableDefPathHash>),
        Vec<(
            usize,
            usize,
            u64,
            SerializableDefPathHash,
            Option<Vec<SerializableDefPathHash>>,
        )>,
    )>,
}

impl From<&Store> for SerializableStore {
    fn from(store: &Store) -> Self {
        let conv_opt_vec = |opt: &Option<Vec<DefPathHash>>| {
            opt.as_ref()
                .map(|v| v.iter().map(|h| SerializableDefPathHash::from(*h)).collect())
        };
        let conv_vec = |v: &Vec<DefPathHash>| -> Vec<SerializableDefPathHash> {
            v.iter().map(|h| SerializableDefPathHash::from(*h)).collect()
        };
        SerializableStore {
            targets: store
                .targets
                .iter()
                .map(|((h, bb, caller_genargs), v)| {
                    (
                        (SerializableDefPathHash::from(*h), *bb, conv_vec(caller_genargs)),
                        v.iter()
                            .map(|(h2, opt)| (SerializableDefPathHash::from(*h2), conv_opt_vec(opt)))
                            .collect(),
                    )
                })
                .collect(),
            tags: store
                .tags
                .iter()
                .map(|((h, bb, caller_genargs), v)| {
                    (
                        (SerializableDefPathHash::from(*h), *bb, conv_vec(caller_genargs)),
                        v.iter()
                            .map(|(bb2, stmt, tag, h2, opt)| {
                                (
                                    *bb2,
                                    *stmt,
                                    *tag,
                                    SerializableDefPathHash::from(*h2),
                                    conv_opt_vec(opt),
                                )
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

impl From<SerializableStore> for Store {
    fn from(s: SerializableStore) -> Self {
        let conv_opt_vec = |opt: Option<Vec<SerializableDefPathHash>>| {
            opt.map(|v| v.into_iter().map(DefPathHash::from).collect())
        };
        let conv_vec =
            |v: Vec<SerializableDefPathHash>| -> Vec<DefPathHash> {
                v.into_iter().map(DefPathHash::from).collect()
            };
        Store {
            targets: s
                .targets
                .into_iter()
                .map(|((h, bb, caller_genargs), v)| {
                    (
                        (DefPathHash::from(h), bb, conv_vec(caller_genargs)),
                        v.into_iter()
                            .map(|(h2, opt)| (DefPathHash::from(h2), conv_opt_vec(opt)))
                            .collect(),
                    )
                })
                .collect(),
            tags: s
                .tags
                .into_iter()
                .map(|((h, bb, caller_genargs), v)| {
                    (
                        (DefPathHash::from(h), bb, conv_vec(caller_genargs)),
                        v.into_iter()
                            .map(|(bb2, stmt, tag, h2, opt)| {
                                (bb2, stmt, tag, DefPathHash::from(h2), conv_opt_vec(opt))
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

/// Returns the absolute path to verifopt_store.json - resolved via
/// VERIFOPT_STORE_DIR (set by cargo-verifopt's own run_cargo_build, on
/// the top-level cargo command it spawns, so every downstream process
/// it transitively spawns inherits it) when present, falling back to a
/// plain, CWD-relative path otherwise (e.g. a standalone, single-crate
/// test case that never goes through cargo-verifopt at all).
///
/// This matters because cargo itself runs each crate's own rustc
/// invocation with its CWD set to *that crate's own manifest
/// directory* - not necessarily the same directory cargo-verifopt
/// itself was invoked from. For a dependency crate that isn't a formal
/// workspace member of the primary crate (which includes essentially
/// every ordinary, crates.io-sourced dependency), a plain, CWD-relative
/// "verifopt_store.json" would never be found at all - not because it
/// doesn't exist, but because that specific rustc invocation is running
/// from an entirely different directory than the one it was written to.
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

/// Produces a small, fixed, deterministic DefPathHash for a primitive
/// type - not a real DefId hash at all (primitives have no DefId), just
/// a stand-in that lets the existing Vec<DefPathHash> key-component slot
/// also represent primitive generic args (bool, char, ints, floats)
/// without introducing a whole new key-component type and re-touching
/// the serialization layer again.
///
/// Implemented identically on this side and the rust fork's own
/// verifopt_rewrite.rs (see that file's own copy of this same
/// function) - both sides must compute the same sentinel for the same
/// primitive, on the same pinned rustc build, for the store's own keys
/// to ever line up across the two, separate processes at all. A real
/// DefPathHash coinciding with one of these specific, small sentinel
/// values is astronomically unlikely, for the same reason DefPathHash
/// collisions in general are treated as negligible risk elsewhere in
/// this codebase - not a new, additional risk being introduced here.
///
/// FNV-1a, not anything cryptographic or rustc-internal - deliberately
/// simple and self-contained so it's trivial to keep byte-for-byte
/// identical between the two, separate copies of this function.
fn primitive_ty_sentinel(tag: &str) -> DefPathHash {
    fn fnv1a_64(bytes: &[u8]) -> u64 {
        let mut hash: u64 = 0xcbf29ce484222325;
        for &b in bytes {
            hash ^= b as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }
    let h1 = fnv1a_64(tag.as_bytes());
    // Different input for the second half (not just re-hashing h1's own
    // bytes) so a short tag's own two halves don't trivially collide
    // with each other.
    let h2 = fnv1a_64(format!("{tag}#verifopt-sentinel").as_bytes());
    DefPathHash(Fingerprint::new(h1, h2))
}

pub struct FsaCallbacks {
    pub options: AnalysisOptions,
}

impl Callbacks for FsaCallbacks {
    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        if self.options.skip_analysis {
            return Compilation::Continue;
        }

        let _ = rustc_internal::run(tcx, || {
            let Ok((targets, tags)) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                start_verifopt(self.options.clone())
            })) else {
                eprintln!(
                    "[verifopt debug] start_verifopt itself panicked - no store written this run"
                );
                return;
            };

            let mut store = store().lock().unwrap();

            let to_hash = |did| -> Option<DefPathHash> {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    tcx.def_path_hash(rustc_internal::internal(tcx, did))
                }))
                .inspect_err(|_| eprintln!("to_hash panicked on {:?}, skipping", did))
                .ok()
            };

            let to_genargs_hashes = |genargs: &Option<rustc_public::ty::GenericArgs>|
             -> Option<Option<Vec<DefPathHash>>> {
                let Some(genargs) = genargs.as_ref() else {
                    return Some(None);
                };
                let mut hashes = Vec::with_capacity(genargs.0.len());
                for arg in &genargs.0 {
                    let rustc_public::ty::GenericArgKind::Type(ty) = arg else {
                        return None;
                    };
                    let rustc_public::ty::TyKind::RigidTy(rigid_ty) = ty.kind() else {
                        return None;
                    };
                    let hash = match rigid_ty {
                        rustc_public::ty::RigidTy::Adt(adtdef, sub_genargs) => {
                            if !sub_genargs.0.is_empty() {
                                return None;
                            }
                            to_hash(adtdef.0)?
                        }
                        rustc_public::ty::RigidTy::Bool => primitive_ty_sentinel("prim:bool"),
                        rustc_public::ty::RigidTy::Char => primitive_ty_sentinel("prim:char"),
                        rustc_public::ty::RigidTy::Int(int_ty) => {
                            let tag = match int_ty {
                                rustc_public::ty::IntTy::Isize => "prim:isize",
                                rustc_public::ty::IntTy::I8 => "prim:i8",
                                rustc_public::ty::IntTy::I16 => "prim:i16",
                                rustc_public::ty::IntTy::I32 => "prim:i32",
                                rustc_public::ty::IntTy::I64 => "prim:i64",
                                rustc_public::ty::IntTy::I128 => "prim:i128",
                            };
                            primitive_ty_sentinel(tag)
                        }
                        rustc_public::ty::RigidTy::Uint(uint_ty) => {
                            let tag = match uint_ty {
                                rustc_public::ty::UintTy::Usize => "prim:usize",
                                rustc_public::ty::UintTy::U8 => "prim:u8",
                                rustc_public::ty::UintTy::U16 => "prim:u16",
                                rustc_public::ty::UintTy::U32 => "prim:u32",
                                rustc_public::ty::UintTy::U64 => "prim:u64",
                                rustc_public::ty::UintTy::U128 => "prim:u128",
                            };
                            primitive_ty_sentinel(tag)
                        }
                        rustc_public::ty::RigidTy::Float(float_ty) => {
                            let tag = match float_ty {
                                rustc_public::ty::FloatTy::F16 => "prim:f16",
                                rustc_public::ty::FloatTy::F32 => "prim:f32",
                                rustc_public::ty::FloatTy::F64 => "prim:f64",
                                rustc_public::ty::FloatTy::F128 => "prim:f128",
                            };
                            primitive_ty_sentinel(tag)
                        }
                        // References, tuples, closures, dyn types, etc. -
                        // not yet handled; returning None here means the
                        // caller-genargs use of this closure panics
                        // rather than silently collapsing distinct
                        // instantiations onto the same key.
                        _ => return None,
                    };
                    hashes.push(hash);
                }
                Some(Some(hashes))
            };

            for ((defid, bb, caller_genargs), (_, ts)) in targets {
                let Some(hash) = to_hash(defid) else {
                    continue;
                };

                let Some(caller_genargs_hash) = to_genargs_hashes(&Some(caller_genargs.clone()))
                else {
                    panic!(
                        "could not hash caller's own generic args for {:?} at bb{} - \
                         genargs: {:?} - without this, this dispatch site's own key \
                         would collapse different monomorphized instantiations of the \
                         same generic function onto the same store entry, silently \
                         merging what may be genuinely different rewrites",
                        defid, bb, caller_genargs
                    );
                };
                let caller_genargs_hash = caller_genargs_hash
                    .expect("caller_genargs was wrapped in Some(...) above, so to_genargs_hashes' own \"was the input None\" case cannot fire here");

                let t_hashes: Vec<(DefPathHash, Option<Vec<DefPathHash>>)> = ts
                    .iter()
                    .filter_map(|(did, genargs)| {
                        let method_hash = to_hash(*did)?;
                        let self_hashes = to_genargs_hashes(genargs)?;
                        Some((method_hash, self_hashes))
                    })
                    .collect();

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

                let Some(caller_genargs_hash) = to_genargs_hashes(&Some(caller_genargs.clone()))
                else {
                    panic!(
                        "could not hash caller's own generic args for {:?} at bb{} - \
                         genargs: {:?} - without this, this dispatch site's own key \
                         would collapse different monomorphized instantiations of the \
                         same generic function onto the same store entry, silently \
                         merging what may be genuinely different rewrites",
                        defid, bb, caller_genargs
                    );
                };
                let caller_genargs_hash = caller_genargs_hash
                    .expect("caller_genargs was wrapped in Some(...) above, so to_genargs_hashes' own \"was the input None\" case cannot fire here");

                let mut next: u64 = 0;
                let mut assigned: HashMap<DefId, u64> = HashMap::default();

                let entry: Vec<(usize, usize, u64, DefPathHash, Option<Vec<DefPathHash>>)> = sites
                    .iter()
                    .filter_map(|(bb, stmt, did, genargs)| {
                        let tag = *assigned.entry(*did).or_insert_with(|| {
                            next += 1;
                            next - 1
                        });
                        let hash = to_hash(*did)?;
                        let self_hashes = to_genargs_hashes(genargs)?;
                        Some((*bb, *stmt, tag, hash, self_hashes))
                    })
                    .collect();

                store.tags.insert((hash, bb, caller_genargs_hash), entry);
            }

            if std::env::var("CARGO_PRIMARY_PACKAGE").is_ok() {
                if let Ok(json) = serde_json::to_string(&SerializableStore::from(&*store)) {
                    let _ = std::fs::write(dep_rewrite_store_path(), json);
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

