// command_exists.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Checks whether a command is installed.
//!
//! This module determines whether a program is installed and reachable through the
//! `PATH` environment variable.

use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// Returns `true` if `path` is a regular file with at least one execute bit set.
fn is_executable_file(path: &Path) -> bool {
    path.metadata()
        .map(|meta| meta.is_file() && meta.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

/// Checks whether a command is available on the system.
///
/// The function distinguishes between two cases:
///
/// 1. **Absolute path:** the file at that location must exist and be executable.
/// 2. **Command name:** (for example `ls` or `git`) every directory listed in `PATH` is
///    searched for an executable file with that name.
///
/// # Arguments
///
/// * `command` - The name of the command or an absolute path to the file.
///
/// # Returns
///
/// `true` if an executable file was found, otherwise `false`.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::command_exists_native;
///
/// assert!(command_exists_native("sh"));
/// assert!(!command_exists_native("this-command-does-not-exist"));
/// ```
pub fn command_exists_native(command: &str) -> bool {
    let path = Path::new(command);
    if path.is_absolute() {
        return is_executable_file(path);
    }

    env::var_os("PATH")
        .map(|paths| env::split_paths(&paths).any(|dir| is_executable_file(&dir.join(command))))
        .unwrap_or(false)
}
