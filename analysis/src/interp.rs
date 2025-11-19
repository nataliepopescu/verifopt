use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::ty::{TyCtxt, TyKind};
use rustc_data_structures::fx::{FxHashSet as HashSet};

use crate::func_collect::FuncMap;
use crate::constraints::{ConstraintMap, MapKey, VarType};
use crate::core::VerifoptRval;

pub struct InterpPass<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub entry_func: DefId,
    pub func_map: &'a FuncMap,
}

impl<'a, 'tcx> InterpPass<'a, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        entry_func: DefId,
        func_map: &'a FuncMap,
    ) -> InterpPass<'a, 'tcx> {
        Self { tcx, entry_func, func_map }
    }

    pub fn run(
        &mut self, 
        cmap: &mut ConstraintMap<'tcx>, 
        body: &Body<'tcx>,
    ) {
        self.visit_body(cmap, body);
    }

    fn visit_body(
        &mut self, 
        cmap: &mut ConstraintMap<'tcx>, 
        body: &Body<'tcx>,
    ) {
        // FIXME how do loops affect this order?
        // 
        // is it correct that we don't _really_ need to worry about order of 
        // traversal (assuming NO loops) due to SSA?
        //
        // TODO instead of visitor, traverse one-by-one like in SimpleInterp
        // (easier for, e.g., conditionals state merging)
        for (bb, data) in traversal::preorder(body) {
            //println!("bb: {:?}", bb);
            self.visit_basic_block_data(cmap, bb, data);
        }
    }

    fn visit_basic_block_data(
        &mut self,
        cmap: &mut ConstraintMap<'tcx>, 
        _block: BasicBlock,
        data: &BasicBlockData<'tcx>,
    ) {
        for (_, stmt) in data.statements.iter().enumerate() {
            self.visit_statement(cmap, stmt);
        }

        if let Some(term) = &data.terminator {
            self.visit_terminator(cmap, &term);
        }
    }

    fn visit_statement(
        &mut self,
        cmap: &mut ConstraintMap<'tcx>, 
        statement: &Statement<'tcx>,
    ) {
        match statement.kind {
            StatementKind::Assign(box (place, ref rvalue)) => {
                //println!("assignment!");
                //println!("place: {:?}", place);
                //println!("rval: {:?}", rvalue);
                let mut set = HashSet::default();
                set.insert(rvalue.into());
                cmap.scoped_set(
                    None,
                    MapKey::Place(place),
                    Box::new(VarType::Values(set)),
                );
                //println!("~~~CMAP: {:?}", cmap);
            },
            _ => {},
        }
    }

    fn visit_terminator(
        &mut self,
        cmap: &mut ConstraintMap<'tcx>, 
        terminator: &Terminator<'tcx>,
    ) {
        match &terminator.kind {
            TerminatorKind::Call { func, args, destination, .. } => {
                println!("\n-----------\n");
                println!("call!");
                println!("func: {:?}", func);
                println!("args: {:?}", args);
                println!("place: {:?}", destination);

                println!("\nfunc_map: {:#?}\n", self.func_map);

                match func {
                    Operand::Constant(box co) => {
                        match co.const_ {
                            Const::Val(_, ty) => {
                                println!("ty: {:?}", ty);
                                match ty.kind() {
                                    TyKind::FnDef(def_id, _) => {
                                        match self.func_map.funcs.get(def_id) {
                                            Some(funcval_vec) => {
                                                println!("GOT funcval_vec: {:?}", funcval_vec);
                                                // TODO modify visitor -> cmap as ARG
                                                //for funcval in funcval_vec.iter() {
                                                //}
                                            },
                                            None => {
                                                println!("no such function (might be a dynamic call): {:?}", def_id);
                                                // TODO dynamic dispatch
                                                // if first arg == self, use constraints to prune funcvals
                                            },
                                        }
                                    },
                                    _ => {},
                                }
                            },
                            _ => {},
                        }
                    },
                    // TODO also handle indirect invokes (via variable name, _not_ in func_map)
                    _ => {},
                }

                let mut set = HashSet::default();
                // FIXME should be func call result
                set.insert(VerifoptRval::Idk());

                cmap.scoped_set(
                    None,
                    MapKey::Place(*destination),
                    Box::new(VarType::Values(set)),
                );
                //println!("~~~CMAP: {:?}", cmap);
            },
            //TailCall
            _ => {},
        }
    }
}

