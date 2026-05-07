//! # Custom Dialog Module
//!
//! This module provides helper functions for displaying user interface dialogs 
//! using `libadwaita` and `GTK4`.
//!
//! The focus is on providing flexible `AlertDialog` wrappers that 
//! can accommodate custom content (`GtkBox`).
/**
* dialog_custom.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/
use adw::prelude::*;
use adw::{AlertDialog, ResponseAppearance};
use gtk4::{Widget, Box as GtkBox};

/// Displays an `AlertDialog` with a custom content area.
///
/// This function abstracts the boilerplate setup for a libadwaita dialog. 
/// The caller is responsible for preparing the `GtkBox` with the desired widgets 
/// (e.g., images, labels, or progress bars).
///
/// # Arguments
///
/// * `parent` - The parent widget (usually the main window) to which the dialog is docked.
/// * `title` - The bolded title of the dialog.
/// * `body` - A short descriptive text below the title.
/// * `button_label` - The text for the primary confirmation button.
/// * `content` - A reference to a `GtkBox` that is displayed as additional content (`extra_child`).
///
/// # Example
///
/// ```rust
/// use gtk::prelude::*;
/// use gtk::{Box as GtkBox, Orientation, Label};
/// 
/// // Preparing the content
/// let content_box = GtkBox::builder()
///     .orientation(Orientation::Vertical)
///     .spacing(10)
///     .build();
/// 
/// content_box.append(&Label::new(Some("Do you want to continue?")));
///
/// // Display the dialog
/// show_custom_content_dialog(
///     &window,
///     "System Update",
///     "A new update is available.",
///     "Install",
///     &content_box
/// );
/// ```
pub fn show_custom_content_dialog(
    parent: &impl IsA<Widget>,
    title: &str,
    body: &str,
    button_label: &str,
    content: &GtkBox,
) {
    let dialog = AlertDialog::builder()
        .heading(title)
        .body(body)
        .build();

    // sets the passed box as additional content in the dialog
    dialog.set_extra_child(Some(content));

    // config for standard response
    dialog.add_response("ok", button_label);
    dialog.set_response_appearance("ok", ResponseAppearance::Suggested);
    dialog.set_close_response("ok");

    // show the dialog
    dialog.present(Some(parent));
}
