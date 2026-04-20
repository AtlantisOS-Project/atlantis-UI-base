//! Simple checks for the existence of files and directories.
//!
//! This module provides lightweight utilities to determine whether a path 
//! exists on the file system and whether it is of the expected type.
/**
 * directory_exists.rs
 *
 * (C) 2026 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 */

use std::path::Path;

/// Checks whether a path exists and is a directory.
///
/// This function is more restrictive than a simple existence check, as it returns `false` 
/// if the path exists but is a regular file.
///
/// # Arguments
/// * `path` - The path to check (accepts `&str`, `String`, `PathBuf`).
///
/// # Usage:
///
/// ```rust
/// fn main() {
///   if directory_exists("/mnt/c/Users") {
///       println!("Directory exists!");
///   }
/// }
/// ```
pub fn directory_exists<P: AsRef<Path>>(path: P) -> bool {
    let p = path.as_ref();
    p.exists() && p.is_dir()
}

/// Checks whether a file or directory exists at the specified path.
///
/// Unlike `directory_exists`, this function only checks for 
/// physical presence on the disk, regardless of file type.
///
/// # Arguments
/// * `path` - The path to be checked.
///
/// # Usage:
///
/// ```rust
/// Usage:
/// fn main() {
///   if directory_exists("/mnt/c/Users") {
///       println!("Directory exists!");
///   }
///   if file_exists("/etc/config.toml") {
///       println!("File exists!");
///   }
/// }
/// ```
pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}
