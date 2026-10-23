// home_dir.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Determines the home directory of the user.
//!
//! The module makes sure that the path of the home directory is available. The lookup itself
//! is [`home_dir`] of the platform layer.

use crate::helper::platform::dirs::home_dir;
use std::path::PathBuf;

/// Returns the home directory of the current user, if there is one.
///
/// The function reads the `$HOME` environment variable and, if that is not set, asks the
/// system (`USERPROFILE` on Windows). In a Snap, `$HOME` is the private data directory of the
/// snap, not the real home directory of the user.
///
/// # Returns
///
/// `None` if no home directory can be found (a service without a user, an empty environment).
/// The process is not ended: the caller decides what to do. It is the same as [`home_dir`] of the
/// platform layer.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::get_home_directory;
///
/// let home = get_home_directory().expect("a home directory");
/// println!("Home: {:?}", home);
/// ```
pub fn get_home_directory() -> Option<PathBuf> {
    home_dir()
}

/// The home directory, if there is one: the one place of the library that asks for it (the
/// functions that need the home directory call this instead of reading the variable
/// themselves).
pub(crate) fn home_directory() -> Option<PathBuf> {
    home_dir()
}
