extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_public;
extern crate rustc_session;
extern crate rustc_span;

use rustc_middle::mir::{
    BasicBlock, BasicBlockData, BinOp, Body, CastKind, CoercionSource, Const, ConstOperand,
    LocalDecl, Mutability, Operand, Place, ProjectionElem, Rvalue, Statement, StatementKind,
    SwitchTargets, Terminator, TerminatorKind, UnOp,
};
use rustc_span::def_id::{DefPathHash, LocalDefId};

use rustc_driver::{Callbacks, Compilation};
use rustc_hir::Safety;
use rustc_interface::interface::{Compiler, Config};
use rustc_middle::mir::pretty::MirWriter;
use rustc_middle::ty::adjustment::PointerCoercion;
use rustc_middle::ty::{
    AssocKind, FnDef, GenericArg, Instance, List, Ty, TyCtxt, TyKind, TypingEnv, VtblEntry,
};
use rustc_public::rustc_internal;
use rustc_span::Span;

use std::io::{self, Write};

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

use crate::start_verifopt;
use crate::util::options::AnalysisOptions;

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
    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
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

static ORIGINAL: OnceLock<for<'tcx> fn(TyCtxt<'tcx>, LocalDefId) -> &'tcx Body<'tcx>> =
    OnceLock::new();

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

enum Edit {
    Single(DefPathHash),
    Multi(Vec<DefPathHash>),
}

fn optimized_mir<'tcx>(tcx: TyCtxt<'tcx>, def_id: LocalDefId) -> &'tcx Body<'tcx> {
    let original = ORIGINAL.get().unwrap();
    let default = original(tcx, def_id);

    let hash = tcx.def_path_hash(def_id.to_def_id());
    let store = store().lock().unwrap();

    let edits: Vec<(usize, Edit)> = default
        .basic_blocks
        .indices()
        .filter_map(|bb| {
            let ts = store.targets.get(&(hash, bb.as_usize()))?;

            if ts.len() == 1 {
                Some((bb.as_usize(), Edit::Single(ts[0])))
            } else if ts.len() > 1 {
                Some((bb.as_usize(), Edit::Multi(ts.to_vec())))
            } else {
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
            Edit::Single(hash) => {
                if let TerminatorKind::Call { func, .. } = &mut bbs[bb].terminator_mut().kind {
                    *func = fn_op(tcx, hash, gen_args, span).unwrap();
                }
            }

            Edit::Multi(hashes) => {
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

                let meta_ty = tcx.normalize_erasing_regions(TypingEnv::fully_monomorphized(), proj); // DynMetadata<dyn X>
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

                let mut fallback = None;
                let n = hashes.len();

                for (i, &hash) in hashes.iter().enumerate() {
                    let fnc = fn_op(tcx, hash, gen_args, span).unwrap();
                    let call_bb = bbs.push(BasicBlockData::new(
                        Some(Terminator {
                            source_info,
                            kind: TerminatorKind::Call {
                                func: fnc.clone(),
                                args: args.clone(),
                                destination: dest,
                                target: target,
                                unwind: unwind,
                                call_source: call_source,
                                fn_span: span,
                            },
                        }),
                        false,
                    ));

                    let Some(fallback_bb) = fallback else {
                        fallback = Some(call_bb);
                        continue;
                    };

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
                            targets: SwitchTargets::static_if(1, call_bb, fallback_bb),
                        },
                    };

                    if i == n - 1 {
                        bbs[bb].statements.push(eq_stmt);
                        bbs[bb].terminator = Some(new_term);
                    } else {
                        fallback = Some(bbs.push(BasicBlockData::new_stmts(
                            vec![eq_stmt],
                            Some(new_term),
                            false,
                        )));
                    }
                }
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
    gen_args: &'tcx List<GenericArg<'tcx>>,
    span: Span,
) -> Result<Operand<'tcx>, ()> {
    let target_did = tcx.def_path_hash_to_def_id(hash).unwrap();
    let instance =
        match Instance::try_resolve(tcx, TypingEnv::fully_monomorphized(), target_did, gen_args) {
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

    Ok(op)
}
