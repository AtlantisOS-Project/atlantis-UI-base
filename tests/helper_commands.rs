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

#[test]
fn command_lines_are_built_from_strings_and_str() {
    assert_eq!(command_line("ls", ["-l", "/tmp"]), ["ls", "-l", "/tmp"]);
    assert_eq!(command_line("true", Vec::<String>::new()), ["true"]);

    let image = String::from("boot.img");
    assert_eq!(
        command_line("fastboot", ["flash", "boot", &image]),
        ["fastboot", "flash", "boot", "boot.img"]
    );
}

#[test]
fn shell_lines_keep_every_word_together() {
    let line = shell_line(&["fastboot".into(), "getvar".into(), "all".into()]);
    assert_eq!(line, "'fastboot' 'getvar' 'all'");
    assert_eq!(
        shell_line(&["echo".into(), "it's".into()]),
        "'echo' 'it'\\''s'"
    );
    assert_eq!(shell_line(&[]), "");

    // a real shell reads it back as the same words
    let words = ["a b".to_string(), "$HOME".into(), "it's".into(), ";".into()];
    let output =
        capture_command_output(&["sh", "-c", &format!("printf '%s|' {}", shell_line(&words))])
            .unwrap();
    assert_eq!(output, "a b|$HOME|it's|;|");
}

#[test]
fn output_with_a_time_limit() {
    use std::time::{Duration, Instant};

    let both = command_line("sh", ["-c", "echo out; echo err >&2"]);
    let text = command_output_timeout(&both, Duration::from_secs(5)).unwrap();
    assert!(text.contains("out") && text.contains("err"), "{text}");

    // the exit status does not matter, the text does
    let failing = command_line("sh", ["-c", "echo no; exit 3"]);
    assert_eq!(
        command_output_timeout(&failing, Duration::from_secs(5)).as_deref(),
        Some("no\n")
    );

    // a program that takes too long is stopped
    let start = Instant::now();
    let slow = command_line("sleep", ["5"]);
    assert_eq!(
        command_output_timeout(&slow, Duration::from_millis(200)),
        None
    );
    assert!(start.elapsed() < Duration::from_secs(3));

    // a program that does not exist, and an empty command
    let missing = command_line("no-such-program-xyz", Vec::<String>::new());
    assert_eq!(
        command_output_timeout(&missing, Duration::from_secs(1)),
        None
    );
    assert_eq!(command_output_timeout(&[], Duration::from_secs(1)), None);

    // a lot of output does not block the program
    let big = command_line("sh", ["-c", "head -c 300000 /dev/zero | tr '\\0' x"]);
    let text = command_output_timeout(&big, Duration::from_secs(10)).unwrap();
    assert_eq!(text.len(), 300_000);
}

#[test]
fn a_chain_of_commands_stops_at_the_first_failure() {
    let ok = run_command_chain(&[
        command_line("echo", ["one"]),
        Vec::new(), // an empty command is skipped
        command_line("sh", ["-c", "echo two; echo oops >&2"]),
    ])
    .unwrap();
    assert_eq!(ok.len(), 2);
    assert_eq!(ok[0].command, "echo one");
    assert_eq!(ok[0].stdout, "one\n");
    assert_eq!(ok[1].stderr, "oops\n");
    assert!(ok.iter().all(|result| result.success));

    // an exit code other than 0: everything up to the failed command, nothing after it
    let failed = run_command_chain(&[
        command_line("echo", ["before"]),
        command_line("sh", ["-c", "echo bad >&2; exit 3"]),
        command_line("echo", ["after"]),
    ])
    .unwrap_err();
    assert_eq!(failed.len(), 2);
    assert!(failed[0].success && !failed[1].success);
    assert_eq!(failed[1].stderr, "bad\n");

    // a program that does not exist
    let missing = run_command_chain(&[command_line("no-such-program-xyz", ["-x"])]).unwrap_err();
    assert_eq!(missing.len(), 1);
    assert!(!missing[0].success && missing[0].stdout.is_empty());
    assert!(
        missing[0].stderr.starts_with("Failed to execute process"),
        "{}",
        missing[0].stderr
    );

    assert_eq!(run_command_chain(&[]).unwrap(), []);
}
