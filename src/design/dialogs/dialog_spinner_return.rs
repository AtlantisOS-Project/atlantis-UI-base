// dialog_spinner_return.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Progress dialogs that return the result of every command.
//!
//! The module runs a chain of system commands ([`std::process::Command`]) in a separate
//! thread, so the interface of the Libadwaita/GTK 4 application keeps responding. While the
//! commands run, a modal dialog that cannot be closed shows a spinner or a progress bar.
//!
//! # Architecture
//!
//! ```text
//!  Main (UI) thread                       Worker thread
//! ┌──────────────────────────┐          ┌──────────────────────────┐
//! │ show_spinner_dialog_…    │          │ run_commands_thread      │
//! │ creates the dialog       │          │                          │
//! │ starts the thread ───────┼─────────►│ for each command:        │
//! │                          │          │   run, collect output    │
//! │ glib timeout polls the   │◄─────────┼── send the results       │
//! │ channel (mpsc)           │          │                          │
//! └──────────────────────────┘          └──────────────────────────┘
//! ```
//!
//! When all commands are done — or as soon as one fails — the results are sent to the main
//! thread, the dialog closes and the callback is called.

use super::dialogs_spinner::{IndicatorType, build_progress_dialog, close_dialog_when_done};
use adw::prelude::*;
use gtk4::Widget;
use std::process::Command;
use std::sync::mpsc;
use std::thread;

/// The outcome of one executed command.
///
/// The worker thread sends a list of these to the callback of
/// [`show_spinner_dialog_return_output`].
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

/// The results of a chain of commands: `Ok` if all succeeded, `Err` up to the first failure.
type ChainResult = Result<Vec<CommandResult>, Vec<CommandResult>>;

/// The callback of [`show_spinner_dialog_return_output`].
type Callback = Box<dyn FnOnce(ChainResult) + 'static>;

/// Runs the commands one after another in a background thread.
///
/// The chain stops at the first command that fails or cannot be started; the later commands
/// are not run, and everything collected so far is sent as `Err`.
fn run_commands_thread(commands: Vec<Vec<String>>, tx: mpsc::Sender<ChainResult>) {
    thread::spawn(move || {
        let mut results = Vec::new();

        for cmd_args in commands {
            let Some((program, args)) = cmd_args.split_first() else {
                continue; // empty command
            };
            let command = cmd_args.join(" ");

            match Command::new(program).args(args).output() {
                Ok(output) => {
                    let success = output.status.success();
                    results.push(CommandResult {
                        command,
                        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
                        success,
                    });

                    if !success {
                        let _ = tx.send(Err(results));
                        return;
                    }
                }
                Err(e) => {
                    results.push(CommandResult {
                        command,
                        stdout: String::new(),
                        stderr: format!("Failed to execute process: {}", e),
                        success: false,
                    });
                    let _ = tx.send(Err(results));
                    return;
                }
            }
        }

        let _ = tx.send(Ok(results));
    });
}

/// Shows a modal dialog while a chain of commands runs, and returns the result of each one.
///
/// The user cannot close the dialog, so the parent window is blocked until the chain has
/// finished or failed. In contrast to
/// [`show_spinner_dialog`](super::dialogs_spinner::show_spinner_dialog) the callback gets
/// one [`CommandResult`] per command with separate `stdout` and `stderr`.
///
/// # Arguments
///
/// * `parent` - The widget (usually the main window) the dialog is shown over.
/// * `title` - The title of the dialog.
/// * `message` - An explanation for the user.
/// * `commands` - The commands, each as the program followed by its arguments, for example
///   `vec![vec!["pkexec".to_string(), "apt".to_string(), "update".to_string()]]`.
/// * `indicator` - A [`IndicatorType::Spinner`] or a pulsing [`IndicatorType::ProgressBar`].
/// * `on_complete` - Optional. Receives `Ok(results)` if all commands succeeded, or
///   `Err(results)` with everything that ran up to and including the failed command.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::{IndicatorType, show_spinner_dialog_return_output};
/// # gtk4::init().unwrap();
/// # let window = adw::ApplicationWindow::builder().build();
///
/// show_spinner_dialog_return_output(
///     &window,
///     "System update",
///     "Updating the system...",
///     vec![
///         vec!["ls".to_string(), "./".to_string()],
///         vec!["ls".to_string(), "-l".to_string()],
///     ],
///     IndicatorType::Spinner,
///     Some(Box::new(|result| match result {
///         Ok(results) => {
///             for command in results {
///                 println!("[{}]: {}", command.command, command.stdout);
///             }
///         }
///         Err(results) => {
///             println!("Aborted!");
///             for command in results {
///                 let state = if command.success { "OK" } else { "ERROR" };
///                 println!(" [{}] {} {}", state, command.command, command.stderr);
///             }
///         }
///     })),
/// );
/// ```
pub fn show_spinner_dialog_return_output(
    parent: &impl IsA<Widget>,
    title: &str,
    message: &str,
    commands: Vec<Vec<String>>,
    indicator: IndicatorType,
    on_complete: Option<Callback>,
) {
    let dialog = build_progress_dialog(title, message, indicator);

    let (tx, rx) = mpsc::channel();
    run_commands_thread(commands, tx);

    close_dialog_when_done(&dialog, rx, move |result| {
        if let Some(callback) = on_complete {
            callback(result.unwrap_or_else(|| {
                Err(vec![CommandResult {
                    command: "Unknown".to_string(),
                    stdout: String::new(),
                    stderr: "Thread disconnected unexpectedly.".to_string(),
                    success: false,
                }])
            }));
        }
    });

    dialog.present(Some(parent));
}
