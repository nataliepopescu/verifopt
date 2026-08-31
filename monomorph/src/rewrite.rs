extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_public;
extern crate rustc_session;
extern crate rustc_span;

use rustc_data_structures::smallvec::SmallVec;
use rustc_index::IndexVec;
use rustc_middle::mir::{
    BasicBlock, BasicBlockData, BinOp, Body, CastKind, CoercionSource, Const, ConstOperand, Local,
    LocalDecl, Mutability, Operand, Place, ProjectionElem, Rvalue, SourceInfo, Statement,
    StatementKind, SwitchTargets, Terminator, TerminatorKind, UnOp,
};
use rustc_span::def_id::{DefPathHash, LocalDefId};

use rustc_driver::{Callbacks, Compilation};
use rustc_hir::Safety;
use rustc_hir::def::DefKind;
use rustc_interface::interface::{Compiler, Config};
use rustc_middle::mir::pretty::MirWriter;
use rustc_middle::ty::adjustment::PointerCoercion;
use rustc_middle::ty::{
    AssocKind, FnDef, GenericArg, Instance, List, Ty, TyCtxt, TyKind, TypingEnv, VtblEntry,
};
use rustc_public::{DefId, rustc_internal};
use rustc_span::Span;

use std::fs::{File, OpenOptions};
use std::io::Write;

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::sync::OnceLock;

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

            // FSA already resolved the concrete generic args each
            // candidate needs (e.g. `Self=Fast` for an inherited
            // `Worker::run` call dispatched to `Fast`, or `[I, A]` for a
            // blanket impl like `impl<I, A> Iterator for Box<I, A>`) -
            // but `Ty`/`GenericArgs`/`Instance` are all tied to *this*
            // compiler session's arena and can't be carried into the
            // rewrite phase's separate session. A `DefPathHash` can
            // (that's its whole purpose), so extract each arg's own
            // concrete type's DefId and hash *that*, for the common case
            // where every arg is a plain, non-generic concrete ADT.
            //
            // Three possible outcomes, not two - the middle one matters:
            //   - `None` (outer): FSA had no genargs for this candidate
            //     at all - a real per-impl override needs no further
            //     substitution, so the rewrite phase's previous
            //     call-site-genargs-based fallback is *correct* here.
            //   - `Some(None)`: same as above, spelled out explicitly.
            //   - `Some(Some(hashes))`: every arg was a plain concrete
            //     ADT and got hashed successfully - reconstruct from
            //     these.
            //   - Dropped entirely (the caller's `?` skips this
            //     candidate): FSA *did* resolve genargs, but at least
            //     one arg isn't a plain concrete ADT (a nested generic
            //     type, a lifetime, a const) - we can't safely
            //     reconstruct it from a hash without risking a subtly
            //     wrong partial substitution, so this candidate just
            //     doesn't get devirtualized at all, leaving the original
            //     (correct, if unoptimized) virtual call in place.
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
                        // Has its own nested generics (e.g. Vec<T> rather
                        // than a bare struct) - reconstructing this
                        // correctly would need to recurse, which risks
                        // getting it subtly wrong without a compiler to
                        // check against. Drop the candidate instead.
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
        });

        Compilation::Stop
    }
}

static ORIGINAL: OnceLock<for<'tcx> fn(TyCtxt<'tcx>, LocalDefId) -> &'tcx Body<'tcx>> =
    OnceLock::new();

static SKIP_REWRITE: OnceLock<bool> = OnceLock::new();

pub struct RewriteCallbacks {
    pub options: AnalysisOptions,
}

impl Callbacks for RewriteCallbacks {
    fn config(&mut self, config: &mut Config) {
        let _ = SKIP_REWRITE.set(self.options.no_rewrite);
        config.override_queries = Some(|_sess, providers| {
            let _ = ORIGINAL.set(providers.optimized_mir);
            providers.optimized_mir = optimized_mir;
        });
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

fn optimized_mir<'tcx>(tcx: TyCtxt<'tcx>, def_id: LocalDefId) -> &'tcx Body<'tcx> {
    let original = ORIGINAL.get().unwrap();
    let default = original(tcx, def_id);

    // Control mode (--no-rewrite): skip every rewrite unconditionally,
    // regardless of what FSA found - the resulting MIR (and therefore
    // codegen) is identical to a plain, unwrapped build, while still
    // going through the same two-phase pipeline and RUSTFLAGS. This is
    // what makes it a valid control for isolating the rewrites'
    // performance effect from anything the pipeline/flags alone might
    // change (e.g. -Z always_encode_mir potentially affecting inlining
    // or other optimization decisions even with zero rewrites applied).
    if *SKIP_REWRITE.get().unwrap_or(&false) {
        return default;
    }

    // Edit::Pointers builds a chain of runtime function-pointer-equality
    // checks, one per candidate, all merging into a shared continuation
    // block. Past a handful of candidates this produces a very wide
    // fan-in into that continuation block (and into its shared unwind
    // path) - observed in practice with ~40-80 candidates on a real
    // ripgrep flag-dispatch call site, where it triggered an LLVM
    // "Instruction does not dominate all uses!" codegen failure. Cap it
    // here: past this many candidates, fall back to leaving the original
    // (correct, if unoptimized) virtual call in place rather than risk
    // broken codegen. Edit::Tagged's integer-switch construction is
    // structurally different (no per-candidate pointer-comparison chain,
    // no shared wide-fan-in merge block the same way) and isn't
    // implicated, so it isn't capped here.
    const MAX_POINTERS_CANDIDATES: usize = 7;

    let hash = tcx.def_path_hash(def_id.to_def_id());
    let store = store().lock().unwrap();

    let edits: Vec<(usize, Edit)> = default
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
        .collect();

    if edits.is_empty() {
        return default;
    }

    let mut body = default.clone();

    dump_body(tcx, &body, "before");

    let local_decls = body.local_decls.clone();
    let mut bbs = body.basic_blocks_mut().to_owned();

    for (bb_idx, edit) in edits {
        let bb = BasicBlock::from_usize(bb_idx);

        let (defid, gen_args, args, dest, target, unwind, call_source, source_info, span) = {
            let term = bbs[bb].terminator();
            let TerminatorKind::Call {
                func,
                args,
                destination,
                target,
                unwind,
                call_source,
                ..
            } = &term.kind
            else {
                continue;
            };
            let (defid, gen_args) = match func {
                Operand::Constant(c) => match c.const_.ty().kind() {
                    FnDef(defid, a) => (*defid, *a), // *a: &'tcx List is Copy
                    _ => continue,
                },
                _ => continue,
            };
            (
                defid,
                gen_args,
                args.clone(),
                *destination,
                *target,
                *unwind,
                *call_source,
                term.source_info,
                term.source_info.span,
            )
        };

        match edit {
            Edit::Single(hash, self_hash) => {
                let (fnc, self_ty) = match fn_op(tcx, hash, self_hash, gen_args, span) {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                let (recv, new_stmts) = narrow_dyn(
                    tcx,
                    &mut body,
                    source_info,
                    args[0].node.clone(),
                    self_ty,
                    span,
                );
                bbs[bb].statements.extend(new_stmts);

                let mut new_args = args.clone();
                new_args[0].node = Operand::Move(recv);

                if let TerminatorKind::Call { func, args: a, .. } =
                    &mut bbs[bb].terminator_mut().kind
                {
                    *func = fnc;
                    *a = new_args;
                }
            }

            Edit::Pointers(hashes) => {
                let op = args[0].node.clone();

                let recv_ty = op.ty(&local_decls, tcx); // &dyn X
                let pointee_ty = recv_ty.builtin_deref(true).unwrap(); // dyn X

                // <dyn X as X>
                let trait_ref = match pointee_ty.kind() {
                    TyKind::Dynamic(preds, _) => {
                        let principal = preds.principal().unwrap();
                        principal.with_self_ty(tcx, pointee_ty).skip_binder()
                    }
                    _ => panic!(),
                };

                let pointee_trait = tcx.require_lang_item(rustc_hir::LangItem::PointeeTrait, span);
                let metadata_assoc = tcx
                    .associated_items(pointee_trait)
                    .in_definition_order()
                    .find(|it| matches!(it.kind, AssocKind::Type { .. }))
                    .unwrap()
                    .def_id;

                // <dyn X as Pointee>::Metadata
                let proj =
                    Ty::new_projection(tcx, metadata_assoc, tcx.mk_args(&[pointee_ty.into()]));

                let meta_ty = match tcx
                    .try_normalize_erasing_regions(TypingEnv::fully_monomorphized(), proj)
                {
                    Ok(ty) => ty, // DynMetadata<dyn X>
                    Err(_) => continue,
                };
                let raw_ptr_ty = Ty::new_ptr(tcx, tcx.types.unit, Mutability::Not); // *const ()

                // DynMetadata<dyn X>
                let meta_place = Place::from(body.local_decls.push(LocalDecl::new(meta_ty, span)));
                bbs[bb].statements.push(Statement::new(
                    source_info,
                    StatementKind::Assign(Box::new((
                        meta_place,
                        Rvalue::UnaryOp(UnOp::PtrMetadata, op),
                    ))),
                ));

                // raw *const ()
                let vt_ptr_place =
                    Place::from(body.local_decls.push(LocalDecl::new(raw_ptr_ty, span)));
                bbs[bb].statements.push(Statement::new(
                    source_info,
                    StatementKind::Assign(Box::new((
                        vt_ptr_place,
                        Rvalue::Cast(CastKind::Transmute, Operand::Move(meta_place), raw_ptr_ty),
                    ))),
                ));

                let entries = tcx.vtable_entries(trait_ref);
                let slot_idx = entries
                    .iter()
                    .position(|e| {
                        matches!(
                            e, VtblEntry::Method(inst) if inst.def_id() == defid
                        )
                    })
                    .unwrap();

                let VtblEntry::Method(vtable_instance) = &entries[slot_idx] else {
                    continue;
                };

                let fn_abi_ty = vtable_instance.ty(tcx, TypingEnv::fully_monomorphized());
                let fn_sig = fn_abi_ty.fn_sig(tcx);
                let fn_ptr_ty = Ty::new_fn_ptr(tcx, fn_sig);

                let vt_typed_ty = Ty::new_ptr(tcx, fn_ptr_ty, Mutability::Not);

                // *const (fn ptr)
                let vt_slots_place =
                    Place::from(body.local_decls.push(LocalDecl::new(vt_typed_ty, span)));
                bbs[bb].statements.push(Statement::new(
                    source_info,
                    StatementKind::Assign(Box::new((
                        vt_slots_place,
                        Rvalue::Cast(CastKind::PtrToPtr, Operand::Copy(vt_ptr_place), vt_typed_ty),
                    ))),
                ));

                let op = Box::new(ConstOperand {
                    span: span,
                    user_ty: None,
                    const_: Const::from_usize(tcx, slot_idx.try_into().unwrap()),
                });

                // vtable as slots + slot idx
                let slot_ptr_loc = body.local_decls.push(LocalDecl::new(vt_typed_ty, span));
                let slot_ptr_place = Place::from(slot_ptr_loc);

                bbs[bb].statements.push(Statement::new(
                    source_info,
                    StatementKind::Assign(Box::new((
                        slot_ptr_place,
                        Rvalue::BinaryOp(
                            BinOp::Offset,
                            Box::new((Operand::Copy(vt_slots_place), Operand::Constant(op))),
                        ),
                    ))),
                ));

                let deref_place = Place {
                    local: slot_ptr_loc,
                    projection: tcx.mk_place_elems(&[ProjectionElem::Deref]),
                };

                // loaded fn
                let slot_fn_place =
                    Place::from(body.local_decls.push(LocalDecl::new(fn_ptr_ty, span)));
                bbs[bb].statements.push(Statement::new(
                    source_info,
                    StatementKind::Assign(Box::new((
                        slot_fn_place,
                        Rvalue::Use(Operand::Copy(deref_place)),
                    ))),
                ));

                let orig = bbs[bb].terminator().clone();
                let mut fallback = bbs.push(BasicBlockData::new_stmts(vec![], Some(orig), false));
                let n = hashes.len();

                for (i, (hash, self_hash)) in hashes.iter().enumerate() {
                    let (fnc, self_ty) = match fn_op(tcx, *hash, self_hash.clone(), gen_args, span)
                    {
                        Ok(v) => v,
                        Err(_) => continue,
                    };

                    let (recv, new_stmts) = narrow_dyn(
                        tcx,
                        &mut body,
                        source_info,
                        args[0].node.clone(),
                        self_ty,
                        span,
                    );
                    let mut new_args = args.clone();
                    new_args[0].node = Operand::Move(recv);

                    let call_bb = bbs.push(BasicBlockData::new_stmts(
                        new_stmts,
                        Some(Terminator {
                            source_info,
                            kind: TerminatorKind::Call {
                                func: fnc.clone(),
                                args: new_args,
                                destination: dest,
                                target: target,
                                unwind: unwind,
                                call_source: call_source,
                                fn_span: span,
                            },
                        }),
                        false,
                    ));

                    let cand_ptr_place =
                        Place::from(body.local_decls.push(LocalDecl::new(fn_ptr_ty, span)));
                    bbs[bb].statements.push(Statement::new(
                        source_info,
                        StatementKind::Assign(Box::new((
                            cand_ptr_place,
                            Rvalue::Cast(
                                CastKind::PointerCoercion(
                                    PointerCoercion::ReifyFnPointer(Safety::Unsafe),
                                    CoercionSource::AsCast,
                                ),
                                fnc.clone(),
                                fn_ptr_ty,
                            ),
                        ))),
                    ));

                    let eq_place =
                        Place::from(body.local_decls.push(LocalDecl::new(tcx.types.bool, span)));

                    let eq_stmt = Statement::new(
                        source_info,
                        StatementKind::Assign(Box::new((
                            eq_place,
                            Rvalue::BinaryOp(
                                BinOp::Eq,
                                Box::new((
                                    Operand::Copy(slot_fn_place),
                                    Operand::Copy(cand_ptr_place),
                                )),
                            ),
                        ))),
                    );

                    let new_term = Terminator {
                        source_info,
                        kind: TerminatorKind::SwitchInt {
                            discr: Operand::Copy(eq_place),
                            targets: SwitchTargets::static_if(1, call_bb, fallback),
                        },
                    };

                    if i == n - 1 {
                        bbs[bb].statements.push(eq_stmt);
                        bbs[bb].terminator = Some(new_term);
                    } else {
                        fallback = bbs.push(BasicBlockData::new_stmts(
                            vec![eq_stmt],
                            Some(new_term),
                            false,
                        ));
                    }
                }
            }

            Edit::Tagged(sites) => {
                let recv_local = match &args[0].node {
                    Operand::Copy(p) | Operand::Move(p) if p.projection.is_empty() => p.local,
                    _ => continue,
                };

                let preds = default.basic_blocks.predecessors();

                let found = find_casts(&bbs, preds, bb_idx, recv_local, &mut HashSet::new());

                let planned: HashSet<(usize, usize)> = sites
                    .iter()
                    .map(|(bb, stmt, _, _, _)| (*bb, *stmt))
                    .collect();
                if found != Some(planned) {
                    continue;
                }

                let tag_local = body.local_decls.push(LocalDecl::new(tcx.types.usize, span));

                for (bb_idx, stmt_idx, tag, _, _) in &sites {
                    let cb = BasicBlock::from_usize(*bb_idx);

                    bbs[cb].statements.insert(
                        stmt_idx + 1,
                        Statement::new(
                            source_info,
                            StatementKind::Assign(Box::new((
                                Place::from(tag_local),
                                Rvalue::Use(Operand::Constant(Box::new(ConstOperand {
                                    span,
                                    user_ty: None,
                                    const_: Const::from_usize(tcx, *tag),
                                }))),
                            ))),
                        ),
                    );
                }

                let orig = bbs[bb].terminator().clone();
                let fallback = bbs.push(BasicBlockData::new_stmts(vec![], Some(orig), false));

                let mut arms = Vec::new();

                for (_, _, tag, impl_hash, self_hash) in &sites {
                    let (fnc, self_ty) =
                        match fn_op(tcx, *impl_hash, self_hash.clone(), gen_args, span) {
                            Ok(v) => v,
                            Err(_) => continue,
                        };
                    let (recv, stmts) = narrow_dyn(
                        tcx,
                        &mut body,
                        source_info,
                        args[0].node.clone(),
                        self_ty,
                        span,
                    );

                    let mut new_args = args.clone();
                    new_args[0].node = Operand::Move(recv);

                    let cb = bbs.push(BasicBlockData::new_stmts(
                        stmts,
                        Some(Terminator {
                            source_info,
                            kind: TerminatorKind::Call {
                                func: fnc,
                                args: new_args,
                                destination: dest,
                                target,
                                unwind,
                                call_source,
                                fn_span: span,
                            },
                        }),
                        false,
                    ));
                    arms.push((*tag as u128, cb));
                }

                bbs[bb].terminator = Some(Terminator {
                    source_info,
                    kind: TerminatorKind::SwitchInt {
                        discr: Operand::Copy(Place::from(tag_local)),
                        targets: SwitchTargets::new(arms.into_iter(), fallback),
                    },
                });
            }
        }
    }

    *body.basic_blocks_mut() = bbs;

    dump_body(tcx, &body, "after");

    tcx.arena.alloc(body)
}

fn fn_op<'tcx>(
    tcx: TyCtxt<'tcx>,
    hash: DefPathHash,
    self_hashes: Option<Vec<DefPathHash>>,
    gen_args: &'tcx List<GenericArg<'tcx>>,
    span: Span,
) -> Result<(Operand<'tcx>, Ty<'tcx>), ()> {
    let target_did = tcx.def_path_hash_to_def_id(hash).unwrap();

    let args = match &self_hashes {
        Some(hashes) => {
            // FSA already resolved the full set of concrete generic args
            // this candidate needs (e.g. `Self=Fast` for an inherited
            // default trait method, or `[I, A]` for a blanket impl like
            // `impl<I, A> Iterator for Box<I, A>`) - a true virtual call
            // site's own `gen_args` carries no monomorphization info at
            // all, so `gen_args.iter().skip(1)` below is empty here, and
            // handing an incomplete/empty args list to a target that
            // needs more is exactly what panics deep inside rustc's own
            // generic-args instantiation code (either an out-of-bounds
            // index, or "has parameters, but no args were provided").
            // Reconstruct every arg from its own DefPathHash instead -
            // unlike a GenericArgs or Instance, that's safe to carry
            // across the two separate compiler sessions FSA and rewrite
            // each run in. Each hash is only usable here if it names a
            // plain, non-generic concrete type (see `to_genargs_hashes`
            // in FsaCallbacks::after_analysis) - anything more complex
            // (a nested-generic type, a lifetime, a const) was already
            // filtered out at that point rather than risking a wrong
            // partial reconstruction here.
            let tys: Vec<Ty<'tcx>> = hashes
                .iter()
                .map(|h| {
                    let did = tcx.def_path_hash_to_def_id(*h).ok_or(())?;
                    Ok(tcx.type_of(did).instantiate_identity())
                })
                .collect::<Result<Vec<_>, ()>>()?;
            let arg_list: Vec<GenericArg<'tcx>> = tys.into_iter().map(|t| t.into()).collect();
            tcx.mk_args(&arg_list)
        }
        // No resolved args available (e.g. a real per-impl override,
        // where the target's own DefId needs no further substitution) -
        // fall back to the previous behavior unchanged.
        None => tcx.mk_args_from_iter(gen_args.iter().skip(1)),
    };

    let instance =
        match Instance::try_resolve(tcx, TypingEnv::fully_monomorphized(), target_did, args) {
            Ok(Some(inst)) => inst,
            _ => return Err(()),
        };

    let fn_ty = instance.ty(tcx, TypingEnv::fully_monomorphized());
    let new_const = Const::zero_sized(fn_ty);

    let op = Operand::Constant(Box::new(ConstOperand {
        span: span,
        user_ty: None,
        const_: new_const,
    }));

    let parent_did = tcx.parent(target_did);
    let raw_self_ty = if tcx.def_kind(parent_did) == DefKind::Trait {
        // Inherited default trait method - target_did's own parent is
        // the *trait* itself (e.g. `Worker`), not an impl block. Calling
        // `tcx.type_of` on a bare trait DefId is invalid and ICEs rustc
        // directly ("compute_type_of_item: unexpected item type:
        // Trait(...)"). There's no impl block to derive Self from here -
        // it's exactly the one resolved arg we already have (a trait
        // default method's only extra generic parameter is Self itself).
        match &self_hashes {
            Some(hashes) if !hashes.is_empty() => {
                let self_did = tcx.def_path_hash_to_def_id(hashes[0]).ok_or(())?;
                tcx.type_of(self_did).instantiate_identity()
            }
            _ => return Err(()),
        }
    } else {
        // A real impl block - either a per-impl override (args is empty,
        // matching the target's own already-concrete DefId) or a
        // blanket impl like `impl<I, A> Iterator for Box<I, A>` (args is
        // our reconstructed [I, A]) - `type_of` on the impl block,
        // applied to those args, gives the correct Self either way (a
        // concrete struct, or `Box<I, A>`).
        tcx.type_of(parent_did).instantiate(tcx, instance.args)
    };
    let self_ty = match tcx.try_normalize_erasing_regions(TypingEnv::fully_monomorphized(), raw_self_ty)
    {
        Ok(ty) => ty,
        // Can genuinely fail to normalize here (e.g. an unresolved
        // Iterator::Item projection through a closure chain) rather than
        // it being a bug on our end - rustc's own ICE message for the
        // panicking variant says to use this fallible one instead.
        Err(_) => return Err(()),
    };

    Ok((op, self_ty))
}

fn narrow_dyn<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &mut Body<'tcx>,
    si: SourceInfo,
    recv: Operand<'tcx>,
    self_ty: Ty<'tcx>,
    span: Span,
) -> (Place<'tcx>, Vec<Statement<'tcx>>) {
    let ptr_ty = Ty::new_ptr(tcx, self_ty, Mutability::Not);
    let ref_ty = Ty::new_ref(tcx, tcx.lifetimes.re_erased, self_ty, Mutability::Not);

    let mut stmts = Vec::new();

    let thin = Place::from(body.local_decls.push(LocalDecl::new(ptr_ty, span)));
    stmts.push(Statement::new(
        si,
        StatementKind::Assign(Box::new((
            thin,
            Rvalue::Cast(CastKind::PtrToPtr, recv, ptr_ty),
        ))),
    ));

    let deref = Place {
        local: thin.local,
        projection: tcx.mk_place_elems(&[ProjectionElem::Deref]),
    };

    let out = Place::from(body.local_decls.push(LocalDecl::new(ref_ty, span)));
    stmts.push(Statement::new(
        si,
        StatementKind::Assign(Box::new((
            out,
            Rvalue::Ref(
                tcx.lifetimes.re_erased,
                rustc_middle::mir::BorrowKind::Shared,
                deref,
            ),
        ))),
    ));

    (out, stmts)
}

fn find_casts<'tcx>(
    bbs: &IndexVec<BasicBlock, BasicBlockData<'tcx>>,
    preds: &IndexVec<BasicBlock, SmallVec<[BasicBlock; 4]>>,
    bb_idx: usize,
    local: Local,
    seen: &mut HashSet<(usize, Local)>,
) -> Option<HashSet<(usize, usize)>> {
    if !seen.insert((bb_idx, local)) {
        return Some(HashSet::new());
    }

    let bb = BasicBlock::from_usize(bb_idx);

    for (i, stmt) in bbs[bb].statements.iter().enumerate().rev() {
        let StatementKind::Assign(b) = &stmt.kind else {
            continue;
        };
        let (p, rv) = *b.clone();
        if p.local != local || !p.projection.is_empty() {
            continue;
        }

        return match rv {
            Rvalue::Cast(CastKind::PointerCoercion(PointerCoercion::Unsize, ..), ..) => {
                Some([(bb_idx, i)].into_iter().collect())
            }
            Rvalue::Use(Operand::Copy(q) | Operand::Move(q)) if q.projection.is_empty() => {
                find_casts(bbs, preds, bb_idx, q.local, seen)
            }
            _ => None,
        };
    }

    let ps = &preds[bb];
    if ps.is_empty() {
        return None;
    }

    let mut out = HashSet::new();
    for p in ps {
        out.extend(find_casts(bbs, preds, p.index(), local, seen)?);
    }

    Some(out)
}
