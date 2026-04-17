//! Function that check a directory exists
/**
 * directory_exists.rs
 *
 * (C) 2026 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 */

use std::path::Path;

/// Check if directory exists
/// ### Usage:
///
/// ```rust
/// fn main() {
///   if directory_exists("/mnt/c/Users") {
///       println!("Directory exists!");
///   }
/// }
/// ```
pub fn directory_exists<P: AsRef<Path>>(path: P) -> bool {
    let p = path.as_ref();
    p.exists() && p.is_dir()
}

/// Check if file exsists
/// ### Usage:
///
/// ```rust
/// Usage:
/// fn main() {
///   if directory_exists("/mnt/c/Users") {
///       println!("Directory exists!");
///   }
///   if file_exists("/etc/config.toml") {
///       println!("File exists!");
///   }
/// }
/// ```
pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}
