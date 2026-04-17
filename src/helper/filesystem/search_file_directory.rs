//! Search if a predefined string exsists in the file of predefined directory
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

/// Checks whether a string appears in the name of a file within a directory
/// ### Notes:
/// - Ok(true): File exists
/// - Ok(false): File not exists
/// - Err(e): Error with reading the file
///
/// ### Usage:
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
