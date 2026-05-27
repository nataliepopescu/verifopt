use log::debug;

use rustc_public::mir::Body;
//use rustc_public::mir::mono::Instance;

use crate::constraints::VOID;

pub fn log_scope(scope: &VOID) {
    debug!("CUR SCOPE: {:?}", scope.0.name()); //, scope);
}

pub fn log_call_stack(call_stack: &Vec<VOID>) {
    debug!("CALL STACK\n[");
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

#[derive(Clone, Debug)]
pub enum VerifOptType {
    FlowSensitive,
}

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
