//! Management of TOML configuration files for AtlantisOS applications.
//!
//! This module uses the `confy` and `serde` crates to 
//! persistently store and read simple key-value pairs 
//! in the user's default configuration directory (e.g., `~/.config/`).
/**
* toml_config.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Data structure for AtlantisOS configurations.
///
/// Uses a [HashMap] to flexibly store various configuration keys 
/// and their values as strings.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AtlConfigs {
	/// Internal storage for key-value pairs.
    data: HashMap<String, String>,
}

/// Reads a value from an application's configuration file.
///
/// Loads the configuration file based on the application name. If the file 
/// does not exist, a default empty structure is used.
///
/// # Arguments
/// * `app_name` - The name of the application (determines the file name/directory).
/// * `key` - The name of the configuration key being searched for.
///
/// # Return Value
/// Returns `Some(String)` if the key exists, otherwise `None`.
///
/// # Usage:
///
/// ```rust
/// fn main() {
///    match get_config("atl_test", "theme") {
///        Some(val) => println!("Found: {}", val),
///        None => println!("Key not found!"),
///    }
/// }
/// ``` 
pub fn get_config(app_name: &str, key: &str) -> Option<String> {
    let cfg: AtlConfigs = confy::load(app_name, None).unwrap_or_default();
    cfg.data.get(key).cloned()
}

/// Saves or updates a value in the configuration file.
///
/// First loads the current state of the file, adds the new key-value pair 
/// or overwrites an existing one, and then writes the entire structure 
/// back to the hard drive.
///
/// # Arguments
/// * `app_name` - The name of the application (determines the storage location at `~/.config/app_name`).
/// * `key` - The identifier for the setting.
/// * `value` - The value to be saved.
///
/// # Error Handling
/// Returns a [confy::ConfyError] if file permissions are missing or 
/// serialization fails.
///
/// # Usage:
///
/// ```rust
/// fn main() {
///    if let Err(e) = set_config("atl_test", "theme", "dark") {
///        eprintln!("Error at saving: {}", e);
///    }
///    set_config_val("atl_test", "port", "8080").unwrap();
///    let port: u16 = get_config_val("port")
///        .and_then(|v| v.parse().ok())
///        .unwrap_or(3000); // default value
///	 println!("Port: {}", port);
/// }
/// ```
pub fn set_config(app_name: &str, key: &str, value: &str) -> Result<(), confy::ConfyError> {
    // load existing configuration or create new 
    let mut cfg: AtlConfigs = confy::load(app_name, None).unwrap_or_default();
    
    // insert the values and update the file
    cfg.data.insert(key.to_string(), value.to_string());
    
    // write the file
    confy::store(app_name, None, cfg)
}
