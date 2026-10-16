// dialogs.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Dialogs for AtlantisOS applications.
//!
//! The module offers graphical dialogs based on `libadwaita` and `gtk4`: from simple
//! messages to task dialogs with a progress indicator.
//!
//! Text that the dialogs show on their own ("OK", "Error", …) is translated with
//! [`gettext!`](crate::gettext!); see [`helper::language`](crate::helper::language).

pub mod dialog;

pub mod about_dialog;

pub mod dialog_entry;

pub mod dialogs_spinner;

pub mod standard_dialogs;

pub mod dialog_function;

pub mod image_dialog;

pub mod dialog_custom;

pub mod dialog_spinner_return;

// Re-exports
pub use about_dialog::*;
pub use dialog::*;
pub use dialog_custom::*;
pub use dialog_entry::*;
pub use dialog_function::*;
pub use dialog_spinner_return::*;
pub use dialogs_spinner::*;
pub use image_dialog::*;
pub use standard_dialogs::*;
