//use rustc_hir::def_id::DefId;
use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_middle::mir::traversal;
use rustc_middle::mir::{BasicBlock, BasicBlockData, Body, TerminatorKind};
use rustc_hir::def_id::DefId;

#[derive(Clone, Debug, PartialEq)]
pub struct BBDeps {
    //pub body: &'tcx Body<'tcx>,
    pub preds: HashMap<BasicBlock, Vec<BasicBlock>>,
    pub ordering: Vec<BasicBlock>,
    pub visited: Vec<BasicBlock>,
    pub debug: bool,
}

impl BBDeps {
    pub fn new<'tcx>(body: &'tcx Body<'tcx>, debug: bool) -> Self {
        let mut bb_deps = BBDeps {
            preds: HashMap::default(),
            ordering: Vec::new(),
            visited: Vec::new(),
            debug,
        };

        if debug {
            println!("\n%%%%%\n");
            println!("total bbs: {:?}", body.basic_blocks.len());
        }
        for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
            if debug {
                println!("getting deps for {:?}", bb);
            }
            bb_deps.get_deps(&bb, bb_data);
            if debug {
                println!("- bb_deps.preds: {:?}", bb_deps.preds);
            }
        }

        if debug {
            println!("self.pred: {:?}", bb_deps.preds);
        }

        let mut ret_bb = BasicBlock::from_u32(0);
        let mut ret_found = false;
        bb_deps.ordering = traversal::reverse_postorder(body)
            .filter(|(_, bbd)| !bbd.is_cleanup)
            .filter(|(bbi, bbd)| {
                if let Some(term) = &bbd.terminator {
                    match term.kind {
                        TerminatorKind::Unreachable => return false,
                        TerminatorKind::Return => {
                            // add the return bb last so retval doesn't get overriden
                            ret_bb = bbi.clone();
                            ret_found = true;
                            return false;
                        }
                        _ => {}
                    }
                }
                true
            })
            .map(|(bb, _)| bb)
            .collect();

        if !ret_found {
            panic!("no return block?");
        }
        bb_deps.ordering.push(ret_bb);
        if debug {
            println!("\nself.ordering: {:?}", bb_deps.ordering);
            println!("\n%%%%%");
        }

        bb_deps
    }

    pub fn prune(&mut self, _cur: &BasicBlock, bb_root: BasicBlock) {
        if self.debug {
            println!("PRUNING from root basicblock: {:?}", bb_root);
            println!("self.preds: {:?}", self.preds);
            println!("self.ordering: {:?}", self.ordering);
        }

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
        if self.debug {
            println!("self.preds: {:?}", self.preds);
            println!("to_remove: {:?}", to_remove);
            println!("self.ordering: {:?}", self.ordering);
        }
    }

    pub fn mark_visited(&mut self, bb: &BasicBlock, cur_scope: DefId) {
        if self.debug {
            println!("\nDONE VISITING {:?} of {:?}", bb, cur_scope);
        }
        self.visited.push(*bb);
    }

    // populate self.preds with the immediate successors of bb
    // to eventually construct a CFG that respects said dependencies
    pub fn get_deps(&mut self, bb: &BasicBlock, bb_data: &BasicBlockData) {
        match &bb_data.terminator {
            Some(term) => {
                // what bbs is this bb a predecessor for?
                let successors: Vec<BasicBlock> = term.successors().collect();
                for successor in successors.iter() {
                    if self.debug {
                        println!("successor: {:?}", successor);
                    }
                    match self.preds.get_mut(successor) {
                        Some(preds_vec) => {
                            if preds_vec.contains(bb) {
                                if self.debug {
                                    println!("skip adding (preds_vec already contains bb)");
                                }
                            } else {
                                preds_vec.push(*bb);
                            }
                        }
                        None => {
                            self.preds.insert(*successor, vec![*bb]);
                        }
                    }
                }
            }
            None => {
                panic!("no term");
            }
        }
    }

    #[allow(dead_code)]
    pub fn update_order(&mut self) -> Vec<BasicBlock> {
        let mut ordering = Vec::new();

        for (bb_key, preds_vec) in self.preds.iter() {
            if self.debug {
                println!("bb_key: {:?}", bb_key);
                println!("preds: {:?}", preds_vec);
            }
            for pred in preds_vec.iter() {
                if !ordering.contains(pred) {
                    ordering.push(*pred);
                    if self.debug {
                        println!("ordering: {:?}", ordering);
                    }
                } else {
                    if self.debug {
                        println!("ordering CONTAINS pred: {:?}", pred);
                    }
                }
            }
            if !ordering.contains(bb_key) {
                ordering.push(*bb_key);
                if self.debug {
                    println!("ordering: {:?}", ordering);
                }
            } else {
                if self.debug {
                    println!("ordering CONTAINS key: {:?}", bb_key);
                }
            }
        }

        ordering
    }
}
