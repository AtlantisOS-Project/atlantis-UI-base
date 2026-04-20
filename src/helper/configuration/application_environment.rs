//! Identification of the application runtime environment.
//!
//! This module detects whether the application is running as an isolated package (Flatpak, Snap) 
//! or as a native system package (PPA, local DEB). This is 
//! essential for correctly adjusting paths and permissions at runtime.
/**
* application_environment.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use std::path::Path;
use std::process;
use std::env;
use crate::helper::filesystem::search_string_name::search_string_file;

/// Defines the possible runtime environments for the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEnvironment {
    /// Runs within a Flatpak sandbox.
    Flatpak,
    /// Runs as a Snap package.
    Snap,
    /// Installation via a registered PPA (Personal Package Archive).
    Ppa,
    /// Installation as a manually installed, local DEB package.
    LocalDeb,
    /// Unknown or unidentifiable environment.
    Unknown,
}

/// Retrieves the name of the current runtime environment as a string.
///
/// This function performs simple file system and environment variable checks
/// to distinguish between the most common sandbox formats.
///
/// # Return value
/// Returns "flatpak", "snap", or "native".
///
/// ```rust
/// let env_str = get_execution_environment();
/// println!("The app runs in the environment: **{}**", env_str);
/// ``` 
pub fn get_execution_environment() -> String {
    // check for flatpak
    if Path::new("/.flatpak-info").exists() {
        return "flatpak".to_string()
    }

    // check for snap
    if env::var("SNAP").is_ok() {
        return "snap".to_string()
    }
	
	// return as native file
    return "native".to_string()
}

/// Analyzes the application's detailed installation source.
///
/// If the application runs natively, this function searches the APT source lists
/// to distinguish between a PPA source and a locally installed package.
///
/// # Arguments
/// * `search_string` - The identifier of the application in the sources lists (e.g., “fastboot-assistant”).
/// * `source_dir` - The directory of the APT lists (usually `/etc/apt/sources.list.d`).
///
/// # Return Value
/// Returns an instance of [AppEnvironment].
///
/// # Error Handling
/// If access to the system directories fails, an error message
/// is printed and the process terminates with status `1`, since the environment is critical
/// for the program's subsequent execution.
///
/// # Usage:
///
/// ```
/// fn main() {
///	  const SEARCH_STRING: &str = "fastboot-assistant";
///	  const SOURCES_DIR: &str = "/etc/apt/sources.list.d";
///   let current_env = check_application_environment(SEARCH_STRING, SOURCES_DIR);    
///   match current_env {
///       AppEnvironment::Flatpak => println!("Action for Flatpak"),
///       AppEnvironment::Ppa => println!("Action for PPA"),
///       _ => println!("Other enviroment"),
///    }
/// }
pub fn check_application_environment(search_string: &str, source_dir: &str) -> AppEnvironment {
    let env_str = get_execution_environment();
    println!("The app runs in the environment: **{}**", env_str);

    match env_str.as_str() {
        "flatpak" => AppEnvironment::Flatpak,
        "snap" => AppEnvironment::Snap,
        _ => {
            // PPA/DEB logic
            println!("Running as PPA/DEB");
            match search_string_file(source_dir, search_string) {
                Ok(true) => AppEnvironment::Ppa,
                Ok(false) => {
                    println!("Running local DEB package.");
                    AppEnvironment::LocalDeb
                },
                Err(e) => {
                    eprintln!("Error check for PPA files: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}
