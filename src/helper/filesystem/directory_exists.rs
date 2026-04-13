/**
 * directory_exists.rs
 *
 * (C) 2026 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * @brief Check if a directory exsists
 *
 * Usage:
 * fn main() {
 *   if directory_exists("/mnt/c/Users") {
 *       println!("Directory exists!");
 *   }
 *   if file_exists("/etc/config.toml") {
 *       println!("File exists!");
 *   }
 * }
 */

use std::path::Path;

/**
 * @brief Check if directory exists
 */
pub fn directory_exists<P: AsRef<Path>>(path: P) -> bool {
    let p = path.as_ref();
    p.exists() && p.is_dir()
}

/**
 * @brief Check if file exsists

 */
pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}
