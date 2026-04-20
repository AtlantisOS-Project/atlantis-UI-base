//! Utility for removing empty directories.
//!
//! This module provides a simple interface for deleting directories. 
//! Since it uses `std::fs::remove_dir`, the operation will fail as a safety measure 
//! if the directory is not empty.
/**
* delete_directory.rs
*
* (C) 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use std::fs;
use std::path::Path;
use std::process;

/// Deletes a directory from the file system.
///
/// This function is designed as a "simple" delete command. It **does not** 
/// delete contents recursively. If the folder contains files or subfolders, an 
/// error is returned and the program terminates.
///
/// # Safety & Error Handling
/// - **Program Termination:** In case of an error (e.g., missing permissions or 
///   non-empty folder), the function outputs an error message to `stderr` 
///   and terminates the entire process with status `1`.
/// - **Generic paths:** Accepts anything that implements `AsRef<Path>` (strings, PathBuf, etc.).
///
/// # Arguments
/// * `path` - The path to the directory to be deleted.
///
/// # Usage:
///
/// ```rust
/// fn main() {
///    delete_directory("/tmp/test_dir");
/// }
/// ```
pub fn delete_directory<P: AsRef<Path>>(path: P) {
    let path_ref = path.as_ref();

    match fs::remove_dir(path_ref) {
        Ok(_) => {
            println!("Directory deleted: {:?}", path_ref);
        }
        Err(e) => {
            eprintln!(
                "Error deleting directory {:?}: {} (Note: Only empty directories can be deleted.)",
                path_ref, e
            );
            // since this is often a critical cleanup step, we exit if errors occur
            process::exit(1);
        }
    }
}
