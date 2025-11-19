use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::mir::visit::Visitor;
use rustc_middle::ty::TyCtxt;
use rustc_data_structures::fx::{FxHashSet as HashSet};

use crate::func_collect::FuncMap;
use crate::constraints::{ConstraintMap, MapKey, VarType};
use crate::core::VerifoptRval;

pub struct InterpPass<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub entry_func: DefId,
    pub func_map: &'a FuncMap,
    pub cmap: &'a mut ConstraintMap<'tcx>,
}

impl<'a, 'tcx> InterpPass<'a, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        entry_func: DefId,
        func_map: &'a FuncMap,
        cmap: &'a mut ConstraintMap<'tcx>,
    ) -> InterpPass<'a, 'tcx> {
        Self { tcx, entry_func, func_map, cmap }
    }

    pub fn run(&mut self, body: &Body<'tcx>) {
        self.visit_body(body);
    }
}

impl<'a, 'tcx> Visitor<'tcx> for InterpPass<'a, 'tcx> {
    fn visit_body(&mut self, body: &Body<'tcx>) {
        // FIXME how do loops affect this order?
        // 
        // is it correct that we don't _really_ need to worry about order of 
        // traversal (assuming NO loops) due to SSA?
        //
        // TODO instead of visitor, traverse one-by-one like in SimpleInterp
        // (easier for, e.g., conditionals state merging)
        for (bb, data) in traversal::preorder(body) {
            //println!("bb: {:?}", bb);
            self.visit_basic_block_data(bb, data);
        }
    }

    fn visit_basic_block_data(
        &mut self,
        block: BasicBlock,
        data: &BasicBlockData<'tcx>,
    ) {
        let mut last_statement_index = None;
        for (statement_index, stmt) in data.statements.iter().enumerate() {
            last_statement_index = Some(statement_index);
            let loc = Location { block, statement_index };
            self.visit_statement(stmt, loc);
        }

        if let Some(term) = &data.terminator {
            let loc;
            if let Some(mut statement_index) = last_statement_index {
                statement_index += 1;
                loc = Location { block, statement_index };
            } else {
                loc = Location { block, statement_index: 0 };
            }
            self.visit_terminator(&term, loc);
        }
    }

    fn visit_terminator(
        &mut self,
        terminator: &Terminator<'tcx>,
        _location: Location
    ) {
        match &terminator.kind {
            TerminatorKind::Call { func, args, destination, .. } => {
                //println!("call!");
                //println!("func: {:?}", func);
                //println!("args: {:?}", args);
                //println!("place: {:?}", destination);
                let mut set = HashSet::default();
                // FIXME should be func call result
                set.insert(VerifoptRval::Idk());
                self.cmap.scoped_set(
                    None,
                    MapKey::Place(*destination),
                    Box::new(VarType::Values(set)),
                );
                //println!("~~~CMAP: {:?}", self.cmap);
            },
            //TailCall
            _ => {},
        }
    }

    fn visit_statement(
        &mut self,
        statement: &Statement<'tcx>,
        _location: Location
    ) {
        match statement.kind {
            StatementKind::Assign(box (place, ref rvalue)) => {
                //println!("assignment!");
                //println!("place: {:?}", place);
                //println!("rval: {:?}", rvalue);
                let mut set = HashSet::default();
                set.insert(rvalue.into());
                self.cmap.scoped_set(
                    None,
                    MapKey::Place(place),
                    Box::new(VarType::Values(set)),
                );
                //println!("~~~CMAP: {:?}", self.cmap);
            },
            _ => {},
        }
    }
}

