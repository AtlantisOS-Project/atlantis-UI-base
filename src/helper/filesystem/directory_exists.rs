// directory_exists.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Checks whether files and directories exist.
//!
//! Two small helpers that tell whether a path exists and whether it is of the expected
//! type.

use std::path::Path;

/// Checks whether a path exists and is a directory.
///
/// Stricter than a plain existence check: a regular file at that path gives `false`.
///
/// # Arguments
///
/// * `path` - The path to check (`&str`, `String`, `PathBuf`, …).
///
/// # Examples
///
/// ```
/// use atlbase::prelude::directory_exists;
///
/// assert!(directory_exists("/"));
/// assert!(!directory_exists("/this/path/does/not/exist"));
/// ```
pub fn directory_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_dir()
}

/// Checks whether a file or directory exists at the path.
///
/// Unlike [`directory_exists`] the type does not matter, only the presence on disk.
///
/// # Arguments
///
/// * `path` - The path to check.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::file_exists;
///
/// assert!(file_exists("/"));
/// assert!(!file_exists("/this/path/does/not/exist"));
/// ```
pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}
