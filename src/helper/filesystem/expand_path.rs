//! Resolves shell tilde paths to absolute system paths.
//!
//! This module provides the functionality to replace the `~` symbol at the beginning of a path 
//! with the actual path of the user's home directory ($HOME).
/**
 * expand_path.rs
 *
 * (C) Copyright 2026 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 */

use std::env;
use std::path::{Path, PathBuf};

/// Replaces a leading tilde (`~`) with the path from the `$HOME` environment variable.
///
/// If the path does not start with a tilde or the `$HOME` environment variable 
/// is not set, the original path is returned unchanged.
///
/// # How it works
/// 1. Checks if the path starts with `~`.
/// 2. Reads `$HOME` from the operating system's environment variables.
/// 3. Removes the tilde and concatenates the rest of the path with the home directory.
///
/// # Arguments
/// * `path` - The path to be resolved (accepts `&str`, `String`, `Path`, etc.).
///
/// # Return Value
/// A [PathBuf] containing the absolute path (if successful) or the original path.
///
/// # Usage:
///
/// ```rust
/// fn main() {
///   let local_conf = "~/config/settings.toml";
///   let expanded = expand_path(local_conf);  
///   println!("Original: {}", local_conf);
///   println!("Expanded: {:?}", expanded);
/// }
/// ```
pub fn expand_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let p = path.as_ref();

    // check if the path starts with '~'
    if !p.starts_with("~") {
        return p.to_path_buf();
    }

    // get the $HOME from the env
    match env::var_os("HOME") {
        None => {
            // if $HOME not set use the path with ~
            p.to_path_buf()
        }
        Some(home) => {
            // remove the ~ and use the rest of the path with everything $HOME
            if let Ok(suffix) = p.strip_prefix("~") {
                let mut full_path = PathBuf::from(home);
                full_path.push(suffix);
                full_path
            } 
            
            else {
                p.to_path_buf()
            }
        }
    }
}
