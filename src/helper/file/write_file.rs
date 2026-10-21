// write_file.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Writes files and creates the directory structure above them.
//!
//! The path is assembled from a base directory (home directory or any other directory) and
//! a list of subdirectories.

use crate::helper::filesystem::home_dir::home_directory;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Writes `content` to a file and creates all missing directories above it.
///
/// 1. **Base path:** `base_path`, or the home directory of the user if it is `None`.
/// 2. **Path:** the segments of `path_list` and finally `filename` are appended.
/// 3. **Directories:** missing directories are created.
/// 4. **Write:** the content is written; an existing file is overwritten.
///
/// # Arguments
///
/// * `filename` - The name of the file.
/// * `content` - The text to store.
/// * `base_path` - The start of the path; `None` means the home directory.
/// * `path_list` - The subdirectories below the base path, for example
///   `&["Documents", "Projects"]`.
///
/// # Errors
///
/// Returns an [`io::Error`] if the home directory cannot be determined, a directory cannot
/// be created or the file cannot be written.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::write_file;
///
/// let root = tempfile::tempdir().unwrap();
///
/// // creates <root>/etc/atlantis/config.toml
/// write_file(
///     "config.toml",
///     "# Config",
///     Some(root.path().to_path_buf()),
///     &["etc", "atlantis"],
/// )
/// .unwrap();
///
/// let file = root.path().join("etc/atlantis/config.toml");
/// assert_eq!(std::fs::read_to_string(file).unwrap(), "# Config");
/// ```
///
/// Without a base path the file lands in the home directory, here `~/Atlantis/logs/session.log`:
///
/// ```no_run
/// # use atlbase::prelude::write_file;
/// write_file("session.log", "Log content", None, &["Atlantis", "logs"]).unwrap();
/// ```
pub fn write_file(
    filename: &str,
    content: &str,
    base_path: Option<PathBuf>,
    path_list: &[&str],
) -> io::Result<()> {
    let mut full_path = match base_path {
        Some(path) => path,
        None => home_directory()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?,
    };

    full_path.extend(path_list);
    full_path.push(filename);

    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&full_path, content)
}
