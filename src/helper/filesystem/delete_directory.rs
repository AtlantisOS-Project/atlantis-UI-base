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
use std::path::Path;
use std::process;

/// Deletes an empty directory.
///
/// This is a deliberately simple delete command: it does **not** delete contents
/// recursively. Use
/// [`delete_files_with_parent`](crate::helper::filesystem::delete_files::delete_files_with_parent)
/// to clean up a directory with files in it.
///
/// # Process exit
///
/// If the directory cannot be deleted (it is not empty, does not exist or the permissions
/// are missing), the function prints an error to `stderr` and **terminates the whole
/// process** with status `1`. This suits cleanup steps of installers and command line
/// tools; do not use it where the application has to carry on after a failure — use
/// [`std::fs::remove_dir`] there.
///
/// # Arguments
///
/// * `path` - The directory to delete (`&str`, `String`, `PathBuf`, …).
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
/// delete_directory(&dir);
/// assert!(!dir.exists());
/// ```
pub fn delete_directory<P: AsRef<Path>>(path: P) {
    let path_ref = path.as_ref();

    match fs::remove_dir(path_ref) {
        Ok(_) => println!("Directory deleted: {:?}", path_ref),
        Err(e) => {
            eprintln!(
                "Error deleting directory {:?}: {} (Note: Only empty directories can be deleted.)",
                path_ref, e
            );
            process::exit(1);
        }
    }
}
