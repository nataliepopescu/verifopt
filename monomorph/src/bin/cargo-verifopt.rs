// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.
//
// Modified by <Natalie Popescu>.

//! This provides an implementation for the "cargo verifopt" subcommand.
//!
//! The subcommand is the same as "cargo build" but with three differences:
//! 1) It implicitly adds the options "-Z always_encode_mir" to the rustc invocation.
//! 2) It calls `verifopt` rather than `rustc` for all the targets of the current package.
//! 3) It runs `cargo test --no-run` for test targets.

#![feature(rustc_private)]

extern crate rustc_driver;

use cargo_metadata::{Package, TargetKind};
use log::info;
use serde_json;
use std::env;
use std::ffi::OsString;
use std::ops::Index;
use std::path::Path;
use std::process::{Command, Stdio};

use monomorph::util;

/// The help message for `cargo-verifopt`
const CARGO_VERIFOPT_HELP: &str = r#"Flow-sensitive analysis tool for Rust programs
Usage:
    cargo verifopt
"#;

/// Set the environment variable `VERIFOPT_BUILD_STD` to enable the building of std library when running verifopt.
const VERIFOPT_BUILD_STD: &str = "VERIFOPT_BUILD_STD";

pub fn main() {
    if std::env::args()
        .take_while(|a| a != "--")
        .any(|a| a == "--help" || a == "-h")
    {
        println!("{}", CARGO_VERIFOPT_HELP);
        return;
    }

    match std::env::args().nth(1).as_ref().map(AsRef::<str>::as_ref) {
        Some(s) if s.ends_with("verifopt") => {
            // Get here for the top level cargo execution, i.e. "cargo verifopt".
            call_cargo();
        }
        Some(s) if s.ends_with("rustc") => {
            // 'cargo rustc ..' redirects here because RUSTC_WRAPPER points to this binary.
            // execute rustc with VerifOpt applicable parameters for dependencies and call VerifOpt
            // to analyze targets in the current package.
            call_rustc_or_verifopt();
        }
        Some(arg) => {
            eprintln!(
                "`cargo-verifopt` called with invalid first argument: {arg}; please only invoke this binary through `cargo verifopt`"
            );
        }
        _ => {
            eprintln!("current args: {:?}", std::env::args());
            eprintln!(
                "`cargo-verifopt` called without first argument; please only invoke this binary through `cargo verifopt`"
            );
        }
    }
}

/// Read the toml associated with the current directory and
/// recursively execute cargo for each applicable package target/workspace member in the toml
fn call_cargo() {
    let manifest_path =
        get_arg_flag_value("--manifest-path").map(|m| Path::new(&m).canonicalize().unwrap());

    let mut cmd = cargo_metadata::MetadataCommand::new();
    if let Some(ref manifest_path) = manifest_path {
        cmd.manifest_path(manifest_path);
    }

    let metadata = if let Ok(metadata) = cmd.exec() {
        metadata
    } else {
        eprintln!("Could not obtain Cargo metadata; likely an ill-formed manifest");
        std::process::exit(1);
    };

    // If a binary is specified, analyze this binary only.
    if let Some(target) = get_arg_flag_value("--bin") {
        call_cargo_on_target(&target, &TargetKind::Bin);
        return;
    }

    if let Some(root) = metadata.root_package() {
        call_cargo_on_each_package_target(root);
        return;
    }

    // There is no root, this must be a workspace, so call_cargo_on_each_package_target on each workspace member
    for package_id in &metadata.workspace_members {
        let package = metadata.index(package_id);
        call_cargo_on_each_package_target(package);
    }
}

fn call_cargo_on_each_package_target(package: &Package) {
    let lib_only = has_arg_flag("--lib");
    for target in &package.targets {
        let kind = target
            .kind
            .first()
            .expect("bad cargo metadata: target::kind");
        if lib_only && *kind != TargetKind::Lib {
            continue;
        }
        call_cargo_on_target(&target.name, kind);
    }
}

fn call_cargo_on_target(target: &String, kind: &TargetKind) {
    // Build a cargo command for target
    let mut cmd =
        Command::new(std::env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo")));
    match kind {
        TargetKind::Bin => {
            cmd.arg("build");
            if get_arg_flag_value("--bin").is_none() {
                cmd.arg("--bin").arg(target);
            }
        }
        TargetKind::Lib => {
            cmd.arg("build");
            cmd.arg("--lib");
        }
        TargetKind::Test => {
            cmd.arg("test");
            cmd.arg("--no-run");
        }
        _ => {
            return;
        }
    }
    cmd.arg("--verbose");

    let mut args = std::env::args().skip(2);
    // Add cargo args to cmd until first `--`.
    for arg in args.by_ref() {
        if arg == "--" {
            break;
        }
        if arg == "--lib" {
            continue;
        }
        cmd.arg(arg);
    }

    // Enable Cargo to compile the standard library from source code as part of a crate graph compilation.
    if env::var(VERIFOPT_BUILD_STD).is_ok() {
        cmd.arg("-Zbuild-std");

        if !has_arg_flag("--target") {
            let toolchain_target = toolchain_target().expect("could not get toolchain target");
            cmd.arg("--target").arg(toolchain_target);
        }
    }

    // Serialize the remaining args into an environment variable.
    let args_vec: Vec<String> = args.collect();
    if !args_vec.is_empty() {
        cmd.env(
            "VERIFOPT_FLAGS",
            serde_json::to_string(&args_vec).expect("failed to serialize args"),
        );
    }

    // Force cargo to recompile all dependencies with VerifOpt friendly flags
    cmd.env("RUSTFLAGS", "-Z always_encode_mir");

    // Replace the rustc executable through RUSTC_WRAPPER environment variable so that rustc
    // calls generated by cargo come back to cargo-verifopt.
    let path = std::env::current_exe().expect("current executable path invalid");
    cmd.env("RUSTC_WRAPPER", path);

    // Communicate the name of the root crate to the calls to cargo-verifopt that are invoked via
    // the RUSTC_WRAPPER setting.
    cmd.env("VERIFOPT_CRATE", target.replace('-', "_"));

    // Communicate the target kind of the root crate to the calls to cargo-verifopt that are invoked via
    // the RUSTC_WRAPPER setting.
    cmd.env("VERIFOPT_TARGET_KIND", kind.to_string());

    // Set the tool chain to be compatible with verifopt
    if let Some(toolchain) = option_env!("RUSTUP_TOOLCHAIN") {
        cmd.env("RUSTUP_TOOLCHAIN", toolchain);
    }

    // Execute cmd
    info!("cmd: {:?}", cmd);
    let exit_status = cmd
        .spawn()
        .expect("could not run cargo")
        .wait()
        .expect("failed to wait for cargo");

    if !exit_status.success() {
        std::process::exit(exit_status.code().unwrap_or(-1))
    }
}

fn call_rustc_or_verifopt() {
    if let Some(crate_name) = get_arg_flag_value("--crate-name") {
        if let Ok(verifopt_crate) = std::env::var("VERIFOPT_CRATE") {
            if crate_name.eq(&verifopt_crate) {
                if let Ok(kind) = std::env::var("VERIFOPT_TARGET_KIND") {
                    if let Some(t) = get_arg_flag_value("--crate-type") {
                        if kind.eq(&t) {
                            call_verifopt();
                            return;
                        }
                    } else if kind == "test" {
                        call_verifopt();
                        return;
                    }
                }
            }
        }
    }
    call_rustc()
}

fn call_verifopt() {
    let mut path = std::env::current_exe().expect("current executable path invalid");
    let extension = path.extension().map(|e| e.to_owned());
    path.pop(); // remove the cargo_verifopt bit
    path.push("verifopt");
    if let Some(ext) = extension {
        path.set_extension(ext);
    }
    let mut cmd = Command::new(path);
    cmd.args(std::env::args().skip(2));
    let exit_status = cmd
        .spawn()
        .expect("could not run verifopt")
        .wait()
        .expect("failed to wait for verifopt");

    if !exit_status.success() {
        std::process::exit(exit_status.code().unwrap_or(-1))
    }
}

fn call_rustc() {
    // todo: invoke the rust compiler for the appropriate tool chain?
    let mut cmd =
        Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc")));
    cmd.args(std::env::args().skip(2));
    let exit_status = cmd
        .spawn()
        .expect("could not run rustc")
        .wait()
        .expect("failed to wait for rustc");

    if !exit_status.success() {
        std::process::exit(exit_status.code().unwrap_or(-1))
    }
}

/// Determines whether a flag `name` is present before `--`.
/// For example, has_arg_flag("-v")
fn has_arg_flag(name: &str) -> bool {
    let mut args = std::env::args().take_while(|val| val != "--");
    args.any(|val| val == name)
}

/// Gets the value of `name`.
/// `--name value` or `--name=value`
fn get_arg_flag_value(name: &str) -> Option<String> {
    let mut args = std::env::args().take_while(|val| val != "--");
    loop {
        let arg = match args.next() {
            Some(arg) => arg,
            None => return None,
        };
        if !arg.starts_with(name) {
            continue;
        }
        // Strip `name`.
        let suffix = &arg[name.len()..];
        if suffix.is_empty() {
            // This argument is `name` and the next one is the value.
            return args.next();
        } else if let Some(arg_value) = suffix.strip_prefix('=') {
            return Some(arg_value.to_owned());
        }
    }
}

/// Returns the target of the toolchain, e.g. "x86_64-unknown-linux-gnu".
fn toolchain_target() -> Option<String> {
    let sysroot = util::find_sysroot();

    // get the supported rustup targets
    let output = String::from_utf8(
        Command::new("rustup")
            .arg("target")
            .arg("list")
            .stdout(Stdio::piped())
            .output()
            .expect("could not run 'rustup target list'")
            .stdout,
    )
    .unwrap();

    let target = output.lines().find_map(|line| {
        let target = line.split_whitespace().next().unwrap().to_owned();
        if sysroot.ends_with(&target) {
            Some(target)
        } else {
            None
        }
    });

    target
}
