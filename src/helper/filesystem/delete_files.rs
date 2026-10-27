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
//! The path is resolved first (`..` and symbolic links), so `/tmp/../etc` and a link to `/etc` are
//! `/etc` for the check. On Windows upper and lower case do not matter.
//!
//! Even so, use them only for data that belongs to your application.

use crate::helper::filesystem::home_dir::home_directory;
use crate::helper::platform::dirs::is_mount_point;
use crate::helper::platform::os::{OsProfile, os_profile};
use crate::helper::platform::target::Os;
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};

/// Checks whether a path is a mount point by comparing its device ID with the parent's.
fn is_mount_point_blocked(path: &Path) -> bool {
    let blocked = is_mount_point(path);
    if blocked {
        println!("[WARN] Blocked: Mount Point detected: {:?}", path);
    }
    blocked
}

/// The path as the system sees it, with links and `..` resolved. Without that, `/tmp/../etc`
/// and a link to `/etc` would pass for harmless paths. A path that does not exist (yet) cannot be
/// resolved by the system and is cleaned of `.` and `..` by its words.
fn resolve(path: &Path) -> PathBuf {
    if let Ok(real) = fs::canonicalize(path) {
        return real;
    }
    let mut clean = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => match clean.components().next_back() {
                Some(Component::Normal(_)) => {
                    clean.pop();
                }
                Some(Component::RootDir | Component::Prefix(_)) => {}
                _ => clean.push(".."),
            },
            other => clean.push(other.as_os_str()),
        }
    }
    clean
}

/// Whether `path` is `base` or below it. On Windows upper and lower case do not matter
/// (`C:\WINDOWS` is `C:\Windows`).
fn is_below(profile: &OsProfile, path: &Path, base: &Path) -> bool {
    if profile.os != Os::Windows {
        return path.starts_with(base);
    }
    let words = |path: &Path| -> Vec<String> {
        path.components()
            .map(|word| word.as_os_str().to_string_lossy().to_lowercase())
            .collect()
    };
    let (path, base) = (words(path), words(base));
    path.len() >= base.len() && path[..base.len()] == base[..]
}

/// Checks whether `path` belongs to a protected system directory of the system of `profile`.
/// `path` must be resolved already (see [`resolve`]).
fn is_critical_for(profile: &OsProfile, path: &Path) -> bool {
    // a root ("/", "C:\\") is protected, but not everything is "below" it
    (path.parent().is_none() && !path.as_os_str().is_empty())
        || profile
            .critical_paths
            .iter()
            .any(|critical| is_below(profile, path, Path::new(critical)))
        || profile.protected_dirs.iter().any(|dir| {
            is_below(profile, path, Path::new(dir))
                && path.components().count() == Path::new(dir).components().count()
        })
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

    // a standard folder may itself be a link to another place (a relocated `Documents`)
    let standard = os_profile().standard_home_folders.iter().any(|folder| {
        let candidate = home.join(folder);
        path == candidate || fs::canonicalize(&candidate).is_ok_and(|real| real == path)
    });
    if standard {
        println!("[ERROR] Blocked: Standard Home Folder detected: {:?}", path);
    }
    standard
}

/// The home directory, resolved (a home that is reached through a link is still the home).
fn resolved_home() -> Option<PathBuf> {
    home_directory().map(|home| resolve(&home))
}

/// Refuses a folder that must not be deleted from: a system path, the home directory and its
/// standard folders. For the helpers that delete a whole folder; they call it before they touch
/// anything.
///
/// # Errors
///
/// `PermissionDenied` with the path in the message.
pub(crate) fn refuse_protected(path: &Path) -> io::Result<()> {
    let path = resolve(path);
    if is_critical_system_path(&path) || is_standard_home_directory(&path, &resolved_home()) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!(
                "refusing to delete from a protected path: {}",
                path.display()
            ),
        ));
    }
    Ok(())
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
    let path = &resolve(path);
    if is_critical_system_path(path) || is_standard_home_directory(path, &resolved_home()) {
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
    let path = &resolve(path);
    let stop_dir = &stop_dir.as_deref().map(resolve);
    if stop_dir.as_deref() == Some(path.as_path()) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::platform::os::os_profile_for;

    fn linux() -> &'static OsProfile {
        os_profile_for(Os::Linux).unwrap()
    }

    /// Only looks: nothing is deleted.
    #[cfg(unix)]
    #[test]
    fn dots_and_links_do_not_hide_a_system_path() {
        assert!(is_critical_for(linux(), &resolve(Path::new("/etc"))));
        // the path as it is written looks harmless
        assert!(!is_critical_for(linux(), Path::new("/tmp/../etc")));
        assert!(is_critical_for(linux(), &resolve(Path::new("/tmp/../etc"))));
        assert!(is_critical_for(
            linux(),
            &resolve(Path::new("/usr/./lib/../bin"))
        ));
        // also when the path does not exist: `..` is resolved by its words
        assert_eq!(
            resolve(Path::new("/home/nobody/../../etc/none")),
            Path::new("/etc/none")
        );
        assert_eq!(resolve(Path::new("/../..")), Path::new("/"));
        assert_eq!(resolve(Path::new("../x/./y")), Path::new("../x/y"));

        // a link to a system path is that system path
        let dir = tempfile::tempdir().unwrap();
        let link = dir.path().join("link");
        std::os::unix::fs::symlink("/usr", &link).unwrap();
        assert!(is_critical_for(linux(), &resolve(&link)));
        assert!(is_critical_for(linux(), &resolve(&link.join("share"))));

        assert!(!is_critical_for(linux(), &resolve(dir.path())));
    }

    #[test]
    fn upper_and_lower_case_are_the_same_on_windows_only() {
        let windows = os_profile_for(Os::Windows).unwrap();
        assert!(is_below(
            windows,
            Path::new("c:/windows/System32"),
            Path::new("C:/Windows")
        ));
        assert!(is_below(
            windows,
            Path::new("C:/WINDOWS"),
            Path::new("C:/Windows")
        ));
        assert!(!is_below(
            windows,
            Path::new("C:/WindowsApps"),
            Path::new("C:/Windows")
        ));
        assert!(!is_below(linux(), Path::new("/ETC"), Path::new("/etc")));
        assert!(is_below(
            linux(),
            Path::new("/etc/hosts"),
            Path::new("/etc")
        ));
    }
}
