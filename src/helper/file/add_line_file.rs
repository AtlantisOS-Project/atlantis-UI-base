//! Function to add a line at the end of a file
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

/// Function that add a special line at the end of a file
/// ### Usage:
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
