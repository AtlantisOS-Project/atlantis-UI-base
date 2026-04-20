//! Utility tools for displaying Libadwaita alert dialogs.
//!
//! This module wraps the creation of `adw::AlertDialog` to display simple 
//! notifications or confirmations with minimal boilerplate code.
/**
* dialog.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* @brief Show adw alert dialog
*
* Usage:
* let my_button = gtk4::Button::with_label("Click me");
* my_button.connect_clicked(move |btn| {
*    show_alert_dialog(
*        btn, 
*        "System Update", 
*        "New Update is there.", 
*        "OK"
*    );
* });
*/

use adw::prelude::*;
use adw::{AlertDialog, ResponseAppearance};
use gtk4::Widget;

/// Displays a standard Libadwaita alert dialog.
///
/// An alert dialog is ideal for short messages that require the user's attention 
/// without requiring complex input.
///
/// # How it works
/// - The dialog uses a **Heading** and a **Body**.
/// - A single response button is added.
/// - The button is automatically marked as `Suggested` (highlighted),
///   as this is typically the expected user action.
///
/// # Arguments
///
/// * `parent` - The widget (usually a button or window) to which the dialog is attached.
/// * `title` - The concise heading of the dialog (Heading).
/// * `body` - The more detailed informational text (Body).
/// * `button_label` - The label of the confirmation button (e.g., “Understood” or “OK”).
///
/// # Usage:
///
/// ```rust
/// let my_button = gtk4::Button::with_label("Click me");
/// my_button.connect_clicked(move |btn| {
///    show_alert_dialog(
///        btn, 
///        "System Update", 
///        "New Update is there.", 
///        "OK"
///    );
/// });
/// ```
pub fn show_alert_dialog(
    parent: &impl IsA<Widget>, 
    title: &str, 
    body: &str, 
    button_label: &str
) {
    // create a new dialog
    let dialog = AlertDialog::builder()
        .heading(title)
        .body(body)
        .build();

    // add response
    dialog.add_response("ok", button_label);

    // set the style of the button
    dialog.set_response_appearance("ok", ResponseAppearance::Suggested);

    // response to close event for the dialog
    dialog.set_close_response("ok");

    // show dialog
    dialog.present(Some(parent));
}
