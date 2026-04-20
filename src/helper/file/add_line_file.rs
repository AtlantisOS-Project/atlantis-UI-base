//! Utility for safely appending to text files.
//!
//! This module allows you to add a line to the end of a file, including 
//! duplicate checking and optional backup creation.
/**
* add_line_file.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use std::fs;
use std::io::{Read, Write};
use std::path::Path;

/// Adds a specific line to the end of a file if it does not yet exist.
///
/// This function is particularly useful for configuration files where 
/// entries must not appear more than once.
///
/// # How it works
/// 1. **Existence check:** If the file does not exist, it is created with the line.
/// 2. **Duplicate check:** The file is read. If the line (trimmed) already 
///    exists, the operation is aborted to avoid redundancy.
/// 3. **Backup:** If `create_backup` is true, a `.bak` copy is created.
/// 4. **Write:** The line is appended to the file in append mode.
///
/// # Arguments
/// * `create_backup` - Whether to create a backup copy of the original file.
/// * `path` - The path to the destination file.
/// * `line` - The text content to be added.
///
/// # Usage:
///
/// ```rust
/// fn main() {
///    let file_path = "test.txt";
///    let new_line = "Some content";
///    let create_backup = true;
///    match add_line_file(&create_backup, file_path, new_line) {
///        Ok(_) => println!("Success!"),
///        Err(e) => eprintln!("Some errors: {}", e),
///    }
/// }
/// ```
pub fn add_line_file(create_backup: &bool, path: &str, line: &str) -> std::io::Result<()> {
	if !Path::new(&path).exists() {
		// create the file if it not exists
		fs::write(path, line)?;
	}
		
	let mut content = String::new();
	fs::File::open(path)?.read_to_string(&mut content)?;
	
	// check if the content is already in the file
	if content.lines().any(|l| l.trim() == line) {
		println!("Entry already exists in {}", path);
		return Ok(());
	}
	
	// backup of the original file
	if *create_backup {
		let backup_path = format!("{}.bak", path);
        if !Path::new(&backup_path).exists() {
            fs::copy(path, &backup_path)?;
            println!("Created backup: {}", backup_path);
        }
    }
	
	// write line to file
    let mut file = fs::OpenOptions::new().append(true).open(path)?;
    writeln!(file, "{}", line)?;
    println!("Entry added to {}", path);

    Ok(())
}
