// dialog_work.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! A dialog with a spinner while a closure works in the background.

use crate::design::dialogs::standard_dialogs::{show_error_dialog, show_success_dialog};
use crate::design::utils::background::run_background;
use adw::prelude::*;
use gtk4::Widget;

/// Shows a dialog with a spinner while `work` runs in a background thread, then closes it
/// and tells the user the result: a success dialog with the text of `Ok`, an error dialog
/// with the text of `Err`.
///
/// Use it for work that is not a command line (copying files, unpacking an archive). For
/// commands see [`show_spinner_dialog`](super::dialogs_spinner::show_spinner_dialog). The
/// dialog cannot be closed by the user while the work runs.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_work_dialog;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_work_dialog(&window, "Copying", "Please wait...", || {
///     std::fs::copy("/tmp/a", "/tmp/b").map_err(|error| error.to_string())?;
///     Ok("The file was copied.".to_string())
/// });
/// ```
pub fn show_work_dialog(
    parent: &impl IsA<Widget>,
    title: &str,
    message: &str,
    work: impl FnOnce() -> Result<String, String> + Send + 'static,
) {
    let dialog = adw::AlertDialog::builder()
        .heading(title)
        .body(message)
        .can_close(false)
        .build();
    let spinner = adw::Spinner::new();
    spinner.set_size_request(64, 64);
    dialog.set_extra_child(Some(&spinner));
    dialog.present(Some(parent));

    let parent: Widget = parent.upcast_ref::<Widget>().clone();
    run_background(work, move |result| {
        dialog.force_close();
        match result {
            Ok(text) => show_success_dialog(&parent, &text),
            Err(text) => show_error_dialog(&parent, &text),
        }
    });
}
