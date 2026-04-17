//! Function that write a file
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

/// Function that write a file
/// ### Note:
/// - None = Home as Base
/// - Some = define a other base
///
/// ### Usage:
///
/// ```rust
/// fn main {
/// 	let create_file = write_file("rsyslog.conf", &content, Some(PathBuf::from("/")), &["etc"]);
/// }
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
