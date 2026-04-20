//! Utility tools for checking the availability of system commands.
//!
//! This module provides functions to determine whether a specific program 
//! or executable file is installed on the system and accessible via the environment variables.
/**
* command_exists.c
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/


use std::env;
use std::path::Path;

/// Checks whether a command is available on the system.
///
/// The function distinguishes between two scenarios:
/// 1. **Absolute path:** If the passed string is an absolute path, 
///    it checks directly whether the file exists at that location.
/// 2. **Command name:** If only a name (e.g., "ls" or "git") is passed, 
///    the function searches all directories defined in the environment variable 
///    `PATH`.
///
/// # Arguments
/// * `command` - The name of the command or an absolute path to the file.
///
/// # Return Value
/// Returns `true` if the command was found and is an executable file, 
/// otherwise `false`.
///
/// # Usage:
///
/// ```rust
/// if command_exists_native("cd") {
/// 	println!("cd exists");
/// }
/// ```
pub fn command_exists_native(command: &str) -> bool {
    // check if the path is absolute
    if Path::new(command).is_absolute() {
        return Path::new(command).exists();
    }

    // search in the path
    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            let full_path = path.join(command);
            if full_path.is_file() {
                return true;
            }
        }
    }
    false
}
