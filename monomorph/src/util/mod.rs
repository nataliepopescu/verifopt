//use rustc_hir::def_id::DefId;
//use rustc_middle::mir;
//use rustc_middle::ty::{GenericArgsRef, TyCtxt, TyKind};
//use std::io::Write;
//use std::rc::Rc;

pub mod options;

/// Returns the location of the rust system binaries that are associated with this build of rust-verifopt.
/// The location is obtained by looking at the contents of the environmental variables that were
/// set at the time rust-verifopt was compiled. If the rust compiler was installed by rustup, the variables
/// `RUSTUP_HOME` and `RUSTUP_TOOLCHAIN` are used and these are set by the compiler itself.
/// If the rust compiler was compiled and installed in some other way, for example from a source
/// enlistment, then the `RUST_SYSROOT` variable must be set in the environment from which rust-verifopt
/// is compiled.
pub fn find_sysroot() -> String {
    let home = option_env!("RUSTUP_HOME");
    let toolchain = option_env!("RUSTUP_TOOLCHAIN");
    match (home, toolchain) {
        (Some(home), Some(toolchain)) => format!("{home}/toolchains/{toolchain}"),
        _ => match option_env!("RUST_SYSROOT") {
            None => {
                panic!(
                    "Could not find sysroot. Specify the RUST_SYSROOT environment variable, \
                    or use rustup to set the compiler to use for Mirai",
                )
            }
            Some(sys_root) => sys_root.to_owned(),
        },
    }
}

///// Returns true if the function identified by `def_id` is defined as part of a trait.
//#[inline]
//pub fn is_trait_method(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
//    if tcx.trait_of_item(def_id).is_some() {
//        true
//    } else {
//        false
//    }
//}
//
///// Returns true if the call to (`callee_def_id`, `callee_substs`) is a dynamic call.
//#[inline]
//pub fn is_dynamic_call<'tcx>(
//    tcx: TyCtxt<'tcx>,
//    callee_def_id: DefId,
//    callee_substs: GenericArgsRef<'tcx>,
//) -> bool {
//    if !is_trait_method(tcx, callee_def_id) {
//        return false;
//    }
//    let arg0_ty = callee_substs
//        .types()
//        .next()
//        .expect("Expect `Self` substition in trait method invocation");
//    if matches!(arg0_ty.kind(), TyKind::Dynamic(..)) {
//        true
//    } else {
//        false
//    }
//}

