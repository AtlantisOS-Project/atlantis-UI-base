/**
* folder_chooser.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* let folder_btn = Button::with_label("Folder?:");
* fn handle_folder(path: PathBuf) {
*    println!("path: {:?}", path);
* }
* // connect button
* folder_btn.connect_clicked(move |btn| {
*    show_folder_chooser(btn, handle_folder);
* });
*/

use gtk4::prelude::*;
use gtk4::{FileDialog, Window, Button};
use std::path::PathBuf;
use crate::gettext;

type FolderProcessorFunc = fn(PathBuf);

/**
* @brief Callback for closing the dialog
*/
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

/**
* @brief Function for the folderchooser dialog
*/
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
