//! Functions to get the execution enviroment
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

/// Define the posible running enviroment
/// ### Note:
/// - Typedefinition of the return type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEnvironment {
    Flatpak,
    Snap,
    Ppa,
    LocalDeb,
    Unknown,
}

/// Function that test the execution enviroment
/// ### Usage:
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

/// ## Function that run the execution environment
/// ### Usage:
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
