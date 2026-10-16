// expand_path.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Resolves shell tilde paths.
//!
//! The module replaces a leading `~` by the home directory (`$HOME`).

use std::env;
use std::path::{Path, PathBuf};

/// Replaces a leading `~` with the content of the `$HOME` environment variable.
///
/// The path is returned unchanged if it does not start with a `~` component (so `~user`
/// and `a/~/b` are not touched) or if `$HOME` is not set.
///
/// # Arguments
///
/// * `path` - The path to resolve (`&str`, `String`, `Path`, …).
///
/// # Examples
///
/// ```
/// use atlbase::prelude::expand_path;
/// use std::path::PathBuf;
///
/// // paths without a tilde stay as they are
/// assert_eq!(expand_path("/etc/hosts"), PathBuf::from("/etc/hosts"));
///
/// // `~/config` becomes `$HOME/config`
/// if let Some(home) = std::env::var_os("HOME") {
///     assert_eq!(expand_path("~/config"), PathBuf::from(home).join("config"));
/// }
/// ```
pub fn expand_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();

    match (path.strip_prefix("~"), env::var_os("HOME")) {
        (Ok(suffix), Some(home)) => PathBuf::from(home).join(suffix),
        _ => path.to_path_buf(),
    }
}
