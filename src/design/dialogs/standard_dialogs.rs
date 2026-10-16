// standard_dialogs.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Ready-made dialogs for common cases.
//!
//! The functions of this module wrap
//! [`show_alert_dialog`] to show consistent information
//! and error messages. The fixed texts ("Information", "Error", "OK") are translated with
//! [`gettext!`](crate::gettext!). Texts that you pass in are shown as they are: translate
//! them yourself.

use super::dialog::show_alert_dialog;
use crate::gettext;
use adw::prelude::*;
use gtk4::Widget;

/// Shows an information dialog.
///
/// The title is "Information" and the button is "OK" (both translated).
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `body` - The message.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_info_dialog;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_info_dialog(&window, "The file was saved.");
/// ```
pub fn show_info_dialog(parent: &impl IsA<Widget>, body: &str) {
    show_alert_dialog(parent, &gettext!("Information"), body, &gettext!("OK"));
}

/// Shows an information dialog with your own button label.
///
/// Useful when "OK" does not fit, for example "Understood".
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `body` - The message.
/// * `button_label` - The label of the button (translate it yourself).
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_info_button_dialog;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_info_button_dialog(&window, "The update takes a few minutes.", "Understood");
/// ```
pub fn show_info_button_dialog(parent: &impl IsA<Widget>, body: &str, button_label: &str) {
    show_alert_dialog(parent, &gettext!("Information"), body, button_label);
}

/// Shows an information dialog with your own title and the button "OK".
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `title` - The heading (translate it yourself).
/// * `body` - The message.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_dialog_title;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_dialog_title(&window, "Backup", "The backup is complete.");
/// ```
pub fn show_dialog_title(parent: &impl IsA<Widget>, title: &str, body: &str) {
    show_alert_dialog(parent, title, body, &gettext!("OK"));
}

/// Shows a standard error dialog.
///
/// The title is "Error" and the button is "OK" (both translated). Use it for general
/// errors of the program.
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `body` - The description of the error.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_error_dialog;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_error_dialog(&window, "The file could not be opened.");
/// ```
pub fn show_error_dialog(parent: &impl IsA<Widget>, body: &str) {
    show_alert_dialog(parent, &gettext!("Error"), body, &gettext!("OK"));
}

/// Shows an error dialog with your own button label.
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `body` - The description of the error.
/// * `button_label` - The label of the button (translate it yourself).
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_error_button_dialog;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_error_button_dialog(&window, "The connection was lost.", "Close");
/// ```
pub fn show_error_button_dialog(parent: &impl IsA<Widget>, body: &str, button_label: &str) {
    show_alert_dialog(parent, &gettext!("Error"), body, button_label);
}

/// Shows an error dialog with a detail in the title.
///
/// The heading is "Error: *title*" (the word "Error" is translated), the button is "OK".
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `title` - The detail after "Error:", for example `"Network timeout"`.
/// * `body` - The description of the error.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_error_title_dialog;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// // the heading reads "Error: Network timeout"
/// show_error_title_dialog(&window, "Network timeout", "The server did not answer.");
/// ```
pub fn show_error_title_dialog(parent: &impl IsA<Widget>, title: &str, body: &str) {
    show_error_title_button_dialog(parent, title, body, &gettext!("OK"));
}

/// Shows an error dialog with a detail in the title and your own button label.
///
/// The most flexible of the error presets.
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `title` - The detail after "Error:".
/// * `body` - The description of the error.
/// * `button_label` - The label of the button (translate it yourself).
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_error_title_button_dialog;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_error_title_button_dialog(&window, "Disk full", "Free some space and try again.", "Close");
/// ```
pub fn show_error_title_button_dialog(
    parent: &impl IsA<Widget>,
    title: &str,
    body: &str,
    button_label: &str,
) {
    let formatted_title = format!("{}: {}", gettext!("Error"), title);
    show_alert_dialog(parent, &formatted_title, body, button_label);
}
