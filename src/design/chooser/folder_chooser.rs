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
use gtk4::{
	FileDialog, 
	Window, 
	Button
};
use std::path::PathBuf;
use crate::gettext;

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
/// // Example that without a box specification
/// let folder_btn = Button::with_label("Folder?:");
/// fn handle_folder(path: PathBuf) {
///    println!("path: {:?}", path);
/// }
/// // connect button
/// folder_btn.connect_clicked(move |btn| {
///    show_folder_chooser(btn, handle_folder);
/// });
///
/// // Example with boxing 
/// let scan_button = create_special_button::create_button_icon_position(
///    "scanner-symbolic",
///    "Scan QR-Code/Barcode",
///    Align::Center,
///    move |btn| {
///        // create the closure and box it
///        let callback = Box::new(handle_folder);
///        file_chooser::show_folder_chooser(btn, callback);
///    }
/// ```
pub fn show_folder_chooser<F>(button: &Button, process_func: F) 
where 
    F: Fn(PathBuf) + 'static 
{
    // get the root window
    let root = button.root();
    let parent_window = root.and_then(|r| r.downcast::<Window>().ok());

    let dialog = FileDialog::new();
    dialog.set_title(&gettext!("Choose folder"));

    // we wrap the function in a box internally so that it 
    // can be passed into the dialog's asynchronous `move` block.
    let boxed_func = Box::new(process_func);

    dialog.select_folder(
        parent_window.as_ref(),
        gio::Cancellable::NONE,
        move |res| {
            match res {
                Ok(folder) => {
                    if let Some(path) = folder.path() {
                        boxed_func(path);
                    }
                }
                
                Err(err) => {
                    eprintln!("Error opening the folder: {}", err);
                }
            }
        },
    );
}
