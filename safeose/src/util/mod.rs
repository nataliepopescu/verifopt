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
