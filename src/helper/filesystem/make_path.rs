/**
* make_path.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
*
* @brief Creates a complete path by creating missing parent directories
*
* Usage:
* fn main() {
*    let target = "/tmp/atlantis/config/settings.toml";
*    if let Err(e) = make_path_dirname(target) {
*        eprintln!("Fehler: {}", e);
*    } else {
*        println!("Parent-Verzeichnisse für {:?} sind bereit.", target);
*    }
*    create_directory("/tmp/atlantis/logs/sys");
* }
*
* Notes: 
* @param path: The path to be created.
* @return 0 on success, -1 on error.
*/

use std::path::Path;
use std::io;
use std::fs;

/**
* @brief Create missing parent directories
*/
pub fn create_directory<P: AsRef<Path>>(path: P) {
    let p = path.as_ref();
    match fs::create_dir_all(p) {
        Ok(_) => println!("[DEBUG] Directory created or already exists: {:?}", p),
        Err(e) => eprintln!("[ERROR] Error when creating the directory {:?}: {}", p, e),
    }
}

/**
* @brief Create a path and returns the success or not
*/
pub fn make_path<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::create_dir_all(path)
}

/**
* @brief Create the parent directory from a file
*/
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
