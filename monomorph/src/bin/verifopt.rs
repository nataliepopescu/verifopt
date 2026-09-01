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

use log::*;
use std::env;
use std::io::Write;

use monomorph::rewrite::{FsaCallbacks, RewriteCallbacks};
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
                //writeln!(buf, "{}: {}", record.file().unwrap(), record.args())
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

    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        eprintln!("PANIC HOOK: {}", info);
        default_hook(info);
    }));

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
        //
        // In --no-rewrite (control) mode, nothing downstream will ever
        // read this invocation's output - RewriteCallbacks's
        // optimized_mir override bypasses the Store entirely regardless
        // of what's in it (see SKIP_REWRITE). Skipping the whole
        // invocation here, not just the interpretation inside
        // FsaCallbacks::after_analysis, avoids paying for a full
        // parse/type-check/borrow-check pass that would otherwise
        // happen twice for no reason. This is the only place
        // FsaCallbacks is ever constructed, so this guard alone is
        // sufficient - after_analysis doesn't duplicate the check.
        //
        // --rewrite-pass is skipped for the same reason: this crate is
        // being rebuilt specifically to apply edits a prior discovery
        // pass already found and persisted (see rewrite.rs's
        // dep_rewrite_store_path), not to re-discover anything - a
        // dependency crate has no entry point of its own to analyze
        // from anyway, so re-running FsaCallbacks here would just be
        // wasted work repeating a failure, not a genuine second
        // analysis.
        if !options.no_rewrite && !options.rewrite_pass {
            let mut callbacks = FsaCallbacks {
                options: options.clone(),
            };
            match rustc_driver::catch_fatal_errors(|| {
                rustc_driver::run_compiler(&rustc_command_line_arguments, &mut callbacks);
            }) {
                Ok(()) => {}
                Err(_) => {
                    debug!(
                        "FsaCallbacks phase returned FatalError - continuing anyway to reach the rewrite/codegen stage"
                    );
                }
            }
        }

        let mut callbacks = RewriteCallbacks { options };
        match rustc_driver::catch_fatal_errors(|| {
            rustc_driver::run_compiler(&rustc_command_line_arguments, &mut callbacks);
        }) {
            Ok(()) => {}
            Err(_) => {
                debug!(
                    "RewriteCallbacks phase returned FatalError - continuing anyway to reach the codegen stage"
                );
            }
        }
    });

    let exit_code = match result {
        Ok(_) => rustc_driver::EXIT_SUCCESS,
        Err(_) => rustc_driver::EXIT_FAILURE,
    };

    std::process::exit(exit_code);
}
