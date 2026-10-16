// get_config_value.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Reads values from traditional configuration files.
//!
//! The module parses files in the `KEY=VALUE` or `KEY="VALUE"` format (for example
//! `/etc/os-release`). It tolerates surrounding whitespace and rejects paths that contain
//! `..`.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Searches a configuration file for a key and returns its value.
///
/// Simple values and values in double quotes are supported; the quotes are removed. Lines
/// that are not valid UTF-8 are skipped. The first matching line wins.
///
/// # Security
///
/// Paths with a `..` component are rejected (path traversal protection), so a path that
/// is assembled from user input cannot leave the intended directory.
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

    for line in BufReader::new(file).lines() {
        let Ok(line) = line else { continue };
        let Some(rest) = line.trim().strip_prefix(key) else {
            continue;
        };
        // the key must be followed by `=` (optionally after spaces), not by more letters
        let Some(value) = rest.trim_start().strip_prefix('=') else {
            continue;
        };

        let mut value = value.trim();
        if let Some(unquoted) = value.strip_prefix('"') {
            value = unquoted;
        }
        if let Some(end) = value.find('"') {
            value = &value[..end];
        }
        return Some(value.to_string());
    }

    None
}
