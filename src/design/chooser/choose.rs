// choose.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The one function behind all selection dialogs.
//!
//! [`show_file_chooser`](super::file_chooser::show_file_chooser),
//! [`show_files_chooser`](super::file_chooser::show_files_chooser),
//! [`show_folder_chooser`](super::folder_chooser::show_folder_chooser) and the file rows of a
//! [`FormDialog`](crate::design::form::FormDialog) all call [`show_chooser`].

use crate::gettext;
use gtk4::prelude::*;
use gtk4::{FileDialog, Widget, Window};
use std::path::{Path, PathBuf};

/// What a selection dialog lets the user choose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pick {
    /// One file.
    File,
    /// Several files.
    Files,
    /// A folder.
    Folder,
}

/// Opens a native dialog to choose a file, several files or a folder.
///
/// The window that contains `widget` becomes the parent. The call returns immediately;
/// `on_chosen` is called later with what the user chose (a list with one path for
/// [`Pick::File`] and [`Pick::Folder`]). If the user cancels the dialog, nothing is called;
/// other errors are logged. `start_in` is the folder the dialog opens in, if it exists.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::{Pick, show_chooser};
/// use gtk4::prelude::*;
/// # gtk4::init().unwrap();
///
/// let button = gtk4::Button::with_label("Choose images");
/// button.connect_clicked(|button| {
///     show_chooser(button, Pick::Files, None, |paths| println!("{} files", paths.len()));
/// });
/// ```
pub fn show_chooser(
    widget: &impl IsA<Widget>,
    pick: Pick,
    start_in: Option<&Path>,
    on_chosen: impl Fn(Vec<PathBuf>) + 'static,
) {
    let parent = widget
        .root()
        .and_then(|root| root.downcast::<Window>().ok());
    let dialog = FileDialog::new();
    if let Some(dir) = start_in.filter(|dir| dir.is_dir()) {
        dialog.set_initial_folder(Some(&gio::File::for_path(dir)));
    }

    // what the user chose, or the reason why nothing was chosen
    fn files(result: Result<Vec<gio::File>, glib::Error>, on_chosen: impl Fn(Vec<PathBuf>)) {
        match result {
            Ok(files) => on_chosen(
                files
                    .iter()
                    .filter_map(|file| {
                        let path = file.path();
                        if path.is_none() {
                            // a file on a server, for example: there is no path to give
                            log::warn!("{} is not a local file and is ignored", file.uri());
                        }
                        path
                    })
                    .collect(),
            ),
            Err(error) if error.matches(gtk4::DialogError::Dismissed) => {}
            Err(error) => log::warn!("the file dialog failed: {error}"),
        }
    }

    match pick {
        Pick::File => {
            dialog.set_title(&gettext!("Choose File"));
            dialog.open(parent.as_ref(), gio::Cancellable::NONE, move |result| {
                files(result.map(|file| vec![file]), on_chosen)
            });
        }
        Pick::Files => {
            dialog.set_title(&gettext!("Choose File"));
            dialog.open_multiple(parent.as_ref(), gio::Cancellable::NONE, move |result| {
                files(
                    result.map(|list| list.iter::<gio::File>().flatten().collect()),
                    on_chosen,
                )
            });
        }
        Pick::Folder => {
            dialog.set_title(&gettext!("Choose folder"));
            dialog.select_folder(parent.as_ref(), gio::Cancellable::NONE, move |result| {
                files(result.map(|folder| vec![folder]), on_chosen)
            });
        }
    }
}
