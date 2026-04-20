//! Abstraction for file selection.
//!
//! This module simplifies working with `gtk4::FileDialog` by encapsulating the 
//! asynchronous logic and providing a simple interface for file paths.
/**
* file_chooser.rs
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

/// Type alias for functions that process a selected file path.
pub type FileProcessorFunc = fn(PathBuf);

/// Internal handler that evaluates the asynchronous result of the dialog.
///
/// Extracts the path from the `gio::File` instance and passes it to the 
/// provided processing function.
fn handle_file_response(
    res: Result<gio::File, glib::Error>, 
    process_func: FileProcessorFunc
) {
    match res {
        Ok(file) => {
            if let Some(path) = file.path() {
                process_func(path);
            }
        }
        Err(err) => {
            eprintln!("Error opening the file: {}", err);
        }
    }
}

/// Opens a native file selection dialog.
///
/// The function automatically determines the parent window of the passed button
/// to anchor the dialog correctly (modally).
///
/// # Arguments
/// * `button` - The button that triggered the dialog (used to determine the main window).
/// * `process_func` - A function or function pointer that is called as soon as a file has been selected.
///
/// # Functionality
/// The dialog is launched asynchronously. The program continues to run during the selection, 
/// and the result is processed internally before being passed to `process_func`.
///
/// # Usage:
///
/// ```rust
/// let btn1 = Button::with_label("File?:");
/// // define callback
/// fn something(pfad: PathBuf) {
///     println!("File: {:?}", pfad);
/// }
/// // connect signal
/// btn1.connect_clicked(move |b| {
///     show_file_chooser(b, something);
/// });
/// ``` 
pub fn show_file_chooser(button: &Button, process_func: FileProcessorFunc) {
    let root = button.root();
    let parent_window = root.and_then(|r| r.downcast::<Window>().ok());

    let dialog = FileDialog::new();
    dialog.set_title(&gettext!("Choose File"));
    
    dialog.open(parent_window.as_ref(), gio::Cancellable::NONE, move |res| {
        handle_file_response(res, process_func);
    });
}
