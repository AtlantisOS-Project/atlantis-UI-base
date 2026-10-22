// delete_files.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Deletes files and cleans up the empty directories above them.
//!
//! The main feature is deleting a directory hierarchy "from the bottom up": the files of
//! one directory are removed, then the directory and each parent directory is removed as
//! long as it is empty.
//!
//! # Safety
//!
//! The functions refuse to touch:
//!
//! * system-critical paths (`/`, `/bin`, `/etc`, `/usr`, `/var`, … and everything below
//!   them; `C:\Windows` and `C:\Program Files` on Windows, see
//!   [`OsProfile::critical_paths`]),
//! * important top-level directories (`/home`, `/tmp`, `/opt`, `/mnt`, `/media`, `/run`,
//!   `/srv`, `/app`, `/snap`) — the directories themselves, not their contents,
//! * the home directory and its standard folders (`Documents`, `Pictures`, `.config`, …),
//! * mount points (they are never removed).
//!
//! Even so, use them only for data that belongs to your application.

use crate::helper::filesystem::home_dir::home_directory;
use crate::helper::platform::dirs::is_mount_point;
use crate::helper::platform::os::{OsProfile, os_profile};
use std::fs;
use std::path::{Path, PathBuf};

/// Checks whether a path is a mount point by comparing its device ID with the parent's.
fn is_mount_point_blocked(path: &Path) -> bool {
    let blocked = is_mount_point(path);
    if blocked {
        println!("[WARN] Blocked: Mount Point detected: {:?}", path);
    }
    blocked
}

/// Checks whether `path` belongs to a protected system directory of the system of `profile`.
fn is_critical_for(profile: &OsProfile, path: &Path) -> bool {
    // a root ("/", "C:\") is protected, but not everything is "below" it
    (path.parent().is_none() && !path.as_os_str().is_empty())
        || profile
            .critical_paths
            .iter()
            .any(|critical| path.starts_with(critical))
        || profile
            .protected_dirs
            .iter()
            .any(|dir| path == Path::new(dir))
}

/// Checks whether the path belongs to a protected system directory.
fn is_critical_system_path(path: &Path) -> bool {
    let critical = is_critical_for(os_profile(), path);
    if critical {
        println!(
            "[ERROR] Blocked: Critical System Path detected: {}",
            path.display()
        );
    }
    critical
}

/// Checks whether the path is the home directory or one of its standard folders.
fn is_standard_home_directory(path: &Path, home_dir: &Option<PathBuf>) -> bool {
    let Some(home) = home_dir else {
        return false;
    };

    if path == home {
        println!("[ERROR] Blocked: Attempt to delete User Home: {:?}", path);
        return true;
    }

    let standard = os_profile()
        .standard_home_folders
        .iter()
        .any(|folder| path == home.join(folder));
    if standard {
        println!("[ERROR] Blocked: Standard Home Folder detected: {:?}", path);
    }
    standard
}

/// Deletes all regular files in a directory.
///
/// Subdirectories and their contents remain untouched; the directory itself is kept.
/// Protected directories (see the [module documentation](self)) are refused: a message is
/// printed and nothing is deleted. Errors are printed as well; the function does not
/// return them.
///
/// # Arguments
///
/// * `path` - The directory whose files are deleted.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::delete_files_in_dir;
///
/// let dir = tempfile::tempdir().unwrap();
/// std::fs::write(dir.path().join("a.txt"), "a").unwrap();
/// std::fs::create_dir(dir.path().join("sub")).unwrap();
///
/// delete_files_in_dir(dir.path());
///
/// assert!(!dir.path().join("a.txt").exists());
/// assert!(dir.path().join("sub").is_dir());
/// ```
pub fn delete_files_in_dir(path: &Path) {
    if is_critical_system_path(path) || is_standard_home_directory(path, &home_directory()) {
        return;
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => {
            println!("[ERROR] Error opening the directory: {:?}", path);
            return;
        }
    };

    for entry in entries.flatten() {
        let file_path = entry.path();
        if file_path.is_file() {
            match fs::remove_file(&file_path) {
                Ok(()) => println!("[INFO] File deleted: {:?}", file_path),
                Err(e) => println!("[ERROR] Error deleting file {:?}: {}", file_path, e),
            }
        }
    }
}

/// Deletes the files of a directory and then removes it and its empty parents.
///
/// 1. The regular files of `path` are deleted (see [`delete_files_in_dir`]).
/// 2. If `path` is now empty, it is removed. The same is repeated for each parent directory
///    until a directory is **not empty**, is `stop_dir`, or is protected (see the
///    [module documentation](self)). The files of the parent directories are never touched.
///
/// # Arguments
///
/// * `path` - The directory to clean up.
/// * `stop_dir` - The directory where the climb stops. It is never removed and also protects
///   its standard folders (`Documents`, …). Usually the home directory.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::delete_files_and_parents;
/// use std::path::PathBuf;
///
/// let root = tempfile::tempdir().unwrap();
/// let cache = root.path().join("projects/old_project/cache");
/// std::fs::create_dir_all(&cache).unwrap();
/// std::fs::write(cache.join("temp_log.txt"), "log").unwrap();
/// // a file in a parent directory keeps that directory alive
/// std::fs::write(root.path().join("projects/keep.txt"), "keep").unwrap();
///
/// delete_files_and_parents(&cache, &Some(PathBuf::from(root.path())));
///
/// assert!(!root.path().join("projects/old_project").exists());
/// assert!(root.path().join("projects/keep.txt").is_file());
/// ```
pub fn delete_files_and_parents(path: &Path, stop_dir: &Option<PathBuf>) {
    if stop_dir.as_deref() == Some(path) {
        return;
    }

    delete_files_in_dir(path);

    let mut current = path.to_path_buf();
    loop {
        if stop_dir.as_deref() == Some(current.as_path()) {
            return;
        }

        let is_empty = fs::read_dir(&current)
            .map(|mut entries| entries.next().is_none())
            .unwrap_or(false);
        if !is_empty {
            println!(
                "[INFO] Stopping recursion: Directory {:?} is not empty.",
                current
            );
            return;
        }

        if is_mount_point_blocked(&current)
            || is_critical_system_path(&current)
            || is_standard_home_directory(&current, stop_dir)
        {
            println!(
                "[WARN] Stopping recursion: Path is protected: {:?}",
                current
            );
            return;
        }

        if let Err(e) = fs::remove_dir(&current) {
            println!(
                "[ERROR] Error deleting empty directory {:?}: {}",
                current, e
            );
            return;
        }
        println!("[INFO] Directory deleted: {:?}", current);

        match current.parent() {
            Some(parent) if parent != Path::new("/") && !parent.as_os_str().is_empty() => {
                current = parent.to_path_buf();
            }
            _ => return,
        }
    }
}

/// Deletes the files of a directory and removes it and its empty parents.
///
/// Convenience wrapper for [`delete_files_and_parents`]: the path is resolved first
/// (symbolic links, `..`), a path that does not exist is ignored, and the home directory
/// (`$HOME`) is the stop point.
///
/// # Warning
///
/// The function removes the whole hierarchy upwards as long as the directories are empty.
/// Never use it on directories of other users or on data that does not belong to your
/// application.
///
/// # Arguments
///
/// * `path_str` - The directory to clean up.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::delete_files_with_parent;
///
/// let root = tempfile::tempdir().unwrap();
/// let cache = root.path().join("projects/old_project/cache");
/// std::fs::create_dir_all(&cache).unwrap();
/// std::fs::write(cache.join("temp_log.txt"), "log").unwrap();
/// // `projects` also holds another project, so the climb stops there
/// std::fs::create_dir(root.path().join("projects/other")).unwrap();
///
/// delete_files_with_parent(cache.to_str().unwrap());
///
/// assert!(!root.path().join("projects/old_project").exists());
/// assert!(root.path().join("projects/other").is_dir());
/// ```
pub fn delete_files_with_parent(path_str: &str) {
    println!("[WARN] Warning: This code deletes an entire path until it finds a stop point.");

    let clean_path = match fs::canonicalize(path_str) {
        Ok(path) => path,
        Err(e) => {
            println!("[ERROR] Cannot resolve {}: {}", path_str, e);
            return;
        }
    };

    if is_critical_system_path(&clean_path) {
        println!(
            "[ERROR] Aborted: Starting path is critical: {:?}",
            clean_path
        );
        return;
    }

    delete_files_and_parents(&clean_path, &home_directory());
}
