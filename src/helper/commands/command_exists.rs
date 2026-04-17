//! Check if a command exists
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

/// Command exists
/// ### Usage:
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
