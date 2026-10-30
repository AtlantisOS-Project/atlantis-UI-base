// guarded_delete.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Deleting folders with a safety check.
//!
//! A program that cleans up "its" working folder must never delete the wrong one because a
//! setting was edited by hand. These functions delete only after the name of the folder was
//! checked ([`require_dir_name`]). A folder that does not exist is never an error, so
//! cleaning up twice works. A system folder, the home directory and its standard folders are
//! refused as well, whatever their name and however they are reached (`..`, symbolic links): the
//! error is `PermissionDenied` and nothing is deleted.

use super::delete_files::refuse_protected;
use std::fs;
use std::io;
use std::path::Path;

/// Checks that the last part of `dir` is called `name`.
///
/// # Errors
///
/// Returns an [`io::ErrorKind::PermissionDenied`] error if the folder has a different name.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::require_dir_name;
/// use std::path::Path;
///
/// assert!(require_dir_name(Path::new("/home/me/ROM-Install"), "ROM-Install").is_ok());
/// assert!(require_dir_name(Path::new("/home/me/Documents"), "ROM-Install").is_err());
/// ```
pub fn require_dir_name(dir: &Path, name: &str) -> io::Result<()> {
    if dir.file_name().is_some_and(|found| found == name) {
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::PermissionDenied,
        format!("refusing to touch {}: not a {name} folder", dir.display()),
    ))
}

/// Deletes `dir` with everything in it. A folder that does not exist is fine.
///
/// # Errors
///
/// Returns the [`io::Error`] if something cannot be deleted, and `PermissionDenied` if `dir` is a
/// protected folder (see the [module documentation](self)).
///
/// # Examples
///
/// ```
/// use atlbase::prelude::remove_dir_all_if_exists;
///
/// let dir = tempfile::tempdir().unwrap();
/// let cache = dir.path().join("cache");
/// std::fs::create_dir_all(cache.join("sub")).unwrap();
/// std::fs::write(cache.join("sub/file"), "x").unwrap();
///
/// remove_dir_all_if_exists(&cache).unwrap();
/// assert!(!cache.exists());
/// // a folder that is not there is fine
/// remove_dir_all_if_exists(&cache).unwrap();
/// ```
pub fn remove_dir_all_if_exists(dir: &Path) -> io::Result<()> {
    refuse_protected(dir)?;
    match fs::remove_dir_all(dir) {
        Err(error) if error.kind() != io::ErrorKind::NotFound => Err(error),
        _ => Ok(()),
    }
}

/// Deletes everything in `dir`, but not the entries called in `keep`, and not `dir` itself.
/// A folder that does not exist is fine.
///
/// # Errors
///
/// Returns the [`io::Error`] if something cannot be deleted, and `PermissionDenied` if `dir` is a
/// protected folder (see the [module documentation](self)).
///
/// # Examples
///
/// ```
/// use atlbase::prelude::clear_directory_except;
///
/// let dir = tempfile::tempdir().unwrap();
/// std::fs::write(dir.path().join("trash.txt"), "x").unwrap();
/// std::fs::create_dir(dir.path().join("Backup")).unwrap();
///
/// clear_directory_except(dir.path(), &["Backup"]).unwrap();
/// assert!(!dir.path().join("trash.txt").exists());
/// assert!(dir.path().join("Backup").is_dir());
/// ```
pub fn clear_directory_except(dir: &Path, keep: &[&str]) -> io::Result<()> {
    refuse_protected(dir)?;
    let entries = match fs::read_dir(dir) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        other => other?,
    };
    for entry in entries {
        let path = entry?.path();
        if path
            .file_name()
            .is_some_and(|name| keep.iter().any(|kept| name == *kept))
        {
            continue;
        }
        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }
    Ok(())
}
