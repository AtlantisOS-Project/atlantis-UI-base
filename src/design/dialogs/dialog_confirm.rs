// dialog_confirm.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! A question before something cannot be undone.

use crate::gettext;
use adw::prelude::*;
use gtk4::Widget;

/// Asks the user to confirm an action that cannot be undone (deleting, formatting, flashing).
///
/// The dialog has the buttons "Cancel" (the default, also for Escape) and `confirm_label`
/// in red. `on_confirm` runs only if the user presses the red button.
///
/// The texts are shown as they are: translate them yourself; only "Cancel" is translated by
/// the library.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_confirm_dialog;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_confirm_dialog(
///     &window,
///     "Delete all backups?",
///     "The backups cannot be restored.",
///     "Delete",
///     || println!("deleting..."),
/// );
/// ```
pub fn show_confirm_dialog(
    parent: &impl IsA<Widget>,
    heading: &str,
    body: &str,
    confirm_label: &str,
    on_confirm: impl Fn() + 'static,
) {
    let dialog = adw::AlertDialog::builder()
        .heading(heading)
        .body(body)
        .build();
    dialog.add_response("cancel", &gettext!("Cancel"));
    dialog.add_response("confirm", confirm_label);
    dialog.set_response_appearance("confirm", adw::ResponseAppearance::Destructive);
    dialog.set_default_response(Some("cancel"));
    dialog.set_close_response("cancel");
    dialog.connect_response(None, move |_, response| {
        if response == "confirm" {
            on_confirm();
        }
    });
    dialog.present(Some(parent));
}
