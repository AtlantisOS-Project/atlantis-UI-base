//! Abstraction for directory selection.
//!
//! This module encapsulates the logic of the `gtk4::FileDialog` for selecting folders.
//! It ensures that the selection process is asynchronous and that the UI
//! does not block during this time.
/**
* folder_chooser.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use gtk4::prelude::*;
use gtk4::{FileDialog, Window, Button};
use std::path::PathBuf;
use crate::gettext;

/// Type alias for functions that process a selected directory path.
pub type FolderProcessorFunc = fn(PathBuf);

/// Internal handler for the result of the directory selection.
///
/// This function evaluates the asynchronous result, extracts the system path,
/// and passes it to the provided [FolderProcessorFunc].
fn handle_folder_response(
    res: Result<gio::File, glib::Error>, 
    process_func: FolderProcessorFunc
) {
    match res {
        Ok(folder) => {
            if let Some(path) = folder.path() {
                process_func(path);
            }
        }
        Err(err) => {
            eprintln!("Error opening the folder: {}", err);
        }
    }
}

/// Opens a dialog for selecting a directory.
///
/// Unlike the file selection dialog, this dialog allows only the selection
/// of folders. It is displayed modally over the window of the button that called it.
///
/// # Arguments
/// * `button` - The trigger for the dialog; used to determine the parent window.
/// * `process_func` - The callback that is called with the [PathBuf] after a successful selection.
///
/// # Functionality
/// Internally uses `FileDialog::select_folder`. The result is processed asynchronously, 
/// so that the application can continue to respond to user interactions.
///
/// # Usage:
///
/// ```rust
/// let folder_btn = Button::with_label("Folder?:");
/// fn handle_folder(path: PathBuf) {
///    println!("path: {:?}", path);
/// }
/// // connect button
/// folder_btn.connect_clicked(move |btn| {
///    show_folder_chooser(btn, handle_folder);
/// });
/// ```
pub fn show_folder_chooser(button: &Button, process_func: FolderProcessorFunc) {
    // get the root window
    let root = button.root();
    let parent_window = root.and_then(|r| r.downcast::<Window>().ok());

    let dialog = FileDialog::new();
    dialog.set_title(&gettext!("Choose folder"));

    dialog.select_folder(
        parent_window.as_ref(),
        gio::Cancellable::NONE,
        move |res| {
            handle_folder_response(res, process_func);
        },
    );
}
