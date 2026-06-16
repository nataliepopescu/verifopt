use crate::rustc_public_bridge::IndexedVal;
use rustc_public::DefId;
use rustc_public::mir::Body;
use rustc_public::ty::AdtDef;

use log::debug;

use crate::constraints::VOID;

#[derive(Clone, Debug)]
pub enum VerifOptType {
    FlowSensitive,
}

pub fn log_scope(scope: &VOID) {
    debug!("CUR SCOPE: {:?}", scope.0.name()); //, scope);
}

pub fn log_call_stack(call_stack: &Vec<VOID>) {
    debug!("CALL STACK ({:?})\n[", call_stack.len());
    for instance in call_stack {
        debug!("\t{:?},", instance.0.name());
    }
    debug!("]");
}

pub fn log_mir(body: &Body) {
    debug!("----START BODY----");
    debug!("arg count: {:?}", body.arg_locals().len());
    debug!("locals count: {:?}", body.locals().len());
    debug!("blocks count: {:?}", &body.blocks.len());
    debug!("{:#?}", body);
    debug!("----END BODY----");

    /*
    let locals = body.locals();
    let blocks = &body.blocks;

    debug!("num LocalDecls: {:?}", locals.len());
    debug!("{{");
    for i in 0..locals.len() {
        debug!("-local{:?}", i);
        debug!("{:?}", locals[i]);
    }
    debug!("}}");

    debug!("num BasicBlocks: {:?}", blocks.len());
    debug!("{{");
    for i in 0..blocks.len() {
        debug!("-bb{:?}", i);
        debug!("{:?}", blocks[i]);
        //for j in 0..blocks[i].statements.len() {
        //    debug!("--stmt{:?}", j);
        //    match panic::catch_unwind(|| {
        //        debug!("{:?}", blocks[i].statements[j]);
        //    }) {
        //        Ok(stmt) => debug!("{:?}", stmt),
        //        _ => debug!("SKIPPING (error)"),
        //    }
        //}
    }
    debug!("}}");
    */
}

//pub fn is_wrapper_type(def: &AdtDef) -> bool {
//    debug!("CHECKING IF DEFID IS A WRAPPER TYPE: {:?}", def);
//
//    // TODO extend to Option, Result, ...
//    def.is_box() || is_option(&def.0)
//}
//
//fn is_option(defid: &DefId) -> bool {
//    let idx = defid.to_index();
//    match idx {
//        28539 => true,
//        _ => false,
//    }
//}

//fn is_box(defid: &DefId) -> bool {
//    // FIXME why multiple defids for box?
//    let idx = defid.to_index();
//    match idx {
//        11 | 12 | 14 | 18965 | 18968 | 18969 | 88530 | 88531 | 88533 | 88536 | 94645 => true,
//        _ => false,
//    }
//}

/*
pub struct AnalysisContext<'tcx> {
    /// The central data structure of the compiler.
    pub tcx: TyCtxt<'tcx>,

    /// The entry function of the analysis.
    pub entry_point: DefId,
}
*/

//pub fn get_entry_point(analysis_optins: AnalysisOptions) -> DefId {
//        info!("Getting entry point");
//        let mut entry_fn_def_id: Option<DefId> = None;
//}
