// make_path.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Creates directory structures.
//!
//! The module makes sure that paths exist. Missing parent directories are created
//! recursively, for paths of directories as well as for paths of files.

use std::fs;
use std::io;
use std::path::{Component, Path};

/// Whether `name` is one plain name: not empty, no separator, not `.` or `..`, not absolute. A name
/// from outside that passes this check cannot leave the folder it is put into.
pub(crate) fn is_plain_name(name: &str) -> bool {
    let mut components = Path::new(name).components();
    matches!(components.next(), Some(Component::Normal(_))) && components.next().is_none()
}

/// Whether `path` is a relative path of plain names (`a/b`): no `..`, no `.`, not absolute.
pub(crate) fn is_plain_relative(path: &str) -> bool {
    Path::new(path).components().next().is_some()
        && Path::new(path)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

/// Creates a directory and all missing parent directories (recursively).
///
/// The function works in "fire and forget" mode: success and errors are only logged to the
/// console. Use [`make_path`] if you have to react to a failure.
///
/// # Arguments
///
/// * `path` - The directory to create (`&str`, `String`, `Path`, …).
///
/// # Examples
///
/// ```
/// use atlbase::prelude::create_directory;
///
/// let root = tempfile::tempdir().unwrap();
/// let logs = root.path().join("atlantis/logs/sys");
///
/// create_directory(&logs);
/// assert!(logs.is_dir());
/// ```
pub fn create_directory<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    match make_path(path) {
        Ok(_) => println!("[DEBUG] Directory created or already exists: {:?}", path),
        Err(e) => eprintln!(
            "[ERROR] Error when creating the directory {:?}: {}",
            path, e
        ),
    }
}

/// Creates a directory and all missing parents and returns the result.
///
/// Unlike [`create_directory`] nothing is logged; the caller handles errors.
///
/// # Errors
///
/// Returns the [`io::Error`] of [`fs::create_dir_all`], for example if the permissions
/// are missing or a file is in the way.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::make_path;
///
/// let root = tempfile::tempdir().unwrap();
/// make_path(root.path().join("atlantis/logs/sys")).unwrap();
/// assert!(root.path().join("atlantis/logs/sys").is_dir());
/// ```
pub fn make_path<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::create_dir_all(path)
}

/// Creates the parent directory of a file path.
///
/// Ideal when you have the path of a file (for example a configuration file) and want to
/// make sure the directory that will contain it exists. The file itself is not created.
/// A path without a directory part (`"settings.toml"`) needs nothing and succeeds.
///
/// # Arguments
///
/// * `filepath` - The complete path of the file.
///
/// # Errors
///
/// Returns the [`io::Error`] of [`fs::create_dir_all`].
///
/// # Examples
///
/// ```
/// use atlbase::prelude::make_path_dirname;
///
/// let root = tempfile::tempdir().unwrap();
/// let target = root.path().join("atlantis/config/settings.toml");
///
/// make_path_dirname(&target).unwrap();
/// assert!(target.parent().unwrap().is_dir());
/// assert!(!target.exists());
/// ```
pub fn make_path_dirname<P: AsRef<Path>>(filepath: P) -> io::Result<()> {
    match filepath.as_ref().parent() {
        Some(parent) if !parent.as_os_str().is_empty() => fs::create_dir_all(parent),
        _ => Ok(()),
    }
}
