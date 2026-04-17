//! Creates a complete path by creating missing parent directories
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

/// Create missing parent directories
/// ### Usage:
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

/// Create a path and returns the success or not
/// ### Usage:
///
/// ```rust
/// fn main() {
///    make_path("/tmp/atlantis/logs/sys");
/// }
/// ```
pub fn make_path<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::create_dir_all(path)
}

/// Create the parent directory from a file
/// ### Usage:
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
