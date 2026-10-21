// dialog_work.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! A dialog with a spinner while a closure works in the background.

use crate::design::dialogs::dialogs_spinner::{IndicatorType, run_with_progress};
use crate::design::dialogs::standard_dialogs::{show_error_dialog, show_success_dialog};
use crate::gettext;
use gtk4::Widget;
use gtk4::prelude::*;

/// Shows a progress dialog with a spinner while `work` runs in a background thread, then closes
/// it and tells the user the result: a success dialog with the text of `Ok`, an error dialog
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
    let target: Widget = parent.upcast_ref::<Widget>().clone();
    run_with_progress(
        parent,
        title,
        message,
        IndicatorType::Spinner,
        work,
        move |result| match result {
            Some(Ok(text)) => show_success_dialog(&target, &text),
            Some(Err(text)) => show_error_dialog(&target, &text),
            None => show_error_dialog(&target, &gettext!("The work ended unexpectedly.")),
        },
    );
}
