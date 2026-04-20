//! Identification of configuration and source files based on their names.
//!
//! This module is primarily used to determine whether specific 
//! repository entries (PPAs) or software sources are stored on the system.
/**
* search_string_name.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use std::fs;
use std::path::Path;

/// Searches a directory for a file whose name contains a specific string.
///
/// This function is used within AtlantisOS to validate the installation source 
/// of the application (e.g., searching for "fastboot-assistant" in the APT sources).
///
/// # How it works
/// 1. First checks whether the target directory exists. If not, `Ok(false)` is returned.
/// 2. Iterates over all entries in the directory.
/// 3. Compares the filename with the search term.
///
/// # Arguments
/// * `directory` - The path to the directory (e.g., `/etc/apt/sources.list.d`).
/// * `search` - The substring to search for in the filename.
///
/// # Return Value
/// * `Ok(true)` - A matching file was found.
/// * `Ok(false)` - No file matches the pattern, or the directory does not exist.
/// * `Err(std::io::Error)` - Access to the directory was denied or failed.
///
/// # Usage:
/// 
/// ```rust
/// match search_string_file(source_dir, search_string) {
///    Ok(true) => AppEnvironment::Ppa,
///    Ok(false) => {
///  	  println!("Running local DEB package.");
///      AppEnvironment::LocalDeb
///    },
///    Err(e) => {
///      eprintln!("Error check for PPA files: {}", e);
///      process::exit(1);
///    }
/// }
/// ```
pub fn search_string_file(directory: &str, search: &str) -> Result<bool, std::io::Error> {
    let path = Path::new(directory);
    if !path.exists() {
        return Ok(false);
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str() {
            if name.contains(search) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
