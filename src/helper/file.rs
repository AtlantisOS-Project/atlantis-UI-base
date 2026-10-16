// file.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Working with the contents of files.
//!
//! While the [`filesystem`](crate::helper::filesystem) module handles the structure
//! (directories, existence), this module works directly with the contents of files: from
//! writing and appending lines to cleaning up strings.

pub mod trim_and_clean;

pub mod get_config_value;

pub mod write_file;

pub mod add_line_file;

pub mod temp_app_file;

// Re-exports
pub use add_line_file::*;
pub use get_config_value::*;
pub use temp_app_file::*;
pub use trim_and_clean::*;
pub use write_file::*;
