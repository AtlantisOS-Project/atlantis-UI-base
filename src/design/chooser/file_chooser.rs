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
use gtk4::{
	FileDialog, 
	Window, 
	Button
};
use std::path::PathBuf;
use crate::gettext;

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
/// # Example:
/// 
/// ```rust
/// // Example that without a box specification
/// let btn1 = Button::with_label("File?:");
/// // define callback
/// fn something(pfad: PathBuf) {
///     println!("File: {:?}", pfad);
/// }
/// // connect signal
/// btn1.connect_clicked(move |b| {
///     show_file_chooser(b, something);
/// });
///
/// // Example with boxing 
/// let scan_button = create_special_button::create_button_icon_position(
///    "scanner-symbolic",
///    "Scan QR-Code/Barcode",
///    Align::Center,
///    move |btn| {
///        // create the closure and box it
///        let callback = Box::new(something);
///        file_chooser::show_file_chooser(btn, callback);
///    }
/// );
///
/// ``` 
pub fn show_file_chooser<F>(button: &Button, process_func: F) 
where 
    F: Fn(PathBuf) + 'static 
{
    let root = button.root();
    let parent_window = root.and_then(|r| r.downcast::<Window>().ok());

    let dialog = FileDialog::new();
    dialog.set_title(&gettext!("Choose File"));
    
    // we wrap the function in a box internally so that it 
    // can be passed into the dialog's asynchronous `move` block.
    let boxed_func = Box::new(process_func);

    dialog.open(parent_window.as_ref(), gio::Cancellable::NONE, move |res| {
        match res {
            Ok(file) => {
                if let Some(path) = file.path() {
                    boxed_func(path);
                }
            }
            
            Err(err) => {
                eprintln!("Error opening the file: {}", err);
            }
        }
    });
}
