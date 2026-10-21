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

use super::choose::{Pick, show_chooser};
use gtk4::Widget;
use gtk4::prelude::*;
use std::path::PathBuf;

/// Opens a native dialog to choose a file.
///
/// The window that contains `button` becomes the parent, so the dialog is attached to it as
/// a modal dialog. The call returns immediately; the application stays responsive and
/// `process_func` is called later with the path if the user chooses a file. If the user
/// cancels the dialog, nothing is called. Other errors are logged.
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
    show_chooser(button, Pick::File, None, move |paths| {
        if let Some(path) = paths.into_iter().next() {
            process_func(path);
        }
    });
}

/// Opens a native dialog to choose one or more files.
///
/// Like [`show_file_chooser`], but the user can select several files; `process_func` is
/// called with all of them.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_files_chooser;
/// use gtk4::prelude::*;
/// # gtk4::init().unwrap();
///
/// let button = gtk4::Button::with_label("Choose images");
/// button.connect_clicked(|button| {
///     show_files_chooser(button, |paths| println!("{} files", paths.len()));
/// });
/// ```
pub fn show_files_chooser<F>(button: &impl IsA<Widget>, process_func: F)
where
    F: Fn(Vec<PathBuf>) + 'static,
{
    show_chooser(button, Pick::Files, None, process_func);
}
