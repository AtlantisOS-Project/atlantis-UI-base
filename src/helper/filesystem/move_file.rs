// move_file.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Moving a file, also to another file system.

use super::make_path::is_plain_name;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Moves `source` into the directory `target_dir` and gives it the name `new_name`.
///
/// The directory is created if it does not exist. A rename is tried first; if the file is on
/// another file system (a USB stick, another partition) it is copied and then deleted. An
/// existing file with the same name is replaced.
///
/// # Returns
///
/// The path of the file after the move.
///
/// # Errors
///
/// Returns the [`io::Error`] if the file cannot be moved. The source is kept in that case.
/// `new_name` must be one plain name (no separator, not `..`): otherwise the error is of kind
/// `InvalidInput` and nothing happens.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::move_file_into;
///
/// let dir = tempfile::tempdir().unwrap();
/// let source = dir.path().join("download.img");
/// std::fs::write(&source, b"image").unwrap();
///
/// let target = move_file_into(&source, &dir.path().join("work"), "boot.img").unwrap();
/// assert_eq!(target, dir.path().join("work/boot.img"));
/// assert!(!source.exists());
/// ```
pub fn move_file_into(source: &Path, target_dir: &Path, new_name: &str) -> io::Result<PathBuf> {
    if !is_plain_name(new_name) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("the new name must be one plain name, not a path: {new_name:?}"),
        ));
    }
    fs::create_dir_all(target_dir)?;
    let target = target_dir.join(new_name);

    if source == target {
        return Ok(target);
    }
    match fs::rename(source, &target) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::CrossesDevices => {
            fs::copy(source, &target)?;
            fs::remove_file(source)?;
        }
        Err(error) => return Err(error),
    }
    Ok(target)
}
