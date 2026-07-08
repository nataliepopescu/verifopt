extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_public;
extern crate rustc_session;
extern crate rustc_span;

use rustc_middle::mir::{BasicBlock, Body, Const, ConstOperand, Operand, TerminatorKind};
use rustc_span::def_id::{DefPathHash, LocalDefId};

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::{Compiler, Config};
use rustc_middle::mir::pretty::MirWriter;
use rustc_middle::ty::{FnDef, Instance, TyCtxt, TypingEnv};
use rustc_public::rustc_internal;

use std::io;
use std::io::Write;

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

use crate::{start_verifopt, util::options::AnalysisOptions};

#[derive(Default)]
pub struct Store {
    pub targets: HashMap<(DefPathHash, usize), Vec<DefPathHash>>,
}

static STORE: OnceLock<Mutex<Store>> = OnceLock::new();

fn store() -> &'static Mutex<Store> {
    STORE.get_or_init(|| Mutex::new(Store::default()))
}

pub struct FsaCallbacks {
    pub options: AnalysisOptions,
}

impl Callbacks for FsaCallbacks {
    fn after_analysis<'a>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'a>) -> Compilation {
        let _ = rustc_internal::run(tcx, || {
            let targets = start_verifopt(self.options.clone());

            let mut store = store().lock().unwrap();

            for ((defid, bb), (_, ts)) in targets {
                let internal = rustc_internal::internal(tcx, defid);
                let hash = tcx.def_path_hash(internal);

                let t_hashes: Vec<DefPathHash> = ts
                    .iter()
                    .map(|(did, _)| {
                        let internal = rustc_internal::internal(tcx, *did);
                        tcx.def_path_hash(internal)
                    })
                    .collect();

                store.targets.insert((hash, bb), t_hashes);
            }
        });

        Compilation::Stop
    }
}

static ORIGINAL: OnceLock<for<'a> fn(TyCtxt<'a>, LocalDefId) -> &'a Body<'a>> = OnceLock::new();

pub struct RewriteCallbacks;

impl Callbacks for RewriteCallbacks {
    fn config(&mut self, config: &mut Config) {
        config.override_queries = Some(|_sess, providers| {
            let _ = ORIGINAL.set(providers.optimized_mir);
            providers.optimized_mir = optimized_mir;
        });
    }
}

fn dump_body<'tcx>(tcx: TyCtxt<'tcx>, body: &Body<'tcx>, label: &str) {
    let mut buf = Vec::new();

    let writer = MirWriter::new(tcx);
    let _ = writer.write_mir_fn(body, &mut buf);

    eprintln!("\n######### MIR {label} #########");
    io::stderr().write_all(&buf).unwrap();
    eprintln!("######### END {label} #########\n");
}

fn optimized_mir<'a>(tcx: TyCtxt<'a>, def_id: LocalDefId) -> &'a Body<'a> {
    let original = ORIGINAL.get().unwrap();
    let default = original(tcx, def_id);

    let hash = tcx.def_path_hash(def_id.to_def_id());
    let store = store().lock().unwrap();

    let edits: Vec<(usize, DefPathHash)> = default
        .basic_blocks
        .indices()
        .filter_map(|bb| {
            let ts = store.targets.get(&(hash, bb.as_usize()))?;

            if ts.len() != 1 {
                return None;
            }
            Some((bb.as_usize(), ts[0]))
        })
        .collect();

    if edits.is_empty() {
        return default;
    }

    let mut body = default.clone();

    dump_body(tcx, &body, "before");

    let bbs = body.basic_blocks_mut();

    for (bb_idx, target_hash) in edits {
        let bb = BasicBlock::from_usize(bb_idx);
        let term = bbs[bb].terminator_mut();

        let TerminatorKind::Call { func, .. } = &mut term.kind else {
            continue;
        };

        let gen_args = match func {
            Operand::Constant(c) => match c.const_.ty().kind() {
                FnDef(_, args) => args,
                _ => {
                    continue;
                }
            },
            _ => {
                continue;
            }
        };

        let target_did = tcx.def_path_hash_to_def_id(target_hash).unwrap();
        let instance = match Instance::try_resolve(
            tcx,
            TypingEnv::fully_monomorphized(),
            target_did,
            gen_args,
        ) {
            Ok(Some(inst)) => inst,
            _ => continue,
        };

        let fn_ty = instance.ty(tcx, TypingEnv::fully_monomorphized());
        let new_const = Const::zero_sized(fn_ty);

        *func = Operand::Constant(Box::new(ConstOperand {
            span: term.source_info.span,
            user_ty: None,
            const_: new_const,
        }));
    }

    dump_body(tcx, &body, "after");

    tcx.arena.alloc(body)
}
