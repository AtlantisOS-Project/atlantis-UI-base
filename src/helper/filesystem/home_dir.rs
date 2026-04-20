//! Centralized determination of the user's home directory.
//!
//! This module ensures that the path to the home directory ($HOME) 
//! is available. 
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

/// Determines the path to the current user's home directory.
///
/// The function reads the `$HOME` environment variable. If this variable is not 
/// set, an error message is displayed and the program terminates, 
/// as safe operation cannot be guaranteed without this basic information.
///
/// # Return Value
/// Returns a [PathBuf] containing the absolute path to the home directory.
///
/// # Error Handling
/// If `$HOME` is not defined:
/// - Output an error message with the prefix `[ERROR]` to `stderr`.
/// - Terminate the process with exit code `1`.
///
/// # Usage:
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
