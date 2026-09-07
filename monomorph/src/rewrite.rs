extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_public;
extern crate rustc_session;
extern crate rustc_span;

use rustc_data_structures::fingerprint::Fingerprint;
use rustc_middle::mir::Body;
use rustc_span::def_id::{DefPathHash, LOCAL_CRATE, LocalDefId};

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler;
use rustc_middle::mir::pretty::MirWriter;
use rustc_middle::ty::TyCtxt;
use rustc_public::{DefId, rustc_internal};

use std::fs::{File, OpenOptions};
use std::io::Write;

use std::collections::HashMap; // FIXME FxHashMap for consistency?
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

use serde::{Deserialize, Serialize};

use crate::interp::TagPlan;
use crate::start_verifopt;
use crate::util::options::AnalysisOptions;

#[derive(Default)]
pub struct Store {
    pub targets: HashMap<(DefPathHash, usize), Vec<(DefPathHash, Option<Vec<DefPathHash>>)>>,
    pub tags: HashMap<
        (DefPathHash, usize),
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
        (SerializableDefPathHash, usize),
        Vec<(SerializableDefPathHash, Option<Vec<SerializableDefPathHash>>)>,
    )>,
    tags: Vec<(
        (SerializableDefPathHash, usize),
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
        SerializableStore {
            targets: store
                .targets
                .iter()
                .map(|((h, bb), v)| {
                    (
                        (SerializableDefPathHash::from(*h), *bb),
                        v.iter()
                            .map(|(h2, opt)| (SerializableDefPathHash::from(*h2), conv_opt_vec(opt)))
                            .collect(),
                    )
                })
                .collect(),
            tags: store
                .tags
                .iter()
                .map(|((h, bb), v)| {
                    (
                        (SerializableDefPathHash::from(*h), *bb),
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
        Store {
            targets: s
                .targets
                .into_iter()
                .map(|((h, bb), v)| {
                    (
                        (DefPathHash::from(h), bb),
                        v.into_iter()
                            .map(|(h2, opt)| (DefPathHash::from(h2), conv_opt_vec(opt)))
                            .collect(),
                    )
                })
                .collect(),
            tags: s
                .tags
                .into_iter()
                .map(|((h, bb), v)| {
                    (
                        (DefPathHash::from(h), bb),
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

pub fn dep_rewrite_store_path() -> &'static str {
    "verifopt_store.json"
}

pub fn needs_rewrite_pass_marker_path() -> &'static str {
    "verifopt_needs_rewrite_pass"
}

static SHARED_STORE: OnceLock<Option<Store>> = OnceLock::new();

fn load_shared_store() -> Option<Store> {
    let contents = std::fs::read_to_string(dep_rewrite_store_path()).ok()?;
    let serializable: SerializableStore = serde_json::from_str(&contents).ok()?;
    Some(Store::from(serializable))
}


fn store() -> &'static Mutex<Store> {
    STORE.get_or_init(|| Mutex::new(Store::default()))
}

pub struct FsaCallbacks {
    pub options: AnalysisOptions,
}

impl Callbacks for FsaCallbacks {
    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        let _ = rustc_internal::run(tcx, || {
            let (targets, tags) = start_verifopt(self.options.clone());

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
                    let rustc_public::ty::TyKind::RigidTy(rustc_public::ty::RigidTy::Adt(
                        adtdef,
                        sub_genargs,
                    )) = ty.kind()
                    else {
                        return None;
                    };
                    if !sub_genargs.0.is_empty() {
                        return None;
                    }
                    hashes.push(to_hash(adtdef.0)?);
                }
                Some(Some(hashes))
            };

            for ((defid, bb), (_, ts)) in targets {
                let Some(hash) = to_hash(defid) else {
                    continue;
                };

                let t_hashes: Vec<(DefPathHash, Option<Vec<DefPathHash>>)> = ts
                    .iter()
                    .filter_map(|(did, genargs)| {
                        let method_hash = to_hash(*did)?;
                        let self_hashes = to_genargs_hashes(genargs)?;
                        Some((method_hash, self_hashes))
                    })
                    .collect();

                store.targets.insert((hash, bb), t_hashes);
            }

            for ((defid, bb), plan) in tags {
                let TagPlan::Tagged(sites) = plan else {
                    continue;
                };
                if sites.is_empty() {
                    continue;
                }

                let Some(hash) = to_hash(defid) else {
                    continue;
                };

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

                store.tags.insert((hash, bb), entry);
            }

            if self.options.rewrite_pass {
            } else if std::env::var("CARGO_PRIMARY_PACKAGE").is_ok() {
                if let Ok(json) = serde_json::to_string(&SerializableStore::from(&*store)) {
                    let _ = std::fs::write(dep_rewrite_store_path(), json);
                }

                let primary_crate_id = tcx.stable_crate_id(LOCAL_CRATE);
                let needs_rewrite_pass = store
                    .targets
                    .keys()
                    .chain(store.tags.keys())
                    .any(|(hash, _bb)| hash.stable_crate_id() != primary_crate_id);

                if needs_rewrite_pass {
                    let _ = std::fs::write(needs_rewrite_pass_marker_path(), "1");
                    let _ = SECOND_PASS_NEEDED.set(());
                }
            }
        });

        Compilation::Stop
    }
}

static ORIGINAL: OnceLock<for<'tcx> fn(TyCtxt<'tcx>, LocalDefId) -> &'tcx Body<'tcx>> =
    OnceLock::new();

static EXTERN_ORIGINAL: OnceLock<
    for<'tcx> fn(TyCtxt<'tcx>, rustc_span::def_id::DefId) -> &'tcx Body<'tcx>,
> = OnceLock::new();

static SKIP_REWRITE: OnceLock<bool> = OnceLock::new();

static SECOND_PASS_NEEDED: OnceLock<()> = OnceLock::new();

static GENERIC_SKIPS: AtomicUsize = AtomicUsize::new(0);
static DYNAMIC_HITS: AtomicUsize = AtomicUsize::new(0);

static FN_OP_ARGS_MISMATCH: AtomicUsize = AtomicUsize::new(0);
static FN_OP_ARGS_OK: AtomicUsize = AtomicUsize::new(0);

static CRATE_NAME: OnceLock<String> = OnceLock::new();

pub fn rewrite_stats_path() -> &'static str {
    "rewrite_pointers_stats.txt"
}

pub fn write_rewrite_stats() {
    let generic = GENERIC_SKIPS.load(Ordering::Relaxed);
    let dynamic = DYNAMIC_HITS.load(Ordering::Relaxed);
    let args_mismatch = FN_OP_ARGS_MISMATCH.load(Ordering::Relaxed);
    let args_ok = FN_OP_ARGS_OK.load(Ordering::Relaxed);

    if generic == 0 && dynamic == 0 && args_mismatch == 0 && args_ok == 0 {
        return;
    }

    let crate_name = CRATE_NAME.get().map(String::as_str).unwrap_or("<unknown>");
    let mut lines = String::new();
    if generic != 0 || dynamic != 0 {
        lines.push_str(&format!(
            "{crate_name}: {} Edit::Pointers attempt(s) - {dynamic} dynamic (rewritten), \
             {generic} generic (skipped)\n",
            generic + dynamic,
        ));
    }
    if args_mismatch != 0 || args_ok != 0 {
        lines.push_str(&format!(
            "{crate_name}: {} fn_op call(s) - {args_ok} args resolved (rewritten), \
             {args_mismatch} args mismatch (skipped)\n",
            args_ok + args_mismatch,
        ));
    }

    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(rewrite_stats_path())
    {
        let _ = f.write_all(lines.as_bytes());
    }
}

static MIR_DUMP_FILE: OnceLock<Mutex<File>> = OnceLock::new();

fn mir_dump_file() -> &'static Mutex<File> {
    MIR_DUMP_FILE.get_or_init(|| {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("mir_dump.txt")
            .expect("failed to open mir_dump.txt for writing");
        Mutex::new(file)
    })
}

fn dump_body<'tcx>(tcx: TyCtxt<'tcx>, body: &Body<'tcx>, label: &str) {
    let mut buf = Vec::new();

    let writer = MirWriter::new(tcx);
    let _ = writer.write_mir_fn(body, &mut buf);

    let mut file = mir_dump_file().lock().unwrap();
    let _ = writeln!(file, "\n######### MIR {label} #########");
    let _ = file.write_all(&buf);
    let _ = writeln!(file, "######### END {label} #########\n");
}

enum Edit {
    Single(DefPathHash, Option<Vec<DefPathHash>>),
    Pointers(Vec<(DefPathHash, Option<Vec<DefPathHash>>)>),
    Tagged(Vec<(usize, usize, u64, DefPathHash, Option<Vec<DefPathHash>>)>),
}

const MAX_POINTERS_CANDIDATES: usize = 4;

fn compute_edits(store: &Store, hash: DefPathHash, default: &Body<'_>) -> Vec<(usize, Edit)> {
    default
        .basic_blocks
        .indices()
        .filter_map(|bb| {
            let key = &(hash, bb.as_usize());

            let tags = store.tags.get(key);
            let targets = store.targets.get(key)?;

            if targets.len() == 1 {
                // directly swap terminator
                Some((bb.as_usize(), Edit::Single(targets[0].0, targets[0].1.clone())))
            } else if let Some(tags) = tags {
                // tag dyn casts and switchint
                Some((bb.as_usize(), Edit::Tagged(tags.to_vec())))
            } else if targets.len() > 1 && targets.len() <= MAX_POINTERS_CANDIDATES {
                // direct conditionals on pointers
                Some((bb.as_usize(), Edit::Pointers(targets.to_vec())))
            } else {
                // leave vtable dyn call
                None
            }
        })
        .collect()
}

fn rewrite_body<'tcx>(
    tcx: TyCtxt<'tcx>,
    default: &'tcx Body<'tcx>,
    hash: DefPathHash,
) -> &'tcx Body<'tcx> {
    if *SKIP_REWRITE.get().unwrap_or(&false) {
        return default;
    }

    if SECOND_PASS_NEEDED.get().is_some() {
        return default;
    }

    let edits: Vec<(usize, Edit)> = match SHARED_STORE.get_or_init(load_shared_store) {
        Some(shared) => compute_edits(shared, hash, default),
        None => compute_edits(&store().lock().unwrap(), hash, default),
    };
    tcx.arena.alloc(apply_edits(tcx, default.clone(), edits))
}

