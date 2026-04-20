//! Advanced deletion utilities with recursive parent cleanup.
//!
//! This module provides functions for cleaning up directories. A key 
//! feature is the ability to delete directory hierarchies “from the bottom up,” 
//! as long as the directories are empty.
//!
//! # Safety Notes
//! The functions include strict safeguards to prevent the deletion of:
//! - System-critical paths (`/bin`, `/etc`, etc.)
//! - Mount points (partitions/network drives)
//! - Standard user directories (Pictures, Documents, etc.)
//! Nevertheless, this module should only be used for application-specific data 
//!
/**
* delete_files.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

/// List of paths that must never be deleted recursively.
const CRITICAL_PATHS: &[&str] = &[
    "/", "/bin", "/sbin", "/usr", "/etc", "/var", "/proc", 
    "/sys", "/dev", "/lib", "/boot", "/root"
];

/// Standard folders in the home directory that are protected.
const STANDARD_HOME_FOLDERS: &[&str] = &[
    "Desktop", "Documents", "Downloads", "Music", 
    "Pictures", "Videos", "Public", "Templates", 
    ".config", ".local"
];

/// Checks whether a path is a mount point.
/// Compares the device ID of the path with that of the parent directory.
fn is_mount_point(path: &Path) -> bool {
    let parent = match path.parent() {
        Some(p) => p,
        None => return false,
    };

    let meta_path = fs::metadata(path);
    let meta_parent = fs::metadata(parent);

    if let (Ok(mp), Ok(mpr)) = (meta_path, meta_parent) {
        if mp.dev() != mpr.dev() {
            println!("[WARN] Blocked: Mount Point detected: {:?}", path);
            return true;
        }
    }
    false
}

/// Checks whether the path belongs to a protected system directory.
fn is_critical_system_path(path: &Path) -> bool {
    if let Some(path_str) = path.to_str() {
        for prefix in CRITICAL_PATHS {
            if path_str == *prefix || path_str.starts_with(&format!("{}/", prefix)) {
                println!("[ERROR] Blocked: Critical System Path detected: {}", path_str);
                return true;
            }
        }
    }
    false
}

/// Checks whether the path is a standard user folder to prevent data loss.
fn is_standard_home_directory(path: &Path, home_dir: &Option<PathBuf>) -> bool {
    let home = match home_dir {
        Some(h) => h,
        None => return false,
    };

    if path == home {
        println!("[ERROR] Blocked: Attempt to delete User Home: {:?}", path);
        return true;
    }

    for folder in STANDARD_HOME_FOLDERS {
        if path == home.join(folder) {
            println!("[ERROR] Blocked: Standard Home Folder detected: {:?}", path);
            return true;
        }
    }
    false
}

/// Deletes all regular files within a directory.
///
/// Subdirectories remain untouched.
///
/// # Usage:
/// 
/// ```rust
/// fn main {
/// let test_file = "/tmp/test_file_path/test.txt";
/// 	delete_files_in_dir(test_file);
/// }
pub fn delete_files_in_dir(path: &Path) {
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => {
            println!("[ERROR] Error opening the directory: {:?}", path);
            return;
        }
    };

    for entry in entries.flatten() {
        let file_path = entry.path();
        if file_path.is_file() {
            if let Err(e) = fs::remove_file(&file_path) {
                println!("[ERROR] Error deleting file {:?}: {}", file_path, e);
            } 
            
            else {
                println!("[INFO] File deleted: {:?}", file_path);
            }
        }
    }
}

/// Core logic for recursively deleting files and empty parent directories.
///
/// Works its way up the directory tree until a directory is no longer empty 
/// or a safety mechanism (stop directory, system path) is triggered.
///
/// # Usage:
///
/// ```rust
/// fn main() {
///    let base_path = "temp_data/projects/old_project/cache";
///	   let file_path = format!("{}/temp_log.txt", base_path);
///    delete_files::delete_files_with_parents(base_path);
/// }
pub fn delete_files_and_parents(path: &Path, stop_dir: &Option<PathBuf>) {
    if let Some(stop) = stop_dir {
        if path == stop { 
        	return; 
        }
    }

    // remove the file
    delete_files_in_dir(path);

    // check if the parent dir is empty
    let is_empty = fs::read_dir(path).map(|mut i| i.next().is_none()).unwrap_or(false);

    if is_empty {
        // security check
        if is_mount_point(path) || is_critical_system_path(path) || is_standard_home_directory(path, stop_dir) {
            println!("[WARN] Stopping recursion: Path is protected: {:?}", path);
            return;
        }

        // remove the directory
        if let Err(e) = fs::remove_dir(path) {
            println!("[ERROR] Error deleting empty directory {:?}: {}", path, e);
        } 
        
        else {
            println!("[INFO] Directory deleted: {:?}", path);

            // remove the parent dirs
            if let Some(parent) = path.parent() {
                if parent != Path::new("/") {
                    delete_files_and_parents(parent, stop_dir);
                }
            }
        }
    } 
    
    else {
        println!("[INFO] Stopping recursion: Directory {:?} is not empty.", path);
    }
}

/// Convenience wrapper for deleting a path, including empty parent directories.
///
/// Initializes security prompts and sets the home directory as the default stopping point.
///
/// # Warning
/// This function deletes the entire path hierarchy upward, provided there are no 
/// other files within it. Never apply to other users' directories!
///
/// # Usage:
///
/// ```rust
/// fn main() {
///    // Deletes the file in 'cache' and removes 'cache', 'old_project', and 'projects' 
///    // if doing so leaves them empty.
///    let base_path = "temp_data/projects/old_project/cache";
///	   let file_path = format!("{}/temp_log.txt", base_path);
///    delete_files::delete_files_with_parent(base_path);
/// }
pub fn delete_files_with_parent(path_str: &str) {
    println!("[WARN] Warning: This code deletes an entire path until it finds a stop point.");
    
    let path = PathBuf::from(path_str);
    // get the real path
    let clean_path = fs::canonicalize(&path).unwrap_or(path);
    
    // get the home directory
    let home_dir = std::env::var_os("HOME").map(PathBuf::from);
	
	// security check
    if is_critical_system_path(&clean_path) {
        println!("[ERROR] Aborted: Starting path is critical: {:?}", clean_path);
        return;
    }
	
	// delete the file
    delete_files_and_parents(&clean_path, &home_dir);
}
