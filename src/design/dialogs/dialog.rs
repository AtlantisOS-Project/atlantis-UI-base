//! Create a Libadwaita Alert dialog
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

/// Show a new adw alert dialog
/// ### Usage:
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
