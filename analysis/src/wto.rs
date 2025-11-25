use rustc_hir::def_id::DefId;
use rustc_middle::mir::{BasicBlock, BasicBlockData, Body};
use rustc_middle::mir::traversal;
use rustc_data_structures::fx::{FxHashMap as HashMap};

pub struct BBDeps<'tcx> {
    pub body: &'tcx Body<'tcx>,
    pub preds: HashMap<BasicBlock, Vec<BasicBlock>>,
    pub ordering: Vec<BasicBlock>,
}

impl<'tcx> BBDeps<'tcx> {
    pub fn new(body: &'tcx Body<'tcx>) -> Self {
        let mut bb_deps = BBDeps {
            body,
            preds: HashMap::default(),
            ordering: Vec::new(),
        };

        for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
            bb_deps.visit(&bb, bb_data);
        }

        println!("self.pred: {:?}", bb_deps.preds);

        println!("pre-order traversal");
        for (bb, _) in traversal::reverse_postorder(body) {
            println!("bb: {:?}", bb);
        }

        bb_deps.ordering = bb_deps.order();
        
        println!("self.ordering: {:?}", bb_deps.ordering);

        println!("%%%%%");

        bb_deps
    }

    pub fn visit(&mut self, bb: &BasicBlock, bb_data: &BasicBlockData) {
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
                        },
                        None => {
                            self.preds.insert(*successor, vec![*bb]);
                        },
                    }
                }
            },
            None => {
                panic!("no term");
            }
        }
    }

    pub fn order(&mut self) -> Vec<BasicBlock> {
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
