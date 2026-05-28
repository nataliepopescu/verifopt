// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

//! The main routine of `rupta`.
//!
//! Implemented as a stub that invokes the rust compiler with a call back to execute
//! pointer analysis during rust compilation.

#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_public;
extern crate rustc_session;

use rustc_public::run;

use log::*;
use std::env;
use std::io::Write;

use monomorph::start_verifopt;
use monomorph::util;
use monomorph::util::options::AnalysisOptions;

fn main() {
    let early_dcx =
        rustc_session::EarlyDiagCtxt::new(rustc_session::config::ErrorOutputType::default());

    // Initialize loggers.
    if env::var("RUSTC_LOG").is_ok() {
        rustc_driver::init_rustc_env_logger(&early_dcx);
    }
    if env::var("VERIFOPT_LOG").is_ok() {
        env_logger::Builder::new()
            .format(|buf, record| {
                //writeln!(buf, "{}: {}", record.level(), record.args())
                writeln!(buf, "{}", record.args())
            })
            .parse_env(
                env_logger::Env::new()
                    .filter("VERIFOPT_LOG")
                    .write_style("VERIFOPT_LOG_STYLE"),
            )
            .init();
    }

    // Get any options specified via the VERIFOPT_FLAGS environment variable
    let mut options = AnalysisOptions::default();
    let pta_flags = env::var("VERIFOPT_FLAGS").unwrap_or_default();
    let pta_args: Vec<String> = serde_json::from_str(&pta_flags).unwrap_or_default();
    let rustc_args = options.parse_from_args(&pta_args[..], true);

    // Let arguments supplied on the command line override the environment variable.
    let mut args = env::args_os()
        .enumerate()
        .map(|(i, arg)| {
            arg.into_string().unwrap_or_else(|arg| {
                early_dcx.early_fatal(format!("Argument {i} is not valid Unicode: {arg:?}"))
            })
        })
        .collect::<Vec<_>>();

    // Setting RUSTC_WRAPPER causes Cargo to pass 'rustc' as the first argument.
    // We're invoking the compiler programmatically, so we remove it if present.
    if args.len() > 1 && std::path::Path::new(&args[1]).file_stem() == Some("rustc".as_ref()) {
        args.remove(1);
    }

    let mut rustc_command_line_arguments = options.parse_from_args(&args[1..], false);
    info!("VerifOpt Options: {:?}", options);

    let result = rustc_driver::catch_fatal_errors(move || {
        // Add back the binary name
        rustc_command_line_arguments.insert(0, args[0].clone());

        // Add rustc arguments supplied via the MIRAI_FLAGS environment variable
        rustc_command_line_arguments.extend(rustc_args);

        let sysroot: String = "--sysroot".into();
        if !rustc_command_line_arguments
            .iter()
            .any(|arg| arg.starts_with(&sysroot))
        {
            // Tell compiler where to find the std library and so on.
            // The compiler relies on the standard rustc driver to tell it, so we have to do likewise.
            rustc_command_line_arguments.push(sysroot);
            rustc_command_line_arguments.push(util::find_sysroot());
        }

        let always_encode_mir: String = "always-encode-mir".into();
        if !rustc_command_line_arguments
            .iter()
            .any(|arg| arg.ends_with(&always_encode_mir))
        {
            // Tell compiler to emit MIR into crate for every function with a body.
            rustc_command_line_arguments.push("-Z".into());
            rustc_command_line_arguments.push(always_encode_mir);
        }
        debug!(
            "rustc command line arguments: {:?}",
            rustc_command_line_arguments
        );

        //let mut callbacks = VerifOptCallbacks::new(options);
        //let compiler = rustc_driver::RunCompiler::new(&rustc_command_line_arguments, &mut callbacks);
        //compiler.run()
        run!(&rustc_command_line_arguments, || start_verifopt(options))
    });

    let exit_code = match result {
        Ok(_) => rustc_driver::EXIT_SUCCESS,
        Err(_) => rustc_driver::EXIT_FAILURE,
    };

    std::process::exit(exit_code);
}
