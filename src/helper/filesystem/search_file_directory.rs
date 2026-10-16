// search_file_directory.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Searches the file names of a directory.
//!
//! The module looks for a substring in the names of the files of one directory. It is
//! useful to check whether files exist whose full names are not known exactly.

use std::fs;
use std::path::Path;

/// Checks whether the name of a file in a directory contains a search term.
///
/// The directory is scanned flat (not recursively). Names that are not valid UTF-8 are
/// skipped.
///
/// # Arguments
///
/// * `directory` - The directory to search (`&str`, `PathBuf`, …).
/// * `search_string` - The text to look for in the file names.
///
/// # Returns
///
/// * `Ok(true)` — at least one entry contains the search term in its name.
/// * `Ok(false)` — no entry matches.
///
/// # Errors
///
/// Returns an [`std::io::Error`] if the directory cannot be read (for example because it
/// does not exist).
///
/// # Examples
///
/// ```
/// use atlbase::prelude::search_file_directory;
///
/// let dir = tempfile::tempdir().unwrap();
/// std::fs::write(dir.path().join("atlantis-sources.list"), "").unwrap();
///
/// assert!(search_file_directory(dir.path(), "atlantis").unwrap());
/// assert!(!search_file_directory(dir.path(), "debian").unwrap());
/// assert!(search_file_directory("/this/path/does/not/exist", "x").is_err());
/// ```
pub fn search_file_directory<P: AsRef<Path>>(
    directory: P,
    search_string: &str,
) -> Result<bool, std::io::Error> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str()
            && name.contains(search_string)
        {
            return Ok(true);
        }
    }

    Ok(false)
}
