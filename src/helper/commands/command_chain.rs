// command_chain.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Running a chain of commands, one after another.
//!
//! This is what every progress dialog of the library does in its worker thread
//! ([`show_spinner_dialog`], [`show_spinner_dialog_return_output`], [`CommandTask`]). It can be
//! used without a window, too: call [`run_command_chain`] in a thread of your own.
//!
//! [`show_spinner_dialog`]: crate::design::dialogs::dialogs_spinner::show_spinner_dialog
//! [`show_spinner_dialog_return_output`]: crate::design::dialogs::dialog_spinner_return::show_spinner_dialog_return_output
//! [`CommandTask`]: crate::design::dialogs::command_task::CommandTask

use crate::helper::commands::command_line::{redacted, run_captured};
use crate::helper::platform::logging::excerpt;
use crate::helper::platform::program::host_command;
use std::time::{Duration, Instant};

/// How long one command of a chain may run: two hours. A command that takes longer is stopped
/// and counts as failed. Without a limit, a command that waits for a device that is not there
/// would block a progress dialog, which the user cannot close, for ever.
const COMMAND_TIMEOUT: Duration = Duration::from_secs(2 * 60 * 60);

/// The outcome of one executed command.
///
/// [`run_command_chain`] returns a list of these, and the callback of
/// [`show_spinner_dialog_return_output`](crate::design::dialogs::dialog_spinner_return::show_spinner_dialog_return_output)
/// receives it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandResult {
    /// The command line with all arguments, joined by spaces.
    ///
    /// For example `"fastboot flash bootloader boot.img"`.
    pub command: String,

    /// What the command wrote to `stdout` (invalid UTF-8 is replaced).
    pub stdout: String,

    /// What the command wrote to `stderr`, or the reason why it could not be started.
    pub stderr: String,

    /// `true` if the command ended with the exit code 0.
    pub success: bool,
}

/// Why a chain stopped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Failure {
    /// The last command ended with an exit code other than 0.
    Status,
    /// The last command could not be started; the reason of the system.
    Spawn(String),
}

/// What a chain did: every command that ran, and why it stopped early (if it did).
pub(crate) struct Chain {
    pub(crate) results: Vec<CommandResult>,
    pub(crate) failure: Option<Failure>,
}

/// Runs the commands one after another; an empty command is skipped. Each command runs for
/// [`COMMAND_TIMEOUT`] at most, and at most 8 MiB of each of its two outputs are kept (a note
/// at the end of `stderr` says so).
pub(crate) fn run_chain(commands: &[Vec<String>]) -> Chain {
    let mut results = Vec::new();

    for command in commands {
        let Some((program, args)) = command.split_first() else {
            continue;
        };
        let line = command.join(" ");
        // the log gets the line without the values of secrets
        let logged = redacted(command);
        log::info!("run: {logged}");
        let started = Instant::now();

        let mut process = host_command(program);
        process.args(args);
        match run_captured(process, Some(COMMAND_TIMEOUT)) {
            Ok(captured) => {
                let success = captured.status.is_some_and(|status| status.success());
                let seconds = started.elapsed().as_secs_f32();
                let how = captured
                    .status
                    .map_or("timeout".to_string(), |status| status.to_string());
                if success {
                    log::info!("done in {seconds:.1} s: {logged}");
                } else {
                    log::warn!(
                        "failed ({how}) after {seconds:.1} s: {logged}\n{}",
                        excerpt(&captured.stderr)
                    );
                }
                let mut stderr = captured.stderr;
                if captured.status.is_none() {
                    stderr.push_str(&format!(
                        "\n[stopped: the command ran for more than {} seconds]",
                        COMMAND_TIMEOUT.as_secs()
                    ));
                }
                if captured.truncated {
                    stderr.push_str("\n[the output was cut: it is longer than 8 MiB]");
                }
                results.push(CommandResult {
                    command: line,
                    stdout: captured.stdout,
                    stderr,
                    success,
                });
                if !success {
                    return Chain {
                        results,
                        failure: Some(Failure::Status),
                    };
                }
            }
            Err(error) => {
                log::error!("cannot start {program}: {error}");
                results.push(CommandResult {
                    command: line,
                    stdout: String::new(),
                    stderr: format!("Failed to execute process: {error}"),
                    success: false,
                });
                return Chain {
                    results,
                    failure: Some(Failure::Spawn(error.to_string())),
                };
            }
        }
    }
    Chain {
        results,
        failure: None,
    }
}

/// Runs a chain of commands, each given as the program followed by its arguments (no shell).
///
/// The chain stops at the first command that cannot be started or ends with an exit code other
/// than 0; the later commands are not run. The function blocks until the chain is done: call it
/// in a background thread if a window is open.
///
/// # Returns
///
/// `Ok` with the result of every command if all succeeded, `Err` with everything that ran up to
/// and including the failed command.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{command_line, run_command_chain};
///
/// let results = run_command_chain(&[
///     command_line("echo", ["one"]),
///     command_line("echo", ["two"]),
/// ])
/// .unwrap();
/// assert_eq!(results[1].stdout, "two\n");
///
/// // the chain stops at the first failure
/// let failed = run_command_chain(&[
///     command_line("false", Vec::<String>::new()),
///     command_line("echo", ["never"]),
/// ])
/// .unwrap_err();
/// assert_eq!(failed.len(), 1);
/// assert!(!failed[0].success);
/// ```
pub fn run_command_chain(
    commands: &[Vec<String>],
) -> Result<Vec<CommandResult>, Vec<CommandResult>> {
    let chain = run_chain(commands);
    match chain.failure {
        None => Ok(chain.results),
        Some(_) => Err(chain.results),
    }
}
