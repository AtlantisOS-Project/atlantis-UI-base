/**
* file_chooser.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* let btn1 = Button::with_label("File?:");
* // define callback
* fn something(pfad: PathBuf) {
*     println!("File: {:?}", pfad);
* }
* // connect signal
* btn1.connect_clicked(move |b| {
*     show_file_chooser(b, something);
* });
*/

use gtk4::prelude::*;
use gtk4::{FileDialog, Window, Button};
use std::path::PathBuf;
use crate::gettext;

pub type FileProcessorFunc = fn(PathBuf);

/**
* @brief Callback for closing the dialog
*/
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

/**
* @brief Function for the filechooser dialog
*/
pub fn show_file_chooser(button: &Button, process_func: FileProcessorFunc) {
    let root = button.root();
    let parent_window = root.and_then(|r| r.downcast::<Window>().ok());

    let dialog = FileDialog::new();
    dialog.set_title(&gettext!("Choose File"));
    
    dialog.open(parent_window.as_ref(), gio::Cancellable::NONE, move |res| {
        handle_file_response(res, process_func);
    });
}
