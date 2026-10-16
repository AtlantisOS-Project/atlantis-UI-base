// folder_chooser.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Choosing a directory.
//!
//! The module wraps [`gtk4::FileDialog`] for choosing folders. The selection runs
//! asynchronously, so the user interface does not block.

use crate::gettext;
use gtk4::prelude::*;
use gtk4::{FileDialog, Widget, Window};
use std::path::PathBuf;

/// Opens a dialog to choose a directory.
///
/// Unlike the file chooser, only folders can be chosen. The window that contains `button`
/// becomes the parent. The call returns immediately; `process_func` is called later with
/// the path if the user chooses a folder. If the user cancels the dialog, nothing is
/// called. Other errors are printed to `stderr`.
///
/// # Arguments
///
/// * `button` - The widget that triggered the dialog; it is only used to find the parent
///   window.
/// * `process_func` - Called with the path of the chosen folder.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_folder_chooser;
/// use gtk4::prelude::*;
/// # gtk4::init().unwrap();
///
/// let button = gtk4::Button::with_label("Choose a folder");
/// button.connect_clicked(|button| {
///     show_folder_chooser(button, |path| println!("Folder: {:?}", path));
/// });
/// ```
pub fn show_folder_chooser<F>(button: &impl IsA<Widget>, process_func: F)
where
    F: Fn(PathBuf) + 'static,
{
    let parent_window = button
        .root()
        .and_then(|root| root.downcast::<Window>().ok());

    let dialog = FileDialog::new();
    dialog.set_title(&gettext!("Choose folder"));

    dialog.select_folder(
        parent_window.as_ref(),
        gio::Cancellable::NONE,
        move |res| match res {
            Ok(folder) => {
                if let Some(path) = folder.path() {
                    process_func(path);
                }
            }
            Err(err) if err.matches(gtk4::DialogError::Dismissed) => {}
            Err(err) => eprintln!("Error opening the folder: {}", err),
        },
    );
}
