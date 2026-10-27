// get_config_value.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Reads values from traditional configuration files.
//!
//! The module parses files in the `KEY=VALUE`, `KEY="VALUE"` or `KEY='VALUE'` format (for
//! example `/etc/os-release`). It tolerates surrounding whitespace and rejects paths that
//! contain `..`.

use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};
use std::path::Path;

/// Searches a configuration file for a key and returns its value.
///
/// Simple values and values in double or single quotes are supported; the quotes are removed.
/// Lines that are not valid UTF-8 are skipped. The first matching line wins.
///
/// # Security
///
/// Paths with a `..` component are rejected, so a path that is put together from a name cannot
/// step out of its folder that way. That is **all** the check does: an absolute path is read as
/// it is, and so is a symbolic link. A program that takes the path from a user has to decide
/// itself which files may be read.
///
/// # Arguments
///
/// * `filename` - The path of the configuration file.
/// * `key` - The name of the variable, for example `"HOSTNAME"`.
///
/// # Returns
///
/// `Some(value)` if the key was found, otherwise `None` (also if the file cannot be opened;
/// the reason is printed to `stderr`).
///
/// # Examples
///
/// ```
/// use atlbase::prelude::get_config_value;
///
/// let dir = tempfile::tempdir().unwrap();
/// let file = dir.path().join("update.conf");
/// std::fs::write(&file, "UPDATE_TYPE=\"daily\"\nPORT = 8080\n").unwrap();
///
/// assert_eq!(get_config_value(&file, "UPDATE_TYPE").as_deref(), Some("daily"));
/// assert_eq!(get_config_value(&file, "PORT").as_deref(), Some("8080"));
/// assert_eq!(get_config_value(&file, "MISSING"), None);
/// ```
pub fn get_config_value<P: AsRef<Path>>(filename: P, key: &str) -> Option<String> {
    let path = filename.as_ref();

    if path.components().any(|c| c.as_os_str() == "..") {
        eprintln!("Error: Path traversal detected!");
        return None;
    }

    let file = File::open(path)
        .map_err(|e| eprintln!("Could not open {:?}: {}", path, e))
        .ok()?;

    // a line that is not valid UTF-8 is skipped; any other read error ends the search
    BufReader::new(file)
        .lines()
        .map_while(|line| match line {
            Ok(line) => Some(Some(line)),
            Err(error) if error.kind() == ErrorKind::InvalidData => Some(None),
            Err(_) => None,
        })
        .flatten()
        .find_map(|line| config_line_value(&line, key))
}

/// The value of `key` in one line `KEY=VALUE` or `KEY="VALUE"`; the parser behind
/// [`get_config_value`], also used for text that is already in memory (`/etc/os-release`).
pub(crate) fn config_line_value(line: &str, key: &str) -> Option<String> {
    let rest = line.trim().strip_prefix(key)?;

    // the key must be followed by `=` (optionally after spaces), not by more letters
    let mut value = rest.trim_start().strip_prefix('=')?.trim();
    if let Some(quoted) = value.strip_prefix('\'') {
        return Some(quoted.split('\'').next().unwrap_or_default().to_string());
    }
    if let Some(unquoted) = value.strip_prefix('"') {
        value = unquoted;
    }
    if let Some(end) = value.find('"') {
        value = &value[..end];
    }
    Some(value.to_string())
}
