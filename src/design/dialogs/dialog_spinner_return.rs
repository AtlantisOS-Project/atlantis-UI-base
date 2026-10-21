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
//! │ show_spinner_dialog_…    │          │ run_command_chain        │
//! │ creates the dialog       │          │                          │
//! │ starts the thread ───────┼─────────►│ for each command:        │
//! │                          │          │   run, collect output    │
//! │ the main loop gets the   │◄─────────┼── the results            │
//! │ results (run_background) │          │                          │
//! └──────────────────────────┘          └──────────────────────────┘
//! ```
//!
//! When all commands are done — or as soon as one fails — the results arrive in the main
//! thread, the dialog closes and the callback is called.

use super::dialogs_spinner::{IndicatorType, run_with_progress};
use crate::helper::commands::command_chain::run_command_chain;
use adw::prelude::*;
use gtk4::Widget;

pub use crate::helper::commands::command_chain::CommandResult;

/// The results of a chain of commands: `Ok` if all succeeded, `Err` up to the first failure.
type ChainResult = Result<Vec<CommandResult>, Vec<CommandResult>>;

/// The callback of [`show_spinner_dialog_return_output`].
type Callback = Box<dyn FnOnce(ChainResult) + 'static>;

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
    run_with_progress(
        parent,
        title,
        message,
        indicator,
        move || run_command_chain(&commands),
        move |result| {
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
        },
    );
}
