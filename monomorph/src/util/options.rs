//! Analysis options.

use clap::error::ErrorKind;
use clap::{Arg, Command};
use itertools::Itertools;

use crate::common::VerifOptType;

const VERIFOPT_USAGE: &str = r#"verifopt [OPTIONS] INPUT -- [RUSTC OPTIONS]"#;

/// The version information from Cargo.toml.
fn version() -> &'static str {
    let version_info = rustc_tools_util::get_version_info!();
    let version = format!(
        "v{}.{}.{}",
        version_info.major, version_info.minor, version_info.patch
    );
    Box::leak(version.into_boxed_str())
}

/// Creates the clap::Command metadata for argument parsing.
fn make_options_parser() -> Command {
    // We could put this into lazy_static! with a Mutex around, but we really do not expect
    // to construct this more then once per regular program run.
    let parser = Command::new("verifopt")
        .no_binary_name(true)
        .override_usage(VERIFOPT_USAGE)
        .version(version())
        .arg(
            Arg::new("entry-func-name")
                .long("entry-func")
                .value_name("func-name")
                .help("The name of entry function from which the flow analysis begins."),
        )
        .arg(
            Arg::new("entry-func-id")
                .long("entry-id")
                .value_name("id")
                .value_parser(clap::value_parser!(u32))
                .help("The def_id of entry function from which the flow analysis begins."),
        )
        .arg(
            Arg::new("verifopt-type")
                .long("verifopt-type")
                .value_name("analysis-type")
                .value_parser(["flow-sensitive", "fsa"])
                .default_value("flow-sensitive")
                .help("The type of analysis.")
                .long_help("Flow-sensitive analyses is supported now."),
        )
        .arg(Arg::new("INPUT")
            .num_args(0..)
            .help("The input file to be analyzed."),
        );

    //.arg(Arg::new("context-depth")
    //    .long("context-depth")
    //    .takes_value(true)
    //    .value_parser(clap::value_parser!(u32))
    //    .default_value("1")
    //    .help("The context depth limit for a context-sensitive pointer analysis."))
    //.arg(Arg::new("dump-stats")
    //    .long("dump-stats")
    //    .takes_value(false)
    //    .help("Dump the statistics of the analysis results."))
    //.arg(Arg::new("call-graph-output")
    //    .long("dump-call-graph")
    //    .takes_value(true)
    //    .help("Dump the call graph in DOT format to the output file."))
    //.arg(Arg::new("pts-output")
    //    .long("dump-pts")
    //    .takes_value(true)
    //    .help("Dump points-to results to the output file."))
    //.arg(Arg::new("mir-output")
    //    .long("dump-mir")
    //    .takes_value(true)
    //    .help("Dump the mir of reachable functions to the output file."))
    //.arg(Arg::new("dyn-calls-output")
    //    .long("dump-dyn-calls")
    //    .takes_value(true)
    //    .hide(true)
    //    .hide(true)
    //    .help("Dump resolved dynamic callsites with their corresponding call targets.")
    //    .long_help("Including both calls on dynamic trait objects and calls via function pointers"))
    parser
}

#[derive(Clone, Debug)]
pub struct AnalysisOptions {
    pub entry_func: String,
    pub entry_def_id: Option<u32>,
    pub verifopt_type: VerifOptType,
}

impl Default for AnalysisOptions {
    fn default() -> Self {
        Self {
            entry_func: String::new(),
            entry_def_id: None,
            verifopt_type: VerifOptType::FlowSensitive,
        }
    }
}

impl AnalysisOptions {
    /// Parses options from a list of strings. Any content beyond the leftmost `--` token
    /// will be returned (excluding this token).
    pub fn parse_from_args(&mut self, args: &[String], from_env: bool) -> Vec<String> {
        let mut verifopt_args_end = args.len();
        let mut rustc_args_start = 0;
        if let Some((p, _)) = args.iter().find_position(|s| s.as_str() == "--") {
            verifopt_args_end = p;
            rustc_args_start = p + 1;
        }
        let verifopt_args = &args[0..verifopt_args_end];

        let matches = if !from_env && rustc_args_start == 0 {
            // The arguments may not be intended for VerifOpt and may get here via some tool, so do not
            // report errors here, but just assume that the arguments were not meant for VerifOpt.
            match make_options_parser().try_get_matches_from(verifopt_args.iter()) {
                Ok(matches) => {
                    // Looks like these are VerifOpt options after all and there are no rustc options.
                    rustc_args_start = args.len();
                    matches
                }
                Err(e) => match e.kind() {
                    ErrorKind::DisplayHelp => {
                        eprintln!("{e}");
                        return args.to_vec();
                    }
                    ErrorKind::UnknownArgument => {
                        // Just send all of the arguments to rustc.
                        // Note that this means that VerifOpt options and rustc options must always
                        // be separated by --. I.e. any VerifOpt options present in arguments list
                        // will stay unknown to VerifOpt and will make rustc unhappy.
                        return args.to_vec();
                    }
                    _ => {
                        e.exit();
                    }
                },
            }
        } else {
            // This will display error diagnostics for arguments that are not valid for VerifOpt.
            match make_options_parser().try_get_matches_from(verifopt_args.iter()) {
                Ok(matches) => {
                    if rustc_args_start == 0 {
                        rustc_args_start = args.len();
                    }
                    matches
                }
                Err(e) => {
                    e.exit();
                }
            }
        };

        if let Some(s) = matches.get_one::<String>("entry-func-name") {
            self.entry_func = s.clone();
        }
        self.entry_def_id = matches.get_one::<u32>("entry-func-id").cloned();

        if matches.contains_id("verifopt-type") {
            self.verifopt_type = match matches.get_one::<String>("verifopt-type").unwrap().as_str()
            {
                "flow-sensitive" | "fsa" => VerifOptType::FlowSensitive,
                _ => unreachable!(),
            }
        }

        //if let Some(depth) = matches.get_one::<u32>("context-depth") {
        //    self.context_depth = *depth;
        //}

        //self.cast_constraint = !matches.contains_id("no-cast-constraint");
        //self.stack_filtering = matches.contains_id("stack-filtering");

        //self.dump_stats = matches.contains_id("dump-stats");
        //self.call_graph_output = matches.get_one::<String>("call-graph-output").cloned();
        //self.pts_output = matches.get_one::<String>("pts-output").cloned();
        //self.mir_output = matches.get_one::<String>("mir-output").cloned();
        //self.unsafe_stat_output = matches.get_one::<String>("unsafe-stats-output").cloned();
        //self.dyn_calls_output = matches.get_one::<String>("dyn-calls-output").cloned();
        //self.type_indices_output = matches.get_one::<String>("type-indices-output").cloned();

        // If the user provide the input source code file path before the `--` token,
        // add it to the rustc arguments.
        let mut rustc_args = args[rustc_args_start..].to_vec();
        if let Some(input) = matches.get_many::<String>("INPUT") {
            rustc_args.extend(input.cloned())
        }

        rustc_args
    }
}
