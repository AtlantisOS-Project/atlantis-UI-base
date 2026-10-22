// temp_app_file.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! A temporary file for the lifetime of the application.
//!
//! The module offers a thread-safe, globally accessible interface for one temporary file
//! that exists only while the application runs. It is handy to pass small amounts of text
//! between the parts of an application without inventing a file name.
//!
//! # How it works
//!
//! * The file is created on first access in the temporary directory of the system
//!   (`$TMPDIR`, usually `/tmp`; Flatpak and Snap give each application a private one) and
//!   only the current user can read it.
//! * A [`OnceLock`] guarantees the thread-safe lazy creation and a [`Mutex`] serializes
//!   the access.
//! * The file has no name: the system deletes it when the last handle is closed, which also
//!   happens when the process is killed. Nothing of the text stays on the disk. (A file with a
//!   name in a `static` would stay, because Rust does not drop statics when a program ends.)
//!
//! # Examples
//!
//! ```
//! use atlbase::prelude::*;
//!
//! write_to_temp("Hello World!").unwrap();
//! write_to_temp("\nSecond Line").unwrap();
//! write_to_temp("\nLast Line").unwrap();
//! assert_eq!(read_from_temp().unwrap(), "Hello World!\nSecond Line\nLast Line");
//!
//! remove_line_from_temp(1).unwrap();
//! assert_eq!(read_from_temp().unwrap(), "Hello World!\nLast Line\n");
//!
//! clear_temp_file().unwrap();
//! assert_eq!(read_from_temp().unwrap(), "");
//! ```

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::sync::{Mutex, MutexGuard, OnceLock};

/// Global storage of the temporary file. It is a file without a name, so the system deletes it
/// by itself, whatever happens to the process.
static TEMP_FILE: OnceLock<Mutex<File>> = OnceLock::new();

/// Locks the temporary file; creates it on first use.
///
/// # Panics
///
/// Panics if the file cannot be created in the temporary directory of the system.
fn lock_temp_file() -> MutexGuard<'static, File> {
    TEMP_FILE
        .get_or_init(|| {
            let file = tempfile::tempfile().expect("temporary file could not be created");
            Mutex::new(file)
        })
        .lock()
        // a panic in another thread cannot leave the file in an inconsistent state
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Appends text to the end of the temporary file.
///
/// The buffer is flushed afterwards so no data is lost.
///
/// # Arguments
///
/// * `data` - The text to append. No line break is added.
///
/// # Errors
///
/// Returns an [`io::Error`] if writing fails.
///
/// # Panics
///
/// Panics if the temporary file cannot be created on first use.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::write_to_temp;
///
/// write_to_temp("Important runtime data").unwrap();
/// ```
pub fn write_to_temp(data: &str) -> io::Result<()> {
    let mut file = lock_temp_file();

    // a previous read or clear may have left the cursor somewhere else
    file.seek(SeekFrom::End(0))?;
    file.write_all(data.as_bytes())?;
    file.flush()
}

/// Reads the entire content of the temporary file.
///
/// # Errors
///
/// Returns an [`io::Error`] if the file cannot be read or does not contain valid UTF-8.
///
/// # Panics
///
/// Panics if the temporary file cannot be created on first use.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::read_from_temp;
///
/// match read_from_temp() {
///     Ok(content) => println!("Content of the file:\n{}", content),
///     Err(e) => eprintln!("Cannot read the file: {}", e),
/// }
/// ```
pub fn read_from_temp() -> io::Result<String> {
    let mut file = lock_temp_file();

    file.seek(SeekFrom::Start(0))?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Deletes the entire content of the temporary file.
///
/// The file itself remains; its size is set to 0 and the cursor returns to the start.
///
/// # Errors
///
/// Returns an [`io::Error`] if the file cannot be truncated.
///
/// # Panics
///
/// Panics if the temporary file cannot be created on first use.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::clear_temp_file;
///
/// clear_temp_file().unwrap();
/// ```
pub fn clear_temp_file() -> io::Result<()> {
    let mut file = lock_temp_file();

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    Ok(())
}

/// Deletes one line of the temporary file.
///
/// A file cannot be shortened in the middle, so the function reads and rewrites the
/// whole file. Avoid it for very large files. The remaining lines are terminated by a
/// line break.
///
/// # Arguments
///
/// * `line_number` - The 0-based index of the line to remove.
///
/// # Errors
///
/// Returns an [`io::Error`] of kind [`InvalidInput`](io::ErrorKind::InvalidInput) if the
/// file has no such line, or any error that occurs while reading or rewriting the file.
///
/// # Panics
///
/// Panics if the temporary file cannot be created on first use.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{clear_temp_file, remove_line_from_temp, write_to_temp};
///
/// clear_temp_file().unwrap();
/// write_to_temp("keep\nremove\n").unwrap();
/// remove_line_from_temp(1).unwrap();
/// ```
pub fn remove_line_from_temp(line_number: usize) -> io::Result<()> {
    let mut file = lock_temp_file();

    file.seek(SeekFrom::Start(0))?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let lines: Vec<&str> = content.lines().collect();
    if line_number >= lines.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Line number outside the range",
        ));
    }

    let mut new_content = String::new();
    for (index, line) in lines.iter().enumerate() {
        if index != line_number {
            new_content.push_str(line);
            new_content.push('\n');
        }
    }

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(new_content.as_bytes())?;
    file.flush()
}
