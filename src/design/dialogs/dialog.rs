// dialog.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The Libadwaita alert dialog.
//!
//! The module wraps the creation of an [`adw::AlertDialog`] to show short messages or
//! confirmations with very little code.

use adw::prelude::*;
use adw::{AlertDialog, ResponseAppearance};
use gtk4::Widget;

/// Shows an alert dialog with a heading, a text and one button.
///
/// An alert dialog suits short messages that need the attention of the user but no
/// complex input. The button has the response id `"ok"`, is highlighted as the suggested
/// action, and is also the response when the dialog is dismissed (Escape key).
///
/// # Arguments
///
/// * `parent` - The widget (usually the window or a button in it) the dialog is shown over.
/// * `title` - The short heading of the dialog.
/// * `body` - The detailed text.
/// * `button_label` - The label of the button, for example `"OK"`.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_alert_dialog;
/// use gtk4::prelude::*;
/// # gtk4::init().unwrap();
///
/// let button = gtk4::Button::with_label("Click me");
/// button.connect_clicked(|button| {
///     show_alert_dialog(button, "System update", "A new update is available.", "OK");
/// });
/// ```
pub fn show_alert_dialog(parent: &impl IsA<Widget>, title: &str, body: &str, button_label: &str) {
    let dialog = AlertDialog::builder().heading(title).body(body).build();

    dialog.add_response("ok", button_label);
    dialog.set_response_appearance("ok", ResponseAppearance::Suggested);
    dialog.set_close_response("ok");

    dialog.present(Some(parent));
}
