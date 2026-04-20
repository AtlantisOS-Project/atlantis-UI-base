//! A universal tool for writing files and creating directory structures.
//!
//! This module simplifies data storage by dynamically constructing paths from 
//! a base directory (Home or Root) and a list of subdirectories.
/**
* write_file.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use directories::UserDirs;
use std::fs;
use std::path::PathBuf;

/// Writes content to a file and creates the entire directory structure if necessary.
///
/// This function is highly flexible: It can operate either in the user's home directory 
/// or at any specified location (e.g., in the root filesystem).
///
/// # How it works
/// 1. **Determine base path:** If `base_path` is `None`, the home directory is used.
/// 2. **Path construction:** The segments from `path_list` are appended to the base path.
/// 3. **Directory creation:** Any missing subdirectories are automatically created.
/// 4. **Write file:** The content is finally written to the file (overwriting existing data).
///
/// # Arguments
/// * `filename` - The name of the file to be created.
/// * `content` - The text content to be saved.
/// * `base_path` - Optional starting point. `None` corresponds to the home directory (`~/`).
/// * `path_list` - An array of subdirectories (e.g., `&["Documents", "Projects"]`).
///
/// # Error Handling
/// Returns a `std::io::Result`. Errors occur if the home directory 
/// cannot be determined or if write permissions are missing in the destination directory.
///
/// # Usage:
///
/// ```rust
/// use std::path::PathBuf;
///
/// // Create ~/Atlantis/logs/session.log
/// let _ = write_file(
///     "session.log", 
///     "Log-Content...", 
///     None, 
///     &["Atlantis", "logs"]
/// );
///
/// // Create /etc/atlantis/config.toml 
/// let _ = write_file(
///     "config.toml", 
///     "# Config", 
///     Some(PathBuf::from("/")), 
///     &["etc", "atlantis"]
/// );
/// ```
pub fn write_file(filename: &str, content: &str, base_path: Option<PathBuf>, path_list: &[&str]) -> std::io::Result<()> {
	// create base path
	let mut full_path = match base_path {
		Some(path) => path,
		None => {
			let user_dirs = UserDirs::new()
				.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found"))?;
			PathBuf::from(user_dirs.home_dir())
		}
	};
	
	// add subdirs
	for direc in path_list {
		full_path.push(direc);
	}
	
	// add filename
	full_path.push(filename);
	
	// create path structure
	if let Some(parent) = full_path.parent() {
		fs::create_dir_all(parent)?;
	}
	
	// write file
	fs::write(&full_path, content)?;
	
	Ok(())
}
