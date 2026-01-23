//use rustc_hir::def_id::DefId;
use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_middle::mir::traversal;
use rustc_middle::mir::{BasicBlock, BasicBlockData, Body};

pub struct BBDeps {
    //pub body: &'tcx Body<'tcx>,
    pub preds: HashMap<BasicBlock, Vec<BasicBlock>>,
    pub ordering: Vec<BasicBlock>,
    pub visited: Vec<BasicBlock>,
}

impl BBDeps {
    pub fn new<'tcx>(body: &'tcx Body<'tcx>) -> Self {
        let mut bb_deps = BBDeps {
            //body,
            preds: HashMap::default(),
            ordering: Vec::new(),
            visited: Vec::new(),
        };

        for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
            bb_deps.get_deps(&bb, bb_data);
        }
        println!("self.pred: {:?}", bb_deps.preds);

        // FIXME if there is a return before error path, the return will execute first...
        bb_deps.ordering = traversal::reverse_postorder(body).map(|(x, _)| x).collect();
        println!("self.ordering: {:?}", bb_deps.ordering);
        println!("%%%%%");

        bb_deps
    }

    pub fn prune(&mut self, _cur: &BasicBlock, bb_root: BasicBlock) {
        println!("PRUNING from root bb {:?}", bb_root);
        println!("self.preds: {:?}", self.preds);
        println!("self.ordering: {:?}", self.ordering);

        let mut to_remove = Vec::new();
        let mut worklist = Vec::new();
        to_remove.push(bb_root);
        worklist.push(bb_root);

        // update predecessors
        while !worklist.is_empty() {
            //println!("~~~worklist: {:?}", worklist);
            let bb_to_prune = worklist.pop().unwrap();

            for (key, val) in self.preds.iter_mut() {
                //println!("__PRE val: {:?}", val);
                if val.contains(&bb_to_prune) {
                    val.retain(|&x| x != bb_to_prune);
                    if val.is_empty() {
                        to_remove.push(*key);
                        worklist.push(*key);
                    }
                }
                //println!("_POST val: {:?}", val);
            }
        }

        // update ordering
        self.ordering.retain(|x| !to_remove.contains(x));
        println!("self.preds: {:?}", self.preds);
        println!("to_remove: {:?}", to_remove);
        println!("self.ordering: {:?}", self.ordering);
    }

    pub fn mark_visited(&mut self, bb: &BasicBlock) {
        println!("DONE VISITING {:?}", bb);
        self.visited.push(*bb);
    }

    pub fn get_deps(&mut self, bb: &BasicBlock, bb_data: &BasicBlockData) {
        match &bb_data.terminator {
            Some(term) => {
                let successors: Vec<BasicBlock> = term.successors().collect();
                for successor in successors.iter() {
                    match self.preds.get_mut(successor) {
                        Some(preds_vec) => {
                            if preds_vec.contains(bb) {
                                panic!("predecessors vector already contain this bb (key)");
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
            println!("bb_key: {:?}", bb_key);
            println!("preds: {:?}", preds_vec);
            for pred in preds_vec.iter() {
                if !ordering.contains(pred) {
                    ordering.push(*pred);
                    println!("ordering: {:?}", ordering);
                } else {
                    println!("ordering CONTAINS pred: {:?}", pred);
                }
            }
            if !ordering.contains(bb_key) {
                ordering.push(*bb_key);
                println!("ordering: {:?}", ordering);
            } else {
                println!("ordering CONTAINS key: {:?}", bb_key);
            }
        }

        ordering
    }
}
