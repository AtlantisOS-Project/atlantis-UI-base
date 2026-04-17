//! Function to get the $HOME path
/**
* home_dir.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use std::env;
use std::path::{PathBuf};
use std::process;

/// Function that get the path to $HOME or quit with error 
/// #### Usage:
///
/// ```rust
/// fn main() {
///    let home = get_home_directory();
///    println!("Home: {:?}", home);
/// }
/// ```
pub fn get_home_directory() -> PathBuf {
    match env::var_os("HOME") {
        Some(path) => PathBuf::from(path),
        None => {
            eprintln!("[ERROR] Could not determine the home directory.");
            process::exit(1);
        }
    }
}
