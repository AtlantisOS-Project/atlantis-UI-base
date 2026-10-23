// delete_directory.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Removes empty directories.
//!
//! The module offers a simple interface for deleting a directory. It uses
//! [`std::fs::remove_dir`], so the operation fails as a safety measure if the directory
//! is not empty.

use std::fs;
use std::io;
use std::path::Path;

/// Deletes an empty directory.
///
/// This is a deliberately simple delete command: it does **not** delete contents
/// recursively. Use
/// [`delete_files_with_parent`](crate::helper::filesystem::delete_files::delete_files_with_parent)
/// to clean up a directory with files in it.
///
/// # Arguments
///
/// * `path` - The directory to delete (`&str`, `String`, `PathBuf`, …).
///
/// # Errors
///
/// Returns the [`io::Error`] of [`std::fs::remove_dir`] if the directory cannot be deleted: it is
/// not empty (`DirectoryNotEmpty`), does not exist (`NotFound`) or the permissions are missing
/// (`PermissionDenied`). The process is not ended: the caller decides what to do.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::delete_directory;
///
/// let parent = tempfile::tempdir().unwrap();
/// let dir = parent.path().join("empty");
/// std::fs::create_dir(&dir).unwrap();
///
/// delete_directory(&dir).unwrap();
/// assert!(!dir.exists());
/// // a directory that is not there is an error, not the end of the program
/// assert!(delete_directory(&dir).is_err());
/// ```
pub fn delete_directory<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    fs::remove_dir(path)?;
    println!("Directory deleted: {:?}", path);
    Ok(())
}
