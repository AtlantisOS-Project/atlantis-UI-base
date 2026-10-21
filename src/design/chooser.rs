// chooser.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Selection dialogs for the file system.
//!
//! The module bundles dialogs for choosing files and directories. It hides the asynchronous
//! calls of GTK4 and offers one consistent interface for the whole user interface.
//!
//! # Packaging
//!
//! The dialogs are based on `gtk4::FileDialog`, which uses the *FileChooser* portal when
//! the application runs in a Flatpak or Snap sandbox. No extra permissions are required.
//! Note that inside a Flatpak the returned path may point to the document portal
//! (`/run/user/<uid>/doc/…`), which is valid for the lifetime of the application.

pub mod choose;

pub mod file_chooser;

pub mod folder_chooser;

// Re-exports
pub use choose::*;
pub use file_chooser::*;
pub use folder_chooser::*;
