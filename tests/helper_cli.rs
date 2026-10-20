// helper_cli.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Tests of the shared command line options (`atlbase::helper::cli`).
//!
//! `apply_debug_options` changes the environment of the process, so it is tested in one
//! sequential test of its own.

use atlbase::prelude::*;

fn read(args: &[&str]) -> Result<CliOptions, CliRequest> {
    read_common_options(args.iter().map(|arg| arg.to_string()))
}

#[test]
fn a_plain_start_passes_everything_to_gtk() {
    let options = read(&["demo", "--gapplication-service"]).unwrap();
    assert!(!options.debug);
    assert_eq!(options.gtk_args, ["demo", "--gapplication-service"]);

    assert_eq!(read(&[]).unwrap().gtk_args, Vec::<String>::new());
}

#[test]
fn the_flags_need_debug_in_front_of_them() {
    let options = read(&["demo", "-debug", "-snp", "-flp", "-lang"]).unwrap();
    assert!(
        options.debug && options.fake_snap && options.fake_flatpak && options.fallback_language
    );
    assert_eq!(options.gtk_args, ["demo"]);

    let options = read(&["demo", "-snp"]).unwrap();
    assert!(!options.fake_snap, "-snp is ignored without -debug");
    assert_eq!(options.gtk_args, ["demo", "-snp"]);

    // order matters: the flag comes before -debug
    assert!(
        !read(&["demo", "-lang", "-debug"])
            .unwrap()
            .fallback_language
    );
}

#[test]
fn every_spelling_of_debug_version_and_help() {
    for arg in ["-d", "--d", "-debug", "--debug"] {
        assert!(read(&["demo", arg]).unwrap().debug, "{arg}");
    }
    for arg in ["-v", "--v", "-version", "--version"] {
        assert_eq!(read(&["demo", arg]), Err(CliRequest::Version), "{arg}");
    }
    for arg in ["-h", "--help"] {
        assert_eq!(read(&["demo", arg]), Err(CliRequest::Help), "{arg}");
    }
}

#[test]
fn the_version_names_the_package() {
    assert_eq!(
        version_text("Demo", "1.0.0", AppEnvironment::Flatpak),
        "Demo [Flatpak] 1.0.0"
    );
    assert!(version_text("Demo", "1.0.0", AppEnvironment::LocalDeb).contains("Local Debian"));
    for flag in ["-debug", "-snp", "-flp", "-lang", "--version", "--help"] {
        assert!(COMMON_OPTIONS_HELP.contains(flag), "{flag}");
    }
}

#[test]
fn the_debug_options_change_the_process() {
    assert!(!debug_mode() && !fallback_language());

    // SAFETY: this is the only test of the process that uses the environment.
    unsafe {
        std::env::remove_var("SNAP");
        std::env::remove_var("FLATPAK_ID");
    }
    let options = read(&["demo", "-debug", "-snp", "-flp", "-lang"]).unwrap();
    apply_debug_options(&options, "org.example.Demo");

    assert!(debug_mode() && fallback_language());
    assert_eq!(std::env::var("SNAP").unwrap(), "/tmp/snap-debug");
    assert_eq!(std::env::var("FLATPAK_ID").unwrap(), "org.example.Demo");
    assert_eq!(get_execution_environment(), "flatpak");

    set_debug_mode(false);
    assert!(!debug_mode());
}
