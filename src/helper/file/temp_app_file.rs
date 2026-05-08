//! # Runtime Tempfile Manager
//!
//! This module provides a thread-safe, globally accessible interface for 
//! managing a temporary file that exists only for the duration of the 
//! application.
//!
//! ## Architecture
//! The module uses the **RAII principle** (Resource Acquisition Is Initialization). 
//! The file is created upon first access and automatically deleted as soon as 
//! the application terminates normally or aborts due to a panic.
//!
//! ### Core Components:
//! - `OnceLock`: Guarantees thread-safe lazy initialization.
//! - `Mutex`: Enables synchronized access from multiple threads.
//! - `NamedTempFile`: Manages the physical file and its deletion.
//!
//! ## Example
//!
//! ```rust
//! fn main() {
//!    write_to_temp("Hello World!").unwrap();
//!    write_to_temp("\nSecond Line").unwrap();
//!	   write_to_temp("\nLast Line").unwrap();
//!
//!    match read_from_temp() {
//!        Ok(content) => println!("Content of file: \n{}", content),
//!        Err(e) => eprintln!("Error with writing to: {}", e),
//!    }
//!
//!    println!("Before:\n{}", read_from_temp().unwrap());
//!
//!    remove_line_from_temp(1).unwrap();
//!    println!("Delete line 1:\n{}", read_from_temp().unwrap());
//!
//!    clear_temp_file().unwrap();
//!    println!("After Clear: '{}'", read_from_temp().unwrap());
//!    
//!    write_to_temp("Hello World!").unwrap();
//!    println!("The file will be deleted as soon as the program closes.");
//! }
//! ```
/**
* temp_app_file.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/
use std::io::{
	self, 
	Read, 
	Write, 
	Seek, 
	SeekFrom
};
use std::sync::{
	Mutex, 
	OnceLock
};
use tempfile::NamedTempFile;

/// Global storage for the temporary file.
/// The file is automatically deleted when this object is dropped.
static TEMP_FILE: OnceLock<Mutex<NamedTempFile>> = OnceLock::new();

/// Initializes the temporary file or returns a reference to the existing one.
///
/// # Panics
/// This function panics if the temporary file cannot be created in the system temp directory 
///
fn get_temp_file() -> &'static Mutex<NamedTempFile> {
    TEMP_FILE.get_or_init(|| {
        let file = NamedTempFile::new().expect("Critical error: Temporary file could not be created");
        Mutex::new(file)
    })
}

/// Writes data to the temporary file.
///
/// The data is written to the end of the file. After writing,
/// the buffer is flushed to prevent data loss.
///
/// # Arguments
/// * `data` - A string slice to be written to the file.
///
/// # Errors
/// Returns a `std::io::Result::Err` if the write operation fails.
///
/// # Example
/// ```rust
/// write_to_temp("Important runtime data").unwrap();
/// ```
pub fn write_to_temp(data: &str) -> io::Result<()> {
    let mutex = get_temp_file();
    let mut guard = mutex.lock().unwrap();
    
    guard.as_file_mut().write_all(data.as_bytes())?;
    guard.as_file_mut().flush()?;
    Ok(())
}

/// Reads the entire contents of the temporary file.
///
/// This function sets the file pointer to the beginning (position 0) before reading.
///
/// # Returns
/// An `io::Result<String>` containing the complete contents of the file.
///
/// # Errors
/// Returns an error if the file cannot be read or 
/// contains invalid UTF-8.
///
/// # Example
///
/// ```rust
/// match read_from_temp() {
///        Ok(content) => println!("Content of file: \n{}", content),
///        Err(e) => eprintln!("Error with writing to: {}", e),
/// }
/// ```
pub fn read_from_temp() -> io::Result<String> {
    let mutex = get_temp_file();
    let mut guard = mutex.lock().unwrap();
    
    guard.as_file_mut().seek(SeekFrom::Start(0))?;
    let mut content = String::new();
    guard.as_file_mut().read_to_string(&mut content)?;
    Ok(content)
}

/// Deletes the entire contents of the temporary file.
///
/// The file remains in existence and reserved, but its size is set to 0
/// and the file pointer is reset to the beginning.
///
/// # Errors
/// Returns an error if the file cannot be truncated.
///
/// # Example
///
/// ```rust
/// clear_temp_file().unwrap();
/// ```
pub fn clear_temp_file() -> io::Result<()> {
    let mutex = get_temp_file();
    let guard = mutex.lock().unwrap();
    
    // set the file size to 0
    guard.as_file().set_len(0)?;
    Ok(())
}

/// Deletes a specific line from the temporary file based on the line number.
///
/// **Performance Note:** Since files in the file system cannot simply be 
/// "truncated" in the middle, this function must read and rewrite the entire file.
/// For very large files, this function should be used sparingly.
///
/// # Arguments
/// * `line_number` - The 0-based index number of the line to be removed.
///
/// # Errors
/// Returns an error if the file cannot be read or rewritten.
///
/// # Example
///
/// ```rust
/// remove_line_from_temp(1).unwrap();
/// ```
pub fn remove_line_from_temp(line_number: usize) -> io::Result<()> {
    let mutex = get_temp_file();
    let mut guard = mutex.lock().unwrap();

    // read content
    guard.as_file_mut().seek(SeekFrom::Start(0))?;
    let mut content = String::new();
    guard.as_file_mut().read_to_string(&mut content)?;

    // filter the lines
    let lines: Vec<&str> = content.lines().collect();
    if line_number >= lines.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Line number outside the range"));
    }

    let mut new_content = String::new();
    for (i, line) in lines.iter().enumerate() {
        if i != line_number {
            new_content.push_str(line);
            new_content.push('\n');
        }
    }

    // remove everything from the file and rewrite the content
    guard.as_file().set_len(0)?;
    guard.as_file_mut().seek(SeekFrom::Start(0))?;
    guard.as_file_mut().write_all(new_content.as_bytes())?;
    guard.as_file_mut().flush()?;

    Ok(())
}
