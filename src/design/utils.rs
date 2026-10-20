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

pub mod background;

pub mod create_entry;

pub mod create_label_icon;

pub mod create_special_button;

pub mod command_pkexec_ui;

pub mod menu_grid;

pub mod page_layout;

pub mod stack_pages;

pub mod stack_window;

pub mod status_row;

// Re-exports
pub use background::*;
pub use command_pkexec_ui::*;
pub use create_entry::*;
pub use create_label_icon::*;
pub use create_special_button::*;
pub use menu_grid::*;
pub use page_layout::*;
pub use stack_pages::*;
pub use stack_window::*;
pub use status_row::*;
