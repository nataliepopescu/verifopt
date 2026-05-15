use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_index::bit_set::DenseBitSet;
use rustc_public::mir::{BasicBlock, Body, Successors, TerminatorKind};

use crate::constraints::ScopeId;

use log::debug;

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
    pub blocks: Vec<BasicBlock>,
    pub preds: HashMap<usize, Vec<usize>>,
    pub ordering: Vec<usize>,
    pub visited: Vec<usize>,
}

impl BBDeps {
    pub fn new<'tcx>(body: &Body) -> Self {
        let mut bb_deps = BBDeps {
            blocks: body.blocks.clone(),
            preds: HashMap::default(),
            ordering: Vec::new(),
            visited: Vec::new(),
        };

        debug!("%%%%%");
        debug!("total bbs: {:?}", body.blocks.len());

        // Get Successor edges
        for (bb, bb_data) in body.blocks.iter().enumerate() {
            bb_deps.get_deps(bb, bb_data);
        }

        debug!("self.pred: {:?}", bb_deps.preds);

        let mut ret_bb: usize = 0;
        let mut ret_set = false;
        let mut ret_found = false;

        // Filter out cleanup/unreachable blocks from Reverse Postorder
        bb_deps.ordering = bb_deps
            .reverse_postorder()
            .into_iter()
            .map(|bb| (bb, &bb_deps.blocks[bb]))
            //.filter(|(_, bbd): (usize, BasicBlock)| !bbd.is_cleanup)
            .filter(|(bbi, bbd): &(usize, &BasicBlock)| {
                match bbd.terminator.kind {
                    TerminatorKind::Unreachable => return false,
                    TerminatorKind::Resume => return false,
                    TerminatorKind::Abort => return false,
                    TerminatorKind::Return => {
                        // add the return bb last so retval doesn't get overriden

                        if ret_set {
                            panic!("return block already visited");
                        }
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
            panic!("no return block?");
        }

        // add return bb last
        bb_deps.ordering.push(ret_bb);
        debug!("self.ordering: {:?}", bb_deps.ordering);
        debug!("%%%%%");

        bb_deps
    }

    fn reverse_postorder(&self) -> Vec<usize> {
        let mut rpo: Vec<_> = Postorder::new(&self.blocks, START_BLOCK).collect();
        debug!("po: {:?}", rpo);
        rpo.reverse();
        debug!("rpo: {:?}", rpo);
        rpo
    }

    pub fn prune(&mut self, _cur: usize, bb_root: usize) {
        debug!("PRUNING from root basicblock: {:?}", bb_root);
        debug!("self.preds: {:?}", self.preds);
        debug!("self.ordering: {:?}", self.ordering);

        let mut to_remove = Vec::new();
        let mut worklist = Vec::new();
        to_remove.push(bb_root);
        worklist.push(bb_root);

        // update predecessors
        while !worklist.is_empty() {
            let bb_to_prune = worklist.pop().unwrap();

            for (key, val) in self.preds.iter_mut() {
                if val.contains(&bb_to_prune) {
                    val.retain(|&x| x != bb_to_prune);
                    if val.is_empty() {
                        to_remove.push(*key);
                        worklist.push(*key);
                    }
                }
            }
        }

        // update ordering
        self.ordering.retain(|x| !to_remove.contains(x));
        debug!("self.preds: {:?}", self.preds);
        debug!("to_remove: {:?}", to_remove);
        debug!("self.ordering: {:?}", self.ordering);
    }

    pub fn mark_visited(&mut self, bb: usize, cur_scope: ScopeId) {
        debug!("DONE VISITING {:?} of {:?}", bb, cur_scope);
        self.visited.push(bb);
    }

    // populate self.preds with the immediate successors of bb
    // to eventually construct a CFG that respects said dependencies
    pub fn get_deps(&mut self, bb: usize, bb_data: &BasicBlock) {
        // what bbs is this bb a predecessor for?
        let successors: Vec<usize> = bb_data.terminator.successors().into_iter().collect();
        for successor in successors.iter() {
            debug!("successor: {:?}", successor);
            match self.preds.get_mut(successor) {
                Some(preds_vec) => {
                    if preds_vec.contains(&bb) {
                        debug!("skip adding (preds_vec already contains bb)");
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
            debug!("bb_key: {:?}", bb_key);
            debug!("preds: {:?}", preds_vec);

            for pred in preds_vec.iter() {
                if !ordering.contains(pred) {
                    ordering.push(*pred);
                    debug!("ordering: {:?}", ordering);
                } else {
                    debug!("ordering CONTAINS pred: {:?}", pred);
                }
            }
            if !ordering.contains(bb_key) {
                ordering.push(*bb_key);
                debug!("ordering: {:?}", ordering);
            } else {
                debug!("ordering CONTAINS key: {:?}", bb_key);
            }
        }

        ordering
    }
}
