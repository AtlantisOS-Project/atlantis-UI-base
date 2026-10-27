// execute_command.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Functions to run a command and to capture its output.

use crate::helper::commands::command_line::redacted;
use crate::helper::platform::program::host_command;
use std::io;
use std::process::{ExitStatus, Stdio};

/// Splits `args` into the program and its arguments.
fn split_command<'a>(args: &'a [&'a str]) -> io::Result<(&'a str, &'a [&'a str])> {
    args.split_first()
        .map(|(program, rest)| (*program, rest))
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "no command given"))
}

/// Runs a command and lets it write directly to the standard streams of this process.
///
/// The function waits until the command has finished.
///
/// # Arguments
///
/// * `args` - The program followed by its arguments. No shell is involved, so quoting and
///   pipes are not interpreted; pass `["sh", "-c", "…"]` if you need them.
///
/// # Errors
///
/// Returns an [`io::Error`] if `args` is empty or the program cannot be started. A command
/// that runs but fails is *not* an error: check the returned [`ExitStatus`].
///
/// # Examples
///
/// ```
/// use atlbase::prelude::run_command;
///
/// let status = run_command(&["true"]).unwrap();
/// assert!(status.success());
/// ```
pub fn run_command(args: &[&str]) -> io::Result<ExitStatus> {
    let (program, rest) = split_command(args)?;
    log::debug!("run: {}", redacted(args));
    host_command(program).args(rest).spawn()?.wait()
}

/// Runs a command and captures what it writes to `stdout`.
///
/// # Arguments
///
/// * `args` - The program followed by its arguments (see [`run_command`]).
///
/// # Errors
///
/// Returns an [`io::Error`] if `args` is empty, the program cannot be started, or it exits
/// with a status other than 0. In the last case the message contains the `stderr` output.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::capture_command_output;
///
/// let output = capture_command_output(&["echo", "hello"]).unwrap();
/// assert_eq!(output.trim(), "hello");
///
/// assert!(capture_command_output(&["false"]).is_err());
/// ```
pub fn capture_command_output(args: &[&str]) -> io::Result<String> {
    let (program, rest) = split_command(args)?;
    log::debug!("run: {}", redacted(args));
    let output = host_command(program)
        .args(rest)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        log::warn!("failed ({}): {}", output.status, redacted(args));
        Err(io::Error::other(format!("Command failed: {}", error)))
    }
}
