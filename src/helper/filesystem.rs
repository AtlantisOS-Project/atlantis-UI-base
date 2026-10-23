// filesystem.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! File system management.
//!
//! This module works with the directory structure: expanding paths, creating and deleting
//! directories, and searching for files.

pub mod delete_directory;

pub mod delete_files;

pub mod directory_exists;

pub mod expand_path;

pub mod guarded_delete;

pub mod home_dir;

pub mod make_path;

pub mod move_file;

pub mod search_file_directory;

// Re-exports
pub use delete_directory::*;
pub use delete_files::*;
pub use directory_exists::*;
pub use expand_path::*;
pub use guarded_delete::*;
pub use home_dir::*;
pub use make_path::*;
pub use move_file::*;
pub use search_file_directory::*;
