// dialogs_spinner.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Progress indicators for system commands.
//!
//! The module runs a list of commands in the background while a dialog that cannot be
//! closed shows a spinner or a progress bar. The same dialog is used by
//! [`show_task_dialog`](super::dialog_function::show_task_dialog) and
//! [`show_spinner_dialog_return_output`](super::dialog_spinner_return::show_spinner_dialog_return_output).

use adw::prelude::*;
use adw::{Dialog, Spinner};
use gtk4::{Align, Box as GtkBox, Label, Orientation, ProgressBar, Widget, glib};
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// The kind of progress indicator in a dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndicatorType {
    /// A circular, endless loading indicator (Libadwaita spinner).
    Spinner,
    /// A horizontal bar that shows activity by moving back and forth (pulse).
    ProgressBar,
}

/// The callback of [`show_spinner_dialog`]: the output of all commands, or the error.
type Callback = Box<dyn FnOnce(Result<String, String>) + 'static>;

/// Builds the modal dialog with a title, a message and an indicator.
///
/// The dialog cannot be closed by the user; close it with [`Dialog::force_close`].
pub(crate) fn build_progress_dialog(
    title: &str,
    message: &str,
    indicator: IndicatorType,
) -> Dialog {
    let dialog = Dialog::builder()
        .title(title)
        .content_width(400)
        .can_close(false)
        .build();

    let root_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(18)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .halign(Align::Center)
        .build();

    root_box.append(
        &Label::builder()
            .label(title)
            .wrap(true)
            .justify(gtk4::Justification::Center)
            .css_classes(["title-3"])
            .build(),
    );
    root_box.append(
        &Label::builder()
            .label(message)
            .wrap(true)
            .justify(gtk4::Justification::Center)
            .build(),
    );

    match indicator {
        IndicatorType::Spinner => {
            root_box.append(
                &Spinner::builder()
                    .halign(Align::Center)
                    .valign(Align::Center)
                    .width_request(96)
                    .height_request(96)
                    .build(),
            );
        }
        IndicatorType::ProgressBar => {
            let progress_bar = ProgressBar::builder()
                .pulse_step(0.1)
                .halign(Align::Fill)
                .width_request(250)
                .build();

            // A weak reference: when the dialog is gone, the timer stops by itself instead
            // of pulsing a destroyed widget forever.
            let weak = progress_bar.downgrade();
            glib::timeout_add_local(Duration::from_millis(100), move || match weak.upgrade() {
                Some(progress_bar) => {
                    progress_bar.pulse();
                    glib::ControlFlow::Continue
                }
                None => glib::ControlFlow::Break,
            });

            root_box.append(&progress_bar);
        }
    }

    dialog.set_child(Some(&root_box));
    dialog
}

/// Waits for the result of a background thread, then closes the dialog.
///
/// `on_done` is called on the main thread with `Some(result)`, or with `None` if the
/// thread ended without sending a result (for example because it panicked).
pub(crate) fn close_dialog_when_done<T: 'static>(
    dialog: &Dialog,
    rx: mpsc::Receiver<T>,
    on_done: impl FnOnce(Option<T>) + 'static,
) {
    let dialog = dialog.clone();
    let mut on_done = Some(on_done);

    glib::timeout_add_local(Duration::from_millis(25), move || {
        let result = match rx.try_recv() {
            Ok(result) => Some(result),
            Err(mpsc::TryRecvError::Empty) => return glib::ControlFlow::Continue,
            Err(mpsc::TryRecvError::Disconnected) => None,
        };

        dialog.force_close();
        if let Some(on_done) = on_done.take() {
            on_done(result);
        }
        glib::ControlFlow::Break
    });
}

/// Runs the commands one after another in a background thread.
///
/// The output of all commands is collected. As soon as a command fails, the chain stops
/// and the error is sent to the main thread.
fn run_commands_thread(commands: Vec<Vec<String>>, tx: mpsc::Sender<Result<String, String>>) {
    thread::spawn(move || {
        let mut combined_output = String::new();

        for cmd_args in commands {
            let Some((program, args)) = cmd_args.split_first() else {
                continue; // empty command
            };
            let cmd_display = cmd_args.join(" ");

            match Command::new(program).args(args).output() {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    combined_output.push_str(&stdout);
                    if !stderr.is_empty() {
                        combined_output.push_str(&format!("\n[Stderr]: {}", stderr));
                    }

                    if !output.status.success() {
                        let _ = tx.send(Err(format!(
                            "Command failed: {}\nOutput:\n{}",
                            cmd_display, combined_output
                        )));
                        return;
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(format!(
                        "Failed to execute command '{}': {}",
                        cmd_display, e
                    )));
                    return;
                }
            }
        }

        let _ = tx.send(Ok(combined_output));
    });
}

/// Shows a dialog while a list of commands runs in the background.
///
/// The user cannot close the dialog. It closes by itself when all commands are done or one
/// of them failed; then `on_complete` is called on the main thread.
///
/// The commands run one after another without a shell. The chain stops at the first command
/// that cannot be started or exits with a status other than 0.
///
/// # Arguments
///
/// * `parent` - The widget (usually the main window) the dialog is shown over.
/// * `title` - The title of the dialog.
/// * `message` - An explanation for the user.
/// * `commands` - The commands, each as the program followed by its arguments, for example
///   `vec![vec!["fastboot".to_string(), "reboot".to_string()]]`.
/// * `indicator` - The kind of indicator ([`IndicatorType`]).
/// * `on_complete` - Optional. Receives `Ok(output)` with the collected output of all
///   commands, or `Err(message)` with the reason for the failure.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::{IndicatorType, show_spinner_dialog};
/// # gtk4::init().unwrap();
/// # let window = adw::ApplicationWindow::builder().build();
///
/// let commands = vec![
///     vec!["fastboot".to_string(), "devices".to_string()],
///     vec!["fastboot".to_string(), "reboot".to_string(), "bootloader".to_string()],
/// ];
///
/// show_spinner_dialog(
///     &window,
///     "Flashing",
///     "Processing commands...",
///     commands,
///     IndicatorType::Spinner,
///     Some(Box::new(|result| match result {
///         Ok(output) => println!("Success: {}", output),
///         Err(error) => eprintln!("Error: {}", error),
///     })),
/// );
/// ```
pub fn show_spinner_dialog(
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
            callback(
                result.unwrap_or_else(|| Err("Thread disconnected unexpectedly.".to_string())),
            );
        }
    });

    dialog.present(Some(parent));
}
