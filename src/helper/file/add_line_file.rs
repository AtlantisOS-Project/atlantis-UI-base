// add_line_file.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Appends lines to text files.
//!
//! The module adds a line to the end of a file, including a duplicate check and an
//! optional backup.

use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Adds a line to the end of a file if the file does not contain it yet.
///
/// Useful for configuration files in which an entry must not appear twice.
///
/// 1. **Duplicate check:** if a line of the file equals `line` (ignoring surrounding
///    whitespace), nothing is changed.
/// 2. **Backup:** if `create_backup` is `true` and the file already exists, a copy named
///    `<path>.bak` is created once. An existing backup is never overwritten.
/// 3. **Write:** the line is appended. A missing file is created, and a missing line break
///    at the end of the file is added first, so the new line is never glued to the old one.
///
/// # Arguments
///
/// * `create_backup` - Whether to create a `.bak` copy of the original file.
/// * `path` - The path of the file.
/// * `line` - The text to add, without line break.
///
/// # Errors
///
/// Returns an [`io::Error`] if the file cannot be read, copied or written. Finding the
/// line already present is not an error.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::add_line_file;
///
/// let dir = tempfile::tempdir().unwrap();
/// let file = dir.path().join("hosts");
/// let file = file.to_str().unwrap();
///
/// add_line_file(false, file, "127.0.0.1 localhost").unwrap();
/// add_line_file(false, file, "127.0.0.1 localhost").unwrap(); // already there
/// add_line_file(false, file, "10.0.0.1 server").unwrap();
///
/// let content = std::fs::read_to_string(file).unwrap();
/// assert_eq!(content, "127.0.0.1 localhost\n10.0.0.1 server\n");
/// ```
pub fn add_line_file(create_backup: bool, path: &str, line: &str) -> io::Result<()> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(e),
    };

    if content
        .lines()
        .any(|existing| existing.trim() == line.trim())
    {
        println!("Entry already exists in {}", path);
        return Ok(());
    }

    if create_backup && Path::new(path).exists() {
        let backup_path = format!("{}.bak", path);
        if !Path::new(&backup_path).exists() {
            fs::copy(path, &backup_path)?;
            println!("Created backup: {}", backup_path);
        }
    }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    if !content.is_empty() && !content.ends_with('\n') {
        writeln!(file)?;
    }
    writeln!(file, "{}", line)?;
    println!("Entry added to {}", path);

    Ok(())
}
