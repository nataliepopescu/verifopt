use rustc_hir::def_id::DefId;
use rustc_middle::mir::*;
use rustc_middle::ty::{TyCtxt, TyKind};
use rustc_data_structures::fx::{FxHashSet as HashSet};
use rustc_index::IndexSlice;
use rustc_span::source_map::Spanned;

use crate::func_collect::FuncMap;
use crate::constraints::{ConstraintMap, MapKey, VarType};
use crate::core::{FuncVal, VerifoptRval};

pub struct InterpPass<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub entry_func: DefId,
    pub func_map: &'a FuncMap<'tcx>,
}

impl<'a, 'tcx> InterpPass<'a, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        entry_func: DefId,
        func_map: &'a FuncMap<'tcx>,
    ) -> InterpPass<'a, 'tcx> {
        Self { tcx, entry_func, func_map }
    }

    pub fn run(
        &mut self, 
        cmap: &mut ConstraintMap<'tcx>, 
        body: &Body<'tcx>,
    ) {
        self.visit_body(cmap, None, body);
    }

    fn visit_body(
        &mut self, 
        cmap: &mut ConstraintMap<'tcx>, 
        enclosing_scope: Option<DefId>,
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
            self.visit_basic_block_data(cmap, body.local_decls.as_slice(), enclosing_scope, bb, data);
        }
    }

    fn visit_basic_block_data(
        &mut self,
        cmap: &mut ConstraintMap<'tcx>, 
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        enclosing_scope: Option<DefId>,
        _block: BasicBlock,
        data: &BasicBlockData<'tcx>,
    ) {
        for (_, stmt) in data.statements.iter().enumerate() {
            self.visit_statement(cmap, body_locals, enclosing_scope, stmt);
        }

        if let Some(term) = &data.terminator {
            self.visit_terminator(cmap, body_locals, enclosing_scope, &term);
        }
    }

    fn visit_statement(
        &mut self,
        cmap: &mut ConstraintMap<'tcx>, 
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        enclosing_scope: Option<DefId>,
        statement: &Statement<'tcx>,
    ) {
        match statement.kind {
            StatementKind::Assign(box (place, ref rvalue)) => {
                println!("assignment!");
                println!("place: {:?}", place);
                println!("rval: {:?}", rvalue);
                println!("enclosing_scope: {:?}", enclosing_scope);

                let mut set = HashSet::default();
                set.insert(VerifoptRval::from_rvalue(body_locals, rvalue));

                // FIXME use enclosing_scope
                cmap.scoped_set(
                    None,
                    MapKey::Place(place),
                    Box::new(VarType::Values(set)),
                );
                println!("~~~CMAP: {:?}", cmap);
            },
            _ => {},
        }
    }

    fn visit_terminator(
        &mut self,
        cmap: &mut ConstraintMap<'tcx>, 
        body_locals: &IndexSlice<Local, LocalDecl<'tcx>>,
        enclosing_scope: Option<DefId>,
        terminator: &Terminator<'tcx>,
    ) {
        match &terminator.kind {
            TerminatorKind::Call { func, args, destination, .. } => {
                println!("\n-----------\n");
                println!("call!");
                println!("func: {:?}", func);
                println!("args: {:?}", args);
                println!("place: {:?}", destination);

                //println!("\nfunc_map: {:#?}\n", self.func_map);

                match func {
                    Operand::Constant(box co) => {
                        match co.const_ {
                            Const::Val(_, ty) => {
                                //println!("ty: {:?}", ty);
                                match ty.kind() {
                                    TyKind::FnDef(def_id, _) => {
                                        match self.func_map.funcs.get(def_id) {
                                            Some(funcval_vec) => {
                                                for funcval in funcval_vec.iter() {
                                                    let mut cmap_clone = cmap.clone();
                                                    self.resolve_args(cmap, enclosing_scope, funcval, args);
                                                }
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

                // FIXME use enclosing_scope
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

    fn resolve_args(
        &self,
        cmap: &mut ConstraintMap<'tcx>,
        enclosing_scope: Option<DefId>,
        funcval: &FuncVal<'tcx>,
        args: &Box<[Spanned<Operand<'tcx>>]>,
    ) {
        println!("RESOLVING ARGS");
        println!("BEFORE CMAP: {:#?}", cmap);

        let mut func_cmap = ConstraintMap::new();

        let arg_vec: Vec<Operand<'tcx>> = args.into_iter().map(|x| x.clone().node).collect();

        // add arg values into func_cmap
        for ((param_name, param_type), arg) in std::iter::zip(funcval.params.clone(), arg_vec) {
            //println!("param_name: {:?}", param_name);
            //println!("param_type: {:?}", param_type);
            //println!("arg: {:?}", arg);
            //println!("enclosing_scope: {:?}", enclosing_scope);

            // FIXME how do outer-scope arg names/places interact w current scope (should
            // disambiguate when bring into scope)
            match arg {
                // TODO cmap.scoped_get + add result to func_cmap
                Operand::Copy(place)
                | Operand::Move(place) => {
                    match cmap.scoped_get(enclosing_scope, &MapKey::Place(place), false) {
                        Some(vartype) => {
                            println!("vartype: {:?}", vartype);
                            match vartype {
                                VarType::Values(constraints) => {
                                    func_cmap.cmap.insert(
                                        MapKey::Place(param_name),
                                        Box::new(VarType::Values(
                                            // TODO add type?
                                            constraints.clone(),
                                        )),
                                    );
                                }
                                _ => {},
                            }
                        }
                        // TODO func name?
                        None => {},
                    }
                }
                _ => {},
            }
        }

        // add func_cmap to outer cmap

        // FIXME assuming "name" (funcname I believe) does not exist in cmap
        cmap.cmap.insert(
            MapKey::ScopeId(funcval.def_id),
            Box::new(VarType::Scope(
                enclosing_scope,
                vec![(
                    // FIXME forgot what this field is for
                    Box::new("func"),
                    func_cmap,
                )],
            )),
        );

        println!("AFTER CMAP: {:#?}", cmap);
    }
}

