// dialog_function.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Dialogs for background tasks written in Rust.
//!
//! The module runs a time-consuming function in a background thread while a dialog that
//! cannot be closed shows a progress indicator.

use super::dialogs_spinner::{IndicatorType, build_progress_dialog, close_dialog_when_done};
use adw::prelude::*;
use gtk4::Widget;
use std::sync::mpsc;
use std::thread;

/// Shows a modal dialog while a Rust function runs in the background.
///
/// The dialog blocks the interaction with the parent window until the task is done. Then
/// it closes automatically. The task runs in its own thread, so it must be [`Send`] and
/// must not touch GTK widgets; its result (`true` for success) is currently only used to
/// end the dialog.
///
/// # Arguments
///
/// * `parent` - The widget (usually the main window) the dialog is shown over.
/// * `title` - The title of the dialog.
/// * `message` - An explanation for the user.
/// * `indicator` - The kind of indicator ([`IndicatorType`]).
/// * `task` - The function to run in the background.
///
/// # Examples
///
/// With a closure:
///
/// ```no_run
/// use atlbase::ui_prelude::{IndicatorType, show_task_dialog};
/// use gtk4::prelude::*;
/// # gtk4::init().unwrap();
/// # let window = adw::ApplicationWindow::builder().build();
///
/// let button = gtk4::Button::with_label("Calculate");
/// button.connect_clicked(move |_| {
///     show_task_dialog(&window, "Calculating", "Complex data...", IndicatorType::Spinner, || {
///         std::thread::sleep(std::time::Duration::from_secs(3));
///         true
///     });
/// });
/// ```
///
/// With an existing function; the values are moved into the closure:
///
/// ```no_run
/// use atlbase::ui_prelude::{IndicatorType, show_task_dialog};
/// # gtk4::init().unwrap();
/// # let window = adw::ApplicationWindow::builder().build();
///
/// fn long_running(value1: &str, value2: &str) -> bool {
///     println!("{} {}", value1, value2);
///     true
/// }
///
/// let (first, second) = ("Content".to_string(), "Other content".to_string());
/// show_task_dialog(&window, "System update", "Please wait...", IndicatorType::ProgressBar, move || {
///     long_running(&first, &second)
/// });
/// ```
pub fn show_task_dialog<F>(
    parent: &impl IsA<Widget>,
    title: &str,
    message: &str,
    indicator: IndicatorType,
    task: F,
) where
    F: FnOnce() -> bool + Send + 'static,
{
    let dialog = build_progress_dialog(title, message, indicator);

    let (tx, rx) = mpsc::channel::<bool>();
    thread::spawn(move || {
        let _ = tx.send(task());
    });

    close_dialog_when_done(&dialog, rx, |_success| {});

    dialog.present(Some(parent));
}
