// helper_commands.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Tests of the command helpers (`atlbase::helper::commands`).

use atlbase::prelude::*;
use std::fs;
use std::io::ErrorKind;
use std::os::unix::fs::PermissionsExt;

#[test]
fn commands_are_found_in_path_and_by_absolute_path() {
    assert!(command_exists_native("sh"));
    assert!(!command_exists_native("this-command-does-not-exist-1234"));

    let dir = tempfile::tempdir().unwrap();
    let script = dir.path().join("tool");
    fs::write(&script, "#!/bin/sh\n").unwrap();

    // a file without the execute bit is not a command
    fs::set_permissions(&script, fs::Permissions::from_mode(0o644)).unwrap();
    assert!(!command_exists_native(script.to_str().unwrap()));

    fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    assert!(command_exists_native(script.to_str().unwrap()));

    // a directory is not a command either
    assert!(!command_exists_native(dir.path().to_str().unwrap()));
}

#[test]
fn running_and_capturing_commands() {
    assert!(run_command(&["true"]).unwrap().success());
    assert!(!run_command(&["false"]).unwrap().success());

    assert_eq!(
        capture_command_output(&["echo", "hello"]).unwrap(),
        "hello\n"
    );

    let error = capture_command_output(&["sh", "-c", "echo oops >&2; exit 3"]).unwrap_err();
    assert!(
        error.to_string().contains("oops"),
        "stderr is part of the error: {error}"
    );

    assert!(run_command(&["this-command-does-not-exist-1234"]).is_err());
}

#[test]
fn an_empty_command_is_an_error_not_a_panic() {
    assert_eq!(
        run_command(&[]).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        capture_command_output(&[]).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
}

#[test]
fn empty_urls_and_terminal_commands_do_nothing() {
    open_url("");
    open_terminal_by_desktop("");
}
