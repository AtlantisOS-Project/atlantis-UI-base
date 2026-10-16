// home_dir.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Determines the home directory of the user.
//!
//! The module makes sure that the path of the home directory (`$HOME`) is available.

use std::env;
use std::path::PathBuf;
use std::process;

/// Returns the home directory of the current user.
///
/// The function reads the `$HOME` environment variable. In a Snap, `$HOME` is the private
/// data directory of the snap, not the real home directory of the user.
///
/// # Process exit
///
/// If `$HOME` is not set, an error with the prefix `[ERROR]` is printed to `stderr` and
/// the process is terminated with status `1`, because the application cannot work
/// correctly without it.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::get_home_directory;
///
/// let home = get_home_directory();
/// println!("Home: {:?}", home);
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
