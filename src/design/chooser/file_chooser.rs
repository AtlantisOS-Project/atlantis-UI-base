// file_chooser.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Choosing a file.
//!
//! The module simplifies [`gtk4::FileDialog`]: it hides the asynchronous handling and
//! passes the chosen path to a callback.

use crate::gettext;
use gtk4::prelude::*;
use gtk4::{FileDialog, Widget, Window};
use std::path::PathBuf;

/// Opens a native dialog to choose a file.
///
/// The window that contains `button` becomes the parent, so the dialog is attached to it as
/// a modal dialog. The call returns immediately; the application stays responsive and
/// `process_func` is called later with the path if the user chooses a file. If the user
/// cancels the dialog, nothing is called. Other errors are printed to `stderr`.
///
/// # Arguments
///
/// * `button` - The widget that triggered the dialog; it is only used to find the parent
///   window.
/// * `process_func` - Called with the path of the chosen file.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_file_chooser;
/// use gtk4::prelude::*;
/// # gtk4::init().unwrap();
///
/// let button = gtk4::Button::with_label("Choose a file");
/// button.connect_clicked(|button| {
///     show_file_chooser(button, |path| println!("File: {:?}", path));
/// });
/// ```
pub fn show_file_chooser<F>(button: &impl IsA<Widget>, process_func: F)
where
    F: Fn(PathBuf) + 'static,
{
    let parent_window = button
        .root()
        .and_then(|root| root.downcast::<Window>().ok());

    let dialog = FileDialog::new();
    dialog.set_title(&gettext!("Choose File"));

    dialog.open(
        parent_window.as_ref(),
        gio::Cancellable::NONE,
        move |res| match res {
            Ok(file) => {
                if let Some(path) = file.path() {
                    process_func(path);
                }
            }
            Err(err) if err.matches(gtk4::DialogError::Dismissed) => {}
            Err(err) => eprintln!("Error opening the file: {}", err),
        },
    );
}
