// search_string_name.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Identifies configuration and source files by their names.
//!
//! The module is mainly used to find out whether a repository entry (for example a PPA) is
//! stored on the system.

use std::fs;
use std::path::Path;

/// Searches a directory for a file whose name contains a string.
///
/// Within AtlantisOS this validates the installation source of an application, for example
/// by searching for `"fastboot-assistant"` in the APT sources. In contrast to
/// [`search_file_directory`](crate::helper::filesystem::search_file_directory::search_file_directory)
/// a directory that does not exist is not an error.
///
/// # Arguments
///
/// * `directory` - The path of the directory, for example `/etc/apt/sources.list.d`.
/// * `search` - The text to look for in the file names.
///
/// # Returns
///
/// * `Ok(true)` — a matching file was found.
/// * `Ok(false)` — no file matches, or the directory does not exist.
///
/// # Errors
///
/// Returns an [`std::io::Error`] if the directory exists but cannot be read.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::search_string_file;
///
/// let dir = tempfile::tempdir().unwrap();
/// std::fs::write(dir.path().join("fastboot-assistant.list"), "").unwrap();
/// let dir = dir.path().to_str().unwrap();
///
/// assert!(search_string_file(dir, "fastboot-assistant").unwrap());
/// assert!(!search_string_file(dir, "other-app").unwrap());
/// assert!(!search_string_file("/this/path/does/not/exist", "x").unwrap());
/// ```
pub fn search_string_file(directory: &str, search: &str) -> Result<bool, std::io::Error> {
    let path = Path::new(directory);
    if !path.exists() {
        return Ok(false);
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str()
            && name.contains(search)
        {
            return Ok(true);
        }
    }
    Ok(false)
}
