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

use crate::helper::platform::program::host_command;

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

/// Runs the commands one after another; an empty command is skipped.
pub(crate) fn run_chain(commands: &[Vec<String>]) -> Chain {
    let mut results = Vec::new();

    for command in commands {
        let Some((program, args)) = command.split_first() else {
            continue;
        };
        let line = command.join(" ");

        match host_command(program).args(args).output() {
            Ok(output) => {
                let success = output.status.success();
                results.push(CommandResult {
                    command: line,
                    stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                    stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
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
