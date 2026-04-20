//! Search for filenames within a directory.
//!
//! This module searches the filenames in a specified directory for 
//! a substring. It is useful for checking whether files exist 
//! whose full names are not known exactly.
/**
* search_file_directory.c
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use std::fs;
use std::path::Path;

/// Checks whether a search term appears in the name of a file within a directory.
///
/// The function scans the directory flat (not recursively) and compares 
/// each filename with the passed `search_string`.
///
/// # Arguments
/// * `directory` - The path to the directory to be searched (accepts `&str`, `PathBuf`, etc.).
/// * `search_string` - The string to search for in the filename.
///
/// # Return Value
/// * `Ok(true)` - At least one file contains the search term in its name.
/// * `Ok(false)` - No file matches the search criteria.
/// * `Err(std::io::Error)` - Error accessing the directory.
///
/// # Usage:
/// 
/// ```rust
/// fn main() {
///    let sources_dir = "./src";
///    let search_string = "helper";
///    match search_file_directory(sources_dir, search_string) {
///        Ok(true) => println!("File!"),
///        Ok(false) => println!("No file."),
///        Err(e) => eprintln!("Error: {}", e),
///    }
/// }
/// ```
 
pub fn search_file_directory<P: AsRef<Path>>(directory: P, search_string: &str) -> Result<bool, std::io::Error> {
    let entries = fs::read_dir(directory)?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        
        // checks whether the filename contains the search string
        // convert OsString to a normal string for comparison
        if let Some(name_str) = file_name.to_str() {
            if name_str.contains(search_string) {
                return Ok(true);
            }
        }
    }

    Ok(false)
}
