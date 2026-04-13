/**
* command_exists.c
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* if (command_exists("cd") != 0) {}
*/

use std::env;
use std::path::Path;

/**
* @brief Check if command exsists
*/
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
