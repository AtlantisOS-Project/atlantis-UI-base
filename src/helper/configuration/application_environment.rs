/**
* application_environment.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* fn main() {
*	const SEARCH_STRING: &str = "fastboot-assistant";
*	const SOURCES_DIR: &str = "/etc/apt/sources.list.d";
*   let current_env = check_application_environment()    
*   match current_env {
*       AppEnvironment::Flatpak => println!("Aktion für Flatpak"),
*       AppEnvironment::Ppa => println!("Aktion für PPA"),
*       _ => println!("Andere Umgebung"),
*    }
* }
*/

use std::fs;
use std::path::Path;
use std::process;
use std::env;

// define the posible running env
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEnvironment {
    Flatpak,
    Snap,
    Ppa,
    LocalDeb,
    Unknown,
}

// helper that search for a string in a filename in a directory
fn has_ppa_file(directory: &str, search: &str) -> Result<bool, std::io::Error> {
    let path = Path::new(directory);
    if !path.exists() {
        return Ok(false);
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str() {
            if name.contains(search) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/**
* @brief Function that test the execution env
*/
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

/**
* @brief Function that run the execution environment
*/
pub fn check_application_environment(search_string: &str, source_dir: &str) -> AppEnvironment {
    let env_str = get_execution_environment();
    println!("The app runs in the environment: **{}**", env_str);

    match env_str.as_str() {
        "flatpak" => AppEnvironment::Flatpak,
        "snap" => AppEnvironment::Snap,
        _ => {
            // PPA/DEB logic
            println!("Running as PPA/DEB");
            match has_ppa_file(source_dir, search_string) {
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
