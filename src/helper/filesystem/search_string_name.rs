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

/// Helper that search for a string in a filename in a directory
/// ### Usage:
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
