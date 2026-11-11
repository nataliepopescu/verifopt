use std::collections::VecDeque;
use std::rc::Rc;

use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_data_structures::fx::FxHashSet as HashSet;
use rustc_index::bit_set::DenseBitSet;
//use rustc_public::mir::mono::Instance;
use rustc_public::mir::{BasicBlock, Body, Successors, TerminatorKind, UnwindAction};

use log::debug;

use crate::constraints::VOID;

const START_BLOCK: usize = 0;

/// Ported from rustc_middle::mir::traversal.rs
#[derive(Debug)]
pub struct Postorder<'a> {
    blocks: &'a Vec<BasicBlock>,
    visited: DenseBitSet<usize>,
    // point bb to successor bbs
    visit_stack: Vec<(usize, Successors)>,
}

impl<'a> Postorder<'a> {
    pub fn new(blocks: &'a Vec<BasicBlock>, root: usize) -> Postorder<'a> {
        let mut po = Postorder {
            blocks,
            visited: DenseBitSet::new_empty(blocks.len()),
            visit_stack: Vec::new(),
        };

        po.visit(root);
        po.traverse_successor();

        po
    }

    fn visit(&mut self, bb: usize) {
        if !self.visited.insert(bb) {
            return;
        }
        let data = &self.blocks[bb];
        let successors = data.terminator.successors();
        self.visit_stack.push((bb, successors));
    }

    fn traverse_successor(&mut self) {
        // See documentation for loop logic here: https://doc.rust-lang.org/nightly/nightly-rustc/src/rustc_middle/mir/traversal.rs.html#138
        while let Some(bb) = self
            .visit_stack
            .last_mut()
            .and_then(|(_, successors)| successors.pop())
        {
            // While loop body
            self.visit(bb);
        }
    }
}

impl<'a> Iterator for Postorder<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let (bb, _) = self.visit_stack.pop()?;
        self.traverse_successor();

        Some(bb)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let lower = self.visit_stack.len();
        let upper = self.blocks.len();
        (lower, Some(upper))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BBDeps {
    // Rc, not a plain Vec: this is a full clone of the function's MIR
    // blocks (statements+terminators), and unlike ordering/visited/dead_edges
    // it's never mutated again after BBDeps::new() builds it - prune() and
    // mark_visited() only ever touch ordering/visited/dead_edges. Since
    // Context::clone() (called once per candidate at every dynamic-dispatch
    // site) derives Clone across the whole wtos: HashMap<VOID, BBDeps> map,
    // every visited function's MIR was getting fully re-copied on every
    // single dispatch-site clone. Rc::clone() is a pointer-copy instead.
    pub blocks: Rc<Vec<BasicBlock>>,
    pub preds: HashMap<usize, Vec<usize>>,
    pub ordering: VecDeque<usize>,
    pub visited: Vec<usize>,
    pub has_ret: bool,
    /// Switch edges (block, target) pruned during this walk (see prune);
    /// empty in the per-scope template.
    pub dead_edges: HashSet<(usize, usize)>,
}

impl BBDeps {
    pub fn new<'tcx>(body: &Body) -> Self {
        let mut bb_deps = BBDeps {
            blocks: Rc::new(body.blocks.clone()),
            preds: HashMap::default(),
            ordering: VecDeque::new(),
            visited: Vec::new(),
            has_ret: false,
            dead_edges: HashSet::default(),
        };

        //debug!("%%%%%");
        //debug!("total bbs: {:?}", body.blocks.len());

        // Get Successor edges
        for (bb, bb_data) in body.blocks.iter().enumerate() {
            bb_deps.get_deps(bb, bb_data);
        }

        //debug!("self.pred: {:?}", bb_deps.preds);

        let mut ret_bb: usize = 0;
        let mut ret_set = false;
        let mut ret_found = false;

        // Filter out cleanup/unreachable blocks from Reverse Postorder
        let cleanup = Self::cleanup_blocks(&bb_deps.blocks);
        bb_deps.ordering = bb_deps
            .reverse_postorder()
            .into_iter()
            .map(|bb| (bb, &bb_deps.blocks[bb]))
            .filter(|(bbi, bbd): &(usize, &BasicBlock)| {
                if cleanup.contains(bbi) {
                    return false;
                }
                match bbd.terminator.kind {
                    /*
                    TerminatorKind::Call {
                        func: _,
                        args: _,
                        destination: _,
                        target: _,
                        unwind: _,
                    } => {
                        //if let UnwindAction::Unreachable = unwind {
                        //    //debug!("UNREACHABLE: {:?}", bbi);
                        //    return false;
                        //}
                        //if let UnwindAction::Continue = unwind {
                        //    debug!("CONTINUE: {:?}", bbi);
                        //    return false;
                        //}
                        //if let UnwindAction::Cleanup(_) = unwind {
                        //    debug!("CLEANUP: {:?}", bbi);
                        //    return false;
                        //}
                    }
                    */
                    TerminatorKind::Unreachable => return false,
                    TerminatorKind::Resume => return false,
                    TerminatorKind::Abort => return false,
                    TerminatorKind::Return => {
                        // add the return bb last so retval doesn't get overriden

                        if ret_set {
                            panic!("return block already visited");
                        }
                        //debug!("RETURN: {:?}", bbi);
                        ret_bb = bbi.clone();
                        ret_found = true;
                        ret_set = true;
                        return false;
                    }
                    _ => {}
                }
                true
            })
            .map(|(bb, _)| bb)
            .collect();

        if !ret_found {
            //debug!("no return block?");
        } else {
            bb_deps.has_ret = true;
        }

        // add return bb last (only if a real one was found — for a
        // diverging function (no Return terminator anywhere, e.g. an
        // embedded kernel's `main` that ends in an infinite scheduler
        // loop or a `-> !` call), `ret_bb` is left at its default of 0,
        // and pushing it here would silently re-add block 0 to the
        // ordering a second time, bogus)
        if ret_found {
            bb_deps.ordering.push_back(ret_bb);
        }
        debug!("%%%%%");
        debug!("self.ordering: {:?}", bb_deps.ordering);
        debug!("%%%%%");

        bb_deps
    }

    fn cleanup_blocks(blocks: &[BasicBlock]) -> HashSet<usize> {
        // Seed with every block directly reachable via an unwind edge,
        // from whatever terminator kind happens to carry one.
        let mut seeds = Vec::new();
        for bbd in blocks {
            let unwind = match &bbd.terminator.kind {
                TerminatorKind::Call { unwind, .. } => Some(unwind),
                TerminatorKind::Drop { unwind, .. } => Some(unwind),
                TerminatorKind::Assert { unwind, .. } => Some(unwind),
                // add InlineAsm { unwind, .. } etc. if your rustc_public version has it
                _ => None,
            };
            if let Some(UnwindAction::Cleanup(target)) = unwind {
                seeds.push(*target);
            }
        }

        // Transitively close over successors — everything reachable from a
        // cleanup seed is itself cleanup, regardless of what terminator it has.
        let mut visited: HashSet<usize> = HashSet::default();
        let mut stack = seeds;
        while let Some(bb) = stack.pop() {
            if visited.insert(bb) {
                stack.extend(blocks[bb].terminator.successors());
            }
        }
        visited
    }

    fn reverse_postorder(&self) -> Vec<usize> {
        let mut rpo: Vec<_> = Postorder::new(&self.blocks, START_BLOCK).collect();
        //debug!("po: {:?}", rpo);
        rpo.reverse();
        //debug!("rpo: {:?}", rpo);
        rpo
    }

    /// Prunes the edge from switch block `cur` to `target` (that arm can't
    /// be taken in this context), then drops from the ordering every block
    /// no longer reachable from the entry over the edges left.
    ///
    /// By reachability, not by predecessor counts. Removing `target` and
    /// then each block whose predecessor list had emptied kept a loop behind
    /// a pruned edge alive - its back edge stays a predecessor (ByteStr's
    /// Display::fmt walked a loop whose only entry was pruned, reading
    /// locals no visited block assigned) - and removed `target` even when
    /// another, live block also jumps to it.
    pub fn prune(&mut self, cur: usize, target: usize) {
        debug!("PRUNING edge bb{cur} -> bb{target}");
        self.dead_edges.insert((cur, target));
        let mut reachable: HashSet<usize> = HashSet::default();
        let mut stack = vec![START_BLOCK];
        while let Some(bb) = stack.pop() {
            if !reachable.insert(bb) {
                continue;
            }
            for succ in self.blocks[bb].terminator.successors() {
                if !self.dead_edges.contains(&(bb, succ)) {
                    stack.push(succ);
                }
            }
        }
        self.ordering.retain(|bb| reachable.contains(bb));
    }

    /// Whether `bb` is reachable from the entry over the edges not pruned.
    pub fn reachable(&self, bb: usize) -> bool {
        let mut seen: HashSet<usize> = HashSet::default();
        let mut stack = vec![START_BLOCK];
        while let Some(b) = stack.pop() {
            if b == bb {
                return true;
            }
            if seen.insert(b) {
                for succ in self.blocks[b].terminator.successors() {
                    if !self.dead_edges.contains(&(b, succ)) {
                        stack.push(succ);
                    }
                }
            }
        }
        false
    }

    /// Whether `bb` lies on a cycle of the CFG (can reach itself), ignoring
    /// pruned edges.
    pub fn in_cycle(&self, bb: usize) -> bool {
        let mut seen: HashSet<usize> = HashSet::default();
        let mut stack: Vec<usize> = self.blocks[bb].terminator.successors();
        while let Some(b) = stack.pop() {
            if b == bb {
                return true;
            }
            if seen.insert(b) {
                stack.extend(self.blocks[b].terminator.successors());
            }
        }
        false
    }

    pub fn mark_visited(&mut self, bb: usize, cur_scope: &VOID) {
        debug!("DONE VISITING BB{:?} of {:?}\n\n", bb, cur_scope);
        self.visited.push(bb);
    }

    // populate self.preds with the immediate successors of bb
    // to eventually construct a CFG that respects said dependencies
    pub fn get_deps(&mut self, bb: usize, bb_data: &BasicBlock) {
        // what bbs is this bb a predecessor for?
        let successors: Vec<usize> = bb_data.terminator.successors().into_iter().collect();
        for successor in successors.iter() {
            //debug!("successor: {:?}", successor);
            match self.preds.get_mut(successor) {
                Some(preds_vec) => {
                    if preds_vec.contains(&bb) {
                        //debug!("skip adding (preds_vec already contains bb)");
                    } else {
                        preds_vec.push(bb);
                    }
                }
                None => {
                    self.preds.insert(*successor, vec![bb]);
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn update_order(&mut self) -> Vec<usize> {
        let mut ordering = Vec::new();

        for (bb_key, preds_vec) in self.preds.iter() {
            //debug!("bb_key: {:?}", bb_key);
            //debug!("preds: {:?}", preds_vec);

            for pred in preds_vec.iter() {
                if !ordering.contains(pred) {
                    ordering.push(*pred);
                    //debug!("ordering: {:?}", ordering);
                } else {
                    //debug!("ordering CONTAINS pred: {:?}", pred);
                }
            }
            if !ordering.contains(bb_key) {
                ordering.push(*bb_key);
                //debug!("ordering: {:?}", ordering);
            } else {
                //debug!("ordering CONTAINS key: {:?}", bb_key);
            }
        }

        ordering
    }
}
