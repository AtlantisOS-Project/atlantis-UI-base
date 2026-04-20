//! Automated creation of directory structures.
//!
//! This module provides functions to ensure that paths exist. 
//! It handles the recursive creation of parent directories, both for 
//! pure directory paths and for paths that point to files.
/**
* make_path.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use std::path::Path;
use std::io;
use std::fs;

/// Creates a directory and all missing parent directories (recursively).
///
/// This function operates in “fire-and-forget” mode and logs success or 
/// errors directly to the console.
///
/// # Arguments
/// * `path` - The path to be created (accepts `&str`, `String`, `Path`, etc.).
///
/// # Usage:
///
/// ```rust
/// fn main() {
///    create_directory("/tmp/atlantis/logs/sys");
/// }
/// ```
pub fn create_directory<P: AsRef<Path>>(path: P) {
    let p = path.as_ref();
    match fs::create_dir_all(p) {
        Ok(_) => println!("[DEBUG] Directory created or already exists: {:?}", p),
        Err(e) => eprintln!("[ERROR] Error when creating the directory {:?}: {}", p, e),
    }
}

/// Creates a path recursively and returns the result of the operation.
///
/// Unlike `create_directory`, there is no automatic logging here; 
/// error handling is the responsibility of the caller.
///
/// # Return value
/// An [io::Result<()>] that confirms the success of the operation or describes the IO error.
///
/// # Usage:
///
/// ```rust
/// fn main() {
///    make_path("/tmp/atlantis/logs/sys");
/// }
/// ```
pub fn make_path<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::create_dir_all(path)
}

/// Extracts the parent directory of a file path and creates it recursively.
///
/// This function is ideal when you have the path to a file (e.g., a config file) 
/// and want to ensure that the folder where the file should be located exists.
///
/// # Arguments
/// * `filepath` - The full path to the file.
///
/// # Usage:
///
/// ```rust
/// fn main() {
///    let target = "/tmp/atlantis/config/settings.toml";
///    if let Err(e) = make_path_dirname(target) {
///        eprintln!("Error: {}", e);
///    } else {
///        println!("Parent-Directory {:?} are ready.", target);
///    }
/// }
/// ```
pub fn make_path_dirname<P: AsRef<Path>>(filepath: P) -> io::Result<()> {
    let path = filepath.as_ref();
    
    // get the path from the full path of the file
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            return fs::create_dir_all(parent);
        }
    }
    
    Ok(())
}
