// dialog_custom.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! An alert dialog with custom content.
//!
//! The module provides a flexible wrapper around [`adw::AlertDialog`] that can show your
//! own widgets in a [`gtk4::Box`].

use adw::prelude::*;
use adw::{AlertDialog, ResponseAppearance};
use gtk4::{Box as GtkBox, Widget};

/// Shows an alert dialog with an area for custom widgets.
///
/// The function hides the boilerplate of the dialog. You prepare a [`gtk4::Box`] with the
/// widgets you want (images, labels, progress bars, …); it is shown below the text as the
/// *extra child*. The button works as in [`show_alert_dialog`](super::dialog::show_alert_dialog).
///
/// # Arguments
///
/// * `parent` - The widget (usually the window) the dialog is shown over.
/// * `title` - The heading of the dialog.
/// * `body` - A short text below the heading.
/// * `button_label` - The label of the button.
/// * `content` - The box that is shown as additional content.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_custom_content_dialog;
/// use gtk4::{Box as GtkBox, Label, Orientation, prelude::*};
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// let content_box = GtkBox::builder()
///     .orientation(Orientation::Vertical)
///     .spacing(10)
///     .build();
/// content_box.append(&Label::new(Some("Do you want to continue?")));
///
/// show_custom_content_dialog(&window, "System update", "A new update is available.", "Install", &content_box);
/// ```
pub fn show_custom_content_dialog(
    parent: &impl IsA<Widget>,
    title: &str,
    body: &str,
    button_label: &str,
    content: &GtkBox,
) {
    let dialog = AlertDialog::builder().heading(title).body(body).build();

    dialog.set_extra_child(Some(content));

    dialog.add_response("ok", button_label);
    dialog.set_response_appearance("ok", ResponseAppearance::Suggested);
    dialog.set_close_response("ok");

    dialog.present(Some(parent));
}
