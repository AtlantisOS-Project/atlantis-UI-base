// command_exists.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Checks whether a command is installed.
//!
//! This module determines whether a program is installed and reachable: through the folders
//! of the application and `PATH` (see [`find_program`]).

use crate::helper::platform::program::find_program;

/// Checks whether a command is available on the system.
///
/// The function distinguishes between two cases:
///
/// 1. **Absolute path:** the file at that location must exist and be executable.
/// 2. **Command name:** (for example `ls` or `git`) the folders registered with
///    [`register_tool_dir`](crate::helper::platform::program::register_tool_dir) and every
///    directory listed in `PATH` are searched for an executable file with that name (on
///    Windows with the ending `.exe`, `.cmd`, … added).
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
    find_program(command).is_some()
}
