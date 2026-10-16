// utils.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Helpers for user interface development.
//!
//! Factory functions that create standardized widgets quickly: input fields, buttons with
//! icons, icon-label combinations, and progress dialogs for privileged commands.

pub mod create_entry;

pub mod create_label_icon;

pub mod create_special_button;

pub mod command_pkexec_ui;

// Re-exports
pub use command_pkexec_ui::*;
pub use create_entry::*;
pub use create_label_icon::*;
pub use create_special_button::*;
